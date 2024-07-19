use std::hash::{DefaultHasher, Hash, Hasher};

const BLOOM_FILTER_SIZE: u64 = 16;

struct BloomFilter {
    filter: u16,         // A 16-bit filter
    seeds: Vec<u16>  // Seeds to be used in the different hashing functions
}

impl BloomFilter {
    pub fn new() -> Self {
       BloomFilter { filter: 0, seeds: vec![142, 112, 654] } 
    }

    fn hash<T: Hash>(&self, item: &T, seed: u16) -> u64 {
        let mut hasher = DefaultHasher::new();
        hasher.write_u16(seed);
        item.hash(&mut hasher);
        hasher.finish() % BLOOM_FILTER_SIZE
    }

    pub fn add<T: Hash>(&mut self, item: &T) {
        let mut indices : Vec<u64> = Vec::new();

        for &seed in &self.seeds {
            let index = self.hash(item, seed);
            indices.push(index);

            self.filter = self.filter | (1 << index);
        }

        print!("Indices: ");
        for index in indices {
            print!("{} ", index);
        }
        println!();
    }

    pub fn contains<T: Hash>(&self, item: &T) -> bool {
        let present = true;
        for &seed in &self.seeds {
            let index = self.hash(item, seed);
            let result = self.filter & (1 << index);
            if result == 0 {
                return false;
            }
        }
        present
    }
}

fn main() {
    let mut bloom_filter = BloomFilter::new();

    bloom_filter.add(&"ritwik");
    bloom_filter.add(&"rust");
    bloom_filter.add(&"hashedtokens");

    let exists = bloom_filter.contains(&"ritwik");
    if exists {
        println!("Value may or may not exist");
    } else {
        println!("Value doesn't exist");
    }

    let exists = bloom_filter.contains(&"wowziee");
    if exists {
        println!("Value may or may not exist");
    } else {
        println!("Value doesn't exist");
    }
}
