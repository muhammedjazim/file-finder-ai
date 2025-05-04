#[derive(Debug)]
pub struct FileEvent {
    pub event_kind: String,
    pub path: String,
    pub extension: String
}