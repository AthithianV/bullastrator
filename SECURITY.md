# Security Policy

## Supported Versions

Security fixes are generally provided for the latest released version of Asyncian.

| Version                         | Supported      |
| ------------------------------- | -------------- |
| Latest release                  | ✅             |
| Older releases                  | ⚠️ Best effort |
| Unreleased development versions | ❌             |

---

## Reporting a Vulnerability

Please **do not report security vulnerabilities through public GitHub issues**.

If you discover a security vulnerability in Asyncian, please report it privately to the project maintainer.

Include as much of the following information as possible:

- Description of the vulnerability
- Steps required to reproduce it
- Potential impact
- Affected versions
- Proof of concept, if available
- Suggested mitigation, if known

Please allow reasonable time for the vulnerability to be investigated and fixed before publicly disclosing it.

---

## Sensitive Information

When reporting issues, never include:

- Redis passwords
- API keys
- Authentication tokens
- Private keys
- Production connection strings
- Personal information
- Customer data

Redact sensitive information from logs and screenshots before sharing them.

---

## Current Security Considerations

Asyncian currently supports direct Redis connections.

> ⚠️ Redis credentials are not yet encrypted at rest.

Until encrypted credential storage is implemented, users should avoid storing sensitive production credentials in Asyncian.

TLS/SSL support and secure credential storage are planned improvements.

---

## Security Updates

Security-related fixes will be included in project releases and documented in the release notes when appropriate.
