use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Debug, Default)]
pub struct Dictionary {
    map: RwLock<HashMap<String, String>>,
}

impl Dictionary {
    fn new() -> Self {
        Self::default()
    }

    pub async fn put(&self, key: String, value: String) {
        let mut map = self.map.write().await;
        map.insert(key, value);
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        let map = self.map.read().await;
        map.get(key).cloned()
    }

    pub async fn get_all_as_json(&self) -> String {
        let map = self.map.read().await;
        serde_json::to_string(&*map).unwrap()
    }
}

pub fn new() -> Dictionary {
    Dictionary::new()
}
