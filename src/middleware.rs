use std::sync::Arc;
use std::error::Error as StdError;

use nickel::{Request, Response, Middleware, Continue, MiddlewareResult};
use r2d2_redis::RedisConnectionManager;
use r2d2::{Pool, HandleError, Config, PooledConnection};
use typemap::Key;
use plugin::Extensible;

pub struct RedisMiddleware {
    pub pool: Arc<Pool<RedisConnectionManager>>,
}

impl RedisMiddleware {
    /// Creates a new connection to `connect_str` with `num_connections` in the
    /// pool and using `error_handler` as the error handler.
    ///
    /// # Examples
    /// ```
    /// # extern crate r2d2;
    /// # extern crate nickel;
    /// # extern crate nickel_redis;
    /// #
    /// # use r2d2::NopErrorHandler;
    /// # use nickel::Nickel;
    /// # use nickel_redis::RedisMiddleware;
    /// #
    /// # fn main() {
    /// let mut app = Nickel::new();
    /// let server_url = "redis://127.0.0.1/".to_string();
    /// let dbpool = RedisMiddleware::new(&*server_url,
    ///                                   5,
    ///                                   Box::new(NopErrorHandler)).unwrap();
    /// app.utilize(dbpool);
    /// # }
    /// ```
    pub fn new(connect_str: &str,
               num_connections: u32,
               error_handler: Box<HandleError<::r2d2_redis::Error>>)
               -> Result<RedisMiddleware, Box<StdError>> {
        let manager = try!(RedisConnectionManager::new(connect_str));

        let config = Config::builder()
            .pool_size(num_connections)
            .error_handler(error_handler)
            .build();

        let pool = try!(Pool::new(config, manager));

        Ok(RedisMiddleware { pool: Arc::new(pool) })
    }
}

impl Key for RedisMiddleware {
    type Value = Arc<Pool<RedisConnectionManager>>;
}

impl<D> Middleware<D> for RedisMiddleware {
    fn invoke<'mw, 'conn>(&self,
                          req: &mut Request<'mw, 'conn, D>,
                          res: Response<'mw, D>)
                          -> MiddlewareResult<'mw, D> {
        req.extensions_mut().insert::<RedisMiddleware>(self.pool.clone());
        Ok(Continue(res))
    }
}

pub trait RedisRequestExtensions {
    /// Returns the pool for the Redis connection.
    ///
    /// Call `.deref()` on the result to get a connection.
    ///
    /// Requires `#![feature(core)]`, `extern crate core`, and `use core::ops::Deref` to use
    /// `.deref()`.
    fn redis_conn(&self) -> PooledConnection<RedisConnectionManager>;
}

impl<'a, 'b> RedisRequestExtensions for Request<'a, 'b> {
    fn redis_conn(&self) -> PooledConnection<RedisConnectionManager> {
        self.extensions().get::<RedisMiddleware>().unwrap().get().unwrap()
    }
}
