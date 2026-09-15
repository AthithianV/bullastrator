use std::collections::HashMap;

use serde_json::Value;

use crate::{
    bull_drivers::bullmq_driver_v5::BullmqV5Driver,
    model::job_model::{JobOptions, JobStatus},
};

pub trait BullmqDriver: Send + Sync {
    fn version(&self) -> &str;

    // --- Queues ---
    fn get_queues(&self) -> &'static str;

    // --- Job Lifecycle ---
    fn add(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str) -> Vec<String> + Send + Sync>,
    );
    fn add_job_args(
        &self,
        prefix: &str,
        queue_name: &str,
        job_name: &str,
        data: &Value,
        opts: &JobOptions,
    ) -> Vec<Vec<u8>>;
    fn add_delayed(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str) -> Vec<String> + Send + Sync>,
    );
    fn add_prioritized(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str) -> Vec<String> + Send + Sync>,
    );
    fn remove(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str, &str) -> Vec<String> + Send + Sync>,
    );
    fn retry(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str, &str) -> Vec<String> + Send + Sync>,
        Box<dyn Fn(&str, bool) -> Vec<String> + Send + Sync>, // Args generator
    );
    fn reprocess(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str, &str) -> Vec<String> + Send + Sync>,
        Box<dyn Fn(&str, bool) -> Vec<String> + Send + Sync>,
    );
    fn promote(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str) -> Vec<String> + Send + Sync>,
    );

    // --- Queue Management ---
    fn pause(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str, bool) -> Vec<String> + Send + Sync>,
    );
    fn clean(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str, &str) -> Vec<String> + Send + Sync>,
    );

    // --- Advanced / Maintenance ---
    fn update(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str, &str) -> String + Send + Sync>,
    );

    // -- Monitoring --
    fn get_counts(&self) -> &'static str;

    fn search(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str, JobStatus) -> Vec<String> + Send + Sync>,
    );
    fn get(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str, &str) -> String + Send + Sync>,
    );

    fn get_job_data(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str, &str) -> String + Send + Sync>,
    );

    fn get_job_logs(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str, &str) -> Vec<String> + Send + Sync>,
    );

    // --- State Transitions ---
    // Used for moving jobs manually (e.g., from 'failed' back to 'wait')
    // fn move_to_active(
    //     &self,
    // ) -> (
    //     &'static str,
    //     Box<dyn Fn(&str, &str) -> Vec<String> + Send + Sync>,
    // );
    // fn move_to_completed(
    //     &self,
    // ) -> (
    //     &'static str,
    //     Box<dyn Fn(&str, &str) -> Vec<String> + Send + Sync>,
    // );
    // fn move_to_failed(
    //     &self,
    // ) -> (
    //     &'static str,
    //     Box<dyn Fn(&str, &str) -> Vec<String> + Send + Sync>,
    // );

    // fn move_stalled(
    //     &self,
    // ) -> (
    //     &'static str,
    //     Box<dyn Fn(&str, &str) -> Vec<String> + Send + Sync>,
    // );

    // fn obliterate_queue(
    //     &self,
    // ) -> (
    //     &'static str,
    //     Box<dyn Fn(&str, &str) -> Vec<String> + Send + Sync>,
    // );
}

pub struct DriverRegistry {
    // We store our drivers here
    drivers: HashMap<String, Box<dyn BullmqDriver>>,
}

impl DriverRegistry {
    pub fn new() -> Self {
        let mut drivers: HashMap<String, Box<dyn BullmqDriver>> = HashMap::new();

        // Populate the address book
        drivers.insert("5.x".to_string(), Box::new(BullmqV5Driver));

        Self { drivers }
    }

    pub fn get(&self, version: &str) -> Option<&Box<dyn BullmqDriver>> {
        // Find the right driver for the version
        self.drivers.get(version)
    }
}
