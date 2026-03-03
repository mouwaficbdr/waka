# Security Policy

## Supported Versions

| Version            | Supported              |
| ------------------ | ---------------------- |
| latest             | ✅                     |
| < latest - 1 minor | ⚠️ Critical fixes only |
| older              | ❌                     |

## Reporting a Vulnerability

**Please do not report security vulnerabilities as public GitHub issues.**

### Option 1: GitHub Private Security Advisory (preferred)

Use [GitHub's private vulnerability reporting](https://github.com/mouwaficbdr/waka/security/advisories/new).

### Option 2: Email

Send details to: **badaroumouwafic@gmail.com**

### What to include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### What to expect

- **Acknowledgement within 72 hours**
- Regular updates on the fix progress
- Credit in the security advisory (unless you prefer anonymity)
- A CVE will be requested for significant vulnerabilities

## Security Design Notes

`waka` is designed with security in mind:

- **API keys** are stored in the OS keychain by default (macOS Keychain, Linux Secret Service, Windows Credential Manager)
- **TLS** is enforced via `rustls` — no system OpenSSL dependency, certificate verification cannot be disabled
- **No telemetry** — `waka` never contacts any server other than the configured WakaTime API endpoint
- **Minimal permissions** — `waka` writes only to its config directory (`~/.config/waka/`) and cache directory (`~/.cache/waka/`)
- **Read-only compatibility** — `~/.wakatime.cfg` is read but never written

## Hall of Fame

_Thank you to everyone who has responsibly disclosed vulnerabilities._

<!-- Will be populated as reports come in -->
