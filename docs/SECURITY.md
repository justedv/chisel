# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in Anvil, please report it responsibly.

**Do not open a public issue.**

Instead, email the maintainers directly or open a private security advisory on GitHub.

## Scope

Anvil is a CPI helper library — it constructs and dispatches cross-program invocations. Security considerations include:

- **Instruction data correctness** — malformed instruction data could cause unexpected program behavior
- **Account meta flags** — incorrect `is_signer` or `is_writable` flags could lead to unauthorized operations
- **Signer seed handling** — improper seed construction in `_signed` variants could expose PDA vulnerabilities

## Audit Status

Anvil has not yet undergone a formal security audit. Use in production at your own discretion. We welcome community review and responsible disclosure.
