pub struct Horse {
    id: u64,
    name: String,
    breed: String,
}

impl Horse {
    pub fn new(name: &str, breed: &str, id: u64) -> Self {
        Horse {
            name: name.to_string(),
            breed: breed.to_string(),
            id,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn breed(&self) -> &str {
        &self.breed
    }
}
