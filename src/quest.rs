use crate::item::Item;
use crate::player::Player;
use crate::job::JobName;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Quest {
    pub name: String,
    pub description: String,
    pub reward: Reward,
    pub completed: bool,
    pub goal: Goal,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Goal {
    pub objective: Objective,
    pub current_amount: u128,
    pub required_amount: u128,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Objective {
    CollectItem(u128),
    CollectGold(u128),
    CollectExperience(u128),
    ReachLevel(u128),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Reward {
    pub experience: (JobName, u128),
    pub items: Vec<Item>,
    pub gold: u128,
}

impl Quest {
    pub fn new(name: String, description: String, reward: Reward, goal: Goal) -> Self {
        Self { name, description, reward, completed: false, goal }
    }

    pub fn update_goal(&mut self, amount: u128) {
        self.goal.current_amount += amount;
    }

    pub fn check_completion(&self) -> bool {
        self.goal.current_amount >= self.goal.required_amount
    }

    pub fn complete(&mut self, player: &mut Player) {
        self.completed = true;
        player.add_experience(self.reward.experience.0.clone(), self.reward.experience.1);
        for item in &self.reward.items {
            player.add_item(item);
        }
        player.add_gold(self.reward.gold);
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_quest() {
        let quest_name = "Test Quest".to_string();
        let quest_description = "Test Description".to_string();
        let quest_reward = Reward { experience: (JobName::Woodcutter, 100), items: vec![], gold: 100 };
        let quest_goal = Goal { objective: Objective::CollectItem(1), current_amount: 0, required_amount: 100 };
        let quest = Quest::new(quest_name.clone(), quest_description.clone(), quest_reward.clone(), quest_goal.clone());

        assert_eq!(quest.name, quest_name);
        assert_eq!(quest.description, quest_description);
        assert_eq!(quest.reward, quest_reward);
        assert_eq!(quest.goal, quest_goal);
    }
}


