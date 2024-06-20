// use serde::{Deserialize, Serialize};
// use diesel::prelude::*;
// use crate::schema::{users, requests};

// #[derive(Queryable, Insertable, Serialize, Deserialize)]
// #[table_name = "users"]
// pub struct User {
//     pub id: i32,
//     pub user_data: serde_json::Value,
//     pub chat_data: serde_json::Value,
//     pub epoch_time: i64,
// }

// #[derive(Queryable, Insertable, Serialize, Deserialize)]
// #[table_name = "requests"]
// pub struct Request {
//     pub id: i32,
//     pub user_id: i32,
//     pub command: String,
//     pub response: String,
//     pub request_time: chrono::NaiveDateTime,
// }
