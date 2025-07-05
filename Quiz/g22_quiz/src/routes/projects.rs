use actix_web::{get, post, web, HttpResponse, Responder};
use actix_session::{Session};
use uuid::Uuid;
use serde_json::json;
use tera::Context;

use crate::models::{Project, NewProject};
use crate::state::AppState;

#[post("/projects")]
pub async fn create_project(
    session: Session,
    data: web::Data<AppState>,
    new_proj: web::Json<NewProject>,
) -> impl Responder {
    // Extract role from session
    let role = session.get::<String>("role").unwrap_or(None);

    if role.as_deref() != Some("Admin") {
        return HttpResponse::Unauthorized().json(json!({
            "Error": "Only admin users can create projects."
        }));
    }

    let mut projects = data.projects.lock().unwrap();
    let project = Project {
        project_id: Uuid::new_v4(),
        name: new_proj.name.clone(),
        description: new_proj.description.clone(),
    };
    projects.push(project);

    HttpResponse::Ok().json(json!({
        "Status": "Project added successfully"
    }))
}

#[get("/projects")]
pub async fn get_projects(data: web::Data<AppState>) -> impl Responder {
    let projects = data.projects.lock().unwrap();
    HttpResponse::Ok().json(&*projects)
}

#[get("/projects")]
pub async fn get_projects_api(state: web::Data<AppState>) -> std::result::Result<impl Responder, actix_web::Error> {
   let list = {
      let guard = state.projects.lock().unwrap();
      guard.clone()
   };
   Ok(web::Json(list))
}


#[get("/projects/view")]
pub async fn get_projects_html(data: web::Data<AppState>) -> std::result::Result<HttpResponse,   actix_web::Error> {
    let list = {
        let guard = data.projects.lock().unwrap();
        guard.clone()
    };

    let mut ctx = Context::new();
    ctx.insert("projects", &list);

    let rendered = data
        .tera
        .render("projects_list.html", &ctx)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered))
}

#[get("/projects/new")]
pub async fn new_project_form(state: web::Data<AppState>) -> std::result::Result<HttpResponse,   actix_web::Error> {
    // grab Tera out of the shared AppState
    let rendered = state
        .tera
        .render("project_form.html", &tera::Context::new())
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered))
}

#[post("/projects/html")]
pub async fn create_project_form(
    session: Session,
    data: web::Data<AppState>,
    form: web::Form<NewProject>,
) -> std::result::Result<HttpResponse,   actix_web::Error> {
    let role = session.get::<String>("role")?.unwrap_or_default();
    if role != "Admin" {
        return Ok(HttpResponse::Unauthorized().body("Only admin"));
    }

    let mut guard = data.projects.lock().unwrap();
    guard.push(Project {
        project_id: uuid::Uuid::new_v4(),
        name: form.name.clone(),
        description: form.description.clone(),
    });
    drop(guard);

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/projects/view"))
        .finish())
}