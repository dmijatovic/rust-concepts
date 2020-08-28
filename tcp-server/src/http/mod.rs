// local use
mod request;
mod method;
mod error;

// expose submodule props directly
// in the parent module
pub use method::Method;
pub use request::Request;