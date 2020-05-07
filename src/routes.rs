//! Application endpoints.

mod bson_doc;
mod jwt;

use super::state::State;
use bson_doc::BsonDoc;
use jwt::Claims;

use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use bson::doc;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    env,
};
use tide::{
    http::{Cookie, StatusCode},
    Error, Request, Response,
};

#[derive(Serialize, Deserialize, Debug)]
struct UserAuth {
    user_name: String,
    password: String,
}

impl UserAuth {
    fn verify(&self, hash: &UserHash) -> Result<bool, BcryptError> {
        Ok(self.user_name == hash.user_name && verify(&self.password, &hash.key)?)
    }
}

impl BsonDoc for UserAuth {}

#[derive(Serialize, Deserialize, Debug)]
struct UserHash {
    user_name: String,
    key: String,
}

impl TryFrom<UserAuth> for UserHash {
    type Error = BcryptError;

    fn try_from(user: UserAuth) -> Result<Self, Self::Error> {
        Ok(Self {
            user_name: user.user_name,
            key: hash(user.password, DEFAULT_COST)?,
        })
    }
}

impl BsonDoc for UserHash {}

pub(crate) async fn authenticate(mut req: Request<State>) -> tide::Result<impl Into<Response>> {
    let user_auth: UserAuth = req.body_json().await?;

    // Query the documents in the collection with a filter and an option.
    let filter = doc! { "user_name": &user_auth.user_name };
    let cursor = req.state().users().find_one(filter, None).await?;

    let authorised = if let Some(hash) = cursor {
        user_auth.verify(&bson::from_bson(bson::Bson::Document(hash))?)?
    } else {
        false
    };

    let res = if authorised {
        login(user_auth.user_name)
    } else {
        Response::new(StatusCode::Ok)
    };

    Ok(res.body_string((authorised).to_string()))
}

pub(crate) async fn create_user(mut req: Request<State>) -> tide::Result<impl Into<Response>> {
    let user_auth: UserAuth = req.body_json().await?;

    let filter = doc! { "user_name": &user_auth.user_name };
    let cursor = req.state().users().find_one(filter, None).await?;

    if cursor.is_none() {
        req.state()
            .users()
            .insert_one(
                {
                    let hashed: UserHash = user_auth.try_into()?;
                    hashed.as_bson()?
                },
                None,
            )
            .await?;
        Ok("Insert successful!")
    } else {
        Err(Error::from_str(
            StatusCode::Conflict,
            "User already exists!",
        ))
    }
}

pub(crate) async fn user_page(req: Request<State>) -> tide::Result<impl Into<Response>> {
    let validation = Validation {
        sub: Some(req.param("user")?),
        ..Validation::default()
    };
    Ok((if let Some(cookie) = req.cookie("login") {
        match decode::<Claims>(
            &cookie.value(),
            &DecodingKey::from_secret(env::var("SECRET").unwrap().as_bytes()),
            &validation,
        ) {
            Ok(c) => true,
            Err(err) => false,
        }
    } else {
        false
    })
    .to_string())
}

fn login(user: String) -> Response {
    let mut res = tide::Response::new(StatusCode::Ok);
    res.set_cookie(Cookie::build("login", Claims::new(user).get_token()).finish());
    res
}
