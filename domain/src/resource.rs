use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    pub id: Option<Uuid>,
    pub key: String,
    pub tags: Option<Value>,
    pub user_id: Option<Uuid>,
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[allow(dead_code)]
impl Resource {
    pub fn with_key(mut self, key: &str) -> Self {
        self.key = key.to_owned();
        self
    }

    pub fn with_tags(mut self, tags: Map<String, Value>) -> Self {
        self.tags = Some(Value::Object(tags));
        self
    }

    pub fn with_user_id(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_metadata(mut self, metadata: Map<String, Value>) -> Self {
        self.metadata = Some(Value::Object(metadata));
        self
    }
}

impl Default for Resource {
    fn default() -> Self {
        Self {
            id: None,
            key: "".to_owned(),
            tags: None,
            user_id: None,
            metadata: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}