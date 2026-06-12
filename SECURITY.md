# Security Policy

## Supported Versions

Only the latest release receives security fixes.

| Version | Supported |
| ------- | --------- |
| 1.x     | ✅        |

## Reporting a Vulnerability

**Please do not open a public GitHub issue for security vulnerabilities.**

Report security issues privately via GitHub's built-in mechanism:

1. Go to the [Security tab](https://github.com/jhonsferg/gvm/security/advisories/new) of this repository.
2. Click **"Report a vulnerability"**.
3. Fill in the details: affected versions, reproduction steps, and potential impact.

You will receive an acknowledgement within **72 hours** and a resolution timeline
within **7 days** for critical issues.

## Scope

- Arbitrary code execution via crafted `.go-version` files or default-packages entries
- Path traversal in archive extraction
- Binary substitution during `gvm install` or `gvm upgrade` (SHA-256 bypass)
- Credential or secret leakage in logs or error messages

## Out of Scope

- Issues in Go toolchains themselves (report to the [Go team](https://go.dev/security))
- Social engineering or phishing
- Vulnerabilities in systems that `gvm` does not control (e.g. go.dev infrastructure)
