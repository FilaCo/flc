use crate::util::config::entry::{Entry, SingletonData};
use crate::util::config::from_config::FromConfig;
use crate::util::config::Config;
use crate::FlcResult;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub enum EntryType<T: ?Sized + FromConfig + 'static> {
    Singleton(SingletonData<T>),
}

impl<T: ?Sized + FromConfig + 'static> Entry<T> for EntryType<T> {
    fn get(&mut self, container: &Config) -> Arc<T> {
        match self {
            EntryType::Singleton(data) => data.get(container),
        }
    }
}
