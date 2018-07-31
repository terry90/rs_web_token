# Web token

Easy to use WebToken for rust

## Usage

```rust
let new_token = WebToken::new();

let incoming_token: WebToken = request.header("X-Custom-Token").parse().unwrap();
assert_eq!(incoming_token, request.header("X-Custom-Token"));
```
