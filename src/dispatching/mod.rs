pub(crate) mod core;
mod dispatcher;
mod dispatcher_context;
pub mod error_handlers;
mod handlers;
#[cfg(test)]
mod tests;
mod update_listeners;
pub mod update_with_cx;
//pub mod dialogue;
//pub(crate) mod repls;

pub use dispatcher::{Dispatcher, DispatcherBuilder};
pub use handlers::updates;
pub use update_with_cx::UpdateWithCx;

pub mod tel {
    pub use super::handlers::commands::Command;
}
