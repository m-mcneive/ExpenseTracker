pub struct Entry {
    pub name: String,
    pub expenseType: String,
    pub category: String,
    pub price: f32,
}

pub struct Items{
    pub items: Vec<Entry>,
}

impl Entry {
    pub fn new(name: String, expenseType: String, category: String, price: f32) -> Entry {
        Entry{ name, expenseType, category, price }
    }
}

impl Items {
    pub fn new() -> Vec<Entry> {
        Vec::new()
    }
}