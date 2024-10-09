use anyhow::anyhow;
use axum::{
    extract::rejection::{JsonRejection, QueryRejection},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub(crate) struct AppError {
    code: StatusCode,
    source: anyhow::Error,
}

impl AppError {
    pub(crate) fn new(code: StatusCode, source: anyhow::Error) -> Self {
        Self { code, source }
    }

    pub(crate) fn service(source: anyhow::Error) -> Self {
        Self::new(StatusCode::SERVICE_UNAVAILABLE, source)
    }
}

impl From<JsonRejection> for AppError {
    fn from(err: JsonRejection) -> Self {
        match err {
            JsonRejection::JsonDataError(err) => Self {
                code: StatusCode::BAD_REQUEST,
                source: anyhow!(err),
            },
            err => Self {
                code: err.status(),
                source: anyhow::anyhow!(err),
            },
        }
    }
}

impl From<QueryRejection> for AppError {
    fn from(err: QueryRejection) -> Self {
        match err {
            QueryRejection::FailedToDeserializeQueryString(err) => Self {
                code: StatusCode::BAD_REQUEST,
                source: anyhow!(err),
            },
            err => Self {
                code: err.status(),
                source: anyhow!(err),
            },
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (code, message) = (self.code, format!("{:?}", self.source));
        let body = Json(serde_json::json!({ "error": message }));

        (code, body).into_response()
    }
}
