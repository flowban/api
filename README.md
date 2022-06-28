# Flowban API

## Build

To build the project you'll need [Rust](https://www.rust-lang.org/learn/get-started) installed.

You will need to create a `.env` file with the database Mongo instance URL:

```
TESTING_URL="mongodb+srv://<username>:<password>@<instance>"
```

Also, add a secret key to sign the JWT tokens:

```
UUID="8de0202c-e1c0-4968-89d9-8a41e7d77c7f"
```

This is just an example.

Finally, run:

```
cargo run
```

You'll be able to access the [`API`](http://127.0.0.1:8000).

## Issues
Feel free to leave an [issue](https://github.com/flowban/api/issues)
