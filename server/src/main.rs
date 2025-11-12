mod db;
mod env;
mod endpoints;
#[allow(unused_imports)]
mod entities;
mod error;
mod extractors;
mod jwt;

#[rocket::launch]
async fn rocket() -> _ {
    let db = db::init()
        .await
        .expect("Failed to initialize database");

    rocket::build()
        .manage(db)
        .mount("/g", endpoints::general::routes())
        .mount("/u", endpoints::user::routes())
}
