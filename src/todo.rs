extern crate serde_json;
extern crate iron;

use iron::typemap::Key;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    #[serde(default)]
    id: u32,
    content: String,
}

pub trait RecordId<T> {
    fn set_id(&mut self, id: T);
    fn id(self) -> T;
}

impl RecordId<u32> for Todo {
    fn set_id(&mut self, id: u32) {
        self.id = id
    }
    fn id(self) -> u32 {
        self.id
    }
}

pub struct Storage<V: RecordId<u32>> {
    data: HashMap<u32, V>,
    last_id: u32,
}

impl<V: RecordId<u32>> Storage<V> {
    pub fn new() -> Storage<V> {
        Storage {
            data: HashMap::new(),
            last_id: 0,
        }
    }
    pub fn get(&self, key: &u32) -> Option<&V> {
        self.data.get(key)
    }

    pub fn add(&mut self, mut v: V) {
        let k = self.next_id();
        v.set_id(k);
        self.data.insert(k, v);
    }
    pub fn remove(&mut self, k: &u32) {
        self.data.remove(k);
    }
    fn next_id(&mut self) -> u32 {
        self.last_id += 1;
        self.last_id
    }
}

impl<V: RecordId<u32> + 'static> Key for Storage<V> {
    type Value = Storage<V>;
}