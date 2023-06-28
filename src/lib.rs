mod feature;
mod util;

pub use crate::util::Config;
pub use crate::util::Error;
pub use crate::util::FromConfig;

pub type FlcResult<T> = Result<T, Error>;
