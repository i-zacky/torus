use actix_web::{delete, get, post, put, web::Json, web::Path, Responder, Result};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use torus_database::establish_connection;
use torus_database::sandbox_repository::get_all;
use ulid::Ulid;

#[derive(Serialize, Deserialize)]
pub struct Sandbox {
    id: String,
    name: String,
    birthday: String,
    height: i32,
    weight: BigDecimal,
    enabled: bool,
}

#[get("/sandbox")]
pub async fn get_sandbox() -> Result<impl Responder> {
    let conn = establish_connection();
    let results = get_all(&conn);
    let response: Vec<Sandbox> = results
        .into_iter()
        .map(|v| Sandbox {
            id: v.id,
            name: match v.name {
                None => String::from(""),
                Some(val) => val,
            },
            birthday: match v.birthday {
                None => String::from(""),
                Some(val) => val.to_string(),
            },
            height: match v.height {
                None => 0,
                Some(val) => val,
            },
            weight: match v.weight {
                None => BigDecimal::from(0),
                Some(val) => val,
            },
            enabled: match v.enabled {
                None => false,
                Some(val) => val,
            },
        })
        .collect();

    Ok(Json(response))
}

#[get("/sandbox/{id}")]
pub async fn get_sandbox_by_id(path: Path<String>) -> Result<impl Responder> {
    let id = path.into_inner();
    println!("request id: {}", id);
    let response = Sandbox {
        id,
        name: String::from("sandbox3"),
        birthday: String::from("1990-04-12"),
        height: 180,
        weight: BigDecimal::from(60.1),
        enabled: true,
    };
    Ok(Json(response))
}

#[post("/sandbox")]
pub async fn create_sandbox(request: Json<Sandbox>) -> Result<impl Responder> {
    println!("name: {}", request.id);
    Ok(request)
}

#[put("/sandbox")]
pub async fn update_sandbox(request: Json<Sandbox>) -> Result<impl Responder> {
    println!("name: {}", request.id);
    Ok(request)
}

#[delete("/sandbox/{id}")]
pub async fn delete_sandbox_by_id(path: Path<String>) -> Result<impl Responder> {
    let id = path.into_inner();
    println!("request id: {}", id);
    Ok(id)
}
