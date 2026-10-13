# quic_trace instrumentation

This branch lets the relays built on web-transport join their MoQ objects to the
`quic_trace:*` events of the instrumented
[Quinn fork](https://github.com/5062/quinn/tree/moq-trace/quinn-0.11). It emits no
events of its own: it reports the identities the transport events carry, so a
relay can stamp them on its `moq_trace:*` events. It is a measurement fork, not
intended for upstream submission.

## Revision

The branch `moq-trace/web-transport-quinn-v0.11.11` starts at `af182f9`, the
release that ships `web-transport-quinn` 0.11.11.

## What the instrumentation emits

| Call site | Exposes |
| --- | --- |
| `web_transport_trait::Session::connection_id` | the process-wide connection ID of the Quinn connection, from `quinn::Connection::quic_trace_connection_id`, which every `quic_trace:*` event of that connection carries |
| `web_transport_trait::SendStream::stream_id`, `RecvStream::stream_id` | the QUIC stream ID and the stream offset of application byte zero |
| `web_transport::Session::connection_id`, `SendStream::stream_id`, `RecvStream::stream_id` | the same identities through the platform router; the browser backend reports none |

## Boundaries

- **Stream offsets.** WebTransport prefixes each stream with its type and session
  ID before application data, so application offset zero is a later QUIC
  offset. A stream identity carries that offset, which a relay adds to an
  application offset to get the range of the `quic_stream_frame` events that
  carried the bytes. A stream the peer opened learns it from the bytes read
  while decoding the prefix.
- **Identity.** The connection ID is the one the Quinn fork allocates with the
  connection and never reuses in the process, unlike Quinn's `stable_id`, an
  address a later connection can reuse.
- **Defaults.** The trait methods default to `None`, so another implementation of
  the traits builds unchanged and reports no identity.

## Build

`web-transport-quinn` reports the Quinn fork's connection ID, so it builds only
against that fork. The workspace manifest patches `quinn`, `quinn-proto`, and
`quic-trace` to the forks for a standalone build, and a consumer applies its own
patch.

## Verification

```sh
cargo test -p web-transport-quinn -p web-transport-trait
```

The tests connect real sessions and check that a session reports the Quinn
connection's trace ID and that both ends of each stream agree on its identity and
prefix offset. The events those identities join are checked in the Quinn fork,
by its `moq_trace/test_trace.sh`.
