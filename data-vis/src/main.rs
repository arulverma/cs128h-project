#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/mapreduce")]
fn mapreduce() -> &'static str {
    "Please upload your dataset here in one of the following accepted formats (.csv, .json): "
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
    .mount("/", routes![mapreduce])
}
