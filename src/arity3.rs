// Generated. All manual edits below this line will be discarded.
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};

/// Owned version of the tuple key, to be used as a `HashMap` key.
#[derive(Hash, PartialEq, Eq)]
pub struct OwnedKey3<O1, O2, O3>(O1, O2, O3);

/// Lookup version of the tuple key; used as a lookup key.
#[repr(transparent)]
pub struct LookupKey3<'a, B1: ?Sized, B2: ?Sized, B3: ?Sized>((&'a B1, &'a B2, &'a B3));

pub trait DynKey3<B1: ?Sized, B2: ?Sized, B3: ?Sized> {
    fn key(&self) -> (&B1, &B2, &B3);
}

impl<'a, B1, B2, B3> Eq for (dyn DynKey3<B1, B2, B3> + 'a)
where
    B1: ?Sized + Eq,
    B2: ?Sized + Eq,
    B3: ?Sized + Eq,
{
}

impl<'a, B1, B2, B3> PartialEq for (dyn DynKey3<B1, B2, B3> + 'a)
where
    B1: ?Sized + PartialEq,
    B2: ?Sized + PartialEq,
    B3: ?Sized + PartialEq,
{
    fn eq(&self, other: &dyn DynKey3<B1, B2, B3>) -> bool {
        self.key() == other.key()
    }
}

impl<'a, B1, B2, B3> Hash for (dyn DynKey3<B1, B2, B3> + 'a)
where
    B1: ?Sized + Hash,
    B2: ?Sized + Hash,
    B3: ?Sized + Hash,
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
pub fn owned_key3<B1, B2, B3>(
    key1: &B1,
    key2: &B2,
    key3: &B3,
) -> OwnedKey3<B1::Owned, B2::Owned, B3::Owned>
where
    B1: ?Sized + ToOwned,
    B2: ?Sized + ToOwned,
    B3: ?Sized + ToOwned,
{
    OwnedKey3(key1.to_owned(), key2.to_owned(), key3.to_owned())
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
pub fn lookup_key3<'a, 'b, B1: ?Sized, B2: ?Sized, B3: ?Sized>(
    key: &'b (&'a B1, &'a B2, &'a B3),
) -> &'b (dyn DynKey3<B1, B2, B3> + 'a) {
    // We use #[repr(transparent)], so this is fine
    unsafe { &*(key as *const _ as *const LookupKey3<B1, B2, B3>) }
}

impl<'a, B1: ?Sized, B2: ?Sized, B3: ?Sized> DynKey3<B1, B2, B3> for LookupKey3<'a, B1, B2, B3> {
    fn key(&self) -> (&B1, &B2, &B3) {
        ((self.0).0, (self.0).1, (self.0).2)
    }
}

impl<B1, B2, B3, O1, O2, O3> DynKey3<B1, B2, B3> for OwnedKey3<O1, O2, O3>
where
    B1: ?Sized,
    B2: ?Sized,
    B3: ?Sized,
    O1: Borrow<B1>,
    O2: Borrow<B2>,
    O3: Borrow<B3>,
{
    fn key(&self) -> (&B1, &B2, &B3) {
        (self.0.borrow(), self.1.borrow(), self.2.borrow())
    }
}

impl<'a, B1, B2, B3, O1: 'a, O2: 'a, O3: 'a> Borrow<dyn DynKey3<B1, B2, B3> + 'a>
    for OwnedKey3<O1, O2, O3>
where
    B1: ?Sized,
    B2: ?Sized,
    B3: ?Sized,
    O1: Borrow<B1>,
    O2: Borrow<B2>,
    O3: Borrow<B3>,
{
    fn borrow(&self) -> &(dyn DynKey3<B1, B2, B3> + 'a) {
        self
    }
}
