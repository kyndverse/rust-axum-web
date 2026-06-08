use crate::{
    error::AppError,
    models::auth::{Claim, UserContext},
    state::AppState,
};
use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};

tokio::task_local! {
    pub static USER_CONTEXT: UserContext;
}

pub async fn validate_auth(
    State(app_state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::InvalidToken)?;

    let claim = decode::<Claim>(
        token,
        &DecodingKey::from_secret(&app_state.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AppError::InvalidToken)?;

    req.extensions_mut().insert(UserContext {
        id: claim.claims.sub,
    });

    Ok(next.run(req).await)

    // let user_context = UserContext {
    //     id: claim.claims.sub,
    // };

    // Ok(USER_CONTEXT.scope(user_context, next.run(req)).await)
}
