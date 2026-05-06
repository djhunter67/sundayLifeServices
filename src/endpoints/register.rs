use actix_web::{
    HttpResponse, get, post,
    web::{self, Data},
};
use askama::Template;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use tracing::{error, instrument};

use crate::{security::PassWorder, settings};

/// All things login that need to be handled for the ``SundayLife`` services website.

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate<'a> {
    title: &'a str,
    content: Vec<&'a str>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct RegisterUser {
    #[serde(rename = "email_input")]
    pub email: String,
    #[serde(rename = "password_input")]
    pub password: String,
    #[serde(rename = "password_2_input")]
    password_2: String,
}

#[derive(Serialize)]
struct RegistrationData {
    email: String,
    password_hash: String,
    password_salt: String,
}

#[get("/register")]
#[instrument(
    name = "User registration attempted",
    level = "info",
    target = "sundayLifeServices web app"
)]
pub async fn register_template() -> HttpResponse {
    let user_name: &str = "sundaylife.mxa";
    let user_password_1: &str = "password_1";
    let user_password_2: &str = "password_2";
    let template = RegisterTemplate {
        title: "Registration",
        content: [user_name, user_password_1, user_password_2].to_vec(),
    };

    let template = template.render().expect("About page render error");

    HttpResponse::Ok().body(template)
}

#[post("/register_user")]
#[instrument(
    name = "User registration attempted",
    level = "info",
    target = "sundayLifeServices web app",
    skip(body, mongo)
)]
pub async fn register_user(
    mongo: Data<mongodb::Database>,
    body: web::Form<RegisterUser>,
) -> HttpResponse {
    // Validate the user data entered
    let password: &str = &body.0.password;
    let password_2: &str = &body.0.password_2;

    // if password.contains('$') {
    // return HttpResponse::NotAcceptable().finish();
    // }

    if !password.eq(password_2) {
        error!("Password not equal during registration");
        return HttpResponse::NotAcceptable().json("Passwords do not match");
    }

    let encrypted_pw: PassWorder = PassWorder::new(password.to_string())
        .encrypt()
        .salt()
        .pepper();

    let (salt, pw, _) = encrypted_pw.deconstruct();

    let db: mongodb::Collection<RegistrationData> = mongo.collection(
        &settings::get()
            .expect("Unable to procure the settings")
            .mongo
            .collection,
    );

    // Save the user to the database
    let result = db
        .insert_one(RegistrationData {
            email: body.0.email,
            password_hash: pw,
            password_salt: salt,
        })
        .await;

    match result {
        Ok(id) => HttpResponse::Created().json(format!("User Registered: {}", id.inserted_id)),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }

    // HttpResponse::Ok().finish()
}
