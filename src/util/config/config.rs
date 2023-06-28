use crate::util::config::builder::Builder;
use crate::util::config::entry::{Entry, EntryType};
use crate::util::config::from_config::FromConfig;
use std::any::{type_name, Any};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct Config {
    map: HashMap<String, Arc<dyn Any>>,
}

impl Config {
    pub fn new(map: HashMap<String, Arc<dyn Any>>) -> Self {
        Self { map }
    }

    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn get<T: ?Sized + FromConfig + 'static>(&self, tag: Option<&str>) -> Option<Arc<T>> {
        self.map
            .get(tag.unwrap_or(type_name::<T>()))
            .and_then(|arc| arc.downcast_mut::<EntryType<T>>())
            .map(|entry| entry.get(self))
    }
}

impl<T: ?Sized + FromConfig + 'static> Deref<T> for Arc<dyn Any> {
    type Target = EntryType<T>;

    fn deref(&mut self) -> &mut Self::Target {
        todo!()
    }
}
