use anyhow::anyhow;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use futures::{StreamExt, TryStreamExt};
use model::{message::ChatMessageContent, share::Share};
use mongodb::bson::{doc, oid::ObjectId};
use redis_om::HashModel;
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    errors::{
        ApplicationError,
        storage::{StorageError, database::DatabaseError},
    },
    middleware::auth::Auth,
    payload::chat::{ChatMessageContentPayload, ChatMessagePayload},
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct ListChatMessagesPayload {
    pub start: usize,
    pub take: usize,
    pub share_id: Option<ObjectId>,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Auth(session): Auth,
    Path(chat_id): Path<ObjectId>,
    Query(payload): Query<ListChatMessagesPayload>,
) -> Result<impl IntoResponse, ApplicationError> {
    let chat = if let Some(id) = payload.share_id {
        let mut conn = state.storage().cache().connection();
        let share = Share::get(chat_id.to_hex(), &mut conn).await.unwrap();
        if id.to_hex() != share.share_id {
            return Err(ApplicationError::InvalidShareLink);
        }
        state
            .storage()
            .database()
            .chats
            .get(doc! { "_id": chat_id })
            .await
    } else {
        state
            .storage()
            .database()
            .chats
            .get(doc! { "user_id": session.user_id, "_id": chat_id })
            .await
    }
    .map_err(|e| {
        ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
    })?;

    let Some(chat) = chat else {
        return Err(ApplicationError::StorageError(StorageError::DatabaseError(
            DatabaseError::ChatDoesNotExist,
        )));
    };

    let messages = state
        .storage()
        .database()
        .messages
        .get_many_sorted(
            doc! { "chat_id": chat.id.unwrap() },
            doc! { "timestamp": -1 },
        )
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(e)))
        })?;

    let messages = messages
        .skip(payload.start)
        .take(payload.take)
        .map(|message| {
            message.map(|msg| ChatMessagePayload {
                id: msg.id.unwrap(),
                content: msg
                    .content
                    .into_iter()
                    .map(|message| match message {
                        ChatMessageContent::Text { value } => {
                            ChatMessageContentPayload::Text { value }
                        }
                        ChatMessageContent::Image { id } => ChatMessageContentPayload::Image { id },
                        ChatMessageContent::Pdf { id } => ChatMessageContentPayload::Pdf { id },
                    })
                    .collect(),
                model: msg.model,
                timestamp: chat.timestamp,
                reasoning: msg.reasoning,
                updated_memory: msg.updated_memory,
                chat_id: msg.chat_id,
                role: msg.role,
            })
        })
        .try_collect::<Vec<_>>()
        .await
        .map_err(|e| {
            ApplicationError::StorageError(StorageError::DatabaseError(DatabaseError::Unknown(
                anyhow!(e),
            )))
        })?;

    Ok((StatusCode::OK, Json(messages)).into_response())
}
