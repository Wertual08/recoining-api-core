pub struct RegistryTransactionUpdateDto {
    pub id: i64, 
    pub source_pack: i64,
    pub target_pack: i64, 
    pub source_updated_at: i64,
    pub target_updated_at: i64,
    pub source_sequence: i16,
    pub target_sequence: i16, 
}