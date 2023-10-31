#[macro_use] extern crate rocket;
use rocket::tokio;

use rocket::data::{Data, ToByteUnit};

#[post("/", data = "<data>")]
async fn stream_data(data: Data<'_>) -> std::io::Result<()> {
    data.open(512.kibibytes())
        .stream_to(tokio::io::stdout())
        .await?;

    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, stream_data])
}
