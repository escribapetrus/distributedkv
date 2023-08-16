mod kv;

use kv::KeyValueStore;

fn main() {
    let mut kv_store = KeyValueStore::new("store");

    kv_store.put("song", "Dreaming");
    kv_store.put("song", "Heaven Tonight");
    kv_store.put("another song", "hold on");

    if let Some(value) = kv_store.get("song") {
        println!("Value for song: {}", value);
    } else {
        println!("Key not found");
    }
}
