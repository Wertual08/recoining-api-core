pub enum CodeSendModel {
    Success(i64, i64),
    Timeout(i64),
    Retry,
    Fail,
}