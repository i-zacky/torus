use actix_web::{get, HttpResponse};

#[get("/health")]
pub async fn get_health_status() -> HttpResponse {
    HttpResponse::Ok().content_type("application/json").body(
        r#"{
            "status": "up"
        }"#,
    )
}
