extern crate diesel;
extern crate torus_database;

use self::diesel::prelude::*;
use self::models::*;
use self::torus_database::*;

fn main() {
    use torus_database::schema::sandbox::dsl::*;

    let connection = establish_connection();
    let results = sandbox
        .limit(5)
        .load::<Sandbox>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} sandboxes", results.len());
    for r in results {
        println!("{:?}", r);
    }
}
