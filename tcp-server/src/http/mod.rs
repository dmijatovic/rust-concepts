// local use
mod request;
mod method;
mod error;
mod query_string;
mod response;

// expose submodule props directly
// in the parent module
pub use method::Method;
pub use request::Request;
pub use response::Response;
pub use response::status::StatusCode;
pub use error::ParseError;