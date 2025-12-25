use serde::{Deserialize, Serialize};
use bevy::app::{App, Plugin};
use bevy::platform::collections::HashMap;
use bevy_common_assets::yaml::YamlAssetPlugin;
// --- entities.yaml 对应的结构 ---

/// 包装实体配置 HashMap 以实现 Asset
#[derive(Debug, Clone, Serialize, Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct EntitiesConfig {
    #[serde(flatten)]
    pub entities: HashMap<String, EntityDescriptor>,
}


/// 实体描述符：定义实体的静态物理属性和数值
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] // 自动将蛇法命名 (snake_case) 映射到 YAML 的小驼峰 (camelCase)
pub struct EntityDescriptor {
    /// 实体的行为类型（基础、随机效果、移动、爆炸）
    #[serde(rename = "type")]
    pub entity_type: EntityType,

    /// 实体的质量：影响钩子拉取的难度和速度
    pub mass: f32,

    /// 实体的分值：抓取成功后增加的金额
    pub bonus: i32,

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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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


pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            (
                YamlAssetPlugin::<LevelsConfig>::new(&["config/levels.yaml"]),
                YamlAssetPlugin::<EntitiesConfig>::new(&["config/entities.yaml"])
            )
        );
    }
}


