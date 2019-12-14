pub struct KvStore {}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {}
    }
    pub fn set(&mut self, _: String, _: String) {
        unimplemented!();
    }
    pub fn get(&self, _: String) -> Option<String> {
        unimplemented!();
    }
    pub fn remove(&mut self, _: String) {
        unimplemented!();
    }
}
