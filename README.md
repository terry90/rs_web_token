# Web token

Easy to use WebToken for rust

## Usage

```rust
// Create new random token
let new_token = WebToken::new();

// Get an existing token
let incoming_token: WebToken = request.header("X-Custom-Token").parse().unwrap();
assert_eq!(incoming_token, request.header("X-Custom-Token"));
```
