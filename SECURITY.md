# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Which versions are eligible for receiving such patches depends on the CVSS v3.0 Rating:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

The Shlack team and community take security bugs seriously. We appreciate your efforts to responsibly disclose your findings, and will make every effort to acknowledge your contributions.

### How to Report a Security Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to: **[contact@ageha734.jp](mailto:contact@ageha734.jp)**

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

### What to Include in Your Report

Please include the requested information listed below (as much as you can provide) to help us better understand the nature and scope of the possible issue:

* Type of issue (e.g. buffer overflow, SQL injection, cross-site scripting, etc.)
* Full paths of source file(s) related to the manifestation of the issue
* The location of the affected source code (tag/branch/commit or direct URL)
* Any special configuration required to reproduce the issue
* Step-by-step instructions to reproduce the issue
* Proof-of-concept or exploit code (if possible)
* Impact of the issue, including how an attacker might exploit the issue

This information will help us triage your report more quickly.

### Preferred Languages

We prefer all communications to be in English or Japanese.

## Security Update Process

1. **Report received**: We acknowledge receipt of your vulnerability report within 48 hours.

2. **Initial assessment**: We perform an initial assessment of the report within 5 business days.

3. **Confirmation**: If the vulnerability is confirmed, we will:
   * Assign a CVE number if applicable
   * Determine the severity using CVSS v3.0
   * Create a timeline for the fix

4. **Fix development**: We develop and test a fix for the vulnerability.

5. **Coordinated disclosure**: We coordinate with the reporter on the disclosure timeline.

6. **Release**: We release the security update and publish a security advisory.

## Security Best Practices for Users

### Token Security

* **Never commit your Slack token to version control**
* Use environment variables to store your token
* Rotate your tokens regularly
* Use the principle of least privilege for token permissions

### Installation Security

* Always download releases from the official GitHub repository
* Verify checksums when available
* Use the official install script from the main branch
* Keep your installation up to date

### Network Security

* Be aware that Shlack communicates with Slack's API over HTTPS
* Monitor your network traffic if required by your security policy
* Consider using Shlack in isolated environments for sensitive operations

## Security Features

### Current Security Measures

* All communications with Slack API use HTTPS
* No sensitive data is logged by default
* Minimal dependencies to reduce attack surface
* Regular dependency updates through automated tools

### Planned Security Enhancements

* Token encryption at rest
* Audit logging capabilities
* Enhanced input validation
* Security scanning in CI/CD pipeline

## Vulnerability Disclosure Policy

We follow responsible disclosure practices:

* We will acknowledge receipt of vulnerability reports within 48 hours
* We will provide regular updates on our progress
* We will credit researchers who report vulnerabilities (unless they prefer to remain anonymous)
* We will not take legal action against researchers who follow this policy

## Bug Bounty Program

Currently, we do not have a formal bug bounty program. However, we greatly appreciate security research and will acknowledge contributors in our security advisories and release notes.

## Contact Information

For security-related questions or concerns:

* **Security Email**: [contact@ageha734.jp](mailto:contact@ageha734.jp)
* **General Contact**: [contact@ageha734.jp](mailto:contact@ageha734.jp)
* **GitHub Issues**: For non-security related bugs only

## Security Hall of Fame

We will recognize security researchers who help improve Shlack's security:

<!-- Security researchers will be listed here -->

*No security vulnerabilities have been reported yet.*

---

Thank you for helping keep Shlack and our users safe!
