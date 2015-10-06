extern crate r2d2;
extern crate redis;
extern crate openssl;
#[macro_use] extern crate nickel;
extern crate nickel_redis;

use std::env;
use r2d2::NopErrorHandler;
use nickel::{Nickel, HttpRouter};
use nickel_redis::{RedisMiddleware, RedisRequestExtensions};

fn main() {
    let mut app = Nickel::new();

    let redis_url = env::var("DATABASE_URL").unwrap();
    let dbpool = RedisMiddleware::new(&*redis_url,
                                         5,
                                         Box::new(NopErrorHandler)).unwrap();
    app.utilize(dbpool);
    app.get("/my_counter", middleware! { |request|
        let _connection = request.redis_conn();
        println!("{:?}", _connection);

        // use connection
    });

    app.get("**", middleware! { println!("!!!") });
}
