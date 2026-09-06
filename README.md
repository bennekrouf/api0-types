# api0-types

The wire types shared by the api0 gateway, store and SDK. One definition per
payload, so the three cannot drift apart.

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
