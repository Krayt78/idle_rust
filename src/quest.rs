use crate::item::Item;
use crate::player::Player;
use crate::job::JobName;
use serde::{Deserialize, Serialize};
use crate::utils::QuestDatabase;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Quest {
    pub id: u128,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestData {
    pub id: u128,
    pub name: String,
    pub description: String,
    pub reward: Reward,
    pub goal: Goal,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Goal {
    pub objective: Objective,
    pub required_amount: u128,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Objective {
    CollectItem(u128),
    CollectGold(),
    ReachJobLevel(JobName),
    ReachLevel(),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct JobExperience {
    pub job: JobName,
    pub amount: u128,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Reward {
    pub experience: Option<JobExperience>,
    pub items: Option<Vec<Item>>,
    pub gold: Option<u128>,
}

impl Quest {
    pub fn new(id: u128, completed: bool) -> Self {
        Self { id, completed }
    }

    pub fn check_completion(&self, quest_data: &QuestData, player: &Player) -> bool {
        match &quest_data.goal.objective {
            Objective::CollectItem(item_id) => {
                player.inventory.get_item_quantity(*item_id) >= quest_data.goal.required_amount
            }
            Objective::CollectGold() => {
                player.inventory.gold >= quest_data.goal.required_amount
            }
            Objective::ReachJobLevel(job_name) => {
                match player.get_job(job_name.clone()) {
                    Some(job) => u128::from(job.level) >= quest_data.goal.required_amount,
                    None => false,
                }
            }
            Objective::ReachLevel() => {
                u128::from(player.level) >= quest_data.goal.required_amount
            }
        }
    }

    pub fn complete(&mut self, player: &mut Player, quest_database: &QuestDatabase) {
        let quest_data = match quest_database.get(&self.id) {
            Some(quest_data) => quest_data,
            None => {
                println!("Quest not found");
                return;
            }
        };

        let is_completed = self.check_completion(quest_data, player);
        if !is_completed {
            println!("Quest not completed");
            return;
        }

        if let Some(experience) = &quest_data.reward.experience {
            player.add_experience(experience.job.clone(), experience.amount);
        }
        if let Some(items) = &quest_data.reward.items {
            for item in items {
                player.add_item(item);
            }
        }
        if let Some(gold) = &quest_data.reward.gold {
            player.add_gold(*gold);
        }

        self.completed = true;
    }
}

mod mock {
    use super::*;

    pub fn quest_database() -> QuestDatabase {
        QuestDatabase::new()
    }
}

mod tests {
    use super::*;
    use mock::*;

    #[test]
    fn test_quest() {
        let quest_id = 1;
        let quest_name = "Test Quest".to_string();
        let quest_description = "Test Description".to_string();
        let quest_reward = Reward { experience: Some(JobExperience { job: JobName::Woodcutter, amount: 100 }), items: None, gold: Some(100) };
        let quest_goal = Goal { objective: Objective::CollectItem(1), required_amount: 10 };

        let quest_data = QuestData { id: quest_id, name: quest_name, description: quest_description, reward: quest_reward, goal: quest_goal };
        let quest = Quest::new(quest_id, false);

        assert_eq!(quest.id, quest_id);
        assert_eq!(quest.completed, false);
    }

    #[test]
    fn test_check_completion_works_for_item() {
        let quest_id = 1;
        let quest_name = "Test Quest".to_string();
        let quest_description = "Test Description".to_string();
        let quest_reward = Reward { experience: Some(JobExperience { job: JobName::Woodcutter, amount: 100 }), items: None, gold: Some(100) };
        let quest_goal = Goal { objective: Objective::CollectItem(1), required_amount: 1 };

        let quest_data = QuestData { id: quest_id, name: quest_name, description: quest_description, reward: quest_reward, goal: quest_goal };
        let quest = Quest::new(quest_id, false);
        let mut player = Player::new();

        assert_eq!(quest.check_completion(&quest_data, &player), false);

        let item = Item::new(1, 1);
        player.add_item(&item);

        assert_eq!(quest.check_completion(&quest_data, &player), true);
    }

    #[test]
    fn test_check_completion_works_for_gold() {
        let quest_id = 1;
        let quest_name = "Test Quest".to_string();
        let quest_description = "Test Description".to_string();
        let quest_reward = Reward { experience: Some(JobExperience { job: JobName::Woodcutter, amount: 100 }), items: None, gold: Some(100) };
        let quest_goal = Goal { objective: Objective::CollectGold(), required_amount: 100 };

        let quest_data = QuestData { id: quest_id, name: quest_name, description: quest_description, reward: quest_reward, goal: quest_goal };
        let quest = Quest::new(quest_id, false);
        let mut player = Player::new();

        assert_eq!(quest.check_completion(&quest_data, &player), false);

        player.add_gold(100);

        assert_eq!(quest.check_completion(&quest_data, &player), true);
    }

    #[test]
    fn test_check_completion_works_for_job_level() {
        let quest_id = 1;
        let quest_name = "Test Quest".to_string();
        let quest_description = "Test Description".to_string();
        let quest_reward = Reward { experience: Some(JobExperience { job: JobName::Woodcutter, amount: 100 }), items: None, gold: Some(100) };
        let quest_goal = Goal { objective: Objective::ReachJobLevel(JobName::Woodcutter), required_amount: 2 };

        let quest_data = QuestData { id: quest_id, name: quest_name, description: quest_description, reward: quest_reward, goal: quest_goal };
        let quest = Quest::new(quest_id, false);
        let mut player = Player::new();

        let initial_player_job = player.get_job(JobName::Woodcutter).unwrap();
        let needed_xp = initial_player_job.get_xp_needed_for_next_level();

        assert_eq!(&initial_player_job.level, &1);
        assert_eq!(quest.check_completion(&quest_data, &player), false);

        player.add_experience(JobName::Woodcutter, needed_xp);

        let updated_player_job = player.get_job(JobName::Woodcutter).unwrap();
        assert_eq!(&updated_player_job.level, &2);
        assert_eq!(quest.check_completion(&quest_data, &player), true);        
    }

    //TODO: Add real tests for level once we have some way to level up
    #[test]
    fn test_check_completion_works_for_level() {
        let quest_id = 1;
        let quest_name = "Test Quest".to_string();
        let quest_description = "Test Description".to_string();
        let quest_reward = Reward { experience: Some(JobExperience { job: JobName::Woodcutter, amount: 100 }), items: None, gold: Some(100) };
        let quest_goal = Goal { objective: Objective::ReachLevel(), required_amount: 2 };

        let quest_data = QuestData { id: quest_id, name: quest_name, description: quest_description, reward: quest_reward, goal: quest_goal };
        let quest = Quest::new(quest_id, false);
        let mut player = Player::new();

        assert_eq!(quest.check_completion(&quest_data, &player), false);

        player.level = 2;

        assert_eq!(quest.check_completion(&quest_data, &player), true);
    }
}


