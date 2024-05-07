use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

use obws::{responses::{recording::RecordStatus, streaming::StreamStatus}, Client};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct output {
    pub stream: Option<StreamStatus>,
    pub recording: Option<RecordStatus>
}