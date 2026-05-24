use crate::utils::config::APP_ID;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

const CONFIG_NAME: &str = "usage_stats";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UsageStats {
    pub counts: HashMap<String, u64>,
}

impl UsageStats {
    pub fn load() -> Self {
        confy::load(APP_ID, Some(CONFIG_NAME)).unwrap_or_default()
    }

    pub fn save(&self) {
        confy::store(APP_ID, Some(CONFIG_NAME), self).expect("Failed to save usage stats")
    }

    pub fn increment(&mut self, path: &Path) {
        let key = path.to_string_lossy().into_owned();
        *self.counts.entry(key).or_insert(0) += 1;
        self.save();
    }

    pub fn get(&self, path: &Path) -> u64 {
        let key = path.to_string_lossy();
        self.counts.get(key.as_ref()).copied().unwrap_or(0)
    }

    pub fn prune(&mut self, root: &Path) {
        self.counts
            .retain(|key, _| Path::new(key).starts_with(root));
        self.save();
    }
}
