use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub name: String,
    pub repo: String,
    pub size: f32,
    pub supports_vision: bool,
    pub params: String,
}
