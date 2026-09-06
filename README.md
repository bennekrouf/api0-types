# api0-types

The wire types shared by the api0 gateway, store and SDK. One definition per
payload, so the three cannot drift apart.

```toml
[dependencies]
api0-types = { git = "https://github.com/bennekrouf/api0-types", rev = "<commit>" }
```

Consumers pin an exact `rev` rather than tracking a branch: a wire-type change
must be a deliberate, reviewable bump in each service, not something that
arrives on the next `cargo update`. To change a type — edit here, push, then
bump the `rev` in the gateway, the store and the SDK. For local iteration,
override the git source with a `[patch]` section pointing at your working copy
(the SDK's `Cargo.toml` carries a commented-out example).

Dependency-light on purpose — serde, serde_json, chrono, uuid — no web
framework and no database driver, so any api0 component can depend on it.

## The error contract

`ApiError` defines the platform-wide failure envelope. Every api0 HTTP service
obeys both halves:

1. **The HTTP status reflects the outcome.** A failure is never 2xx.
2. **A failure body is exactly** `{"success": false, "error": "<message>", "code": "<code>"}`.

`error` is a plain string so existing clients that read `data.error` keep
working; `code` is the machine-readable discriminant clients should branch on.
`ErrorCode` deserialises unknown values to `Unknown`, so a client built against
an older version of this crate never fails to parse a newer service's error.

The gateway enforces this structurally: handlers return `ApiResult<T>`, and the
`ApiFailure` responder derives the status from the code it carries. The store
keeps its own internal shapes; the gateway normalises them at the boundary
(`ApiFailure::from_store`), which is the only surface public clients see.
