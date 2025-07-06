# NOT FOR PRODUCTION USE
# this is a toy project, and is not secure or reliable


# Premise

The idea of this project is to learn a bit about `rust`, and to build a basic identity provider supporting OAuth2.0, JWT, and/or OpenID-connect.

# Diagram

![Sequence Diagram](https://www.plantuml.com/plantuml/png/XLBDRjim3BxhAGYVEc11iEqMj6cxjXWs54kmf0_0o9mjH9OwYQgHFVqenKQyecXkDlM9ty-dcyWOE7LMeMOVe7gA5E1Y5tml7nygm0Xvf5Gvc2rXixw51LYFDJnZGt2vGxrVw0EwvUtNkyjv_bsgG9o2AmYbd6S2_qBXkEhE6hsFK5dV62SCheQ6E4AWsWJIJBKYcULR9zhgqJ1Rcds_NrSB-FQmXJacRdqmV-bQexMPz-fTCIrt5WffqVLQreDZ-TIYWXmRjB4eqraiL9kqwmnNQjiI944xxAYSFsEClG-rka3hNGEzuHPOKACc-9Ce70GNv0fBfahCGXIEdErl7pgeaV6ivVLgC_KyKJ_5gHUqHlRHr8YIR-IZtiW-4EGre7qzrcTDHqqWSijFdx-CiHsFPCrokxfV3TF_lQLW8RO-sHfs12Dt3TYWSP41GP2-draJ-UHmw6Fi6zYVDZxzUdpxILrdkLVlo7tLvzlo1THBWKde3i13cIKOUTUASKyoDR5ayn4cbI94qY9SPMmWARNOOxydTqGOFjCX3E_Mv_JdvUPPKqDK0y-FpUgdKZT2aphx3m00)

## Endpoints

* `GET  /oauth/authorize?callback=AAA` load the login form
* `POST /oauth/authorize` with formdata content, submit the username, password and callback url. Redirects to the callback URL with the auth code as a `code` query parameter
* `GET  /oauth/token?code=BBB` exchange an auth code for the token
* `GET /.well-known/openid-configuration` returns a minimal payload (following the [spec](https://openid.net/specs/openid-connect-discovery-1_0.html#WellKnownRegistry)) describing configuration information like endpoints

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