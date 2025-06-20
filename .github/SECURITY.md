# Security Policy

## Supported Versions

We actively support the following versions of open-lark:

| Version | Supported          |
| ------- | ------------------ |
| 0.4.x   | :white_check_mark: |
| 0.3.x   | :white_check_mark: |
| < 0.3   | :x:                |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security vulnerability in open-lark, please report it to us privately.

### How to Report

1. **Email**: Send details to [zhooul@gmail.com] with subject "SECURITY: open-lark vulnerability"
2. **GitHub Security Advisory**: Use GitHub's private vulnerability reporting feature

### What to Include

Please include the following information in your report:

- Description of the vulnerability
- Steps to reproduce the issue
- Potential impact assessment
- Any suggested fixes or mitigations

### Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Fix Timeline**: Varies based on severity, typically within 30 days for critical issues

### Security Best Practices

When using open-lark:

1. **API Keys**: Never hardcode API keys in your source code
2. **Environment Variables**: Use `.env` files for sensitive configuration (excluded from version control)
3. **HTTPS**: Always use HTTPS endpoints when communicating with Lark/Feishu APIs
4. **Input Validation**: Validate all user inputs before processing
5. **Error Handling**: Avoid exposing sensitive information in error messages

### Responsible Disclosure

We follow responsible disclosure practices:

1. We will acknowledge receipt of your vulnerability report
2. We will investigate and validate the reported vulnerability
3. We will develop and test a fix
4. We will coordinate the release of the fix
5. We will publicly acknowledge your contribution (if desired)

Thank you for helping keep open-lark and its users safe!