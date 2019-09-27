// Generated. All manual edits below this line will be discarded.
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};

/// Owned version of the tuple key, to be used as a `HashMap` key.
#[derive(Hash, PartialEq, Eq)]
pub struct OwnedKey2<O1, O2>(O1, O2);

/// Lookup version of the tuple key; used as a lookup key.
#[repr(transparent)]
pub struct LookupKey2<'a, B1: ?Sized, B2: ?Sized>((&'a B1, &'a B2));

pub trait DynKey2<B1: ?Sized, B2: ?Sized> {
    fn key(&self) -> (&B1, &B2);
}

impl<'a, B1, B2> Eq for (dyn DynKey2<B1, B2> + 'a)
where
    B1: ?Sized + Eq,
    B2: ?Sized + Eq,
{
}

impl<'a, B1, B2> PartialEq for (dyn DynKey2<B1, B2> + 'a)
where
    B1: ?Sized + PartialEq,
    B2: ?Sized + PartialEq,
{
    fn eq(&self, other: &dyn DynKey2<B1, B2>) -> bool {
        self.key() == other.key()
    }
}

impl<'a, B1, B2> Hash for (dyn DynKey2<B1, B2> + 'a)
where
    B1: ?Sized + Hash,
    B2: ?Sized + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key().hash(state)
    }
}

/// Create an owned key. Example:
///
/// ```rust
/// use std::collections::HashMap;
/// use tuple_keys::{OwnedKey2, owned_key2, lookup_key2};
/// let mut map: HashMap<OwnedKey2<String, String>, String> = HashMap::new();
/// map.insert(owned_key2("hello", "bye"), "what?".into());
/// assert_eq!(map.get(lookup_key2(&("hello", "bye"))).unwrap(), "what?");
/// ```
pub fn owned_key2<B1, B2>(key1: &B1, key2: &B2) -> OwnedKey2<B1::Owned, B2::Owned>
where
    B1: ?Sized + ToOwned,
    B2: ?Sized + ToOwned,
{
    OwnedKey2(key1.to_owned(), key2.to_owned())
}

/// Create a lookup key. Example:
///
/// ```rust
/// use std::collections::HashMap;
/// use tuple_keys::{OwnedKey2, owned_key2, lookup_key2};
/// let mut map: HashMap<OwnedKey2<String, String>, String> = HashMap::new();
/// map.insert(owned_key2("hello", "bye"), "what?".into());
/// assert_eq!(map.get(lookup_key2(&("hello", "bye"))).unwrap(), "what?");
/// ```
pub fn lookup_key2<'a, 'b, B1: ?Sized, B2: ?Sized>(
    key: &'b (&'a B1, &'a B2),
) -> &'b (dyn DynKey2<B1, B2> + 'a) {
    // We use #[repr(transparent)], so this is fine
    unsafe { &*(key as *const _ as *const LookupKey2<B1, B2>) }
}

impl<'a, B1: ?Sized, B2: ?Sized> DynKey2<B1, B2> for LookupKey2<'a, B1, B2> {
    fn key(&self) -> (&B1, &B2) {
        ((self.0).0, (self.0).1)
    }
}

impl<B1, B2, O1, O2> DynKey2<B1, B2> for OwnedKey2<O1, O2>
where
    B1: ?Sized,
    B2: ?Sized,
    O1: Borrow<B1>,
    O2: Borrow<B2>,
{
    fn key(&self) -> (&B1, &B2) {
        (self.0.borrow(), self.1.borrow())
    }
}

impl<'a, B1, B2, O1: 'a, O2: 'a> Borrow<dyn DynKey2<B1, B2> + 'a> for OwnedKey2<O1, O2>
where
    B1: ?Sized,
    B2: ?Sized,
    O1: Borrow<B1>,
    O2: Borrow<B2>,
{
    fn borrow(&self) -> &(dyn DynKey2<B1, B2> + 'a) {
        self
    }
}
