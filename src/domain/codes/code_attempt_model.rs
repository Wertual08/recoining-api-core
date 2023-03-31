pub enum CodeAttemptModel {
    Success,
    Absent,
    Fail(i16),
    Retry,
}