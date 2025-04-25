use std::cmp::PartialEq;
use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Job {
    pub name: JobName,
    pub description: String,
    pub experience: u128,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
    pub fn new(name: JobName, description: String, experience: u128) -> Self {
        Self {
            name,
            description,
            experience,
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_job_new() {
        let job = Job::new(JobName::Woodcutter, "Woodcutter".to_string(), 100);
        assert_eq!(job.name, JobName::Woodcutter);
        assert_eq!(job.description, "Woodcutter");
        assert_eq!(job.experience, 100);
    }
}
