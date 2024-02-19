/*            March Hale DB
   Key-Value database for some temporary data

    Author: Daniil <nullxjanus> Ermolaev.
*/

use std::collections::HashMap;

pub enum MHValues {
    Str(&'static str),
    Int8(i8),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt32(u32),
    UInt64(u64),
}

pub struct Item {
    pub name: String,
    pub value: MHValues,
}


pub struct MarchHaleDb {
    pub hm: HashMap<String, MHValues>
}

impl MarchHaleDb {
    pub fn init() -> Self {
        let mut hm = HashMap::new();
        Self { hm }
    }

    pub fn insert(&mut self, item: Item) {
        self.hm.insert(item.name, item.value);
    }

    pub fn remove(&mut self, key: String) {
        self.hm.remove(&key);
    }

    pub fn get(&self, key: String) -> Option<&MHValues> {
        return match self.hm.get(&key) {
            Some(item) => Some(item),
            None => None,
        }       
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        
    }
}
