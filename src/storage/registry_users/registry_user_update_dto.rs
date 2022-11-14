use bigdecimal::BigDecimal;

pub struct RegistryUserUpdateDto {
    pub registry_id: i64,
    pub user_id: i64,
    pub updated_at: i64,
    pub current_pack: i64,
    pub current_sequence: i16,
    pub currency: String,
    pub source_value: Option<BigDecimal>,
    pub target_value: BigDecimal,
}