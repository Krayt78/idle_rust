use crate::activity::Activity;
use crate::inventory::Inventory;
use crate::item::Item;
use crate::job::Job;
use crate::job::JobName;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Player {
    pub health: u8,
    pub mana: u8,
    pub attack_power: u8,
    pub defense: u8,
    pub level: u8,
    pub jobs: Vec<Job>,
    pub gold: u8,
    pub inventory: Inventory,
    pub current_activity: Option<Activity>,
}

impl Player {
    pub fn new() -> Self {
        // 100 levels, each level requires 100 * 2^level experience or each level is double the previous level
        let level_up_experience: Vec<u128> = (0..100).map(|i| 100 * 2u128.pow(i as u32)).collect();
        let jobs = vec![
            Job::new(JobName::Woodcutter, "Cut down trees".to_string(), 0, 1, level_up_experience.clone()),
            Job::new(JobName::Miner, "Mine rocks".to_string(), 0, 1, level_up_experience.clone()),
            Job::new(JobName::Farmer, "Grow crops".to_string(), 0, 1, level_up_experience.clone()),
        ];
        Self {
            health: 100,
            mana: 100,
            attack_power: 1,
            defense: 1,
            level: 1,
            jobs: jobs,
            gold: 0,
            inventory: Inventory::new(),
            current_activity: None,
        }
    }

    pub fn update(&mut self, delta_time: f32) -> Result<(), String> {
        // Update player stats based on current occupation
        match &mut self.current_activity {
            Some(activity) => {
                activity.update(delta_time, &mut self.jobs, &mut self.inventory)?;
            }
            None => {}
        }
        Ok(())
    }

    pub fn update_from_time_elapsed(&mut self, time_elapsed: u64) -> Result<(), String> {
        match &mut self.current_activity {
            Some(activity) => activity.update_from_time_elapsed(time_elapsed, &mut self.jobs, &mut self.inventory)?,
            None => {}
        }
        Ok(())
    }

    pub fn set_activity(&mut self, activity: Activity) {
        self.current_activity = Some(activity);
    }

    pub fn get_activity(&self) -> Option<&Activity> {
        self.current_activity.as_ref()
    }

    pub fn get_jobs(&self) -> &Vec<Job> {
        &self.jobs
    }

    pub fn get_inventory(&self) -> &Inventory {
        &self.inventory
    }
}
