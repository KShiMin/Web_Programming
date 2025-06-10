use actix_session:: {Session, SessionMiddleware, storage::CookieSessionStore};
use actix_web:: {web, App, HttpServer, HttpResponse, Responder};
use actix_web::cookie::Key;
use tera::{Tera, Context};
use serde::Deserialize;
use bcrypt::{verify, hash};

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String
}

#[derive(Deserialize)]
struct LoginQuery{
    error: Option<String>,
}

// Simulate user data
fn mock_user_password() -> (String, String) {
    let username = "admin".to_string();
    let hashed = hash("password123", 4).unwrap();   // bcrypt hash
    (username, hashed)
}

async fn login_form(
    tmpl: web::Data<Tera>,
    query: web::Query<LoginQuery>,
) -> impl Responder {
    let mut ctx = tera::Context::new();

    if let Some(err) = &query.error {
        if err == "!" {
            ctx.insert("error", "Invalide username or password");
        }
    }

    let s = tmpl.render("login.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}


async fn login_process(
    form: web::Form<LoginForm>,
    session: Session,
) -> actix_web::Result<HttpResponse> {
    let (expected_user, expected_hash) = mock_user_password();

    if form.username == expected_user && verify(&form.password, &expected_hash).unwrap() {
        session.insert("username", &form.username)?;
        Ok(HttpResponse::Found()
            .append_header(("Location", "/dashboard"))
            .finish())
    } else {
        Ok(HttpResponse::Found()
            .append_header(("Location", "/login?error=1"))
            .finish())
    }
}


async fn dashboard(session: Session) -> impl Responder {
    match session.get::<String>("username") {
        Ok(Some(username)) => HttpResponse::Ok().body(format!("Welcome, {}!", username)),
        _ => HttpResponse::Found().append_header(("Location", "/login")).finish(),
    }
}

async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Found().append_header(("Location", "/login")).finish()
}


async fn index(tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("site_name", "My Rust Site");
    let rendered = tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").unwrap();
    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .wrap(SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .cookie_secure(false)   // disable secure cookie for localhost dev
                .build())
            .route("/", web::get().to(index))
            .route("/login", web::get().to(login_form))
            .route("/login", web::post().to(login_process))
            .route("/dashboard", web::get().to(dashboard))
            .route("/logout", web::get().to(logout))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
