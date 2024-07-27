use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SystemFetch {
    full_qualified_name: String,
    // todo
}

impl SystemFetch {
    pub fn new() -> SystemFetch {
        SystemFetch {
            full_qualified_name: "bebra".to_string(),
        }
    }
}

impl Default for SystemFetch {
    fn default() -> Self {
        Self::new()
    }
}
