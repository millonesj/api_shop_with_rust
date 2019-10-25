/* #![feature(plugin)] */
/* #![plugin(rocket_codegen)] */
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

/* use rocket_contrib::{json, value}; */
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

mod product;
use product::{Product};

#[get("/")]
fn greeting() -> String {
    format!("Welcome Everybody!!")
}

#[post("/", data = "<product>")]
fn create_product(product: Json<Product>) -> Json<Product> {
    product
}

#[put("/<id>", data = "<product>")]
fn update_product(id: i32, product: Json<Product>) -> Json<Product> {
    product
}

#[delete("/<id>")]
fn delete_product(id: i32) -> JsonValue {
    json!({"status": "ok"})
}

#[get("/<id>")]
fn get_one_product(id: i32) -> String{
    format!("Product {}", id)
}

#[get("/")]
fn list_products() -> JsonValue {
    json!([
        "product 1",
        "product 2"
    ])
}

fn main() {
    rocket::ignite()
    .mount("/", routes![greeting])
    .mount("/products", routes![list_products,get_one_product, create_product, update_product, delete_product])
    .launch();
}
