use crate::models::Sandbox;
use crate::schema::sandbox::dsl::sandbox;
use diesel::{PgConnection, RunQueryDsl};

pub fn get_all(conn: &PgConnection) -> Vec<Sandbox> {
    sandbox
        .load::<Sandbox>(conn)
        .expect("failed to load sandbox")
}
