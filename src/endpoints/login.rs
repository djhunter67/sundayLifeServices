use actix_web::{HttpResponse, Responder, get, post, web};
use askama::Template;
use serde::{Deserialize, Serialize};
use tracing::{instrument, warn};

/// All things login that need to be handled for the ``SundayLife`` services website.

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate<'a> {
    title: &'a str,
    content: Vec<&'a str>,
}

#[derive(Deserialize, Debug, Serialize)]
struct LoginUser {
    email_input: String,
    password_input: String,
}

#[get("/login")]
#[instrument(
    name = "User login attempted",
    level = "info",
    target = "sundayLifeServices web app"
)]
pub async fn login() -> HttpResponse {
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
    skip(body)
)]
pub async fn login_user(body: web::Form<LoginUser>) -> impl Responder {
    warn!("The user data entered: {:#?}", body.0);

    HttpResponse::Created().json(&body)
}
