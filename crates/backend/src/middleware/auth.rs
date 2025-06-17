use std::{str::FromStr, sync::Arc};

use axum::{
    body::Body,
    extract::{FromRequestParts, Request},
    http::StatusCode,
    response::Response,
};
use model::session::Session as SessionModel;
use mongodb::{BoxFuture, bson::oid::ObjectId};
use redis_om::HashModel;
use tower::{Layer, Service};
use uuid::Uuid;

use crate::{
    routes::auth::{SESSION_ID_COOKIE_NAME, SessionId},
    state::AppState,
};
#[derive(Clone)]
pub struct AuthMiddlewareLayer {
    pub state: Arc<AppState>,
}

impl<S> Layer<S> for AuthMiddlewareLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware {
            inner,
            state: Arc::clone(&self.state),
        }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
    state: Arc<AppState>,
}

impl<S> Service<Request<Body>> for AuthMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<Body>> + Send + Clone + 'static,
    S::Future: Send + 'static,
{
    type Response = Response<Body>;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<Body>) -> Self::Future {
        let Some(cookies) = request
            .headers()
            .get("Cookie")
            .and_then(|header| header.to_str().ok())
            .map(cookie::Cookie::split_parse)
        else {
            let fut = self.inner.call(request);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        };
        let Some(session_cookie) = cookies
            .flat_map(|c| c.ok())
            .find(|c| c.name() == SESSION_ID_COOKIE_NAME)
        else {
            let fut = self.inner.call(request);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        };

        let Some((session_id, signature)) = session_cookie.value().split_once('.') else {
            let fut = self.inner.call(request);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        };

        let session_id: Option<SessionId> = if self
            .state
            .crypto()
            .verify_session_signature(session_id.as_bytes(), signature.as_bytes())
            .unwrap_or(false)
        {
            Uuid::parse_str(session_id).ok()
        } else {
            None
        };
        let mut inner = self.inner.clone();
        let mut conn = self.state.storage().cache().connection();
        Box::pin(async move {
            if let Some(session_id) = session_id {
                let session = SessionModel::get(&session_id.to_string(), &mut conn).await;
                if let Ok(session) = session {
                    request.extensions_mut().insert::<SessionModel>(session);
                }
            }

            let res = inner.call(request).await?;

            Ok(res)
        })
    }
}

pub struct SessionData {
    pub session_id: String,
    pub user_id: ObjectId,
}

pub struct Auth(pub SessionData);

impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _: &S,
    ) -> Result<Self, Self::Rejection> {
        let s = parts
            .extensions
            .get::<SessionModel>()
            .ok_or(StatusCode::UNAUTHORIZED)?
            .clone();
        Ok(Self(SessionData {
            session_id: s.id,
            user_id: ObjectId::from_str(&s.user_id).unwrap(),
        }))
    }
}
