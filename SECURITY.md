# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| Latest  | ✅                 |
| < 1.0   | ❌                 |

## Reporting a Vulnerability

### Preferred Method

**Email**: security@gatemate.app

Please include the following information in your report:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Your suggested fix (if any)
- Your contact information

### Alternative Method

If you cannot use email, you can:

1. Create a private GitHub Security Advisory
2. Describe the vulnerability in detail
3. Include reproduction steps

### Security Response Process

1. **Acknowledgment**: We will acknowledge receipt of your report within 48 hours
2. **Assessment**: Our security team will assess the severity and impact
3. **Fix Development**: We will develop and test a fix
4. **Notification**: We will notify you when the fix is ready
5. **Public Disclosure**: We will publicly disclose the vulnerability after the fix is released
6. **Acknowledgment**: We will acknowledge your contribution in the release notes

## Security Best Practices

### Data Protection

- GateMate uses AES-256-GCM encryption for API key storage
- Encryption keys are generated securely using OS-native random sources
- All sensitive data is zeroized after use

### Network Security

- LAN proxy binds to `127.0.0.1` by default
- External access requires explicit configuration
- HTTPS/TLS is required for external API communications

### Plugin Security

- Only load plugins from trusted sources
- Review plugin code before loading
- Report suspicious plugin behavior

## Encryption Standards

- **Algorithm**: AES-256-GCM
- **Key Length**: 256 bits
- **IV Length**: 12 bytes
- **Tag Length**: 16 bytes
- **Padding**: None (authenticated encryption)

## Responsible Disclosure

We encourage responsible disclosure of security vulnerabilities. If you discover a security issue, please:

1. Do not exploit the vulnerability
2. Do not disclose it publicly until we have had a chance to fix it
3. Provide detailed information to help us understand and fix the issue
4. Allow reasonable time for us to respond and fix the issue before public disclosure

## Contact

For security-related inquiries:

- **Email**: security@gatemate.app
- **PGP**: Available upon request

## License

GateMate is open source software licensed under the MIT License. See [LICENSE](LICENSE) for details.