use std::collections::HashMap;

pub struct Registry<T> {
    ids_by_name: HashMap<String, usize>,
    names_by_id: Vec<String>,
    items_by_id: Vec<T>,
}

impl<T> Registry<T> {
    /// Create an empty Registry.
    pub fn new() -> Self {
        Self {
            ids_by_name: HashMap::new(),
            names_by_id: Vec::new(),
            items_by_id: Vec::new(),
        }
    }

    /// Register a new item in the Registry with a given name.
    pub fn register<S>(&mut self, name: S, item: T) -> usize
    where
        S: Into<String>,
    {
        let name = name.into();
        if self.ids_by_name.contains_key(&name) {
            panic!(format!("Key {} already exists in registry!", name));
        } else {
            let id = self.names_by_id.len();
            self.ids_by_name.insert(name.clone(), id);
            self.names_by_id.push(name);
            self.items_by_id.push(item);
            id
        }
    }

    /// Get an item by its id. Panics if the item doesn't exist.
    /// TODO: proper error handling
    pub fn get_item(&self, id: usize) -> &T {
        &self.items_by_id[id]
    }

    /// Get an item's id given its name.
    pub fn get_item_id<S>(&self, name: S) -> Option<usize>
    where
        S: Into<String>,
    {
        let name = name.into();
        self.ids_by_name.get(&name).map(|id| *id)
    }
}
