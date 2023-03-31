use super::TransactionModel;

pub enum TransactionStateModel {
    Fail,
    Pending(TransactionModel),
    Sent(TransactionModel),
}