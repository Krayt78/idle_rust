use crate::activity::Activity;
use crate::inventory::Inventory;
use crate::item::Item;
use crate::job::Job;
use crate::job::JobName;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
        let jobs = vec![
            Job::new(JobName::Woodcutter, "Cut down trees".to_string(), 0),
            Job::new(JobName::Miner, "Mine rocks".to_string(), 0),
            Job::new(JobName::Farmer, "Grow crops".to_string(), 0),
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

    pub fn update(&mut self, delta_time: f32) {
        // Update player stats based on current occupation
        match &mut self.current_activity {
            Some(activity) => {
                activity.update(delta_time, &mut self.jobs, &mut self.inventory);
            }
            None => {}
        }
    }

    pub fn set_activity(&mut self, activity: Activity) {
        self.current_activity = Some(activity);
    }

    pub fn get_activity(&self) -> Option<&Activity> {
        self.current_activity.as_ref()
    }
}
