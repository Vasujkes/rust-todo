extern crate iron;
extern crate serde_json;
extern crate bodyparser;

use iron::prelude::*;
use iron::{headers, status};
use iron::modifiers::Header;
use router::Router;
use persistent::State;
use std::error::Error;

use crate::todo::*;

pub fn get_version(_: &mut Request) -> IronResult<Response> {
    let version = env!("CARGO_PKG_VERSION");
    return Ok(Response::with((status::Ok, version)));
}

pub fn get_todo(r: &mut Request) -> IronResult<Response> {
    let id_option = r.extensions
        .get::<Router>()
        .unwrap()
        .find("id")
        .unwrap()
        .parse::<u32>();

    match id_option {
        Ok(id) => {
            let mutex = r.get::<State<Storage<Todo>>>().unwrap();
            let storage = mutex.read().unwrap();
            match storage.get(&id) {
                Some(todo) => Ok(Response::with((
                    status::Ok,
                    Header(headers::ContentType::json()),
                    serde_json::to_string(&todo).unwrap(),
                ))),
                None => Ok(Response::with((
                    status::NotFound,
                    format!("todo with ID {} was not found", id),
                ))),
            }
        }
        Err(e) => Ok(Response::with((
            status::BadRequest,
            format!("could not parse input: {}", e.description()),
        ))),
    }
}

pub fn create_todo(r: &mut Request) -> IronResult<Response> {
    let body = r.get::<bodyparser::Struct<Todo>>();
    match body {
        Ok(Some(todo)) => {
            let mutex = r.get::<State<Storage<Todo>>>().unwrap();
            let mut storage = mutex.write().unwrap();
            storage.add(todo);

            Ok(Response::with((status::Created)))
        }
        Ok(None) => Ok(Response::with(
            (status::BadRequest, String::from("no data provided")),
        )),
        Err(err) => Ok(Response::with((
            status::BadRequest,
            format!("bad request body provided: {:?}", err.cause()),
        ))),
    }
}


pub fn delete_todo(r: &mut Request) -> IronResult<Response> {
    let id_option = r.extensions
        .get::<Router>()
        .unwrap()
        .find("id")
        .unwrap()
        .parse::<u32>();

    match id_option {
        Ok(id) => {
            let mutex = r.get::<State<Storage<Todo>>>().unwrap();
            let mut storage = mutex.write().unwrap();
            storage.remove(&id);
            Ok(Response::with((status::NoContent)))
        }
        Err(e) => Ok(Response::with((
            status::BadRequest,
            format!("could not parse input: {}", e.description()),
        ))),
    }
}