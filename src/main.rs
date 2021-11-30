#[macro_use] extern crate rocket;

mod algorithms;

use std::path::Path; 
use rocket::fs::NamedFile;
use algorithms::map_reduce::{map_reduce, get_graph_points};

#[get("/")]
async fn index() -> Option<NamedFile> {
    let page_directory_path = 
        format!("{}/../client", env!("CARGO_MANIFEST_DIR"));
    NamedFile::open(Path::new(&page_directory_path).join("index.html")).await.ok()
}

#[get("/mapreduce")]
fn mapreduce() -> &'static str {
    let to_reduce = String::from("HI");
    let results = get_graph_points(map_reduce(to_reduce));

    println!("{:#?}", results);

    "Please upload your dataset here in one of the following accepted formats (.csv, .json): "
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
    .mount("/", routes![mapreduce])
}
