use crate::asset_tracking::LoadResource;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<AudioAssets>();
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "music" category (e.g. global background music, soundtrack).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Music;

/// A music audio instance.
pub fn music(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::LOOP, Music)
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "sound effect" category (e.g. footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SoundEffect;

/// A sound effect audio instance.
pub fn sound_effect(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::DESPAWN, SoundEffect)
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct AudioAssets {
    // --- Sounds ---
    #[dependency]
    money: Handle<AudioSource>,
    #[dependency]
    hook_reset: Handle<AudioSource>,
    #[dependency]
    grab_start: Handle<AudioSource>,
    #[dependency]
    grab_back: Handle<AudioSource>,
    #[dependency]
    explosive: Handle<AudioSource>,
    #[dependency]
    high: Handle<AudioSource>,
    #[dependency]
    normal: Handle<AudioSource>,
    #[dependency]
    low: Handle<AudioSource>,

    // --- Music ---
    #[dependency]
    goal_music: Handle<AudioSource>,
    #[dependency]
    made_goal_music: Handle<AudioSource>,
}

impl FromWorld for AudioAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            money: assets.load("audios/money.wav"),
            hook_reset: assets.load("audios/hook_reset.wav"),
            grab_start: assets.load("audios/grab_start.mp3"),
            grab_back: assets.load("audios/grab_back.wav"),
            explosive: assets.load("audios/explosive.wav"),
            high: assets.load("audios/high_value.wav"),
            normal: assets.load("audios/normal_value.wav"),
            low: assets.load("audios/low_value.wav"),

            goal_music: assets.load("audios/goal.mp3"),
            made_goal_music: assets.load("audios/made_goal.mp3"),
        }
    }
}

impl AudioAssets {
    pub fn get_audio(&self, id: &str) -> Option<Handle<AudioSource>> {
        match id {
            "Money" => Some(self.money.clone()),
            "HookReset" => Some(self.hook_reset.clone()),
            "GrabStart" => Some(self.grab_start.clone()),
            "GrabBack" => Some(self.grab_back.clone()),
            "Explosive" => Some(self.explosive.clone()),
            "High" => Some(self.high.clone()),
            "Normal" => Some(self.normal.clone()),
            "Low" => Some(self.low.clone()),

            "Goal" => Some(self.goal_music.clone()),
            "MadeGoal" => Some(self.made_goal_music.clone()),

            _ => None,
        }
    }
}
