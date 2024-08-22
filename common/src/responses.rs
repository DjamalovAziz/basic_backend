use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DeleteResponseResult {
    pub status_code: u32,
}
