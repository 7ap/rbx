use crate::stl::functional::{EqualTo, Hash};
use crate::stl::utility::Pair;
use crate::stl::vector::Vector;

#[derive(Debug)]
#[repr(C)]
pub struct DenseHashSet;

#[derive(Debug)]
#[repr(C)]
pub struct DenseHashMap<Key, Value> {
    pub data: Vector<Pair<Key, Value>>,
    pub count: usize,
    pub empty_key: *const Key,
    pub hasher: Hash<Key>,
    pub eq: EqualTo<Key>,
}
