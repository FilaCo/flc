use crate::util::config::from_config::FromConfig;
use crate::util::config::Config;
use crate::FlcResult;
use std::sync::Arc;

pub struct SingletonData<T: ?Sized + FromConfig + 'static> {
    value: Option<Arc<T>>,
}

impl<T: ?Sized + FromConfig + 'static> SingletonData<T> {
    pub fn new() -> Self {
        Self { value: None }
    }

    pub fn get(&mut self, container: &Config) -> Arc<T> {
        if self.value.is_none() {
            self.value = Some(T::from_config(container));
        }

        self.value
            .as_ref()
            .expect("Something went wrong with singleton entry")
            .clone()
    }
}
