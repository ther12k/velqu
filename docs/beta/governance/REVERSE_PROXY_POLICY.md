---
type: Deployment Policy
title: Velqu Public Beta Reverse-Proxy-First Policy
status: accepted
date: 2026-08-21
version: 0.1.0-beta.1
tags:
- beta
- deployment
- reverse-proxy
- tls
---

# Velqu Public Beta Reverse-Proxy-First Policy

## Accepted beta posture

Public TLS terminates at a trusted reverse proxy. The Velqu runtime serves
plain HTTP behind that proxy, normally bound to loopback. Direct runtime
TLS/HTTPS is not a supported beta promise.

The default runtime bind is `127.0.0.1`; operators must keep the listener on a
private interface when the proxy shares the host. Binding `0.0.0.0` exposes the
plain HTTP listener and is an explicit deployment choice, not the recommended
public posture.

## Proxy boundary

The reverse proxy is responsible for:

- public certificate management and TLS policy;
- HTTP-to-runtime forwarding;
- request size, connection, and timeout limits at the edge;
- access logging and external health checks;
- graceful drain coordination during deployment.

The proxy must forward only traffic intended for the Velqu runtime. Forwarded
host, scheme, and client-address metadata must be accepted only from a trusted
proxy boundary; clients must not be allowed to spoof trusted forwarding headers
by reaching the runtime directly.

## Runtime boundary

The runtime remains plain HTTP/1.1 and does not load certificates or private
keys in this beta. Readiness, liveness, startup, and drain behavior must be
wired through the deployment profile and verified before traffic is admitted or
removed.

This policy does not add forwarded-header parsing or native TLS behavior. Such
features require separate design, implementation, tests, and owner approval.

## Beta limits

- Reverse-proxy-first is a deployment posture, not a universal hosting claim.
- The beta is non-SLA, trusted-code-only, and not production-ready GA.
- Direct public exposure of the plain HTTP listener is unsupported as a secure
  public deployment pattern.
- Native runtime TLS, HTTP/2 termination, certificate rotation inside the
  runtime, and direct-TLS support require a new owner decision.
