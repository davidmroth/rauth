use crate::auth::{Auth, Session};
use crate::db::AccountShort;
use crate::util::{Error, Result};

use mongodb::bson::{doc, from_document};
use mongodb::options::FindOneOptions;
use rocket::State;
use rocket_contrib::json::JsonValue;

impl Auth {
    pub async fn get_account(&self, session: Session) -> Result<AccountShort> {
        let user = self
            .collection
            .find_one(
                doc! {
                    "_id": &session.user_id,
                    "sessions.token": &session.session_token
                },
                FindOneOptions::builder()
                    .projection(doc! {
                        "_id": 1,
                        "email": 1,
                        "verification": 1
                    })
                    .build(),
            )
            .await
            .map_err(|_| Error::DatabaseError {
                operation: "find_one",
                with: "account",
            })?
            .ok_or(Error::UnknownUser)?;

        Ok(from_document(user).map_err(|_| Error::DatabaseError {
            operation: "from_document",
            with: "account",
        })?)
    }
}

#[get("/user")]
pub async fn fetch_account(
    auth: State<'_, Auth>,
    session: Session,
) -> crate::util::Result<JsonValue> {
    Ok(json!(auth.get_account(session).await?))
}
