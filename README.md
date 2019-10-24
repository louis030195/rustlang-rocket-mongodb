# rustlang-rocket-mongodb
[![Try it on gitpod](https://img.shields.io/badge/try-on%20gitpod-brightgreen.svg)](https://gitpod.io/#https://github.com/louis030195/rustlang-rocket-mongodb)

Code for the [tutorial](https://medium.com/@louis.beaumont/rest-api-with-rust-mongodb-10eeb6bd51d7)

## Installation

```bash
sudo apt update
sudo apt install mongodb-org
```

Check if mongodb is healthy

```bash
service mongodb status
```

## Usage

```bash
echo -e "MONGO_ADDR=localhost
DB_NAME=rustlang-rocket-mongodb
MONGO_PORT=27017" > .env
```


```bash
rustup default nightly # Pear requires a nightly or dev version of Rust
```

```bash
cargo run &

# POST
curl -d '{"name": "chichi"}' -H "Content-Type: application/json" -X POST http://localhost:8001/cats
# Or
curl -d '{"name": "chacha", "age": 12, "color": "grey"}' \
-H "Content-Type: application/json" -X POST http://localhost:8001/cats
# Or empty
curl -d '{}' -H "Content-Type: application/json" -X POST http://localhost:8001/cats

# PUT
curl -d '{"$oid": "5db15a686539303d5708901f", "name": "chichi"}' -H "Content-Type: application/json" \
-X PUT http://localhost:8001/cats/5db15a686539303d5708901f

# GET
curl http://localhost:8001/cats
# Find by id
curl http://localhost:8001/cats/5db15a1f6539303d5708901e

# DELETE
curl -H "Content-Type: application/json" -X DELETE http://localhost:8001/cats/5db15a1f6539303d5708901e

# DELETE all
curl -H "Content-Type: application/json" -X DELETE http://localhost:8001/cats
```

## Tests

To avoid running parallel tests we use --test-threads=1 because we modify database, otherwise tests would fail.

```rust
cargo test -- --test-threads=1
```