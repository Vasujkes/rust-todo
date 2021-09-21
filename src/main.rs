extern crate clap;
extern crate iron;
extern crate router;
extern crate persistent;
extern crate bodyparser;

#[macro_use]
extern crate serde_derive;

use clap::App;
use iron::prelude::*;
use router::Router;
use persistent::{State, Read};

mod todo;
use todo::*;

mod handlers;
use handlers::*;

fn main() {
    let matches = App::new("todo-app")
        .about("Example of a TODO API in Rust")
        .args_from_usage(
            "-l, --listen=[address:port] 'Sets an address and port to listen''",
        )
        .get_matches();
    let listen = matches.value_of("listen").unwrap_or("127.0.0.1:3000");
    let mut router = Router::new();
    router.get("/version", get_version, "version");
    router.get("/todo/:id", get_todo, "get_todo");
    router.delete("/todo/:id", delete_todo, "delete_todo");
    router.post("/todo", create_todo, "create_todo");

    let mut chain = Chain::new(router);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(8192));
    let storage = Storage::new();
    chain.link(State::<Storage<Todo>>::both(storage));
    Iron::new(chain).http(listen).unwrap();
}