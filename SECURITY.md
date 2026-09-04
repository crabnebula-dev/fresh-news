# Security Policy

## Supported Versions

This project is a Tauri v2 boilerplate. Only the `main` branch receives
security fixes.

| Version | Supported |
| ------- | --------- |
| `main`  | Yes       |
| Tagged releases older than the latest | No |

## Reporting a Vulnerability

**Please do not open a public GitHub issue for security problems.**

Report vulnerabilities privately through one of these channels:

1. **GitHub private vulnerability reporting** (preferred):
   <https://github.com/crabnebula-dev/fresh-news/security/advisories/new>
2. **Email**: <denjell@crabnebula.dev>

Include as much of the following as you can:

- A description of the issue and its impact
- Steps to reproduce, or a proof of concept
- The affected commit, tag, or version
- The platform (macOS, Windows, Linux) and Tauri/Rust/Node versions involved

A machine-readable copy of this contact information is in
[`.well-known/security.txt`](.well-known/security.txt).

### PGP Key

If you want to encrypt your report, use Daniel Thompson-Yvetot's public key.
It is also available at [`.well-known/pgp-key.txt`](.well-known/pgp-key.txt).

- **Fingerprint:** `30B1 B32B CBA8 3B55 EEB1 465A B16A 24AF 591B 824B`
- **User ID:** Daniel Thompson-Yvetot <denjell@crabnebula.dev>
- **Expires:** 2028-09-01

```
-----BEGIN PGP PUBLIC KEY BLOCK-----

mDMEaphfXhYJKwYBBAHaRw8BAQdAKWaHNRbUwFV+iQb28mPGoV1bCoe9Qoz3fOup
BRlJKc60L0RhbmllbCBUaG9tcHNvbi1ZdmV0b3QgPGRlbmplbGxAY3JhYm5lYnVs
YS5kZXY+iLUEExYKAF0WIQQwsbMry6g7Ve6xRlqxaiSvWRuCSwUCaphfXhsUgAAA
AAAEAA5tYW51MiwyLjUrMS4xMiwwLDMCGwMFCQPCZwAFCwkIBwICIgIGFQoJCAsC
BBYCAwECHgcCF4AACgkQsWokr1kbgktJRwD+Pal3qbUjAIHE9KOpIR3tqXEjLY2a
7cr9TqGS24FRLg8A/jBuJtlv1Y17sk74dCuM9FVl8YvLRXq5WkPmRKsa760IuDgE
aphfXhIKKwYBBAGXVQEFAQEHQM07ZUluQXCema2Ky8NwpKrTcTgbSi7+sHOeBHlG
nzccAwEIB4iUBBgWCgA8FiEEMLGzK8uoO1XusUZasWokr1kbgksFAmqYX14bFIAA
AAAABAAObWFudTIsMi41KzEuMTIsMCwzAhsMAAoJELFqJK9ZG4JLt+QA/2mlxT5Y
rQlZWb8qJXTDEsRoWqcrz38VLv5hRMjnCEE/AP9uX3mUuW9J2OcaJ0Y3mZsG6PQZ
yKiDdbugFXEtf0aJCg==
=Ol3T
-----END PGP PUBLIC KEY BLOCK-----
```

You should receive an acknowledgement within **3 business days**. We aim to
confirm the issue and agree on a fix timeline within **14 days**, and to ship
a fix for confirmed high-severity issues within **90 days** of the report.

We ask that you give us reasonable time to address the issue before any public
disclosure. We will credit reporters in the fix commit or advisory unless you
ask us not to.

## Scope

In scope:

- The Rust backend under `src-tauri/`, including all `#[tauri::command]`
  functions in `src-tauri/src/commands.rs`
- The IPC surface defined in `src/bindings.ts`
- Capability and permission grants in `src-tauri/capabilities/`
- The Tauri configuration in `src-tauri/tauri.conf.json`, including the
  Content Security Policy and window settings
- The build and release pipeline defined in this repository

Out of scope:

- Vulnerabilities in Tauri itself, in `@tauri-apps/*` packages, or in other
  upstream dependencies. Report those to the upstream project. Tauri's own
  policy is at <https://github.com/tauri-apps/tauri/security/policy>.
- Issues that require a compromised developer machine or a modified build
- Denial of service against the local dev server (`pnpm dev`), which is not
  intended to be exposed beyond `localhost`

## Security Model

A few properties of this codebase that reporters and contributors should know:

- **Deny by default.** Tauri v2 denies every plugin API not explicitly listed
  in `src-tauri/capabilities/default.json`. Adding a permission there widens
  the attack surface of the webview and should be reviewed as such.
- **Custom commands are the trust boundary.** Every function registered in
  `tauri::generate_handler!` is callable from the webview. Treat all arguments
  as untrusted input and validate them in Rust.
- **Content Security Policy.** `app.security.csp` in `src-tauri/tauri.conf.json`
  restricts every resource type to `'self'` and only allows connections to
  the Tauri IPC origin. A looser `devCsp` applies under `tauri dev` to permit
  Vite's HMR websocket and injected styles. Loosening the production CSP,
  especially adding `'unsafe-inline'` or remote origins, widens the attack
  surface and should be reviewed as such.
- **Global Tauri object.** `withGlobalTauri` is enabled, which exposes
  `window.__TAURI__` to all scripts in the webview. Any script injection in
  the frontend therefore reaches the IPC layer.
- **No secrets in the repository.** Signing keys, updater keys, and API
  credentials must never be committed. Use environment variables or your
  CI provider's secret store.

## Dependency Updates

Frontend dependencies are locked in `pnpm-lock.yaml` and Rust dependencies in
`src-tauri/Cargo.lock`. Run `pnpm audit` and `cargo audit` (from `src-tauri/`)
periodically and before tagging a release.
