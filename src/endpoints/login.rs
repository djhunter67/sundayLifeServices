// use actix_web::{HttpResponse, get};
// use askama::Template;

// /// All things login that need to be handled for the ``SundayLife`` services website.

// #[derive(Template)]
// #[template(path = "login.html")]
// struct LoginTemplate<'a> {
//     title: &'a str,
//     content: Vec<&'a str>,
// }

// #[get("/login")]
// pub async fn login() -> HttpResponse {
//     let user_login: &str = "The company started in Golden Valley, Arizona in 2006";
//     let user_password: &str = "Nahan Loka is the sole proprietor of SundayLife Services";
//     let template = AboutTemplate {
//         title: "About",
//         content: [user_login, user_password].to_vec(),
//     };

//     let template = template.render().expect("About page render error");

//     HttpResponse::Ok().finish()
// }
