use crate::inventory::Inventory;
use crate::item::Item;
use crate::job::Job;
use crate::job::JobName;
use std::fmt;
use serde::{Deserialize, Serialize};    

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum ActivityName {
    Woodcutting,
    Mining,
    Farming,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Activity {
    pub name: ActivityName,
    pub description: String,
    pub duration: f32,
    pub timer: f32,
    pub experience: Vec<(JobName, u128)>,
    pub items: Vec<Item>,
}

impl Activity {
    pub fn new(
        name: ActivityName,
        description: String,
        duration: f32,
        experience: Vec<(JobName, u128)>,
        items: Vec<Item>,
    ) -> Self {
        Self {
            name,
            description,
            duration,
            experience,
            timer: 0.0,
            items,
        }
    }

    pub fn update(&mut self, delta_time: f32, jobs: &mut Vec<Job>, inventory: &mut Inventory) -> Result<(), String> {
        self.timer += delta_time;
        if self.timer >= self.duration {
            self.timer = 0.0;
            self.reward(jobs, inventory)?;
        }
        Ok(())
    }

    pub fn update_from_time_elapsed(&mut self, time_elapsed: u64, jobs: &mut Vec<Job>, inventory: &mut Inventory) -> Result<(), String> {
        let number_of_updates: u32 = ((time_elapsed as f32 + self.timer) / self.duration) as u32;
        for _ in 0..number_of_updates {
            self.reward(jobs, inventory)?;
        }

        self.timer = (time_elapsed as f32 + self.timer) % self.duration;
        Ok(())
    }

    fn reward(&mut self, jobs: &mut Vec<Job>, inventory: &mut Inventory) -> Result<(), String> {
        self.reward_experience(jobs, inventory)?;
        self.reward_items(inventory);
        Ok(())
    }

    fn reward_experience(&mut self, jobs: &mut Vec<Job>, inventory: &mut Inventory) -> Result<(), String> {
        for (job, experience) in &self.experience {
            match jobs.iter_mut().find(|j| j.name == *job) {
                Some(job) => job.add_experience(*experience),
                None => return Err(format!("Job not found: {}", job)),
            }
        }
        Ok(())
    }

    fn reward_items(&mut self, inventory: &mut Inventory) {
        for item in &self.items {
            inventory.add_item(item);
        }
    }
    
}

impl fmt::Display for Activity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name {
            ActivityName::Woodcutting => write!(f, "Woodcutting"),
            ActivityName::Mining => write!(f, "Mining"),
            ActivityName::Farming => write!(f, "Farming"),
        }
    }
}

mod tests {
    use super::*;
    use crate::constants::LEVEL_UP_EXPERIENCE;
    
    #[test]
    fn test_activity_new() {
        let activity = Activity::new(
            ActivityName::Woodcutting,
            "Cutting down trees".to_string(),
            1000.0,
            vec![(JobName::Woodcutter, 100)],
            vec![Item::new("Wood".to_string(), "Wood".to_string(), 1)],
        );
        assert_eq!(activity.name, ActivityName::Woodcutting);
        assert_eq!(activity.description, "Cutting down trees");
        assert_eq!(activity.duration, 1000.0);
        assert_eq!(activity.experience, vec![(JobName::Woodcutter, 100)]);
        assert_eq!(
            activity.items,
            vec![Item::new("Wood".to_string(), "Wood".to_string(), 1)]
        );
    }

    #[test]
    fn test_activity_update() {
        let mut jobs = vec![Job::new(JobName::Woodcutter, "Woodcutter".to_string(), 0, 1, LEVEL_UP_EXPERIENCE.to_vec())];
        let mut inventory = Inventory::new();
        let mut activity = Activity::new(
            ActivityName::Woodcutting,
            "Cutting down trees".to_string(),
            1000.0,
            vec![(JobName::Woodcutter, LEVEL_UP_EXPERIENCE[0] / 2)],
            vec![Item::new("Wood".to_string(), "Wood".to_string(), 1)],
        );
        activity.update(500.0, &mut jobs, &mut inventory).unwrap();
        assert_eq!(activity.timer, 500.0);
        assert_eq!(activity.experience, vec![(JobName::Woodcutter, LEVEL_UP_EXPERIENCE[0] / 2)]);
        assert_eq!(inventory.items.len(), 0);
        assert_eq!(jobs[0].experience, 0);
        assert_eq!(jobs[0].level, 1);
    }

    #[test]
    fn test_activity_complete() {
        let mut jobs = vec![Job::new(JobName::Woodcutter, "Woodcutter".to_string(), 0, 1, LEVEL_UP_EXPERIENCE.to_vec())];
        let mut inventory = Inventory::new();
        let mut activity = Activity::new(
            ActivityName::Woodcutting,
            "Cutting down trees".to_string(),
            1000.0,
            vec![(JobName::Woodcutter, LEVEL_UP_EXPERIENCE[0] / 2)],
            vec![Item::new("Wood".to_string(), "Wood".to_string(), 1)],
        );
        activity.update(1000.0, &mut jobs, &mut inventory).unwrap() ;
        assert_eq!(activity.timer, 0.0);
        assert_eq!(activity.experience, vec![(JobName::Woodcutter, LEVEL_UP_EXPERIENCE[0] / 2)]);
        assert_eq!(inventory.items.len(), 1);
        assert_eq!(inventory.items["Wood"].amount, 1);
        assert_eq!(jobs[0].experience, LEVEL_UP_EXPERIENCE[0] / 2);
        assert_eq!(jobs[0].level, 1);
    }

    #[test]
    fn test_activity_update_jobs() {
        let mut jobs = vec![
            Job::new(JobName::Woodcutter, "Woodcutter".to_string(), 0, 1, LEVEL_UP_EXPERIENCE.to_vec()),  
            Job::new(JobName::Miner, "Miner".to_string(), 0, 1, LEVEL_UP_EXPERIENCE.to_vec()),
            Job::new(JobName::Farmer, "Farmer".to_string(), 0, 1, LEVEL_UP_EXPERIENCE.to_vec()),
        ];
        let mut inventory = Inventory::new();
        let mut activity = Activity::new(
            ActivityName::Woodcutting,
            "Cutting down trees".to_string(),
            1000.0,
            vec![
                (JobName::Woodcutter, LEVEL_UP_EXPERIENCE[0] / 2),
                (JobName::Miner, LEVEL_UP_EXPERIENCE[0]),
                (JobName::Farmer, LEVEL_UP_EXPERIENCE[0] * 2),
            ],
            vec![
                Item::new("Wood".to_string(), "Wood".to_string(), 1),
                Item::new("Stone".to_string(), "Stone".to_string(), 2),
                Item::new("Wheat".to_string(), "Wheat".to_string(), 3),
            ],
        );

        activity.update(1000.0, &mut jobs, &mut inventory).unwrap();
        assert_eq!(jobs[0].experience, LEVEL_UP_EXPERIENCE[0] / 2);
        assert_eq!(jobs[0].level, 1);
        assert_eq!(jobs[1].experience, 0);
        assert_eq!(jobs[1].level, 2);
        assert_eq!(jobs[2].experience, LEVEL_UP_EXPERIENCE[0]);
        assert_eq!(jobs[2].level, 2);
        assert_eq!(inventory.items.len(), 3);
        assert_eq!(inventory.items["Wood"].amount, 1);
        assert_eq!(inventory.items["Stone"].amount, 2);
        assert_eq!(inventory.items["Wheat"].amount, 3);
    }

    #[test]
    fn test_activity_update_from_time_elapsed() {
        // 100 levels, each level requires 100 * 2^level experience or each level is double the previous level
        // using some custom level up experience for testing
        let level_up_experience: Vec<u128> = (0..100).map(|i| 100 * 2u128.pow(i as u32)).collect();
        let mut jobs = vec![Job::new(JobName::Woodcutter, "Woodcutter".to_string(), 0, 1, level_up_experience.clone())];
        let mut inventory = Inventory::new();
        let activity_duration = 1000.0;
        let activity_experience = level_up_experience[0];
        let mut activity = Activity::new(ActivityName::Woodcutting, "Cutting down trees".to_string(), activity_duration, vec![(JobName::Woodcutter, activity_experience)], vec![Item::new("Wood".to_string(), "Wood".to_string(), 1)]);
        activity.update(500.0, &mut jobs, &mut inventory).unwrap();
        
        /// level 2 -> 100
        /// level 3 -> 200
        /// level 4 -> 400
        /// level 5 -> 800
        /// so 500 + 9700 = 10200 which should be 10 cycles and 200 time left over
        /// 10 cycles is 100 * 10 = 1000 xp
        /// so we should level up to 4 and have 300 xp left
        activity.update_from_time_elapsed(9700, &mut jobs, &mut inventory).unwrap();
        
        assert_eq!(activity.timer, 200.0);
        assert_eq!(inventory.items.len(), 1);
        assert_eq!(inventory.items["Wood"].amount, 10);
        assert_eq!(jobs[0].experience, 300);
        assert_eq!(jobs[0].level, 4);
    }
}
