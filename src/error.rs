// espera::error
//
//! Error types.
//

use core::result;

#[cfg(feature = "std")]
use sixbit::EncodeError;

/// `espera` result type.
pub type EsperaResult<N> = result::Result<N, EsperaError>;

/// `espera` error type.
#[non_exhaustive]
#[derive(Debug)]
pub enum EsperaError {
    /// An error involving the encoding of a rate's name.
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    RateName(EncodeError),
}

mod core_impls {
    use super::EsperaError;
    use core::fmt;

    #[cfg(feature = "std")]
    use super::EncodeError;

    impl fmt::Display for EsperaError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                #[cfg(feature = "std")]
                EsperaError::RateName(r) => fmt::Debug::fmt(r, f),

                #[allow(unreachable_patterns)] // TEMP
                _ => write!(f, ""),
            }
        }
    }

    #[cfg(feature = "std")]
    impl From<EncodeError> for EsperaError {
        fn from(err: EncodeError) -> Self {
            EsperaError::RateName(err)
        }
    }
}

#[cfg(feature = "std")]
mod std_impls {
    use super::EsperaError;
    use std::error::Error;

    impl Error for EsperaError {}
}
