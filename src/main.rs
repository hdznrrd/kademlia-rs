extern crate rustc_serialize;
extern crate rand;

use std::fmt::{Error, Debug, Formatter};

// Length of an ID, in bytes
const ID_SIZE: usize = 1;
// Number of buckets (length of ID in bits)
const N_BUCKETS: usize = ID_SIZE * 8;
// Number of contacts in each bucket
const BUCKET_SIZE: usize = 20;

#[derive(Ord,PartialOrd,Eq,PartialEq,Copy,Clone)]
struct Key([u8; ID_SIZE]);

impl Debug for Key {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for x in self.0.iter().rev() {
            try!(write!(f, "{0:08b}", x));
        }
        Ok(())
    }
}

#[derive(Ord,PartialOrd,Eq,PartialEq,Copy,Clone)]
struct Distance([u8; ID_SIZE]);

impl Debug for Distance {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for x in self.0.iter() {
            try!(write!(f, "{0:08b}", x));
        }
        Ok(())
    }
}

impl Distance {
    fn zeroes_in_prefix(&self) -> usize {
        for i in 0..ID_SIZE {
            for j in 8us..0 {
                if (self.0[i] >> (7 - j)) & 0x1 != 0 {
                    return i * 8 + j;
                }
            }
        }
        ID_SIZE * 8 - 1
    }
}

#[derive(Debug,Ord,PartialOrd,Eq,PartialEq,Copy,Clone)]
struct Contact {
    id: Key,
}

struct RoutingTable {
    origin: Key,
    buckets: Vec<Vec<Contact>>
}

impl RoutingTable {
    fn new(origin: Key) -> RoutingTable{
        let mut buckets = Vec::new();
        for _ in 0..N_BUCKETS {
            buckets.push(Vec::new());
        }
        RoutingTable { origin: origin, buckets: buckets }
    }

    fn update(&mut self, contact: Contact) {
        let bucket_index = dist(self.origin, contact.id).zeroes_in_prefix();
        let bucket = &mut self.buckets[bucket_index];
        let index = bucket.iter().position(|x| *x == contact);
        match index {
            Some(i) => {
                let swap = bucket[i];
                bucket[i] = bucket[0];
                bucket[0] = swap;
            },
            None => {
                if bucket.len() < BUCKET_SIZE {
                    bucket.push(contact);
                }
            },
        }
    }

    fn find_closest_nodes(&self, item: Key, count: usize) -> Vec<(Contact, Distance)> {
        if count == 0 {
            return Vec::new();
        }
        let bucket_index = dist(self.origin, item).zeroes_in_prefix();
        let mut ret = Vec::with_capacity(count);
        for i in bucket_index..N_BUCKETS {
            for c in &self.buckets[i] {
                ret.push( (*c, dist(c.id, item)) );
                if ret.len() == count {
                    ret.sort_by(|&(_,a), &(_,b)| a.cmp(&b));
                    return ret;
                }
            }
        }
        if bucket_index == 0 {
            return ret;
        }
        for i in (bucket_index-1)..0 {
            for c in &self.buckets[i] {
                ret.push( (*c, dist(c.id, item)) );
                if ret.len() == count {
                    ret.sort_by(|&(_,a), &(_,b)| a.cmp(&b));
                    return ret;
                }
            }
        }
        return ret;
    }

}

fn new_random_node_id() -> Key {
    let mut res = [0; ID_SIZE];
    for i in 0us..ID_SIZE {
        res[i] = rand::random::<u8>();
    }
    Key(res)
}

fn dist(x: Key, y: Key) -> Distance{
    let mut res = [0; ID_SIZE];
    for i in 0us..ID_SIZE {
        res[i] = x.0[i] ^ y.0[i];
    }
    Distance(res)
}

fn main() {
    let mut r = RoutingTable::new(new_random_node_id());
    println!("routing table id: {:?}", r.origin);
    for _ in 0..4 {
        let k = new_random_node_id();
        r.update( Contact { id: k } );
        println!("new node: {:?}", k);
    }
    let item_id = new_random_node_id();
    println!("looking for item: {:?}", item_id);
    let results = r.find_closest_nodes(item_id, 3);
    println!("{:?}", results);
}
