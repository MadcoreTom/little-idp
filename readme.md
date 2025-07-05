# NOT FOR PRODUCTION USE
# this is a toy project, and is not secure or reliable


# Premise

The idea of this project is to learn a bit about `rust`, and to build a basic identity provider supporting OAuth2.0, JWT, and/or OpenID-connect.

# Diagram

![Sequence Diagram](https://www.plantuml.com/plantuml/png/XL9DQnin4BthLppsb49DQ7ipYPCq2QKGiAcT7p3MJjV2MgcLHZ7kh-_8zk9j2EbjTlJsVUZfDWibARqtXgp4XEOvSyB5EiMT3XyQK4PT6NCucComNDnX2X-fnI_g65ThijkftLEOtNszlxgidpEJM4eA1uGn8GeZld0wKjrxPxSPZO-T2weGMdGi6ObRbzWAjuPrL_ywopPFJiJpvFl3efdYs-CAbrHa4vFxmpUMlA-wr--QsKZlKKMFlbxRUWoYerKN7CIHpmTdjff5iophtabZLXj6KNIWdkkvCkMyYwar5-HZwB1pie4eQl24tuNJNd59Vn6zKwDXaLK3bF4pfXuj2VqtycA-76KUkn_Zp0jvftmy3c9q9ZdwpH8JezB0ndQepxkUkyGSPfy-VnckxNYaDBFL_65-slwRhIIFl8d5jrWp1kqMr94BMK1GPDnDlCjoJdZgO-WR4i-Cpp-UtdvIDzNkzJjsN_Nvjdq5xL1oKRazv5HclPZwhetGbdNhCghOHumg9MIsYSLKR68jzH1FufR34U562SsjZgNtVm40)

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