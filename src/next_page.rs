use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NextPage {
    pub offset: String,
    pub path: String,
    pub uri: String,
}