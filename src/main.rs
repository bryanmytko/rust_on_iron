#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate core;
extern crate iron;
extern crate router;
extern crate serde;
extern crate serde_json;

use core::fmt::Debug;
use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
struct Greeting {
  msg: String
}

struct Student {
  id: i32,
  name: String,
  email: String,
  age: i32
}

fn main() {

  let mut router = Router::new();

  router.get("/", hello_world);
  router.post("/students/:id", students);

  fn hello_world(_: &mut Request) -> IronResult<Response>{
    let greeting = Greeting { msg: String::from("Hello Universe!") };
    let payload = serde_json::to_string(&greeting).unwrap();

    Ok(Response::with((status::Ok, payload)))
  }

  fn students(request: &mut Request) -> IronResult<Response>{
    let student = Student {
      id: 1,
      name: String::from("Bryan"),
      email: String::from("bryan@email.com"),
      age: 33
    };

    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let request: Greeting = serde_json::from_str(&payload).unwrap();
    let mut greeting = Greeting { msg: request.msg };
    let payload = serde_json::to_string(&greeting).unwrap();

    /* match student.id == 1 { */
    /*   true => { greeting.msg = payload }, */
    /*   false => { greeting.msg = String::from("BOOO") } */
    /* } */

    Ok(Response::with((status::Ok, payload)))
  }

  Iron::new(router).http("localhost:3000").unwrap();
}
