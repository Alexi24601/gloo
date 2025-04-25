//! WebSocket Events

/// Data emitted by `onclose` event
#[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Eq, Ord)]
pub struct CloseEvent {
    /// Close code
    pub code: u16,
    /// Close reason
    pub reason: String,
    /// If the websockets was closed cleanly
    pub was_clean: bool,
}
