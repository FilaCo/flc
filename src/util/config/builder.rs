use crate::util::config::entry::{EntryType, SingletonData};
use crate::util::config::from_config::FromConfig;
use crate::util::config::Config;
use crate::FlcResult;
use std::any::{type_name, Any};
use std::collections::HashMap;
use std::sync::Arc;

pub struct Builder {
    config_map: HashMap<String, EntryType<Arc<T>>>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            config_map: HashMap::new(),
        }
    }

    pub fn add_singleton<T: ?Sized + FromConfig + 'static>(
        &mut self,
        tag: Option<&str>,
    ) -> &mut Self {
        let entry: Arc<EntryType<T>> = Arc::new(EntryType::Singleton(SingletonData::new()));

        self.config_map
            .insert(tag.unwrap_or(type_name::<T>()).to_string(), entry);

        self
    }

    // TODO: Validate config_map
    pub fn build(self) -> FlcResult<Config> {
        Ok(Config::new(self.config_map))
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}
