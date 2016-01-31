extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {

  let mut router = Router::new();

  router.get("/", hello_world);
  router.get("/test", test);

  fn hello_world(_: &mut Request) -> IronResult<Response>{
    let greeting = "Hello Universe!";
    Ok(Response::with((status::Ok, greeting)))
  }

  fn test(_: &mut Request) -> IronResult<Response>{
    Ok(Response::with((status::Ok, "hi")))
  }

  Iron::new(router).http("localhost:3000").unwrap();
}
