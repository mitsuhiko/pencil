// This module implements a number of types.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::error;


pub use self::PencilResult::{
    PenValue, PenError
};
pub use self::PencilError::{
    PencilHTTPError
};


/// The HTTP Error type.
#[deriving(Clone)]
pub struct HTTPError {
    pub code: int,
    pub desc: &'static str,
}

impl error::Error for HTTPError {

    fn description(&self) -> &str {
        self.desc
    }

    fn detail(&self) -> Option<String> {
        None
    }
}


/// The Pencil Error type.
#[deriving(Clone)]
pub enum PencilError {
    PencilHTTPError(HTTPError),
}

impl error::FromError<HTTPError> for PencilError {

    fn from_error(err: HTTPError) -> PencilError {
        PencilHTTPError(err)
    }
}

impl error::Error for PencilError {

    fn description(&self) -> &str {
        match *self {
            PencilHTTPError(err) => err.desc,
        }
    }

    fn detail(&self) -> Option<String> {
        match *self {
            PencilHTTPError(err) => err.detail(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &PencilHTTPError(ref err) => Some(&*err as &error::Error),
        }
    }
}


/// Result type.
#[deriving(Clone)]
pub enum PencilResult {
    PenValue(String),
    PenError(PencilError),
}
