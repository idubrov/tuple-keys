# tuple-keys

**Tuple Keys**

[![crates.io][Crate Logo]][Crate]
[![Documentation][Doc Logo]][Doc]

Tuple keys is a small crate that allows storing "owned" tuples as keys in a `HashMap` and using tuples with "borrowed"
keys for lookup. For example, the following code stores `(String, String)` as `HashMap` key and looks it up using
`(&str, &str)` key:

```rust
#[test]
fn test() {
    let mut map: HashMap<OwnedKey2<String, String>, String> = HashMap::new();
    map.insert(owned_key2("hello", "bye"), "what?".to_owned());
    assert_eq!(map.get(lookup_key2(&("hello", "bye"))).unwrap(), "what?");
}
``` 

[Crate]: https://crates.io/crates/tuple-keys
[Crate Logo]: https://img.shields.io/crates/v/tuple-keys.svg

[Doc]: https://docs.rs/tuple-keys
[Doc Logo]: https://docs.rs/tuple-keys/badge.svg
