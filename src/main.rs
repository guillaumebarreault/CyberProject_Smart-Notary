#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket_contrib::serve::StaticFiles;



#[get("/acceuil")]
fn acceuil() -> Template {
    let context: HashMap<String,String> = HashMap::new();
    Template::render("acceuil", &context)
}

#[get("/certif")]
fn certif() -> Template {
    let context: HashMap<String,String> = HashMap::new();
    Template::render("certif", &context)
}

#[get("/")]
fn landing_page() -> Template {
    let context: HashMap<String,String> = HashMap::new();
    Template::render("landing_page", &context)
}

#[get("/signin_form")]
fn signin_form() -> Template {
    let context: HashMap<String,String> = HashMap::new();
    Template::render("signin_form", &context)
}

#[get("/signup_form")]
fn signup_form() -> Template {
    let context: HashMap<String,String> = HashMap::new();
    Template::render("signup_form", &context)
}

#[get("/mesdocuments")]
fn doc() -> Template {
    let context: HashMap<String,String> = HashMap::new();
    Template::render("MesDocuments", &context)
}

#[get("/parametres")]
fn parametre() -> Template {
    let context: HashMap<String,String> = HashMap::new();
    Template::render("Parametre", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![acceuil])
        .mount("/", routes![landing_page])
        .mount("/", routes![signin_form])
        .mount("/", routes![signup_form])
        .mount("/", routes![certif])
        .mount("/", routes![doc])
        .mount("/", routes![parametre])
        .mount("/static", StaticFiles::from("./static"))
        .attach(Template::fairing())
        .launch();

}
