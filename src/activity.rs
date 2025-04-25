use crate::item::Item;
use crate::job::JobName;
use crate::job::Job;
use crate::inventory::Inventory;

pub struct Activity {
    pub name: String,
    pub description: String,
    pub duration: u128,
    pub timer: u128,
    pub experience: Vec<(JobName, u128)>,
    pub items: Vec<Item>,
}

impl Activity {
    pub fn new(name: String, description: String, duration: u128, experience: Vec<(JobName, u128)>, items: Vec<Item>) -> Self {
        Self { name, description, duration, experience, timer: 0, items }
    }

    pub fn update(&mut self, delta_time: u128, jobs: &mut Vec<Job>, inventory: &mut Inventory) {
        self.timer += delta_time;
        if self.timer >= self.duration {
            self.timer = 0;

            for (job, experience) in &self.experience {
                jobs.iter_mut().find(|j| j.name == *job).unwrap().experience += experience;
            }

            for item in &self.items {
                inventory.add_item(item);
            }
        }
    }
}

mod tests {
    use super::*;
    
    #[test]
    fn test_activity_new() {
        let activity = Activity::new("Woodcutting".to_string(), "Cutting down trees".to_string(), 1000, vec![(JobName::Woodcutter, 100)], vec![Item::new("Wood".to_string(), "Wood".to_string(), 1)]);
        assert_eq!(activity.name, "Woodcutting");
        assert_eq!(activity.description, "Cutting down trees");
        assert_eq!(activity.duration, 1000);
        assert_eq!(activity.experience, vec![(JobName::Woodcutter, 100)]);
        assert_eq!(activity.items, vec![Item::new("Wood".to_string(), "Wood".to_string(), 1)]);
    }

    #[test]
    fn test_activity_update() {
        let mut jobs = vec![Job::new(JobName::Woodcutter, "Woodcutter".to_string(), 0)];
        let mut inventory = Inventory::new();
        let mut activity = Activity::new("Woodcutting".to_string(), "Cutting down trees".to_string(), 1000, vec![(JobName::Woodcutter, 100)], vec![Item::new("Wood".to_string(), "Wood".to_string(), 1)]);
        activity.update(500, &mut jobs, &mut inventory);
        assert_eq!(activity.timer, 500);
        assert_eq!(activity.experience, vec![(JobName::Woodcutter, 100)]);
        assert_eq!(inventory.items.len(), 0);
        assert_eq!(jobs[0].experience, 0);
    }

    #[test]
    fn test_activity_complete() {
        let mut jobs = vec![Job::new(JobName::Woodcutter, "Woodcutter".to_string(), 0)];
        let mut inventory = Inventory::new();
        let mut activity = Activity::new("Woodcutting".to_string(), "Cutting down trees".to_string(), 1000, vec![(JobName::Woodcutter, 100)], vec![Item::new("Wood".to_string(), "Wood".to_string(), 1)]);
        activity.update(1000, &mut jobs, &mut inventory);
        assert_eq!(activity.timer, 0);
        assert_eq!(activity.experience, vec![(JobName::Woodcutter, 100)]);
        assert_eq!(inventory.items.len(), 1);
        assert_eq!(inventory.items["Wood"].amount, 1);
        assert_eq!(jobs[0].experience, 100);
    }

    #[test]
    fn test_activity_update_jobs() {
        let mut jobs = vec![
            Job::new(JobName::Woodcutter, "Woodcutter".to_string(), 0),
            Job::new(JobName::Miner, "Miner".to_string(), 0),
            Job::new(JobName::Farmer, "Farmer".to_string(), 0),
        ];
        let mut inventory = Inventory::new();
        let mut activity = Activity::new(
            "Woodcutting".to_string(),
            "Cutting down trees".to_string(),
            1000,
            vec![
                (JobName::Woodcutter, 100),
                (JobName::Miner, 200),
                (JobName::Farmer, 300),
            ],
            vec![
                Item::new("Wood".to_string(), "Wood".to_string(), 1),
                Item::new("Stone".to_string(), "Stone".to_string(), 2),
                Item::new("Wheat".to_string(), "Wheat".to_string(), 3),
            ]
        );

        activity.update(1000, &mut jobs, &mut inventory);
        assert_eq!(jobs[0].experience, 100);
        assert_eq!(jobs[1].experience, 200);
        assert_eq!(jobs[2].experience, 300);
        assert_eq!(inventory.items.len(), 3);
        assert_eq!(inventory.items["Wood"].amount, 1);
        assert_eq!(inventory.items["Stone"].amount, 2);
        assert_eq!(inventory.items["Wheat"].amount, 3);
    }
}

