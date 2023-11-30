pub struct Cache {
    pub items: Vec<Item>,
}

#[derive(Clone, Debug)]
pub struct Item {
    pub command_name: String,
    pub data: String,
}

impl Item {
    pub fn new(command_name: &str, data: &str) -> Item {
        return Item { command_name: command_name.to_string(), data: data.to_string()};
    }

    pub fn unpack(&self) {
        todo!();
    }
    pub async fn async_unpack(&self) {
        todo!();
    }
}

impl Cache {
    pub fn new() -> Cache {
        let mut items: Vec<Item> = Vec::new();
        return Cache { items };
    }
    pub fn new_with_capacity(capacity: usize) -> Cache {
        let mut items: Vec<Item> = Vec::with_capacity(capacity);
        return Cache { items };
    }
    pub fn push_back(&mut self, item: Item) {
        self.items.push(item);
    }
    pub fn get_first(&mut self) -> Item {
        let item: Item = self.items[0].clone();
        self.items.pop();
        return item;
    }
    pub fn get_elems(&self) -> usize {
        return self.items.len();
    }
    pub fn clear(&mut self) {
        self.items.clear()
    }
}