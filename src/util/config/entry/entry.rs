use crate::util::config::from_config::FromConfig;
use crate::util::config::Config;
use std::sync::Arc;

pub trait Entry<T: ?Sized + FromConfig> {
    fn get(&mut self, container: &Config) -> Arc<T>;
}
