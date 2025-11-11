#![allow(missing_docs)]

// Include macros first
#[macro_use]
mod macros;

pub mod compensation_management;
pub mod hire;

pub mod prelude {
    #[allow(ambiguous_glob_reexports)]
    pub use crate::compensation_management::*;
    #[allow(ambiguous_glob_reexports)]
    pub use crate::hire::*;
    pub use openlark_core::{client::LarkClient, SDKResult};
}

#[allow(ambiguous_glob_reexports)]
pub use crate::compensation_management::*;
#[allow(ambiguous_glob_reexports)]
pub use crate::hire::*;
