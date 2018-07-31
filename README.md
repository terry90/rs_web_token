# Web token

Easy to use WebToken for rust

[![Build Status](https://travis-ci.org/terry90/rs_web_token.svg?branch=master)](https://travis-ci.org/terry90/rs_web_token)
[![codecov](https://codecov.io/gh/terry90/rs_web_token/branch/master/graph/badge.svg)](https://codecov.io/gh/terry90/rs_web_token)

## Usage

```rust
// Create new random token
let new_token = WebToken::new();

// Get an existing token
let incoming_token: WebToken = request.header("X-Custom-Token").parse().unwrap();
assert_eq!(incoming_token, request.header("X-Custom-Token"));
```
