use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize)]
pub enum ResponseStatus {
    #[default]
    Unknown,
    Successful,
    Unsucessful,
}
#[derive(Serialize, Clone, Debug, Default)]
pub struct DetailedResponse {
    pub status: ResponseStatus,
    pub message: String,
    pub success: bool,
}
#[derive(Serialize, Clone, Debug, Default)]
pub struct GenericResponse {
    pub success: bool,
}
impl GenericResponse {
    pub fn successful() -> Self {
        GenericResponse { success: true }
    }
    pub fn unsuccessful() -> Self {
        GenericResponse { success: false }
    }
}
impl DetailedResponse {
    pub fn rows_affected(&self, rows: usize, message: &str) -> Self {
        DetailedResponse {
            success: rows > 0,
            message: message.to_owned(),
            ..Default::default()
        }
    }
    fn set_message(&mut self, message: &str) -> &mut Self {
        self.message.push_str(message);
        self
    }
    pub fn unsuccessful_with_message(&self, message: &str) -> Self {
        let mut resp: DetailedResponse = DetailedResponse::unsuccessful();
        resp.set_message(message).clone()
    }
    pub fn unsuccessful() -> DetailedResponse {
        DetailedResponse {
            status: ResponseStatus::Unsucessful,
            success: false,
            ..Default::default()
        }
    }
    pub fn successful_with_message(&self, message: &str) -> Self {
        let mut resp: DetailedResponse = DetailedResponse::successful();
        resp.set_message(message).clone()
    }
    pub fn successful() -> DetailedResponse {
        DetailedResponse {
            status: ResponseStatus::Successful,
            success: true,
            ..Default::default()
        }
    }
}
