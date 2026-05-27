use serde::Deserialize;

/// Topic 01 DTO
#[derive(Deserialize)]
pub(super) struct Topic01Dto {
    pub example01: String,
}

/// Topic 02 DTO
#[derive(Deserialize)]
pub(super) struct Topic02Dto {
    pub example01: String,
}
