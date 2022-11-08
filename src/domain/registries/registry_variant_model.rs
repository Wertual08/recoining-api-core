#[derive(Clone)]
pub enum RegistryVariantModel {
    Invalid,
    Direct,
}

impl From<i16> for RegistryVariantModel {
    fn from(id: i16) -> Self {
        match id {
            1 => RegistryVariantModel::Direct,
            _ => RegistryVariantModel::Invalid,
        }
    }
}

impl From<RegistryVariantModel> for i16 {
    fn from(variant: RegistryVariantModel) -> Self {
        match variant {
            RegistryVariantModel::Invalid => 0,
            RegistryVariantModel::Direct => 1,
        }
    }
}