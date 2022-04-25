use crate::models::Sandbox;
use crate::schema::sandbox;
use diesel::{EqAll, PgConnection, QueryDsl, RunQueryDsl};

pub fn get_all(conn: &PgConnection) -> Vec<Sandbox> {
    sandbox::dsl::sandbox
        .load::<Sandbox>(conn)
        .expect("failed to load sandbox")
}

pub fn get_by_id(conn: &PgConnection, id: &String) -> Option<Sandbox> {
    sandbox::dsl::sandbox
        .filter(sandbox::id.eq_all(id))
        .limit(1)
        .first::<Sandbox>(conn)
        .ok()
}
