use actix_web::{delete, get, post, put, web::Json, web::Path, Responder, Result};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use torus_database::establish_connection;
use torus_database::sandbox_repository::{get_all, get_by_id};
use ulid::Ulid;

#[derive(Serialize, Deserialize)]
pub struct SandboxDTO {
    id: String,
    name: Option<String>,
    birthday: Option<String>,
    height: Option<i32>,
    weight: Option<BigDecimal>,
    enabled: Option<bool>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[get("/sandbox")]
pub async fn get_sandbox() -> Result<impl Responder> {
    let conn = establish_connection();
    let results = get_all(&conn);
    let response: Vec<SandboxDTO> = results
        .into_iter()
        .map(|v| SandboxDTO {
            id: v.id,
            name: v.name,
            birthday: match v.birthday {
                None => None,
                Some(val) => Some(val.to_string()),
            },
            height: v.height,
            weight: v.weight,
            enabled: v.enabled,
            created_at: match v.created_at {
                None => None,
                Some(val) => Some(val.format("%Y-%m-%d %H:%M:%S").to_string()),
            },
            updated_at: match v.updated_at {
                None => None,
                Some(val) => Some(val.format("%Y-%m-%d %H:%M:%S").to_string()),
            },
        })
        .collect();

    Ok(Json(response))
}

#[get("/sandbox/{id}")]
pub async fn get_sandbox_by_id(path: Path<String>) -> Result<impl Responder> {
    let id = path.into_inner();
    println!("request id: {}", &id);
    let conn = establish_connection();
    let result = get_by_id(&conn, &id);
    let response = result.map(|v| SandboxDTO {
        id: v.id,
        name: v.name,
        birthday: match v.birthday {
            None => None,
            Some(val) => Some(val.to_string()),
        },
        height: v.height,
        weight: v.weight,
        enabled: v.enabled,
        created_at: match v.created_at {
            None => None,
            Some(val) => Some(val.format("%Y-%m-%d %H:%M:%S").to_string()),
        },
        updated_at: match v.updated_at {
            None => None,
            Some(val) => Some(val.format("%Y-%m-%d %H:%M:%S").to_string()),
        },
    });
    Ok(Json(response))
}

#[post("/sandbox")]
pub async fn create_sandbox(request: Json<SandboxDTO>) -> Result<impl Responder> {
    println!("name: {}", request.id);
    Ok(request)
}

#[put("/sandbox")]
pub async fn update_sandbox(request: Json<SandboxDTO>) -> Result<impl Responder> {
    println!("name: {}", request.id);
    Ok(request)
}

#[delete("/sandbox/{id}")]
pub async fn delete_sandbox_by_id(path: Path<String>) -> Result<impl Responder> {
    let id = path.into_inner();
    println!("request id: {}", id);
    Ok(id)
}
