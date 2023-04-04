//! TODO

use std::marker::PhantomData;

#[derive(Debug)]
#[repr(C)]
pub struct UnorderedMap<Key, Value> {
    key: PhantomData<Key>,
    value: PhantomData<Value>,
}
