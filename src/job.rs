use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Job {
    pub name: JobName,
    pub description: String,
    pub level: u8,
    pub experience: u128,
    pub level_up_experience: Vec<u128>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum JobName {
    Woodcutter,
    Miner,
    Farmer,
}

impl fmt::Display for JobName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Job {
    pub fn new(
        name: JobName,
        description: String,
        experience: u128,
        level: u8,
        level_up_experience: Vec<u128>,
    ) -> Self {
        Self {
            name,
            description,
            experience,
            level,
            level_up_experience,
        }
    }

    pub fn add_experience(&mut self, amount: u128) {
        self.experience += amount;
        if self.experience >= self.level_up_experience[self.level as usize - 1] {
            self.level_up();
        }
    }

    //need to loop in case we need to level up multiple times
    fn level_up(&mut self) {
        while self.experience >= self.level_up_experience[self.level as usize - 1] {
            self.experience -= self.level_up_experience[self.level as usize - 1];
            self.level += 1;
        }
    }
}

mod tests {
    use super::*;
    use crate::constants::LEVEL_UP_EXPERIENCE;

    #[test]
    fn test_job_new() {
        let job = Job::new(
            JobName::Woodcutter,
            "Woodcutter".to_string(),
            0,
            1,
            LEVEL_UP_EXPERIENCE.to_vec(),
        );
        assert_eq!(job.name, JobName::Woodcutter);
        assert_eq!(job.description, "Woodcutter");
        assert_eq!(job.experience, 0);
        assert_eq!(job.level, 1);
        assert_eq!(job.level_up_experience, LEVEL_UP_EXPERIENCE.to_vec());
    }

    #[test]
    fn test_job_add_experience() {
        let mut job = Job::new(
            JobName::Woodcutter,
            "Woodcutter".to_string(),
            0,
            1,
            LEVEL_UP_EXPERIENCE.to_vec(),
        );
        job.add_experience(50);
        assert_eq!(job.experience, 50);
        assert_eq!(job.level, 1);
    }

    #[test]
    fn test_job_level_up() {
        let mut job = Job::new(
            JobName::Woodcutter,
            "Woodcutter".to_string(),
            0,
            1,
            LEVEL_UP_EXPERIENCE.to_vec(),
        );
        let xp_needed_for_level_2 = LEVEL_UP_EXPERIENCE[0];
        job.add_experience(xp_needed_for_level_2 - 1);
        assert_eq!(job.level, 1);
        assert_eq!(job.experience, xp_needed_for_level_2 - 1);
        job.add_experience(1);
        assert_eq!(job.level, 2);
        assert_eq!(job.experience, 0);
    }

    #[test]
    fn test_job_level_up_multiple_times() {
        let mut job = Job::new(
            JobName::Woodcutter,
            "Woodcutter".to_string(),
            0,
            1,
            LEVEL_UP_EXPERIENCE.to_vec(),
        );
        job.add_experience(
            LEVEL_UP_EXPERIENCE[0] + LEVEL_UP_EXPERIENCE[1] + LEVEL_UP_EXPERIENCE[2] + 10,
        );
        assert_eq!(job.level, 4);
        assert_eq!(job.experience, 10);
    }
}
