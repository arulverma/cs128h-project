#[macro_use] extern crate rocket;

mod algorithms;

use algorithms::map_reduce::run;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/mapreduce")]
fn mapreduce() -> &'static str {
    let to_reduce = String::from("HI");
    let results = run(to_reduce);

    println!("{:#?}", results);

    "Please upload your dataset here in one of the following accepted formats (.csv, .json): "
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
    .mount("/", routes![mapreduce])
}
