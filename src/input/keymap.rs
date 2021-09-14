use std::any::{Any, TypeId};
use std::collections::{HashMap, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use std::fmt::Debug;

use num_traits::{ToPrimitive, FromPrimitive};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct AnyKey(TypeId, u16);

pub trait AsAnyKey {
    fn key(&self) -> AnyKey;
}

impl<T: 'static + Any + ToPrimitive> From<T> for AnyKey {
    fn from(this: T) -> Self {
        AnyKey(this.type_id(), this.to_u16().unwrap())
    }
}

pub fn from_key<T: 'static + FromPrimitive>(key: &AnyKey) ->  Option<T> {
    if TypeId::of::<T>() != key.0 {
        return None
    } else {
        T::from_u16(key.1)
    }
}