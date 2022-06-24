use crate::{
    auth,
    db::{ProjectId, User, UserId},
    rpc::{self, ResultExt},
    AppState, Error, Result,
};
use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{Path, Query},
    http::{self, Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post, put},
    Extension, Json, Router,
};
use axum_extra::response::ErasedJson;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use time::OffsetDateTime;
use tower::ServiceBuilder;
use tracing::instrument;

pub fn routes(rpc_server: &Arc<rpc::Server>, state: Arc<AppState>) -> Router<Body> {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route(
            "/users/:id",
            put(update_user).delete(destroy_user).get(get_user),
        )
        .route("/users/:id/access_tokens", post(create_access_token))
        .route("/bulk_users", post(create_users))
        .route("/users_with_no_invites", get(get_users_with_no_invites))
        .route("/invite_codes/:code", get(get_user_for_invite_code))
        .route("/panic", post(trace_panic))
        .route("/rpc_server_snapshot", get(get_rpc_server_snapshot))
        .route(
            "/project_activity_summary",
            get(get_project_activity_summary),
        )
        .route("/project_metadata", get(get_project_metadata))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(state))
                .layer(Extension(rpc_server.clone()))
                .layer(middleware::from_fn(validate_api_token)),
        )
}

pub async fn validate_api_token<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let token = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| {
            Error::Http(
                StatusCode::BAD_REQUEST,
                "missing authorization header".to_string(),
            )
        })?
        .strip_prefix("token ")
        .ok_or_else(|| {
            Error::Http(
                StatusCode::BAD_REQUEST,
                "invalid authorization header".to_string(),
            )
        })?;

    let state = req.extensions().get::<Arc<AppState>>().unwrap();

    if token != state.api_token {
        Err(Error::Http(
            StatusCode::UNAUTHORIZED,
            "invalid authorization token".to_string(),
        ))?
    }

    Ok::<_, Error>(next.run(req).await)
}

#[derive(Debug, Deserialize)]
struct GetUsersQueryParams {
    query: Option<String>,
    page: Option<u32>,
    limit: Option<u32>,
}

async fn get_users(
    Query(params): Query<GetUsersQueryParams>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<Json<Vec<User>>> {
    let limit = params.limit.unwrap_or(100);
    let users = if let Some(query) = params.query {
        app.db.fuzzy_search_users(&query, limit).await?
    } else {
        app.db
            .get_all_users(params.page.unwrap_or(0), limit)
            .await?
    };
    Ok(Json(users))
}

#[derive(Deserialize, Debug)]
struct CreateUserParams {
    github_login: String,
    invite_code: Option<String>,
    email_address: Option<String>,
    admin: bool,
}

async fn create_user(
    Json(params): Json<CreateUserParams>,
    Extension(app): Extension<Arc<AppState>>,
    Extension(rpc_server): Extension<Arc<rpc::Server>>,
) -> Result<Json<User>> {
    let user_id = if let Some(invite_code) = params.invite_code {
        let invitee_id = app
            .db
            .redeem_invite_code(
                &invite_code,
                &params.github_login,
                params.email_address.as_deref(),
            )
            .await?;
        rpc_server
            .invite_code_redeemed(&invite_code, invitee_id)
            .await
            .trace_err();
        invitee_id
    } else {
        app.db
            .create_user(
                &params.github_login,
                params.email_address.as_deref(),
                params.admin,
            )
            .await?
    };

    let user = app
        .db
        .get_user_by_id(user_id)
        .await?
        .ok_or_else(|| anyhow!("couldn't find the user we just created"))?;

    Ok(Json(user))
}

#[derive(Deserialize)]
struct UpdateUserParams {
    admin: Option<bool>,
    invite_count: Option<u32>,
}

async fn update_user(
    Path(user_id): Path<i32>,
    Json(params): Json<UpdateUserParams>,
    Extension(app): Extension<Arc<AppState>>,
    Extension(rpc_server): Extension<Arc<rpc::Server>>,
) -> Result<()> {
    let user_id = UserId(user_id);

    if let Some(admin) = params.admin {
        app.db.set_user_is_admin(user_id, admin).await?;
    }

    if let Some(invite_count) = params.invite_count {
        app.db.set_invite_count(user_id, invite_count).await?;
        rpc_server.invite_count_updated(user_id).await.trace_err();
    }

    Ok(())
}

async fn destroy_user(
    Path(user_id): Path<i32>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<()> {
    app.db.destroy_user(UserId(user_id)).await?;
    Ok(())
}

async fn get_user(
    Path(login): Path<String>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<Json<User>> {
    let user = app
        .db
        .get_user_by_github_login(&login)
        .await?
        .ok_or_else(|| Error::Http(StatusCode::NOT_FOUND, "User not found".to_string()))?;
    Ok(Json(user))
}

#[derive(Deserialize)]
struct CreateUsersParams {
    users: Vec<CreateUsersEntry>,
}

#[derive(Deserialize)]
struct CreateUsersEntry {
    github_login: String,
    email_address: String,
    invite_count: usize,
}

async fn create_users(
    Json(params): Json<CreateUsersParams>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<Json<Vec<User>>> {
    let user_ids = app
        .db
        .create_users(
            params
                .users
                .into_iter()
                .map(|params| {
                    (
                        params.github_login,
                        params.email_address,
                        params.invite_count,
                    )
                })
                .collect(),
        )
        .await?;
    let users = app.db.get_users_by_ids(user_ids).await?;
    Ok(Json(users))
}

struct GetUsersWithNoInvites {
    invited_by_another_user: bool,
}

async fn get_users_with_no_invites(
    Query(params): Query<GetUsersWithNoInvites>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<Json<Vec<User>>> {
    Ok(Json(
        app.db
            .get_users_with_no_invites(params.invited_by_another_user)
            .await?,
    ))
}

#[derive(Debug, Deserialize)]
struct Panic {
    version: String,
    text: String,
}

#[instrument(skip(panic))]
async fn trace_panic(panic: Json<Panic>) -> Result<()> {
    tracing::error!(version = %panic.version, text = %panic.text, "panic report");
    Ok(())
}

async fn get_rpc_server_snapshot(
    Extension(rpc_server): Extension<Arc<rpc::Server>>,
) -> Result<ErasedJson> {
    Ok(ErasedJson::pretty(rpc_server.snapshot().await))
}

#[derive(Deserialize)]
struct GetProjectActivityParams {
    #[serde(with = "time::serde::iso8601")]
    start: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    end: OffsetDateTime,
}

async fn get_project_activity_summary(
    Query(params): Query<GetProjectActivityParams>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<ErasedJson> {
    let summary = app
        .db
        .summarize_project_activity(params.start..params.end, 100)
        .await?;
    Ok(ErasedJson::pretty(summary))
}

#[derive(Deserialize)]
struct GetProjectMetadataParams {
    project_id: u64,
}

async fn get_project_metadata(
    Query(params): Query<GetProjectMetadataParams>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<ErasedJson> {
    let extensions = app
        .db
        .get_project_extensions(ProjectId::from_proto(params.project_id))
        .await?;
    Ok(ErasedJson::pretty(json!({ "extensions": extensions })))
}

#[derive(Deserialize)]
struct CreateAccessTokenQueryParams {
    public_key: String,
    impersonate: Option<String>,
}

#[derive(Serialize)]
struct CreateAccessTokenResponse {
    user_id: UserId,
    encrypted_access_token: String,
}

async fn create_access_token(
    Path(login): Path<String>,
    Query(params): Query<CreateAccessTokenQueryParams>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<Json<CreateAccessTokenResponse>> {
    //     request.require_token().await?;

    let user = app
        .db
        .get_user_by_github_login(&login)
        .await?
        .ok_or_else(|| anyhow!("user not found"))?;

    let mut user_id = user.id;
    if let Some(impersonate) = params.impersonate {
        if user.admin {
            if let Some(impersonated_user) = app.db.get_user_by_github_login(&impersonate).await? {
                user_id = impersonated_user.id;
            } else {
                return Err(Error::Http(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    format!("user {impersonate} does not exist"),
                ));
            }
        } else {
            return Err(Error::Http(
                StatusCode::UNAUTHORIZED,
                format!("you do not have permission to impersonate other users"),
            ));
        }
    }

    let access_token = auth::create_access_token(app.db.as_ref(), user_id).await?;
    let encrypted_access_token =
        auth::encrypt_access_token(&access_token, params.public_key.clone())?;

    Ok(Json(CreateAccessTokenResponse {
        user_id,
        encrypted_access_token,
    }))
}

async fn get_user_for_invite_code(
    Path(code): Path<String>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<Json<User>> {
    Ok(Json(app.db.get_user_for_invite_code(&code).await?))
}
