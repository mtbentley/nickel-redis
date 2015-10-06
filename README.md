nickel-redis
=======

A redis middleware for nickel.rs

Adapted almost 100% from nickel-org/nickel-postgres, using nvdelap/r2d2-redis 
for pooled connections.

## Usage

See examples for usage.

## Lib vs Unboxed closures

Much of the utility of this library can be gained from using unboxed
closures to capture a connection pool, this also removes the risk of
accessing a connection pool in a handler attached before the middleware.

### `Cargo.toml`:

    [dependencies.nickel_redis]
    git = "[TODO: github URL here]"

## Licence

MIT
