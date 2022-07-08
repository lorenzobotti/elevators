use std::{collections::HashMap, hash::Hash};

#[derive(Default, Debug)]
pub struct Id<T> {
    id: usize,
    items: HashMap<T, usize>,
}

impl<T> Id<T>
where
    T: Eq + Hash,
{
    pub fn get(&mut self, item: T) -> (usize, bool) {
        match self.items.get(&item) {
            Some(id) => (*id, true),
            None => {
                let id = self.consume_id();
                self.items.insert(item, id);
                (id, false)
            }
        }
    }

    fn consume_id(&mut self) -> usize {
        let current = self.id;
        self.id += 1;
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let keys_to_try = [
            ("ciao", 0),
            ("giovanni", 1),
            ("ciao", 0),
            ("giovanni", 1),
            ("giovanni", 1),
            ("ciao", 0),
            ("ciao", 0),
            ("lalo", 2),
            ("ciao", 0),
            ("ciao", 0),
        ];

        let mut gen = Id::default();

        for (name, id) in keys_to_try {
            assert_eq!((gen.get(name).0), id);
        }
    }
}
