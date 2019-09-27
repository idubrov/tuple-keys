mod arity2;
mod arity3;

pub use arity2::*;
pub use arity3::*;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{OwnedKey2, owned_key2, lookup_key2};

    #[test]
    fn test_arity2() {
        let mut map: HashMap<OwnedKey2<String, String>, String> = HashMap::new();
        map.insert(owned_key2("hello", "bye"), "what?".to_owned());
        assert_eq!(map.get(lookup_key2(&("hello", "bye"))).unwrap(), "what?");
    }
}
