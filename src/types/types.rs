use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Todo {
    pub map: HashMap<String, (bool, String, u64)>,
}
