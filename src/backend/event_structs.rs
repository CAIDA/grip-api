use std::fmt;
use elastic::prelude::*;
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug, ElasticType, Serialize, Deserialize, Clone, Default)]
pub struct Event {
    event_type: Option<String>,
    id: Option<String>,
    fingerprint: Option<String>,
    view_ts: Option<String>,
    position: Option<String>,
    pfx_events_cnt: Option<i32>,
    tags: Option<serde_json::Value>,
    pfx_events: Option<serde_json::Value>,
}