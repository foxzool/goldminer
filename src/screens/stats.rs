use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct LevelStats {
    pub money: u32,
    pub goal: u32,
    pub goal_addon: u32,
    pub level: u32,
    pub timer: f32,
    pub is_first_init: bool,
    /// 实际关卡配置ID (如 "L1_1", "L3_2")
    pub real_level_str: String,
}

impl Default for LevelStats {
    fn default() -> Self {
        Self {
            money: 0,
            goal: 375,
            goal_addon: 275,
            level: 1,
            timer: 61.0,
            is_first_init: true,
            real_level_str: "L1_1".to_string(),
        }
    }
}

impl LevelStats {
    /// 计算并更新下一关的目标金额
    /// 第 1 关: goal = 375 + 275 = 650
    /// 第 2-9 关: goal_addon += 270, 然后 goal += goal_addon
    /// 第 10+ 关: goal_addon 不再增加, goal 继续 += goal_addon
    pub fn update_goal(&mut self) {
        if self.level > 1 && self.level <= 9 {
            self.goal_addon += 270;
        }
        self.goal += self.goal_addon;
    }

    pub fn reach_goal(&self) -> bool {
        self.money >= self.goal
    }

    pub fn reset_timer(&mut self) {
        self.timer = 61.0;
    }

    /// 计算实际关卡配置
    /// 前3关正常递增，之后在3-9之间循环
    pub fn calculate_real_level(&mut self) {
        let real_level = if self.level <= 3 {
            self.level
        } else {
            ((self.level - 3) % 7) + 3
        };
        // 随机变体 1-3
        let variant = rand::random::<u32>() % 3 + 1;
        self.real_level_str = format!("L{}_{}", real_level, variant);
    }
}
