use actix_web::{delete, get, post, put, web::Json, web::Path, Responder, Result};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
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
    let response = [
        Sandbox {
            id: Ulid::new().to_string(),
            name: String::from("sandbox1"),
            birthday: String::from("1990-04-10"),
            height: 160,
            weight: BigDecimal::from(50.4),
            enabled: true,
        },
        Sandbox {
            id: Ulid::new().to_string(),
            name: String::from("sandbox2"),
            birthday: String::from("1990-04-11"),
            height: 170,
            weight: BigDecimal::from(55.8),
            enabled: false,
        },
        Sandbox {
            id: Ulid::new().to_string(),
            name: String::from("sandbox3"),
            birthday: String::from("1990-04-12"),
            height: 180,
            weight: BigDecimal::from(60.1),
            enabled: true,
        },
    ];

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
