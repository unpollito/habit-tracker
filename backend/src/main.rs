#[macro_use]
extern crate rocket;

use habit_tracker_backend::{password, random};

#[get("/")]
fn index() -> &'static str {
    log::info!("Random token: {}", random::generate_secure_random_bytes_as_base64(128));
    let hash = password::generate_hash("hunter44");
    log::info!("Hash: {}", hash);
    let hash = password::generate_hash(" awf5b1 5in! 651%");
    log::info!("Hash: {}", hash);
    let hash = password::generate_hash("hunter43");
    log::info!("Hash: {}", hash);
    let hash = password::generate_hash("hunter42");
    log::info!("Hash: {}", hash);
    log::info!("Hash matches: {}", password::verify_hash("hunter42", &hash));
    "See console"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/hello", routes![index])
        .launch()
        .await?;

    Ok(())
}