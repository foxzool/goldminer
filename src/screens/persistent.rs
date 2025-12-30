use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
pub struct PersistentData {
    pub high_score: u32,
    pub high_level: u32,
}

// 以后可以添加从文件加载/保存的逻辑
