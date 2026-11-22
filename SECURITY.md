# Security Policy

## Supported Versions
| Version | Supported |
| --- | --- |
| 0.1.0-beta.1 | Yes |
| < 0.1.0 | No |

## Reporting a Vulnerability
If you discover a potential security vulnerability in AD Launcher, please report it privately:
- Contact: `security@example.com` or private maintainer inbox.
- Do not file public GitHub issues for security vulnerabilities.
- Provide steps to reproduce and any diagnostic logs with personal data redacted.

## Security Commitments
- **Zero Token Leakage**: Diagnostics and logs strictly exclude access tokens and refresh tokens.
- **Local-First**: No remote telemetry or unverified background daemons.
- **Permission Boundary**: Filesystem writes are scoped to the configured Minecraft directory and launcher configuration root.
