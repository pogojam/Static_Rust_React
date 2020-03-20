
#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate rocket;
mod server;

fn main() {
    println!("Hello, world!");

    server::init();
}
