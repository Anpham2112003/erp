use chrono::{DateTime, Utc};


pub struct AvatarHistory {
    pub url: String,
    pub last_modified: DateTime<Utc>,
}
