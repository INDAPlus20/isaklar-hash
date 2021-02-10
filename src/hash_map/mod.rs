use std::{ptr::{null, NonNull}};

#[cfg(test)]
mod tests;

const MAX_LOAD: f32 = 0.7;
#[derive(Clone)]
struct Entry {
    key: String,
    value: String,
    chain: Option<Box<Entry>>,
}

impl Entry {
    /// Chains the entry onto a linked list, returns `Ok(0)` if no duplicate was found and `Ok(1)` otherwise
    pub fn chain(&mut self, entry: Entry) -> Result<i32, String> {
        if self.key != entry.key {
            if let Some(pointer) = &mut self.chain {
                return pointer.chain(entry);
            } else {
                self.chain = Some(Box::new(entry));
                return Ok(0);
            }
        } else {
            return Ok(1);
        }
    }
    /// Finds and returns a reference to the corresponding entry down the chain
    pub fn get_entry_by_key(&self, key: &str) -> Option<&Box<Entry>> {
        if let Some(pointer) = &self.chain {
            if pointer.key == key {
                return Some(pointer);
            } else {
                return pointer.get_entry_by_key(key);
            }
        } else {
            return None;
        }
    }
    /// Finds and removes an entry, chained to `self`. 
    pub fn remove_by_key(&mut self, key: &str) -> Result<usize, String>{
        if self.key == key {
            return Err("Can't remove root entry".to_string())
        } else {

        
        if let Some(first) = &mut self.chain {
            if first.key == key {
                self.chain = first.chain.clone();
                return Ok(0);
            } else {
                return first.remove_by_key(key);
            }
        } else {
            return Err("Entry not found".to_string());
        }
    }
    }
    /*
    pub fn get_remove_last(&mut self) -> Option<Box<Entry>> {
        if let Some(second) = self.chain.clone() {
            if let Some(third) = &mut pointer.chain {
                return pointer.get_remove_last();
            } else {
                self.chain = None;
                return Some();
            }
        } else {
            // This is the only entry
            return None
        }
    }
    */
    /// Returns the chained elements of this Entry. If self doesn't have any chained elements, returns None
    pub fn get_all(&mut self) -> Vec<Entry> {
        let mut entries = vec![self.clone()];
        if let Some(next) = self.chain.clone() {
            let mut next = next;
            loop {
                entries.push(*next.clone());
                if let Some(last) = next.chain {
                    next = last;
                } else {
                    break;
                }
            }
            return entries;
        } else {
            return entries;
        }
    }
}

struct HashMap {
    values: Vec<Option<Entry>>,
    load: usize,
}

impl HashMap {
    pub fn new() -> HashMap {
        HashMap {
            values: vec![None; 10],
            load: 0,
        }
    }

    pub fn len(&self) -> usize{
        return self.values.len()
    }
    /// Makes a new entry and inserts it, if the key is not duplicate
    pub fn insert(&mut self, key: &str, value: &str) {
        //println!("load: {:?}", self.load as f32 / self.values.len() as f32);
        if self.load as f32 / self.values.len() as f32  >= MAX_LOAD {
            
            self.resize();
        }
        let hash = hash(key, self.values.len());
        if let Some(entry) = &mut self.values[hash] {
            // Collision
            // Chain the entry
            let new_entry = Entry {
                key: key.to_string(),
                value: value.to_string(),
                chain: None,
            };
            entry.chain(new_entry);
            //println!("Collision!");
        } else {
            // New entry
            self.values[hash] = Some(Entry {
                key: key.to_string(),
                value: value.to_string(),
                chain: None,
            });
        }
        self.load += 1;
    }
    /// Get the value connected to key
    pub fn get(&self, key: &str) -> Option<&str> {
        let hash = hash(key, self.values.len());
        if let Some(entry) = &self.values[hash] {
            if entry.key == key {
                return Some(&entry.value)
            } else if let Some(entry) = entry.get_entry_by_key(key) {
                return Some(&entry.value)
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    /// Removes the entry 
    pub fn remove(&mut self, key: &str) {
        let hash = hash(key, self.values.len());
        if let Some(entry) = &mut self.values[hash] {
            if entry.key == key {
                if let Some(chain) = entry.chain.clone() {
                    self.values[hash] = Some(*chain);
                } else {
                    self.values[hash] = None;
                }
                self.load -= 1

            } else {
                match entry.remove_by_key(key){
                    Ok(_) => self.load -= 1,
                    Err(e) => println!("{:?}", e)
                };
            }

        }
    }

    /// Double the capacity and rehash every entry
    fn resize(&mut self) {

        // allocate new list
        let len = self.values.len();
        let mut new_values: Vec<Option<Entry>> = vec![None; len * 2];
        // rehash every entry
        for e in &mut self.values {
            if let Some(entry) = e {
                // get chained entries and rehash
                let entries = entry.get_all();
                for mut entry in entries {
                    let hash = hash(&entry.key, len * 2);
                    entry.chain = None;
                    if let Some(existing) = &mut new_values[hash] {
                        existing.chain(entry);
                    } else {
                        new_values[hash] = Some(entry);
                    }
                }
            }
        }

        // insert new list
        self.values = new_values;
    }
}
/// TODO: better algorithm
fn hash(key: &str, range: usize) -> usize {
    let key = key.as_bytes();
    let mut sum: usize = 0;
    for c in key {
        sum = (sum * 31 + key[(*c as usize % key.len()) ] as usize) % range ;
    }

    let output = sum % range;
    return output;
}
