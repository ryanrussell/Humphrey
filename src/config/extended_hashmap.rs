use std::collections::HashMap;
use std::str::FromStr;

pub trait ExtendedMap<K, V> {
    /// Gets an item from the map by value.
    fn get_owned(&self, key: K) -> Option<V>;

    /// Gets an item from the map or returns a given default.
    fn get_optional(&self, key: K, default: V) -> V;

    /// Gets an item from the map or returns the given error.
    fn get_compulsory(&self, key: K, error: &'static str) -> Result<V, &'static str>;

    /// Gets an item from the map or a given default and parses it, or returns the given error.
    fn get_optional_parsed<T>(
        &self,
        key: K,
        default: T,
        error: &'static str,
    ) -> Result<T, &'static str>
    where
        T: FromStr;

    /// Gets an item from the map and parses it, or returns the given error.
    fn get_compulsory_parsed<T>(&self, key: K, error: &'static str) -> Result<T, &'static str>
    where
        T: FromStr;
}

impl ExtendedMap<&'static str, String> for HashMap<String, String> {
    fn get_owned(&self, key: &str) -> Option<String> {
        self.get(key).map(|s| s.to_string())
    }

    fn get_optional(&self, key: &str, default: String) -> String {
        self.get(key).unwrap_or(&default).to_string()
    }

    fn get_compulsory(&self, key: &str, error: &'static str) -> Result<String, &'static str> {
        self.get(key).map_or(Err(error), |s| Ok(s.clone()))
    }

    fn get_optional_parsed<T>(
        &self,
        key: &str,
        default: T,
        error: &'static str,
    ) -> Result<T, &'static str>
    where
        T: FromStr,
    {
        self.get(key)
            .map_or(Ok(default), |s| s.parse::<T>().map_err(|_| error))
    }

    fn get_compulsory_parsed<T>(&self, key: &str, error: &'static str) -> Result<T, &'static str>
    where
        T: FromStr,
    {
        self.get(key)
            .map_or(Err(error), |s| s.parse::<T>().map_err(|_| error))
    }
}