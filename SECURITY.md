# Security policy

## Reporting

Please report vulnerabilities privately to `security@sociobot.in`. Do not include live access tokens, private repository content, or archive data in a report.

## Token hygiene

Use narrowly scoped tokens in environment variables, never in `forge-sync.toml`. Rotate both source and target credentials after suspected exposure. forge-sync redacts configured token values from its own errors and does not persist them, but Git and forge server logs remain under the operator's control.
