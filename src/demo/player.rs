//! Player-specific behavior and animation.

use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::AppSystems;
use crate::asset_tracking::LoadResource;
use crate::screens::Screen;
use crate::utils::love_to_bevy_coords;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();
    app.init_resource::<PlayerResource>();

    app.add_systems(OnEnter(Screen::Gameplay), spawn_player);

    // 动画及状态更新系统
    app.add_systems(
        Update,
        (
            update_animation_timer.in_set(AppSystems::TickTimers),
            update_animation_atlas.in_set(AppSystems::Update),
            update_dynamite_status.in_set(AppSystems::Update),
        ),
    );
}

/// 更新玩家扔炸药的状态计时器
fn update_dynamite_status(
    time: Res<Time>,
    mut player: ResMut<PlayerResource>,
    mut q_player_anim: Query<&mut PlayerAnimation>,
) {
    if player.is_using_dynamite {
        player.using_dynamite_timer -= time.delta_secs();
        if player.using_dynamite_timer <= 0.0 {
            player.is_using_dynamite = false;
            player.using_dynamite_timer = 0.39;
            // 恢复到正常状态 (如果正在回收则是 GrabBack，否则 Idle)
            // 这里简化处理：如果在扔炸药动作结束，先切回 Idle，hook 系统会根据状态切回 GrabBack
            for mut anim in &mut q_player_anim {
                anim.update_state(PlayerAnimationState::Idle);
            }
        }
    }
}

// ============================================================================
// Player Entity
// ============================================================================

/// The player character.
fn spawn_player(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.init_resource::<PlayerResource>();
    // miner_sheet.png 布局: 8 帧横排，每帧 32x40 像素
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 40), 8, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    commands.spawn((
        Name::new("Player"),
        PlayerMarker,
        player_animation,
        Sprite::from_atlas_image(
            player_assets.miner.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        Transform::from_translation(love_to_bevy_coords(165.0, 39.0).extend(0.0)),
        Anchor::BOTTOM_CENTER,
        DespawnOnExit(Screen::Gameplay),
    ));
}

#[derive(Component)]
struct PlayerMarker;

#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerResource {
    /// 当前关卡数
    pub level: i32,
    /// 对应的关卡配置 ID (如 'L1_1')
    pub real_level: String,
    /// 玩家当前实际拥有的原始金钱
    pub money: i32,
    /// 当前关卡需要达到的目标金额
    pub goal: i32,
    /// 每一关增加的目标金额幅度
    pub goal_add_on: i32,

    /// 玩家的基础力量倍率
    pub strength: i32,
    /// 拥有的炸药数量
    pub dynamite_count: i32,
    /// 是否持有力量药水
    pub has_strength_drink: bool,
    /// 是否持有幸运草
    pub has_lucky_clover: bool,
    /// 是否持有石头收藏书
    pub has_rock_collectors_book: bool,
    /// 是否持有宝石抛光剂
    pub has_gem_polish: bool,

    /// 标识玩家是否正在扔炸药
    pub is_using_dynamite: bool,
    /// 扔炸药动作的倒计时
    pub using_dynamite_timer: f32,
}

impl Default for PlayerResource {
    fn default() -> Self {
        Self {
            level: 1,
            real_level: "LDEBUG".to_string(),
            money: 375,
            goal: 275,
            goal_add_on: 0,
            strength: 1,
            dynamite_count: 0,
            has_strength_drink: false,
            has_lucky_clover: false,
            has_rock_collectors_book: false,
            has_gem_polish: false,
            is_using_dynamite: false,
            using_dynamite_timer: 0.39,
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    miner: Handle<Image>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            miner: assets.load("images/miner_sheet.png"),
        }
    }
}

// ============================================================================
// Player Animation
// ============================================================================

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
}

impl Default for PlayerAnimation {
    fn default() -> Self {
        Self::new()
    }
}
