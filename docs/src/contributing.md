# Contributing

Thank you for your interest in contributing to `waka`!

## Getting started

1. Fork the repository on [GitHub](https://github.com/mouwaficbdr/waka)
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/waka`
3. Install Rust (MSRV 1.82): [rustup.rs](https://rustup.rs)
4. Run the tests: `cargo nextest run`

## Code of conduct

Please read [CODE_OF_CONDUCT.md](https://github.com/mouwaficbdr/waka/blob/main/CODE_OF_CONDUCT.md)
before contributing.

## Development workflow

- Check `DEVELOPMENT_PLAN.md` before starting work
- Write tests alongside implementation
- Run `cargo fmt --all`, `cargo clippy -- -D warnings`, and `cargo nextest run` before committing
- Follow [Conventional Commits](https://www.conventionalcommits.org/)

## Reporting bugs

Open a [GitHub Issue](https://github.com/mouwaficbdr/waka/issues/new?template=bug_report.md).
Include:

- `waka --version`
- Your OS and terminal
- Steps to reproduce
- Expected vs actual behavior

## Security vulnerabilities

Please do **not** open a public issue for security vulnerabilities.
See [SECURITY.md](https://github.com/mouwaficbdr/waka/blob/main/SECURITY.md) for the responsible disclosure policy.
