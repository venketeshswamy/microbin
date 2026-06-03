use crate::args::{Args, ARGS};
use crate::endpoints::errors::ErrorTemplate;
use crate::pasta::Pasta;
use crate::util::misc::remove_expired;
use crate::AppState;
use actix_web::{get, web, HttpResponse};
use askama::Template;

#[derive(Template)]
#[template(path = "auth_upload.html")]
struct AuthPasta<'a> {
    args: &'a Args,
    id: String,
    status: String,
    encrypted_key: String,
    encrypt_client: bool,
    path: String,
}

fn render_auth_pasta(
    pastas: &[Pasta],
    id: String,
    status: String,
    path: &str,
) -> HttpResponse {
    if let Some(index) = Pasta::find_index(pastas, &id) {
        let pasta = &pastas[index];
        HttpResponse::Ok().content_type("text/html; charset=utf-8").body(
            AuthPasta {
                args: &ARGS,
                id,
                status,
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from(path),
            }
            .render()
            .unwrap(),
        )
    } else {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(ErrorTemplate { args: &ARGS }.render().unwrap())
    }
}

#[get("/auth/{id}")]
pub async fn auth_upload(data: web::Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let mut pastas = data.pastas.lock().unwrap();
    remove_expired(&mut pastas);
    render_auth_pasta(&pastas, id.into_inner(), String::from(""), "upload")
}

#[get("/auth/{id}/{status}")]
pub async fn auth_upload_with_status(
    data: web::Data<AppState>,
    param: web::Path<(String, String)>,
) -> HttpResponse {
    let mut pastas = data.pastas.lock().unwrap();
    remove_expired(&mut pastas);
    let (id, status) = param.into_inner();
    render_auth_pasta(&pastas, id, status, "upload")
}

#[get("/auth_raw/{id}")]
pub async fn auth_raw_pasta(data: web::Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let mut pastas = data.pastas.lock().unwrap();
    remove_expired(&mut pastas);
    render_auth_pasta(&pastas, id.into_inner(), String::from(""), "raw")
}

#[get("/auth_raw/{id}/{status}")]
pub async fn auth_raw_pasta_with_status(
    data: web::Data<AppState>,
    param: web::Path<(String, String)>,
) -> HttpResponse {
    let mut pastas = data.pastas.lock().unwrap();
    remove_expired(&mut pastas);
    let (id, status) = param.into_inner();
    render_auth_pasta(&pastas, id, status, "raw")
}

#[get("/auth_edit_private/{id}")]
pub async fn auth_edit_private(data: web::Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let mut pastas = data.pastas.lock().unwrap();
    remove_expired(&mut pastas);
    render_auth_pasta(&pastas, id.into_inner(), String::from(""), "edit_private")
}

#[get("/auth_edit_private/{id}/{status}")]
pub async fn auth_edit_private_with_status(
    data: web::Data<AppState>,
    param: web::Path<(String, String)>,
) -> HttpResponse {
    let mut pastas = data.pastas.lock().unwrap();
    remove_expired(&mut pastas);
    let (id, status) = param.into_inner();
    render_auth_pasta(&pastas, id, status, "edit_private")
}

#[get("/auth_file/{id}")]
pub async fn auth_file(data: web::Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let mut pastas = data.pastas.lock().unwrap();
    remove_expired(&mut pastas);
    render_auth_pasta(&pastas, id.into_inner(), String::from(""), "secure_file")
}

#[get("/auth_file/{id}/{status}")]
pub async fn auth_file_with_status(
    data: web::Data<AppState>,
    param: web::Path<(String, String)>,
) -> HttpResponse {
    let mut pastas = data.pastas.lock().unwrap();
    remove_expired(&mut pastas);
    let (id, status) = param.into_inner();
    render_auth_pasta(&pastas, id, status, "secure_file")
}

#[get("/auth_remove_private/{id}")]
pub async fn auth_remove_private(data: web::Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let mut pastas = data.pastas.lock().unwrap();
    remove_expired(&mut pastas);
    render_auth_pasta(&pastas, id.into_inner(), String::from(""), "remove")
}

#[get("/auth_remove_private/{id}/{status}")]
pub async fn auth_remove_private_with_status(
    data: web::Data<AppState>,
    param: web::Path<(String, String)>,
) -> HttpResponse {
    let mut pastas = data.pastas.lock().unwrap();
    remove_expired(&mut pastas);
    let (id, status) = param.into_inner();
    render_auth_pasta(&pastas, id, status, "remove")
}
