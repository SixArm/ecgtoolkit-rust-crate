// ECG config
//
// Configuration class for the ECG Conversion Toolkit.
// Original author: Maarten JB van Ettinger.

use std::collections::BTreeMap;

/// Configuration for an ECG format.
#[derive(Clone, Debug)]
pub struct EcgConfig {
    /// Number of mandatory config values (from the start of possible_configs).
    must_value: usize,
    /// All possible configuration keys.
    possible_configs: Vec<String>,
    /// Current configuration values.
    configs: BTreeMap<String, String>,
    /// Optional check function.
    check_config: Option<fn() -> bool>,
}

impl EcgConfig {
    /// Create a new config with mandatory and optional config keys.
    pub fn new(
        must_conf: &[&str],
        pos_conf: &[&str],
        check: Option<fn() -> bool>,
    ) -> Self {
        let mut possible_configs = Vec::with_capacity(must_conf.len() + pos_conf.len());
        for s in must_conf {
            possible_configs.push(s.to_string());
        }
        let must_value = possible_configs.len();
        for s in pos_conf {
            possible_configs.push(s.to_string());
        }

        Self {
            must_value,
            possible_configs,
            configs: BTreeMap::new(),
            check_config: check,
        }
    }

    /// Create a config where all values are optional or all mandatory.
    pub fn new_all(configs: &[&str], all_must: bool, check: Option<fn() -> bool>) -> Self {
        let possible_configs: Vec<String> = configs.iter().map(|s| s.to_string()).collect();
        let must_value = if all_must { possible_configs.len() } else { 0 };

        Self {
            must_value,
            possible_configs,
            configs: BTreeMap::new(),
            check_config: check,
        }
    }

    /// Get config item name by index.
    pub fn get_by_index(&self, index: usize) -> Option<&str> {
        self.possible_configs.get(index).map(|s| s.as_str())
    }

    /// Get config value by key.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.configs.get(key).map(|s| s.as_str())
    }

    /// Set config value by key.
    pub fn set(&mut self, key: &str, value: Option<&str>) {
        if !self.is_part_of_config(key) {
            return;
        }
        match value {
            Some(v) if !v.is_empty() => {
                self.configs.insert(key.to_string(), v.to_string());
            }
            _ => {
                self.configs.remove(key);
            }
        }
    }

    /// Number of config items.
    pub fn nr_config_items(&self) -> usize {
        self.possible_configs.len()
    }

    /// Get config item info.
    pub fn get_config_item(&self, n: usize) -> Option<(&str, bool)> {
        self.possible_configs.get(n).map(|name| {
            (name.as_str(), n < self.must_value)
        })
    }

    /// Check if a key is part of the config.
    pub fn is_part_of_config(&self, key: &str) -> bool {
        self.possible_configs.iter().any(|s| s == key)
    }

    /// Check if configuration works (all mandatory values set).
    pub fn configuration_works(&self) -> bool {
        for i in 0..self.must_value {
            if !self.configs.contains_key(&self.possible_configs[i]) {
                return false;
            }
        }
        self.check_config.map_or(true, |f| f())
    }

    /// Copy config from another config of the same kind.
    pub fn set_from(&mut self, conf: &EcgConfig) -> bool {
        if conf.possible_configs.len() != self.possible_configs.len()
            || conf.must_value != self.must_value
        {
            return false;
        }

        for i in 0..self.possible_configs.len() {
            if conf.possible_configs[i] != self.possible_configs[i] {
                return false;
            }
        }

        self.configs = conf.configs.clone();
        true
    }
}
