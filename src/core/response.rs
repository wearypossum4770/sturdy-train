use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GenericResponse {
    pub success: bool,
}

impl GenericResponse {
    pub fn unsuccessful() -> GenericResponse {
        GenericResponse { success: false }
    }
    pub fn successful() -> GenericResponse {
        GenericResponse { success: true }
    }
}
