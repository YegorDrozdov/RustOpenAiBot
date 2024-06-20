// use diesel::backend::Backend;
// use diesel::deserialize::{self, FromSql};
// use diesel::pg::{Pg, PgValue};
// use diesel::serialize::{self, IsNull, Output, ToSql};
// use diesel::sql_types::Jsonb;
// use serde::{Deserialize, Serialize};
// use serde_json::Value;
// use std::io::Write;

// #[derive(SqlType)]
// #[postgres(type_name = "jsonb")]
// pub struct CustomJsonb;

// #[derive(Debug, Clone, FromSqlRow, AsExpression, Serialize, Deserialize)]
// #[sql_type = "Jsonb"]
// pub struct CustomJson(pub Value);

// impl From<Value> for CustomJson {
//     fn from(value: Value) -> Self {
//         CustomJson(value)
//     }
// }

// impl ToSql<Jsonb, Pg> for CustomJson {
//     fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
//         let json_str = serde_json::to_string(&self.0).map_err(|_| serialize::Error::SerializationError)?;
//         out.write_all(json_str.as_bytes())?;
//         Ok(IsNull::No)
//     }
// }

// impl FromSql<Jsonb, Pg> for CustomJson {
//     fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
//         let value: Value = serde_json::from_slice(bytes.as_bytes()).map_err(|_| deserialize::Error::DeserializationError)?;
//         Ok(CustomJson(value))
//     }
// }
