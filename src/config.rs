use crate::asset_tracking::LoadResource;
use bevy::app::{App, Plugin};
use bevy::asset::{Asset, AssetServer, Handle};
use bevy::audio::AudioSource;
use bevy::image::Image;
use bevy::platform::collections::HashMap;
use bevy::prelude::{Component, Font};
use bevy::prelude::ReflectResource;
use bevy::prelude::{FromWorld, Reflect, Resource, World};
use bevy_common_assets::yaml::YamlAssetPlugin;
use serde::{Deserialize, Serialize};

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            YamlAssetPlugin::<LevelsConfig>::new(&["config/levels.yaml"]),
            YamlAssetPlugin::<EntitiesConfig>::new(&["config/entities.yaml"]),
        ));
        app.load_resource::<BackgroundsAssets>();
        app.load_resource::<SpritesAssets>();
        app.load_resource::<SoundAssets>();
        app.load_resource::<MusicAssets>();
    }
}

// --- entities.yaml 对应的结构 ---

/// 包装实体配置 HashMap 以实现 Asset
#[derive(Debug, Clone, Serialize, Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct EntitiesConfig {
    #[serde(flatten)]
    pub entities: HashMap<String, EntityDescriptor>,
}

/// 实体描述符：定义实体的静态物理属性和数值
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] // 自动将蛇法命名 (snake_case) 映射到 YAML 的小驼峰 (camelCase)
pub struct EntityDescriptor {
    /// 实体的行为类型（基础、随机效果、移动、爆炸）
    #[serde(rename = "type")]
    pub entity_type: EntityType,

    /// 实体的质量：影响钩子拉取的难度和速度
    pub mass: Option<f32>,

    /// 实体的分值：抓取成功后增加的金额
    pub bonus: Option<i32>,

    /// 分值类别：用于触发不同的音效（High, Normal, Low）
    pub bonus_type: Option<String>,
    // --- 以下为特定类型实体的扩展字段 (使用 Option 处理可选值) ---
    /// 随机配置：最小随机质量
    pub random_mass_min: Option<f32>,
    /// 随机配置：最大随机质量
    pub random_mass_max: Option<f32>,
    /// 随机配置：基础分值
    pub bonus_base: Option<i32>,
    /// 随机配置：最小随机倍率
    pub random_bonus_ratio_min: Option<i32>,
    /// 随机配置：最大随机倍率
    pub random_bonus_ratio_max: Option<i32>,
    /// 随机配置：触发额外效果（如增加力量、获得炸药）的概率
    pub extra_effect_chances: Option<f32>,

    /// 移动实体（如地鼠）：移动速度
    pub speed: Option<f32>,
    /// 移动实体：左右巡逻的范围距离
    pub move_range: Option<f32>,

    /// 爆炸实体（如TNT）：被摧毁后显示的精灵图类型
    pub destroyed_type: Option<String>,
    /// 爆炸实体：被抓取的判定是否使用小型钩子动画
    pub is_destroyed_tiny: Option<bool>,
}

/// 实体行为分类
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EntityType {
    /// 基础物体：位置固定，属性固定
    Basic,
    /// 随机：抓取时动态计算质量和得分
    RandomEffect,
    /// 巡逻物体：会在指定范围内左右移动
    MoveAround,
    /// 爆炸物：碰撞后会销毁周围物体
    Explosive,
}

// --- levels.yaml 对应的结构 ---

#[derive(Debug, Clone, Serialize, Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct LevelDescriptor {
    /// 关卡类型：决定使用的背景图片 (对应 LevelA 到 LevelE)
    #[serde(rename = "type")]
    pub level_type: String,

    /// 该关卡中包含的所有实体实例列表
    pub entities: Vec<LevelEntity>,
}

/// 关卡中的实体实例
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct LevelEntity {
    /// 引用 Entities.config 中的 Key (例如 "MiniGold")
    #[serde(rename = "type")]
    pub entity_id: String,

    /// 实体在屏幕上的生成坐标 {x, y}
    pub pos: Position,

    /// 可选的移动方向（仅对 MoveAround 类型有效）
    pub dir: Option<Direction>,
}

/// 坐标位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
/// 移动方向枚举
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "Left")]
    Left,
    #[serde(rename = "Right")]
    Right,
}

/// 包装关卡配置 HashMap 以实现 Asset
#[derive(Debug, Clone, Serialize, Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct LevelsConfig {
    #[serde(flatten)]
    pub levels: HashMap<String, LevelDescriptor>,
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct SpritesAssets {
    #[dependency]
    mini_gold: Handle<Image>,
    #[dependency]
    normal_gold: Handle<Image>,
    #[dependency]
    normal_gold_plus: Handle<Image>,
    #[dependency]
    big_gold: Handle<Image>,
    #[dependency]
    mini_rock: Handle<Image>,
    #[dependency]
    normal_rock: Handle<Image>,
    #[dependency]
    big_rock: Handle<Image>,
    #[dependency]
    question_bag: Handle<Image>,
    #[dependency]
    diamond: Handle<Image>,
    #[dependency]
    skull: Handle<Image>,
    #[dependency]
    bone: Handle<Image>,
    #[dependency]
    tnt: Handle<Image>,
    #[dependency]
    tnt_destroyed: Handle<Image>,

    #[dependency]
    menu_arrow: Handle<Image>,
    #[dependency]
    panel: Handle<Image>,
    #[dependency]
    dialogue_bubble: Handle<Image>,
    #[dependency]
    title: Handle<Image>,
    #[dependency]
    selector: Handle<Image>,
    #[dependency]
    dynamite_ui: Handle<Image>,
    #[dependency]
    strength: Handle<Image>,
    #[dependency]
    table: Handle<Image>,
    #[dependency]
    dynamite: Handle<Image>,
    #[dependency]
    strength_drink: Handle<Image>,
    #[dependency]
    lucky_colver: Handle<Image>,
    #[dependency]
    rock_collector_book: Handle<Image>,
    #[dependency]
    gem_polish: Handle<Image>,
}

impl FromWorld for SpritesAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            mini_gold: assets.load("images/gold_mini.png"),
            normal_gold: assets.load("images/gold_normal.png"),
            normal_gold_plus: assets.load("images/gold_normal_plus.png"),
            big_gold: assets.load("images/gold_big.png"),
            mini_rock: assets.load("images/rock_mini.png"),
            normal_rock: assets.load("images/rock_normal.png"),
            big_rock: assets.load("images/rock_big.png"),
            question_bag: assets.load("images/question_bag.png"),
            diamond: assets.load("images/diamond.png"),
            skull: assets.load("images/skull.png"),
            bone: assets.load("images/bone.png"),
            tnt: assets.load("images/tnt.png"),
            tnt_destroyed: assets.load("images/tnt_destroyed.png"),
            menu_arrow: assets.load("images/menu_arrow.png"),
            panel: assets.load("images/panel.png"),
            dialogue_bubble: assets.load("images/ui_dialogue_bubble.png"),
            title: assets.load("images/text_goldminer.png"),
            selector: assets.load("images/ui_selector.png"),
            dynamite_ui: assets.load("images/ui_dynamite.png"),
            strength: assets.load("images/text_strength.png"),
            table: assets.load("images/shop_table.png"),
            dynamite: assets.load("images/dynamite.png"),
            strength_drink: assets.load("images/strength_drink.png"),
            lucky_colver: assets.load("images/lucky_clover.png"),
            rock_collector_book: assets.load("images/rock_collectors_book.png"),
            gem_polish: assets.load("images/gem_polish.png"),
        }
    }
}

impl SpritesAssets {
    pub fn get_sprite(&self, id: &str) -> Option<Handle<Image>> {
        match id {
            "MiniGold" => Some(self.mini_gold.clone()),
            "NormalGold" => Some(self.normal_gold.clone()),
            "NormalGoldPlus" => Some(self.normal_gold_plus.clone()),
            "BigGold" => Some(self.big_gold.clone()),
            "MiniRock" => Some(self.mini_rock.clone()),
            "NormalRock" => Some(self.normal_rock.clone()),
            "BigRock" => Some(self.mini_gold.clone()),
            "QuestionBag" => Some(self.mini_gold.clone()),
            "Diamond" => Some(self.diamond.clone()),
            "Skull" => Some(self.skull.clone()),
            "Bone" => Some(self.bone.clone()),
            "TNT" => Some(self.tnt.clone()),
            "TNT_Destroyed" => Some(self.tnt_destroyed.clone()),

            "MenuArrow" => Some(self.menu_arrow.clone()),
            "Panel" => Some(self.panel.clone()),
            "DialogueBubble" => Some(self.dialogue_bubble.clone()),
            "Title" => Some(self.title.clone()),
            "Selector" => Some(self.selector.clone()),
            "DynamiteUI" => Some(self.dynamite_ui.clone()),
            "Strength!" => Some(self.strength.clone()),

            "Table" => Some(self.table.clone()),
            "Dynamite" => Some(self.dynamite.clone()),
            "StrengthDrink" => Some(self.strength_drink.clone()),
            "LuckyClover" => Some(self.lucky_colver.clone()),
            "RockCollectorsBook" => Some(self.rock_collector_book.clone()),
            "GemPolish" => Some(self.gem_polish.clone()),

            _ => None,
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct SoundAssets {
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
}

impl FromWorld for SoundAssets {
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
        }
    }
}

impl SoundAssets {
    pub fn get_sound(&self, id: &str) -> Option<Handle<AudioSource>> {
        match id {
            "Money" => Some(self.money.clone()),
            "HookReset" => Some(self.hook_reset.clone()),
            "GrabStart" => Some(self.grab_start.clone()),
            "GrabBack" => Some(self.grab_back.clone()),
            "Explosive" => Some(self.explosive.clone()),
            "High" => Some(self.high.clone()),
            "Normal" => Some(self.normal.clone()),
            "Low" => Some(self.low.clone()),

            _ => None,
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct MusicAssets {
    #[dependency]
    goal: Handle<AudioSource>,
    #[dependency]
    made_goal: Handle<AudioSource>,
}

impl FromWorld for MusicAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            goal: assets.load("audios/goal.mp3"),
            made_goal: assets.load("audios/made_goal.mp3"),
        }
    }
}

impl MusicAssets {
    pub fn get_music(&self, id: &str) -> Option<Handle<AudioSource>> {
        match id {
            "Goal" => Some(self.goal.clone()),
            "MadeGoal" => Some(self.made_goal.clone()),
            _ => None,
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct BackgroundsAssets {
    menu: Handle<Image>,
    level_common_top: Handle<Image>,
    level_a: Handle<Image>,
    level_b: Handle<Image>,
    level_c: Handle<Image>,
    level_d: Handle<Image>,
    level_e: Handle<Image>,
    goal: Handle<Image>,
    shop: Handle<Image>,
}

impl FromWorld for BackgroundsAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            menu: assets.load("images/bg_start_menu.png"),
            level_common_top: assets.load("images/bg_top.png"),
            level_a: assets.load("images/bg_level_A.png"),
            level_b: assets.load("images/bg_level_B.png"),
            level_c: assets.load("images/bg_level_C.png"),
            level_d: assets.load("images/bg_level_D.png"),
            level_e: assets.load("images/bg_level_E.png"),
            goal: assets.load("images/bg_goal.png"),
            shop: assets.load("images/bg_shop.png"),
        }
    }
}

impl BackgroundsAssets {
    pub fn get_background(&self, id: &str) -> Option<Handle<Image>> {
        match id {
            "Menu" => Some(self.menu.clone()),
            "LevelCommonTop" => Some(self.level_common_top.clone()),
            "LevelA" => Some(self.level_a.clone()),
            "LevelB" => Some(self.level_b.clone()),
            "LevelC" => Some(self.level_c.clone()),
            "LevelD" => Some(self.level_d.clone()),
            "LevelE" => Some(self.level_e.clone()),
            "Goal" => Some(self.goal.clone()),
            "Shop" => Some(self.shop.clone()),

            _ => None,
        }
    }
}
