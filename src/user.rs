// use std::sync::Arc;

// use diesel::prelude::*;
// use diesel::r2d2::{self, ConnectionManager};
// use serde_json::json;
// use teloxide::types::{ChatKind, ChatId, Message};
// use crate::models::{User, Request};
// use crate::schema::{users, requests};
// use chrono::Utc;

// type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// pub async fn handle_user(pool: Arc<DbPool>, message: &Message) -> Result<(i32), diesel::result::Error> {
//     let conn = pool.get().expect("Failed to get DB connection from pool");
//     let user_data = message.from();
//     let chat_kind = match &message.chat.kind {
//         ChatKind::Private(_) => "private",
//         ChatKind::Public(_) => "public",
//         _ => "unknown",
//     };
//     let chat_data = json!({ "id": chat_id, "type": chat_kind.to_string() });
//     let epoch_time = message.date;

//     save_new_user(pool, user_data, chat_data, epoch_time).await
// }

// pub async fn save_new_user(pool: DbPool, user_data: serde_json::Value, chat_data: serde_json::Value, epoch_time: i64) -> Result<(i32, i32), diesel::result::Error> {
//     let conn = pool.get()?;
//     let user_id = user_data["id"].as_i64().unwrap() as i32;

//     let existing_user: Option<User> = users::table
//         .filter(users::user_data->>"id".eq(user_id.to_string()))
//         .first(&conn)
//         .optional()?;

//     if let Some(user) = existing_user {
//         Ok((user.id, user_id))
//     } else {
//         let new_user = User {
//             id: 0,
//             user_data: user_data.clone(),
//             chat_data: chat_data.clone(),
//             epoch_time,
//         };

//         diesel::insert_into(users::table)
//             .values(&new_user)
//             .execute(&conn)?;

//         let inserted_user: User = users::table
//             .filter(users::user_data->>"id".eq(user_id.to_string()))
//             .first(&conn)?;

//         Ok((inserted_user.id, user_id))
//     }
// }
