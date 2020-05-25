#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> String {
    format!("Hello World")
}

#[get("/jojo/<name>")]
fn attack(name: String) -> String {
    format!("{}\n{}\n{}", name, name, name)
}

fn main() {
    rocket::ignite().mount("/", routes![hello, attack]).launch();
}
