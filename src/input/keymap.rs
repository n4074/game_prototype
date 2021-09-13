use std::any::{Any, TypeId};
use std::collections::{HashMap, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use std::fmt::Debug;

pub trait Key: Send + Sync + Debug + 'static {
    fn eq(&self, other: &dyn Key) -> bool;
    fn hash(&self) -> u64;
    fn as_any(&self) -> &dyn Any;
}

impl<T: Eq + Hash + 'static + Sync + Send + Debug + 'static> Key for T {
    fn eq(&self, other: &dyn Key) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<T>() {
            return self == other;
        }
        false
    }

    fn hash(&self) -> u64 {
        let mut h = DefaultHasher::new();
        // mix the typeid of T into the hash to make distinct types
        // provide distinct hashes
        Hash::hash(&(TypeId::of::<T>(), self), &mut h);
        h.finish()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PartialEq for Box<dyn Key> {
    fn eq(&self, other: &Self) -> bool {
        Key::eq(self.as_ref(), other.as_ref())
    }
}

impl Eq for Box<dyn Key> {}

impl Hash for Box<dyn Key> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let key_hash = Key::hash(self.as_ref());
        state.write_u64(key_hash);
    }
}