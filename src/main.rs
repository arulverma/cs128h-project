#[macro_use] extern crate rocket;

mod algorithms;

use algorithms::map_reduce::map_reduce;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/mapreduce")]
fn mapreduce() -> String {
    let to_reduce = vec![String::from("REDUCE ME")];

    let reduced = map_reduce(to_reduce).into_iter().reduce(|a, b| a + &b).unwrap_or(String::from("ERROR!"));

    "Please upload your dataset here in one of the following accepted formats (.csv, .json): ".to_owned() + &reduced
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
    .mount("/", routes![mapreduce])
}
