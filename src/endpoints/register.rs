use actix_web::{HttpResponse, get};
use askama::Template;

/// All things login that need to be handled for the ``SundayLife`` services website.

#[derive(Template)]
#[template(path = "login.html")]
struct RegisterTemplate<'a> {
    title: &'a str,
    content: Vec<&'a str>,
}

#[get("/register")]
pub async fn register() -> HttpResponse {
    let user_name: &str = "sundaylife.mxa";
    let user_password_1: &str = "password_1";
    let user_password_2: &str = "password_2";
    let template = RegisterTemplate {
        title: "About",
        content: [user_name, user_password_1, user_password_2].to_vec(),
    };

    let template = template.render().expect("About page render error");

    HttpResponse::Ok().body(template)
}
