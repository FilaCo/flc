mod feature;
mod util;

use crate::util::Error;

pub type FlcResult<T> = Result<T, Error>;
