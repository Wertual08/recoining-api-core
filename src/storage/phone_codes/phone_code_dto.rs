use rand::{rngs::OsRng, RngCore};

pub struct PhoneCodeDto {
    pub phone: i64,
    pub code: i64,
    pub created_at: i64,
    pub attempts: i16,
    pub ttl: i32,
}

impl PhoneCodeDto {
    pub fn new(phone: i64, max_code: i64, ttl: i32) -> Self {
        Self {
            created_at: -1,
            phone,
            code: (OsRng.next_u64() % max_code as u64) as i64,
            attempts: 0,
            ttl,
        }
    }
}