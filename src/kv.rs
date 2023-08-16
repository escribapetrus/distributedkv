use std::collections::HashMap;

#[derive(Debug)]
pub struct KeyValueStore<'a> {
    id: &'a str,
    data: HashMap<String, String>
}


impl<'a> KeyValueStore<'a>{
    pub fn new(id: &'a str) -> Self {
	KeyValueStore{
	    id,
	    data: HashMap::new(),
	}
    }

    pub fn get(&self, key: &str) -> Option<&String> {
	 self.data.get(key)
    }

    pub fn put(&mut self, key: &str, value: &str) {
	self.data.insert(key.to_string(), value.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
	let mut kv_store = KeyValueStore::new("test_store");

	kv_store.put("key1", "value1");
	kv_store.put("key2", "value2");
	kv_store.put("key1", "value3");

	assert_eq!(kv_store.get("key1"), Some(&"value3".to_string()));
	assert_eq!(kv_store.get("key2"), Some(&"value2".to_string()));
	assert_eq!(kv_store.get("key3"), None);
    }
}
