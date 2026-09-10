use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub is_kids: bool,
    pub is_default: Option<bool>,
    pub pin: Option<String>,
    pub sort_order: i32,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            id: "default-profile".to_string(),
            name: "User".to_string(),
            avatar_url: None,
            is_kids: false,
            is_default: None,
            pin: None,
            sort_order: 0,
        }
    }
}
