extern crate nickel;
extern crate redis;
extern crate r2d2;
extern crate r2d2_redis;
extern crate plugin;
extern crate typemap;

pub use middleware::{ RedisMiddleware, RedisRequestExtensions };

mod middleware;
