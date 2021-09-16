use std::any::{Any, TypeId};
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use num_traits::{FromPrimitive, ToPrimitive};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct AnyKey(TypeId, u16);

pub trait AsAnyKey {
    fn key(&self) -> AnyKey;
}

impl<T: 'static + Any + ToPrimitive> From<T> for AnyKey {
    fn from(this: T) -> Self {
        AnyKey(this.type_id(), this.to_u16().unwrap())
    }
}

pub fn from_key<T: 'static + FromPrimitive>(key: &AnyKey) -> Option<T> {
    if TypeId::of::<T>() != key.0 {
        return None;
    } else {
        T::from_u16(key.1)
    }
}
