use crate::FlcResult;
use std::any::{type_name, Any};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

trait FromConfig {
    fn from_config(container: &Config) -> FlcResult<Arc<Self>>;
}

trait Entry<T: ?Sized + FromConfig> {
    fn get(&mut self, container: &Config) -> FlcResult<Arc<T>>;
}

enum EntryType<T: ?Sized + FromConfig> {
    Singleton(SingletonData<T>),
    Transient(TransientData<T>),
}

impl<T: ?Sized + FromConfig> Entry<T> for EntryType<T> {
    fn get(&mut self, container: &Config) -> FlcResult<Arc<T>> {
        match self {
            EntryType::Singleton(data) => data.get(container),
            EntryType::Transient(data) => data.get(container),
        }
    }
}

struct SingletonData<T: ?Sized + FromConfig> {
    value: Option<FlcResult<Arc<T>>>,
}

impl<T: ?Sized + FromConfig> SingletonData<T> {
    pub fn new() -> Self {
        Self { value: None }
    }

    pub fn get(&mut self, container: &Config) -> FlcResult<Arc<T>> {
        if self.value.is_none() {
            self.value = Some(T::from_config(container));
        }

        self.value
            .expect("Something went wrong with singleton entry")
    }
}

struct TransientData<T: ?Sized + FromConfig> {
    _p: PhantomData<T>,
}

impl<T: ?Sized + FromConfig> TransientData<T> {
    pub fn new() -> Self {
        Self {
            _p: PhantomData::default(),
        }
    }

    pub fn get(&mut self, container: &Config) -> FlcResult<Arc<T>> {
        T::from_config(container)
    }
}

pub struct Config {
    config_map: HashMap<String, Arc<dyn Any>>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            config_map: HashMap::new(),
        }
    }

    pub fn get<T: ?Sized + FromConfig>(&self, tag: Option<&str>) -> Option<FlcResult<Arc<T>>> {
        self.config_map
            .get(tag.unwrap_or(type_name::<T>()))
            .and_then(|arc| arc.downcast_mut::<EntryType<T>>())
            .map(|entry| entry.get(self))
    }

    pub fn add_singleton<T: ?Sized + FromConfig>(&mut self, tag: Option<&str>) -> &mut Self {
        let entry: Arc<EntryType<T>> = Arc::new(EntryType::Singleton(SingletonData::new()));

        self.config_map
            .insert(tag.unwrap_or(type_name::<T>()).to_string(), entry);

        self
    }

    pub fn add_transient<T: ?Sized + FromConfig>(&mut self, tag: Option<&str>) -> &mut Self {
        let entry: Arc<EntryType<T>> = Arc::new(EntryType::Transient(TransientData::new()));

        self.config_map
            .insert(tag.unwrap_or(type_name::<T>()).to_string(), entry);

        self
    }
}
