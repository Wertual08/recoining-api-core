use uuid::Uuid;

pub struct UserTokenDto {
    pub user_id: i64,
    pub id: String,
}

impl UserTokenDto {
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            id: base64_url::encode(Uuid::new_v4().as_bytes()),
        }
    }
}