# Security Policy

## Reporting a vulnerability

Report suspected vulnerabilities through GitHub Security Advisories for
[`ther12k/velqu`](https://github.com/ther12k/velqu/security/advisories/new).
This private channel keeps vulnerability details away from public issues until
maintainers coordinate disclosure.

Do not report vulnerabilities in public GitHub issues, pull requests, or
community discussions. Do not publish exploit details, credentials, private
user data, or proof-of-concept material before coordinated disclosure.

Include, when safe to share:

- affected version, commit, package, or runtime profile;
- impact and attack prerequisites;
- minimal reproduction steps or a safe proof of concept;
- logs, traces, or configuration needed to reproduce the issue;
- any known workaround or suspected remediation;
- whether the report may contain sensitive data.

If the advisory form is unavailable, do not move sensitive details to a public
issue. Use the repository's private GitHub security reporting capability and
provide only non-sensitive context publicly if a tracking issue is necessary.

## Triage and disclosure

Maintainers will acknowledge and triage reports through GitHub's private
security workflow. They may request additional reproduction details, assign a
severity, prepare a fix, and coordinate disclosure through the advisory record.
No response-time or remediation SLA is promised for beta releases.

Confirmed issues may result in a patched release, advisory publication,
version withdrawal, package yank, or documented workaround. Release authority
and withdrawal rules are defined in
[`docs/beta/governance/RELEASE_AUTHORITY.md`](docs/beta/governance/RELEASE_AUTHORITY.md).

## Supported scope

This policy covers the Velqu repository, its first-party packages, the
`velqu-runtime` binary, and the trusted application-code execution model. The
public beta is non-SLA and not production-ready GA. Velqu is not described as a
hostile-code sandbox.
