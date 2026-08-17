

use crate::identity::{entities::avatar_history::AvatarHistory, value_objects::avatar::Avatar};

pub struct User {
    pub id: u32,
    pub public_id: String,
    pub username: String,
    pub email: String,
    pub avatar: Option<Avatar>,
    pub avatar_history: Vec<AvatarHistory>,
}

impl User {
    pub fn new(
        public_id: String,
        username: String,
        email: String,
    ) -> Self {
        Self {
            id: 0,
            public_id,
            username,
            email,
            avatar: None,
            avatar_history: Vec::new(),
        }
    }

    pub fn change_avatar(&mut self, avatar: String) {
        
        if let Some(current_avatar) = &self.avatar {
               let avatar_history = AvatarHistory {
                   url: current_avatar.url.clone(),
                   last_modified: chrono::Utc::now(),
               };
       
               self.avatar_history.push(avatar_history);
           }
    
        self.avatar = Some(Avatar { url: avatar });
    }
}
