use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    pub gid: String,
    pub name: String,
    pub resource_type: String,
}
