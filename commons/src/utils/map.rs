//! An hashmap that directly uses hashes instead of hashing keys.

use std::mem::MaybeUninit;

const BUCKET_EMPTY: u8 = 0x00;
const BUCKET_TOMBSTONE: u8 = 0x01;

const MAP_LOAD_FACTOR: f64 = 0.85;

#[derive(Debug)]
pub struct HashedMap<V> {
	meta: Vec<u8>,
	buckets: Vec<MaybeUninit<(u64, V)>>,
	capacity: usize,
	load: usize
}

impl<V> HashedMap<V> {
	pub fn new(capacity: usize) -> Self {
		let mut cap: usize = capacity;

		if !cap.is_power_of_two() {
			cap = cap.next_power_of_two();
		}

		let mut buckets: Vec<MaybeUninit<(u64, V)>> = Vec::with_capacity(cap);

		let mut meta: Vec<u8> = vec![BUCKET_EMPTY; cap];

		unsafe {
            buckets.set_len(cap);
        }

        return HashedMap { meta , buckets, capacity: cap, load: 0 }
	}

	pub fn put(&mut self, key: u64, val: V) {
		let index = self.index_from_hash(key);
		let fingerprint = self.fingerprint_from_hash(key);

		let mut insertion: Option<usize> = None;

        for i in index..self.capacity {
            let meta: u8 = self.meta[i];
            let bucket: &MaybeUninit<(u64, V)> = &self.buckets[i];

            match meta {
                BUCKET_EMPTY | BUCKET_TOMBSTONE => {
                    // Claim bucket as own

                    insertion = Some(i);
                    break;
                },

                _ => {
                    if meta != fingerprint {
                        continue;
                    }

                    unsafe {
                        if &key == &bucket.assume_init_ref().0 {
                            insertion = Some(i);
                            break;
                        }
                    }
                }
            }
        }        

        if insertion.is_some() {
            let target: usize = insertion.unwrap();

            let ref mut bucket = self.buckets[target];
            let prefilled_table = self.meta[target] == fingerprint;

            unsafe {
                bucket.write((key, val));
            }

            if !prefilled_table {
                self.meta[target] = fingerprint;
                self.load += 1;
            }

            return;
        }

        if self.check_for_map_load() {
            self.change_capacity((self.capacity + 1).next_power_of_two());

            // We didn't find a bucket yet. We increment the capacity. 
            
            // Since new capacity > old capacity. We can simply claim the old capacity + 1 bucket as ours
        
            self.put(key, val);
        }
	}

	pub fn get(&self, key: u64) -> Option<&V> {
		let index = self.index_from_hash(key);
        let fingerprint = self.fingerprint_from_hash(key);

        for i in index..self.capacity {
            if self.meta[i] == fingerprint {
                unsafe {
                    let bucket: &(u64, V) = self.buckets[i].assume_init_ref();
                    
                    if bucket.0 == key {
                        return Some(&bucket.1);
                    }
                }
            }
        }

        return None;
	}

	pub fn entries(&self) -> Vec<&(u64, V)> {
		let mut vec = Vec::new();

		for i in 0..self.capacity {
			if self.meta[i] == BUCKET_EMPTY || self.meta[i] == BUCKET_TOMBSTONE {
				continue;
			}

			unsafe { vec.push(self.buckets[i].assume_init_ref()); }
		}
		
		return vec;
	}


	pub fn erase(&mut self, key: u64) {
        let index = self.index_from_hash(key);
        let fingerprint = self.fingerprint_from_hash(key);

        for i in index..self.capacity {
            if self.meta[i] == fingerprint {
                unsafe {
                    let bucket: &(u64, V) = self.buckets[i].assume_init_ref();

                    if bucket.0 == key {
                        unsafe {
                            self.buckets[i] = MaybeUninit::uninit();
                        }

                        self.meta[i] = BUCKET_TOMBSTONE;
                    }
                }
            }
        }
    }

	fn check_for_map_load(&self) -> bool {
        let curr_load = (self.load / self.capacity) as f64;

        return curr_load >= MAP_LOAD_FACTOR;
    }

    fn change_capacity(&mut self, new_capacity: usize) {
        if self.capacity == new_capacity {
            return;
        }

        let mut temp: HashedMap<V> = HashedMap::new(new_capacity);
        
        for i in 0..self.capacity {
            if self.meta[i] != BUCKET_EMPTY && self.meta[i] != BUCKET_TOMBSTONE {
                unsafe {
                    let bucket: (u64, V) = self.buckets[i].assume_init_read();

                    let key: u64 = bucket.0.clone();
                    let val: V = bucket.1;

                    temp.put(key, val);
                }
            }
            
            // Append bucket to temp
        }

        self.buckets = temp.buckets;
        self.meta = temp.meta;
        self.capacity = temp.capacity;
    }


}

impl<V> HashedMap<V> {
    fn index_from_hash(&self, hash: u64) -> usize {
        (hash & (self.capacity as u64 - 1)) as usize // Get the most significant self.capacity - 1 bits 
    }

    fn fingerprint_from_hash(&self, hash: u64) -> u8 {
        (hash >> 57) as u8 // Get the 7 least significant bits
    }
}