use std::io;
use std::fmt::Debug;

use snafu::Snafu;
use wasmer_runtime_core::cache::{Error as CacheError};

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum Error {
    #[snafu(display("Cache error: {}", msg))]
    CacheErr {
        msg: String,
        #[cfg(feature = "backtraces")]
        backtrace: snafu::Backtrace,
    },
    #[snafu(display("Filesystem error: {}", source))]
    IoErr {
        source: io::Error,
        #[cfg(feature = "backtraces")]
        backtrace: snafu::Backtrace,
    },
    #[snafu(display("Hash doesn't match stored data"))]
    IntegrityErr {
        #[cfg(feature = "backtraces")]
        backtrace: snafu::Backtrace,
    },
    #[snafu(display("Wasmer error: {}", msg))]
    WasmerErr {
        msg: String,
        #[cfg(feature = "backtraces")]
        backtrace: snafu::Backtrace,
    },
}

pub trait CacheExt<T: Debug> {
    fn convert_cache(self) -> Result<T, Error>;
}

impl<T: Debug> CacheExt<T> for Result<T, CacheError> {
    fn convert_cache(self) -> Result<T, Error> {
        self.map_err(|err| {
            let msg = format!("{:?}", err);
            // construct like this (not just Err(Error::CacheErr)) to allow backtraces
            let res: Result<T, Error> = CacheErr { msg }.fail();
            res.unwrap_err()
        })
    }
}
