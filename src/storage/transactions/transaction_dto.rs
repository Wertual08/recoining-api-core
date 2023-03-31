use bigdecimal::BigDecimal;

pub struct TransactionDto {
    pub registry_id: i64,
    pub pack: i64,
    pub created_at: i64,
    pub source_user_id: i64,
    pub target_user_id: i64,
    pub sequence: i16,
    pub variant: i16,
    pub amount: BigDecimal,
    pub currency: String,
    pub label: String,
    pub description: String,
    pub hash: Vec<u8>,
}