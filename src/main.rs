#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old name {}!", age, name)
}

#[post("/upload", format="plain", data="<data>")]
fn upload(data: Data) -> io::Result<String> {
    # call commander module function.
}

#[post("/backup", )]


fn main() {
    rocket::ignite()
        .mount("/", routes![hello])
        .launch();
}
