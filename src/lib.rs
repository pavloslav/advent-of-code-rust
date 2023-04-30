#[macro_use]
pub mod common;

pub mod prelude {
    pub use crate::common::Error;
    pub use crate::common::Error::TaskError;
    pub use crate::common::Result;

    pub use crate::common::floyd_hare_tortoise::*;
    pub use crate::common::Md5Hasher;
}

pub use prelude::*;
