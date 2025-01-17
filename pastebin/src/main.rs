#[macro_use] extern crate rocket;

mod paste_id;

use paste_id::PasteId;

use rocket::data::{Data, ToByteUnit};
use rocket::http::uri::Absolute;
use rocket::tokio::fs::File;

const ID_LENGTH: usize = 5;
const HOST: Absolute<'static> = uri!("http://localhost:8000");

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

        POST /

            accepts raw data in the body of the request and responds with a URL of a page containing the body's content

        GET /<id>

            retrieves the content for the paste with id `<id>`
    "
}

#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<File> {
    File::open(id.file_path()).await.ok()
}

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    paste.open(128.kibibytes()).into_file(id.file_path()).await?;
    Ok(uri!(HOST, retrieve(id)).to_string())
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build().mount("/", routes![index, retrieve, upload]).launch().await?;
    Ok(())
}
