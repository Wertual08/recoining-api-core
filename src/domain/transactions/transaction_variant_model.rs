#[derive(Clone, Copy)]
pub enum TransactionVariantModel {
    Invalid,
    Basic,
}

impl From<i16> for TransactionVariantModel {
    fn from(variant: i16) -> Self {
        match variant {
            1 => TransactionVariantModel::Basic,
            _ => TransactionVariantModel::Invalid,
        }
    }
}

impl From<TransactionVariantModel> for i16 {
    fn from(variant: TransactionVariantModel) -> Self {
        match variant {
            TransactionVariantModel::Basic => 1,
            TransactionVariantModel::Invalid => 0,
        }
    }
}