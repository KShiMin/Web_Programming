use actix_web::{get, post, web, HttpResponse, Responder};
use actix_session::{Session};
use uuid::Uuid;
use serde_json::json;

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

