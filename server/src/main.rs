extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;

fn main() {
    let mut router = Router::new();
    router.get("/", get_form, "root");
    // router.post("/gcd", post_gcd, "gcd");

    println!("serving on localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut("Text/Html; Charset=Utf8".parse::<Mime>().unwrap());
    response.set_mut(r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
          <input type="text" name="n"/>
          <input type="text" name="n"/>
          <button type="submit">Compute GCD</button>
        </form>
    "#);

    Ok(response)
}

fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15),1);
}
