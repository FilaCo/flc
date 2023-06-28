use crate::util::config::Config;
use std::sync::Arc;

pub trait FromConfig {
    fn from_config(container: &Config) -> Arc<Self>;
}
