use std::sync::Arc;

use actix_web::{
    HttpResponse, Responder, get, post,
    web::{self, Data},
};
use askama::Template;
use mongodb::bson;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument, warn};

use crate::{endpoints::register::RegisterUser, settings};

/// All things login that need to be handled for the ``SundayLife`` services website.

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate<'a> {
    title: &'a str,
    content: Vec<&'a str>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct LoginUser {
    #[serde(rename = "email_input")]
    pub email: String,
    #[serde(rename = "password_input")]
    pub password: String,
}

impl From<RegisterUser> for LoginUser {
    fn from(value: RegisterUser) -> Self {
        Self {
            email: value.email,
            password: value.password,
        }
    }
}

#[get("/login")]
#[instrument(
    name = "User login attempted",
    level = "info",
    target = "sundayLifeServices web app"
)]
pub async fn login_template() -> HttpResponse {
    debug!("Login page loaded");
    let user_login: &str = "user_email";
    let user_password: &str = "super_duper_secret_password";
    let template = LoginTemplate {
        title: "Login",
        content: [user_login, user_password].to_vec(),
    };

    let template = template.render().expect("Login page render error");

    HttpResponse::Ok().body(template)
}

#[post("/login_user")]
#[instrument(
    name = "User login attempted",
    level = "info",
    target = "sundayLifeServices web app",
    skip(body, mongo)
)]
pub async fn login_user(
    mongo: Data<mongodb::Database>,
    body: web::Form<LoginUser>,
) -> impl Responder {
    warn!("The user data en tered: {:#?}", body.0);

    // Validate the user data entered
    let useremail: &str = body.0.email.as_str();
    let password: &str = body.0.password.as_str();

    // if password.contains('$') {
    //     return HttpResponse::NotAcceptable().finish();
    // }

    // Check against the database
    let filter = mongodb::bson::doc! {
    "useremail": useremail,
    "passw": password
    };

    let Some(db) = Arc::into_inner(mongo.into_inner()) else {
        return HttpResponse::InternalServerError().finish();
    };

    let db: mongodb::Collection<bson::Document> = db.collection(
        &settings::get()
            .expect("Unable to acquire settings")
            .mongo
            .collection,
    );

    let user = db.find_one(filter).await;

    match user {
        Ok(Some(_)) => HttpResponse::Ok().body("Login successful"),
        Ok(None) => HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(err) => HttpResponse::Ok().body(err.to_string()),
    }
}
