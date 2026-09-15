//! A generic WebTransport interface.
//!
//! The underlying implementation switches based on the platform:
//!  - native: [web-transport-quinn](https://docs.rs/web-transport-quinn/latest)
//!  - web: [web-transport-wasm](https://docs.rs/web-transport-wasm/latest)
//!
//! WASM lacks server support, so for native you first establish a [web_transport_quinn::Session] and then use [Session::from()] to cast to this generic interface.

#[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
#[path = "quinn.rs"]
mod quic;

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
#[path = "wasm.rs"]
mod quic;

pub use quic::*;

/// A process-local transport connection identity, as reported by the platform.
///
/// The value is stable for the lifetime of the connection and unique within the
/// process, which makes it usable as a key when correlating streams across the
/// layers above the transport. Browsers do not expose one.
pub use web_transport_trait::ConnectionId;

/// A transport stream identity paired with the offset it starts at.
///
/// WebTransport prefixes each stream, so the transport offset of application
/// byte zero is not necessarily zero; add it to an application offset to get the
/// absolute QUIC stream offset.
pub use web_transport_trait::StreamId;
