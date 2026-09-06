use actix_web::{http::StatusCode, HttpResponse};
use mediatr::MediatrError;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ValidationProblem {
    pub detail: String,
    pub pointer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProblemDetails {
    #[serde(rename = "type")]
    #[schema(rename = "type")]
    pub problem_type: String,
    pub title: String,
    pub status: u16,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ValidationProblem>>,
}

pub fn from_mediatr_error(err: MediatrError) -> (StatusCode, ProblemDetails) {
    match err {
        MediatrError::ValidationFailed(errors) => {
            let problems: Vec<ValidationProblem> = errors
                .into_iter()
                .map(|e| ValidationProblem {
                    detail: e.error_message,
                    pointer: e.field.to_string(),
                    code: Some(e.code),
                })
                .collect();

            (
                StatusCode::BAD_REQUEST,
                ProblemDetails {
                    problem_type: "/problems/validation".to_string(),
                    title: "Validation Failed".to_string(),
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    detail: "One or more validation errors occurred.".to_string(),
                    errors: Some(problems),
                },
            )
        }
        MediatrError::HandlerNotFound(name) => (
            StatusCode::NOT_IMPLEMENTED,
            ProblemDetails {
                problem_type: "/problems/not-found".to_string(),
                title: "Query or comamnd not found".to_string(),
                status: StatusCode::NOT_IMPLEMENTED.as_u16(),
                detail: format!("Query or comamnd not found: {}", name),
                errors: None,
            },
        ),
        MediatrError::HandlerFailed(message) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            ProblemDetails {
                problem_type: "about:blank".to_string(),
                title: "Internal Server Error".to_string(),
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                detail: message,
                errors: None,
            },
        ),
    }
}

pub fn into_response(err: MediatrError) -> HttpResponse {
    let (status, body) = from_mediatr_error(err);
    HttpResponse::build(status)
        .content_type("application/problem+json")
        .json(body)
}
