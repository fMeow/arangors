<!-- cargo-sync-readme start -->

An example to use reqwest with rustls.

1. use vanilla arangors without any http client implementation by disabling
`reqwest_async`, `reqwest_blocking` and `surf_async` on arangors.
2. implement custom reqwest client and enable `rustls` feature gate like in `src/client.rs`.
3. use custom client with `arangors::GenericConnection`.

<!-- cargo-sync-readme end -->
