use axum::{
    response::Response,
    middleware::Next,
    extract::Request
};

use crate::strategies::authentication::Claims;

pub async fn authenticated(
    _claims: Claims,
    request: Request,
    next: Next,
) -> Response {
    next.run(request).await
}