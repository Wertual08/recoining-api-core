use crate::storage::registries::RegistryDto;

use super::RegistryVariantModel;

#[derive(Clone)]
pub struct RegistryModel {
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub current_pack: i64,
    pub variant: RegistryVariantModel,
    pub current_sequence: i16, 
    pub name: String,
    pub image: String,
}

impl RegistryModel {
    pub fn direct(timestamp: i64, id: i64, name: String, image: String) -> Self {
        Self {
            id,
            created_at: timestamp,
            updated_at: timestamp,
            current_pack: 0,
            variant: RegistryVariantModel::Direct,
            current_sequence: -1,
            name,
            image,
        }
    }
}

impl From<RegistryDto> for RegistryModel {
    fn from(dto: RegistryDto) -> Self {
        Self {
            id: dto.id,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
            current_pack: dto.current_pack,
            variant: dto.variant.into(),
            current_sequence: dto.current_sequence,
            name: dto.name,
            image: dto.image,
        }
    }
}

impl From<RegistryModel> for RegistryDto {
    fn from(model: RegistryModel) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            updated_at: model.updated_at,
            current_pack: model.current_pack,
            variant: model.variant.into(),
            current_sequence: model.current_sequence,
            name: model.name,
            image: model.image,
        }
    }
}