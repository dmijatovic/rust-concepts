use std::error::Error;
// imports for implementing Debug and Display trait
// required by std::error
use std::fmt::{Debug, Display, Formatter,Result as FmtResult};
// enables use of ? in TryFrom trait
use std::str::Utf8Error;

// Custom request parse error
pub enum ParseError{
  InvalidRequest,
  InvalidEncoding,
  InvalidProtocol,
  InvalidMethod
}

// implemenation of Display
// for out custom error (as required by std:Error)
impl Display for ParseError{
  fn fmt(&self, f: &mut Formatter) -> FmtResult{
    write!(f,"{}", self.message())
  }
}

// implementa message method on ParseError
impl ParseError{
  fn message(&self)-> &str{
    match self{
      Self::InvalidRequest => {"Invalid request"},
      Self::InvalidEncoding => {"Invalid encoding"},
      Self::InvalidProtocol => {"Invalid protocol"},
      Self::InvalidMethod => {"Invalid method"},
      _ => {"Unknown Parse Error"},
    }
  }
}

// implemenation of Debug trait
// for out custom error (as required by std:Error)
impl Debug for ParseError{
  fn fmt(&self, f: &mut Formatter) -> FmtResult{
    write!(f,"{}", self.message())
  }
}

// implements conversion from Utf8Error
// into our custom ParseError::InvalidEnciding
impl From<Utf8Error> for ParseError{
  fn from(_: Utf8Error) -> Self{
    Self::InvalidEncoding
  }
}

// standard error trait requires
// implementation of Display + Debug
impl Error for ParseError{}
