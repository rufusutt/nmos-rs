use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use nmos_rs_schema::is_04;

#[derive(Debug)]
pub struct ServiceError {
    status: StatusCode,
    debug: Option<String>,
}

impl ServiceError {
    pub fn new(status: StatusCode, debug: Option<String>) -> Self {
        Self { status, debug }
    }
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let error = is_04::v1_0_x::Error {
            code: self.status.as_u16() as i64,
            debug: self.debug,
            error: self.status.to_string(),
        };
        let body = Json(error);
        (self.status, body).into_response()
    }
}
