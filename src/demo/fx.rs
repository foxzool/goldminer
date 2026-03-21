use std::time::Duration;

use bevy::prelude::*;

use crate::AppSystems;
use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (sync_fx_position_system, animate_fx_system)
            .chain()
            .in_set(AppSystems::Update)
            .run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component, Clone, Debug)]
pub struct FXAnimation {
    timer: Timer,
    frame_count: usize,
    current_frame: usize,
    playback: FXPlayback,
    placement: FXPlacement,
    z_layer: f32,
}

impl FXAnimation {
    pub fn new(
        frame_count: usize,
        frame_duration: f32,
        playback: FXPlayback,
        placement: FXPlacement,
    ) -> Self {
        assert!(frame_count > 0, "FXAnimation 至少需要一帧");
        assert!(frame_duration > 0.0, "FXAnimation 帧时长必须大于 0");

        Self {
            timer: Timer::from_seconds(frame_duration, TimerMode::Repeating),
            frame_count,
            current_frame: 0,
            playback,
            placement,
            z_layer: 0.0,
        }
    }

    pub fn with_z_layer(mut self, z_layer: f32) -> Self {
        self.z_layer = z_layer;
        self
    }

    pub fn follow_target(&self) -> Option<Entity> {
        match self.placement {
            FXPlacement::Fixed(_) => None,
            FXPlacement::Follow { entity, .. } => Some(entity),
        }
    }

    pub fn translation_for(&self, target_translation: Option<Vec3>) -> Option<Vec3> {
        match self.placement {
            FXPlacement::Fixed(position) => Some(position.extend(self.z_layer)),
            FXPlacement::Follow { offset, .. } => target_translation
                .map(|translation| (translation.truncate() + offset).extend(self.z_layer)),
        }
    }

    pub fn tick(&mut self, delta: Duration) -> FXTickResult {
        self.timer.tick(delta);

        if !self.timer.just_finished() {
            return FXTickResult::default();
        }

        let next_frame = self.current_frame + 1;

        if next_frame >= self.frame_count {
            match self.playback {
                FXPlayback::Loop => {
                    self.current_frame = 0;
                    FXTickResult {
                        atlas_index: Some(self.current_frame),
                        should_despawn: false,
                    }
                }
                FXPlayback::Once => FXTickResult {
                    atlas_index: None,
                    should_despawn: true,
                },
            }
        } else {
            self.current_frame = next_frame;
            FXTickResult {
                atlas_index: Some(self.current_frame),
                should_despawn: false,
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FXPlayback {
    Loop,
    Once,
}

#[derive(Clone, Copy, Debug)]
pub enum FXPlacement {
    Fixed(Vec2),
    Follow { entity: Entity, offset: Vec2 },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FXTickResult {
    pub atlas_index: Option<usize>,
    pub should_despawn: bool,
}

fn sync_fx_position_system(
    mut commands: Commands,
    mut q_fx: Query<(Entity, &FXAnimation, &mut Transform)>,
    q_targets: Query<&GlobalTransform>,
) {
    for (entity, fx, mut transform) in q_fx.iter_mut() {
        let target_translation = fx
            .follow_target()
            .and_then(|target| q_targets.get(target).ok().map(GlobalTransform::translation));

        match fx.translation_for(target_translation) {
            Some(translation) => transform.translation = translation,
            None if fx.follow_target().is_some() => {
                commands.entity(entity).despawn();
            }
            None => {}
        }
    }
}

fn animate_fx_system(
    mut commands: Commands,
    time: Res<Time>,
    mut q_fx: Query<(Entity, &mut FXAnimation, &mut Sprite)>,
) {
    for (entity, mut fx, mut sprite) in q_fx.iter_mut() {
        let update = fx.tick(time.delta());

        if let Some(index) = update.atlas_index
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = index;
        }

        if update.should_despawn {
            commands.entity(entity).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::prelude::*;

    use super::{FXAnimation, FXPlacement, FXPlayback};

    #[test]
    fn once_animation_advances_then_requests_despawn() {
        let mut animation = FXAnimation::new(
            3,
            0.1,
            FXPlayback::Once,
            FXPlacement::Fixed(Vec2::new(4.0, 8.0)),
        );

        let first = animation.tick(Duration::from_secs_f32(0.1));
        assert_eq!(first.atlas_index, Some(1));
        assert!(!first.should_despawn);

        let second = animation.tick(Duration::from_secs_f32(0.1));
        assert_eq!(second.atlas_index, Some(2));
        assert!(!second.should_despawn);

        let third = animation.tick(Duration::from_secs_f32(0.1));
        assert_eq!(third.atlas_index, None);
        assert!(third.should_despawn);
    }

    #[test]
    fn looping_animation_wraps_back_to_first_frame() {
        let mut animation =
            FXAnimation::new(3, 0.1, FXPlayback::Loop, FXPlacement::Fixed(Vec2::ZERO));

        assert_eq!(
            animation.tick(Duration::from_secs_f32(0.1)).atlas_index,
            Some(1)
        );
        assert_eq!(
            animation.tick(Duration::from_secs_f32(0.1)).atlas_index,
            Some(2)
        );

        let looped = animation.tick(Duration::from_secs_f32(0.1));
        assert_eq!(looped.atlas_index, Some(0));
        assert!(!looped.should_despawn);
    }

    #[test]
    fn follow_placement_offsets_target_translation() {
        let target = Entity::from_bits(7);
        let animation = FXAnimation::new(
            8,
            0.05,
            FXPlayback::Loop,
            FXPlacement::Follow {
                entity: target,
                offset: Vec2::new(3.0, -2.0),
            },
        )
        .with_z_layer(11.0);

        assert_eq!(animation.follow_target(), Some(target));
        assert_eq!(
            animation.translation_for(Some(Vec3::new(10.0, 20.0, 1.0))),
            Some(Vec3::new(13.0, 18.0, 11.0))
        );
    }

    #[test]
    fn fixed_placement_uses_configured_position_and_z() {
        let animation = FXAnimation::new(
            8,
            0.05,
            FXPlayback::Once,
            FXPlacement::Fixed(Vec2::new(-6.0, 12.0)),
        )
        .with_z_layer(9.0);

        assert_eq!(animation.follow_target(), None);
        assert_eq!(
            animation.translation_for(None),
            Some(Vec3::new(-6.0, 12.0, 9.0))
        );
    }
}
