//! 商店界面：购买道具

use crate::audio::{AudioAssets, sound_effect};
use crate::config::ImageAssets;
use crate::constants::{COLOR_GREEN, COLOR_YELLOW};
use crate::demo::player::PlayerResource;
use crate::screens::{Screen, stats::LevelStats};
use crate::utils::love_to_bevy_coords;
use bevy::prelude::*;
use rand::Rng;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Shop), spawn_shop_ui);
    app.add_systems(
        Update,
        (handle_shop_input, update_shop_ui).run_if(in_state(Screen::Shop)),
    );
}

/// 道具类型
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PropType {
    Dynamite,
    StrengthDrink,
    LuckyClover,
    RockCollectorsBook,
    GemPolish,
}

impl PropType {
    fn description(&self) -> &'static str {
        match self {
            PropType::Dynamite => "Destroy grabbed entity",
            PropType::StrengthDrink => "Pull faster next level",
            PropType::LuckyClover => "2x luck on bags",
            PropType::RockCollectorsBook => "3x rock value",
            PropType::GemPolish => "1.5x diamond value",
        }
    }

    fn image_id(&self) -> &'static str {
        match self {
            PropType::Dynamite => "Dynamite",
            PropType::StrengthDrink => "StrengthDrink",
            PropType::LuckyClover => "LuckyClover",
            PropType::RockCollectorsBook => "RockCollectorsBook",
            PropType::GemPolish => "GemPolish",
        }
    }

    fn get_price(&self, level: u32) -> u32 {
        let mut rng = rand::rng();
        match self {
            PropType::Dynamite => rng.random_range(1..=300) + 1 + level * 2,
            PropType::StrengthDrink => rng.random_range(100..=400),
            PropType::LuckyClover => rng.random_range(1..=(level * 50).max(1)) + 1 + level * 2,
            PropType::RockCollectorsBook => rng.random_range(1..=150) + 1,
            PropType::GemPolish => rng.random_range(201..=(level * 100 + 201)),
        }
    }
}

/// 商店中的道具实例
#[derive(Clone)]
struct ShopItem {
    prop_type: PropType,
    price: u32,
}

/// 商店状态资源
#[derive(Resource)]
struct ShopState {
    items: Vec<ShopItem>,
    selector_index: usize,
    is_finish_shopping: bool,
    finish_timer: Timer,
    player_bought: bool,
}

impl Default for ShopState {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            selector_index: 0,
            is_finish_shopping: false,
            finish_timer: Timer::from_seconds(1.5, TimerMode::Once),
            player_bought: false,
        }
    }
}

/// UI 组件标记
#[derive(Component)]
struct ShopDialogueText;
#[derive(Component)]
struct ShopItemSprite;
#[derive(Component)]
struct ShopItemPrice;
#[derive(Component)]
struct ShopSelector;
#[derive(Component)]
struct ShopDescriptionText;
#[derive(Component)]
struct ShopMoneyText;
const SHOP_ITEM_PADDING: f32 = 50.0;

fn spawn_shop_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    image_assets: Res<ImageAssets>,
    stats: Res<LevelStats>,
) {
    // 初始化商店状态
    let mut rng = rand::rng();
    let all_props = [
        PropType::Dynamite,
        PropType::StrengthDrink,
        PropType::LuckyClover,
        PropType::RockCollectorsBook,
        PropType::GemPolish,
    ];

    let mut items = Vec::new();
    for prop in all_props.iter() {
        // 约 66% 概率出现
        if rng.random_range(1..=3) >= 2 {
            items.push(ShopItem {
                prop_type: *prop,
                price: prop.get_price(stats.level),
            });
        }
    }
    // 确保至少有一个商品
    if items.is_empty() {
        items.push(ShopItem {
            prop_type: PropType::Dynamite,
            price: PropType::Dynamite.get_price(stats.level),
        });
    }

    commands.insert_resource(ShopState {
        items: items.clone(),
        selector_index: 0,
        is_finish_shopping: false,
        finish_timer: Timer::from_seconds(1.5, TimerMode::Once),
        player_bought: false,
    });

    // 背景
    commands.spawn((
        Name::new("Shop Background"),
        Sprite::from_image(image_assets.get_image("Shop").unwrap()),
        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
        DespawnOnExit(Screen::Shop),
    ));

    // 标题 (水平居中, y=5)
    commands.spawn((
        Name::new("Shop Title"),
        Sprite::from_image(image_assets.get_image("Title").unwrap()),
        Transform::from_translation(love_to_bevy_coords(160.0, 5.0).extend(0.0)),
        DespawnOnExit(Screen::Shop),
    ));

    // 对话气泡 (25, 70)
    commands.spawn((
        Name::new("Dialogue Bubble"),
        Sprite::from_image(image_assets.get_image("DialogueBubble").unwrap()),
        Transform::from_translation(love_to_bevy_coords(25.0, 70.0).extend(0.0)),
        bevy::sprite::Anchor::TOP_LEFT,
        DespawnOnExit(Screen::Shop),
    ));

    // 店主对话文字 (30, 75)
    let font = asset_server.load("fonts/Kurland.ttf");
    commands.spawn((
        Name::new("Shop Dialogue"),
        Text2d::new("Left/Right: select\nEnter: buy\nSpace: exit"),
        TextFont {
            font: font.clone(),
            font_size: 12.0,
            ..default()
        },
        TextColor(Color::BLACK),
        Transform::from_translation(love_to_bevy_coords(30.0, 75.0).extend(1.0)),
        bevy::sprite::Anchor::TOP_LEFT,
        ShopDialogueText,
        DespawnOnExit(Screen::Shop),
    ));

    // 桌子 (7, 176)
    commands.spawn((
        Name::new("Shop Table"),
        Sprite::from_image(image_assets.get_image("Table").unwrap()),
        Transform::from_translation(love_to_bevy_coords(7.0, 176.0).extend(0.0)),
        bevy::sprite::Anchor::TOP_LEFT,
        DespawnOnExit(Screen::Shop),
    ));

    // 商品 (从 30, 176 开始)
    for (i, item) in items.iter().enumerate() {
        let x = 30.0 + i as f32 * SHOP_ITEM_PADDING;
        // 商品图片
        commands.spawn((
            Name::new(format!("Shop Item {i}")),
            Sprite::from_image(image_assets.get_image(item.prop_type.image_id()).unwrap()),
            Transform::from_translation(love_to_bevy_coords(x, 160.0).extend(1.0)),
            bevy::sprite::Anchor::CENTER,
            ShopItemSprite,
            DespawnOnExit(Screen::Shop),
        ));

        // 价格 (商品下方)
        commands.spawn((
            Name::new(format!("Shop Price {i}")),
            Text2d::new(format!("${}", item.price)),
            TextFont {
                font: font.clone(),
                font_size: 10.0,
                ..default()
            },
            TextColor(COLOR_GREEN),
            Transform::from_translation(love_to_bevy_coords(x, 175.0).extend(1.0)),
            bevy::sprite::Anchor::CENTER,
            ShopItemPrice,
            DespawnOnExit(Screen::Shop),
        ));
    }

    // 选择器 (y=130)
    if !items.is_empty() {
        commands.spawn((
            Name::new("Shop Selector"),
            Sprite::from_image(image_assets.get_image("Selector").unwrap()),
            Transform::from_translation(love_to_bevy_coords(30.0, 130.0).extend(2.0)),
            bevy::sprite::Anchor::CENTER,
            ShopSelector,
            DespawnOnExit(Screen::Shop),
        ));
    }

    // 道具描述 (25, 195)
    let desc = if !items.is_empty() {
        items[0].prop_type.description()
    } else {
        ""
    };
    commands.spawn((
        Name::new("Shop Description"),
        Text2d::new(desc),
        TextFont {
            font: font.clone(),
            font_size: 12.0,
            ..default()
        },
        TextColor(COLOR_YELLOW),
        Transform::from_translation(love_to_bevy_coords(25.0, 195.0).extend(1.0)),
        bevy::sprite::Anchor::TOP_LEFT,
        ShopDescriptionText,
        DespawnOnExit(Screen::Shop),
    ));

    // 显示玩家金钱
    commands.spawn((
        Name::new("Player Money"),
        Text2d::new(format!("Your Money: ${}", stats.money)),
        TextFont {
            font: font.clone(),
            font_size: 14.0,
            ..default()
        },
        TextColor(COLOR_GREEN),
        Transform::from_translation(love_to_bevy_coords(160.0, 220.0).extend(1.0)),
        bevy::sprite::Anchor::CENTER,
        ShopMoneyText,
        DespawnOnExit(Screen::Shop),
    ));
}

fn handle_shop_input(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    audio_assets: Res<AudioAssets>,
    mut stats: ResMut<LevelStats>,
    mut shop_state: ResMut<ShopState>,
    mut player: ResMut<PlayerResource>,
) {
    if shop_state.is_finish_shopping {
        return;
    }

    let mut left = input.just_pressed(KeyCode::ArrowLeft);
    let mut right = input.just_pressed(KeyCode::ArrowRight);
    let mut buy = input.just_pressed(KeyCode::Enter)
        || input.just_pressed(KeyCode::NumpadEnter)
        || input.just_pressed(KeyCode::KeyJ)
        || input.just_pressed(KeyCode::KeyK);
    let mut exit_shop = input.just_pressed(KeyCode::Space);

    for gamepad in &gamepads {
        if gamepad.just_pressed(GamepadButton::DPadLeft) {
            left = true;
        }
        if gamepad.just_pressed(GamepadButton::DPadRight) {
            right = true;
        }
        if gamepad.just_pressed(GamepadButton::South)
            || gamepad.just_pressed(GamepadButton::East)
            || gamepad.just_pressed(GamepadButton::Start)
        {
            buy = true;
        }
        if gamepad.just_pressed(GamepadButton::Select) {
            exit_shop = true;
        }
    }

    // 左右切换选择
    if left && shop_state.selector_index > 0 {
        shop_state.selector_index -= 1;
    }
    if right && shop_state.selector_index < shop_state.items.len().saturating_sub(1) {
        shop_state.selector_index += 1;
    }

    // 购买
    if buy {
        let selector_index = shop_state.selector_index;
        if let Some(item) = shop_state.items.get(selector_index).cloned()
            && stats.money >= item.price
        {
            stats.money -= item.price;
            shop_state.player_bought = true;

            // 播放购买音效
            if let Some(audio) = audio_assets.get_audio("Money") {
                commands.spawn(sound_effect(audio));
            }

            // 应用道具效果到 PlayerResource
            match item.prop_type {
                PropType::Dynamite => {
                    // 炸药数量 +1，上限 12
                    player.dynamite_count = (player.dynamite_count + 1).min(12);
                }
                PropType::StrengthDrink => {
                    player.has_strength_drink = true;
                }
                PropType::LuckyClover => {
                    player.has_lucky_clover = true;
                }
                PropType::RockCollectorsBook => {
                    player.has_rock_collectors_book = true;
                }
                PropType::GemPolish => {
                    player.has_gem_polish = true;
                }
            }

            // 移除商品
            shop_state.items.remove(selector_index);
            if shop_state.selector_index >= shop_state.items.len() && shop_state.selector_index > 0
            {
                shop_state.selector_index -= 1;
            }

            // 商品售罄则自动完成购物
            if shop_state.items.is_empty() {
                shop_state.is_finish_shopping = true;
            }
        }
    }

    // 完成购物
    if exit_shop {
        shop_state.is_finish_shopping = true;
    }
}

fn update_shop_ui(
    time: Res<Time>,
    stats: Res<LevelStats>,
    mut shop_state: ResMut<ShopState>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut q_selector: Query<&mut Transform, With<ShopSelector>>,
    mut q_dialogue: Query<
        &mut Text2d,
        (
            With<ShopDialogueText>,
            Without<ShopDescriptionText>,
            Without<ShopMoneyText>,
        ),
    >,
    mut q_description: Query<
        &mut Text2d,
        (
            With<ShopDescriptionText>,
            Without<ShopDialogueText>,
            Without<ShopMoneyText>,
        ),
    >,
    mut q_money: Query<
        &mut Text2d,
        (
            With<ShopMoneyText>,
            Without<ShopDialogueText>,
            Without<ShopDescriptionText>,
        ),
    >,
) {
    // 更新选择器位置
    if let Ok(mut transform) = q_selector.single_mut() {
        let x = 30.0 + shop_state.selector_index as f32 * SHOP_ITEM_PADDING;
        transform.translation = love_to_bevy_coords(x, 130.0).extend(2.0);
    }

    // 更新描述文字
    let selector_index = shop_state.selector_index;
    let desc = shop_state
        .items
        .get(selector_index)
        .map(|item| item.prop_type.description().to_string())
        .unwrap_or_default();
    if let Ok(mut text) = q_description.single_mut() {
        text.0 = desc;
    }

    if let Ok(mut text) = q_money.single_mut() {
        text.0 = format!("Your Money: ${}", stats.money);
    }

    // 处理完成购物计时器
    if shop_state.is_finish_shopping {
        if let Ok(mut text) = q_dialogue.single_mut() {
            if shop_state.player_bought {
                text.0 = "Thank you!\nGood luck!".to_string();
            } else {
                text.0 = "  :(".to_string();
            }
        }

        shop_state.finish_timer.tick(time.delta());
        if shop_state.finish_timer.just_finished() {
            next_screen.set(Screen::NextGoal);
        }
    }
}
