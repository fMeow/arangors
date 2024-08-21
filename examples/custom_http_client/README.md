<!-- cargo-sync-readme start -->

An example to implement custom http client

1. use vanilla arangors without any http client implementation by disabling
`reqwest_async`, `reqwest_blocking` and `surf_async` on arangors.
2. implement custom client, like the custom `reqwest` client in `src/client.rs`.
3. use custom client with `arangors::GenericConnection`.

<!-- cargo-sync-readme end -->
