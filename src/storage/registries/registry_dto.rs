pub struct RegistryDto {
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub current_pack: i64,
    pub current_sequence: i16, 
    pub variant: i16,
    pub name: String,
    pub image: String,
}

pub const REGISTRY_DIRECT: i16 = 1;