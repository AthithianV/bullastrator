use bullastrator_storage::models::Queue;
use serde::{Deserialize, Serialize};

use crate::models::job::QueueJobCounts;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReadQueueWithCounts {
    #[serde(flatten)]
    pub queue: Queue,
    pub counts: QueueJobCounts,
}
