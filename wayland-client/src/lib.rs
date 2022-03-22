//! wayland-client

#![warn(missing_docs, missing_debug_implementations)]
#![forbid(improper_ctypes, unsafe_op_in_unsafe_fn)]
#![cfg_attr(coverage, feature(no_coverage))]

use wayland_backend::{
    client::{InvalidId, ObjectId, WaylandError},
    protocol::{Interface, Message},
};

mod conn;
mod event_queue;
pub mod globals;

/// Backend reexports
pub mod backend {
    pub use wayland_backend::client::{
        Backend, Handle, InvalidId, NoWaylandLib, ObjectData, ObjectId, ReadEventsGuard,
        WaylandError,
    };
    pub use wayland_backend::protocol;
    pub use wayland_backend::smallvec;
}

pub use wayland_backend::protocol::WEnum;

pub use conn::{ConnectError, Connection, ConnectionHandle};
pub use event_queue::{
    DelegateDispatch, DelegateDispatchBase, Dispatch, EventQueue, QueueHandle, QueueProxyData,
};

/// Generated protocol definitions
///
/// This module is automatically generated from the `wayland.xml` protocol specification,
/// and contains the interface definitions for the core Wayland protocol.
#[allow(missing_docs)]
pub mod protocol {
    use self::__interfaces::*;
    use crate as wayland_client;
    pub mod __interfaces {
        wayland_scanner::generate_interfaces!("wayland.xml");
    }
    wayland_scanner::generate_client_code!("wayland.xml");
}

/// Trait representing a Wayland interface
pub trait Proxy: Sized {
    /// The event enum for this interface
    type Event;
    /// The request enum for this interface
    type Request;

    /// The interface description
    fn interface() -> &'static Interface;

    /// he ID of this object
    fn id(&self) -> ObjectId;

    /// The version of this object
    fn version(&self) -> u32;

    /// Access the user-data associated with this object
    fn data<U: Send + Sync + 'static>(&self) -> Option<&U>;

    /// Create an object proxy from its ID
    ///
    /// Returns an error this the provided object ID does not correspond to
    /// the `Self` interface.
    ///
    /// **Note:** This method is mostly meant as an implementation detail to be
    /// used by code generated by wayland-scanner.
    fn from_id(conn: &mut ConnectionHandle, id: ObjectId) -> Result<Self, InvalidId>;

    /// Parse a event for this object
    ///
    /// **Note:** This method is mostly meant as an implementation detail to be
    /// used by code generated by wayland-scanner.
    fn parse_event(
        conn: &mut ConnectionHandle,
        msg: Message<ObjectId>,
    ) -> Result<(Self, Self::Event), DispatchError>;

    /// Serialize a request for this object
    ///
    /// **Note:** This method is mostly meant as an implementation detail to be
    /// used by code generated by wayland-scanner.
    fn write_request(
        &self,
        conn: &mut ConnectionHandle,
        req: Self::Request,
    ) -> Result<Message<ObjectId>, InvalidId>;
}

/// Wayland dispatching error
#[derive(thiserror::Error, Debug)]
pub enum DispatchError {
    /// An invalid message was received
    #[error("Bad message for interface {interface} : {msg:?}")]
    BadMessage {
        /// The faulty message
        msg: Message<ObjectId>,
        /// The interface of the target object
        interface: &'static str,
    },
    /// The backend generated an error
    #[error("Backend error: {0}")]
    Backend(#[from] WaylandError),
}
