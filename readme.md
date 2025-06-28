The idea of this project is to learn a bit about `rust`, and to build a basic identity provider supporting OAuth2.0, JWT, and/or OpenID-connect.

# Commands

## Build
```
cargo build
```

The build result goes in `./target/debug`

Add `--release` for an optimised build, that goes in `./target/release`

## Run (debug)

```
./target/debug/little-idp
```
_little-idp_ is the name of the project in `Cargo.toml`

## Run and Build (in one step)

```
cargo run
```