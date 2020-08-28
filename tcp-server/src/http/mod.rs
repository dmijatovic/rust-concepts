// local use
mod request;
mod method;
mod error;
mod query_string;

// expose submodule props directly
// in the parent module
pub use method::Method;
pub use request::Request;