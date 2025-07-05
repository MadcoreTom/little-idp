# NOT FOR PRODUCTION USE
# this is a toy project, and is not secure or reliable


# Premise

The idea of this project is to learn a bit about `rust`, and to build a basic identity provider supporting OAuth2.0, JWT, and/or OpenID-connect.

# Diagram

![Sequence Diagram](https://www.plantuml.com/plantuml/png/RP31QW9138RlUOgVFRK7wRs7ua4f1K6f-W1n9dK7iplBR4QnJzzHkdIGta8I_k_dcu_6oNAdr4SJnBCax4T9j1ypgrvXhI2N1IungyTHZ9BuaCIPU7obsE4QSm8F0rcylmS_u1Ai1SEnwe7TYN9IUjgj5-jdajxZ1YCYTXOdteqnUqEp2cOgiu_5fdd5-tA75yxMnXI-fP0jfnwjTGgDnz1ZuACyLb_sLaQNn4jlWNK45wJJu4vejldG1Mkg5l-JD-jjXQuPzH-TMODdanf7PIwhEtMqc0It6BZefT8Qnb-bayv9t-GC-yyL7fnD1lu2QLvkkTCV)

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