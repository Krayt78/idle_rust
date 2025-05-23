use crate::activity::Activity;
use crate::constants::LEVEL_UP_EXPERIENCE;
use crate::inventory::Inventory;
use crate::job::Job;
use crate::job::JobName;
use crate::item::Item;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Player {
    pub health: u8,
    pub mana: u8,
    pub attack_power: u8,
    pub defense: u8,
    pub level: u8,
    pub jobs: Vec<Job>,
    pub inventory: Inventory,
    pub current_activity: Option<Activity>,
}

impl Player {
    pub fn new() -> Self {
        let jobs = vec![
            Job::new(
                JobName::Woodcutter,
                "Cut down trees".to_string(),
                0,
                1,
                LEVEL_UP_EXPERIENCE.to_vec(),
            ),
            Job::new(
                JobName::Miner,
                "Mine rocks".to_string(),
                0,
                1,
                LEVEL_UP_EXPERIENCE.to_vec(),
            ),
            Job::new(
                JobName::Farmer,
                "Grow crops".to_string(),
                0,
                1,
                LEVEL_UP_EXPERIENCE.to_vec(),
            ),
        ];
        Self {
            health: 100,
            mana: 100,
            attack_power: 1,
            defense: 1,
            level: 1,
            jobs: jobs,
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
            Some(activity) => activity.update_from_time_elapsed(
                time_elapsed,
                &mut self.jobs,
                &mut self.inventory,
            )?,
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

    pub fn get_job(&self, job_name: JobName) -> Option<&Job> {
        self.jobs.iter().find(|job| job.name == job_name)
    }

    pub fn get_inventory(&self) -> &Inventory {
        &self.inventory
    }

    pub fn add_experience(&mut self, job_name: JobName, experience: u128) {
        let job = self.jobs.iter_mut().find(|job| job.name == job_name).unwrap();
        job.add_experience(experience);
    }

    pub fn add_item(&mut self, item: &Item) {
        self.inventory.add_item(item);
    }

    pub fn add_gold(&mut self, amount: u128) {
        self.inventory.add_gold(amount);
    }

    pub fn remove_gold(&mut self, amount: u128) -> Result<(), String> {
        self.inventory.remove_gold(amount)
    }
}
