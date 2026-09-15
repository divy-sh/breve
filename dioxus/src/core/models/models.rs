use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub name: String,
    pub repo: String,
    pub size: f32,
    pub is_thinking: bool,
    pub params: String,
    pub is_premium: bool,
}
