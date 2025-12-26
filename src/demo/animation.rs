//! Player sprite animation for the miner character.
//! Implements animation states that sync with the hook mechanism.

use bevy::prelude::*;
use std::time::Duration;

use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_animation_timer.in_set(AppSystems::TickTimers),
            update_animation_atlas.in_set(AppSystems::Update),
        )
            .in_set(PausableSystems),
    );
}

/// Update the animation timer.
fn update_animation_timer(time: Res<Time>, mut query: Query<&mut PlayerAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// Update the texture atlas to reflect changes in the animation.
fn update_animation_atlas(mut query: Query<(&PlayerAnimation, &mut Sprite)>) {
    for (animation, mut sprite) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        if animation.changed() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

/// 矿工动画状态枚举
/// 与 Lua 版本对齐:
/// - Idle: 待机 (帧 0)
/// - Grab: 抓取中 (帧 2)
/// - GrabBack: 回收中 (帧 0,1,2 循环)
/// - UseDynamite: 使用炸药 (帧 3,4,5 循环)
/// - Strengthen: 力量增强 (帧 6,7,6,7 循环)
#[derive(Reflect, PartialEq, Clone, Copy, Default)]
pub enum PlayerAnimationState {
    #[default]
    Idle,
    Grab,
    GrabBack,
    UseDynamite,
    Strengthen,
}

/// Component that tracks player's animation state.
/// 对应 miner_sheet.png (8x1 帧，每帧 32x40 像素)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerAnimation {
    timer: Timer,
    frame: usize,
    state: PlayerAnimationState,
    /// 标记动画是否在本帧发生变化（用于强制刷新）
    state_changed: bool,
}

impl PlayerAnimation {
    // --- 帧序列定义 (0-indexed) ---
    const IDLE_FRAMES: &'static [usize] = &[0];
    const GRAB_FRAMES: &'static [usize] = &[2];
    const GRAB_BACK_FRAMES: &'static [usize] = &[0, 1, 2];
    const USE_DYNAMITE_FRAMES: &'static [usize] = &[3, 4, 5];
    const STRENGTHEN_FRAMES: &'static [usize] = &[6, 7, 6, 7];

    // --- 时间间隔 ---
    const IDLE_INTERVAL: Duration = Duration::from_millis(1000);
    const GRAB_INTERVAL: Duration = Duration::from_millis(1000);
    const GRAB_BACK_INTERVAL: Duration = Duration::from_millis(130);
    const USE_DYNAMITE_INTERVAL: Duration = Duration::from_millis(130);
    const STRENGTHEN_INTERVAL: Duration = Duration::from_millis(130);

    pub fn new() -> Self {
        Self {
            timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Idle,
            state_changed: true, // 初始化时强制刷新
        }
    }

    /// 获取当前状态对应的帧序列
    fn get_frames(&self) -> &'static [usize] {
        match self.state {
            PlayerAnimationState::Idle => Self::IDLE_FRAMES,
            PlayerAnimationState::Grab => Self::GRAB_FRAMES,
            PlayerAnimationState::GrabBack => Self::GRAB_BACK_FRAMES,
            PlayerAnimationState::UseDynamite => Self::USE_DYNAMITE_FRAMES,
            PlayerAnimationState::Strengthen => Self::STRENGTHEN_FRAMES,
        }
    }

    /// 获取当前状态对应的帧间隔
    fn get_interval(&self) -> Duration {
        match self.state {
            PlayerAnimationState::Idle => Self::IDLE_INTERVAL,
            PlayerAnimationState::Grab => Self::GRAB_INTERVAL,
            PlayerAnimationState::GrabBack => Self::GRAB_BACK_INTERVAL,
            PlayerAnimationState::UseDynamite => Self::USE_DYNAMITE_INTERVAL,
            PlayerAnimationState::Strengthen => Self::STRENGTHEN_INTERVAL,
        }
    }

    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        // 重置 state_changed 标记
        self.state_changed = false;

        let frames = self.get_frames();
        // 单帧动画无需更新计时器
        if frames.len() > 1 {
            self.timer.tick(delta);
            if self.timer.just_finished() {
                self.frame = (self.frame + 1) % frames.len();
                self.state_changed = true;
            }
        }
    }

    /// Update animation state if it changes.
    pub fn update_state(&mut self, new_state: PlayerAnimationState) {
        if self.state != new_state {
            self.state = new_state;
            self.frame = 0;
            self.timer = Timer::new(self.get_interval(), TimerMode::Repeating);
            self.state_changed = true;
        }
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.state_changed
    }

    /// Return sprite index in the atlas.
    pub fn get_atlas_index(&self) -> usize {
        let frames = self.get_frames();
        frames[self.frame]
    }

    /// 获取当前动画状态
    pub fn state(&self) -> PlayerAnimationState {
        self.state
    }
}

impl Default for PlayerAnimation {
    fn default() -> Self {
        Self::new()
    }
}
