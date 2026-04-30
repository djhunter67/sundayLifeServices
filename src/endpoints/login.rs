use actix_web::{HttpResponse, get};
use askama::Template;

/// All things login that need to be handled for the ``SundayLife`` services website.

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate<'a> {
    title: &'a str,
    content: Vec<&'a str>,
}

#[get("/login")]
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
