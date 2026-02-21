use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub name: String,
    pub repo: String,
    pub size: i64,
    pub prefix: String,
    pub suffix: String,
    pub eot: String,
    pub sys: String,
    pub us: String,
    pub ast: String,
    pub supports_vision: bool,
    pub params: String,
}
