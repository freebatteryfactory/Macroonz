# Compiler compile refusals

This lane asks rustc to observe public boundaries that lawful runtime values cannot cross.
Common fixtures run in every compiler posture.
With the `host` feature enabled, the host fixture requires source-span custody for emission.
Without it, the core fixture requires the host module to be unavailable.
The same import must compile when `host` is enabled, distinguishing feature absence from an independently malformed fixture.
That positive control uses a separate trybuild batch so it does not turn the compile-fail batch's metadata checks into code-generation observations.
Each posture has its own expected diagnostic; an unavailable API does not substitute for a malformed call to an available API.
The driver selects those fixtures explicitly, so neither feature-specific claim depends on an unrelated workspace member enabling the feature.
