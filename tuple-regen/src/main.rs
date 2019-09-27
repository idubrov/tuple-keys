use proc_macro2::{TokenStream, Span, Ident, Literal};
use quote::quote;
use std::path::Path;
use sourcegen_cli::tokens::{NewLine, PlainComment};

fn generate_tokens(arity: usize) -> TokenStream {
    let owned_ident = Ident::new(&format!("OwnedKey{}", arity), Span::call_site());
    let lookup_ident = Ident::new(&format!("LookupKey{}", arity), Span::call_site());
    let key_ident = Ident::new(&format!("DynKey{}", arity), Span::call_site());
    let lookup_func = Ident::new(&format!("lookup_key{}", arity), Span::call_site());
    let owned_func = Ident::new(&format!("owned_key{}", arity), Span::call_site());
    
    let ot = (1..=arity).map(|idx| Ident::new(&format!("O{}", idx), Span::call_site())).collect::<Vec<_>>();
    let bt = (1..=arity).map(|idx| Ident::new(&format!("B{}", idx), Span::call_site())).collect::<Vec<_>>();
    let vars = (1..=arity).map(|idx| Ident::new(&format!("key{}", idx), Span::call_site())).collect::<Vec<_>>();
    let indices = (0..arity).map(Literal::usize_unsuffixed).collect::<Vec<_>>();
    quote! {
        use std::borrow::Borrow;
        use std::hash::{Hash, Hasher};
        #NewLine
        /// Owned version of the tuple key, to be used as a `HashMap` key.
        #[derive(Hash, PartialEq, Eq)]
        pub struct #owned_ident<#(#ot),*>(#(#ot),*);
        #NewLine
        /// Lookup version of the tuple key; used as a lookup key.
        // Note: extra tuple is required as we want a single field for `#[transparent]`
        #[repr(transparent)]
        pub struct #lookup_ident<'a, #(#bt: ?Sized),*>((#(&'a #bt),*));
        #NewLine
        #[doc(hidden)]
        pub trait #key_ident<#(#bt: ?Sized),*> {
            fn key(&self) -> (#(&#bt),*);
        }
        #NewLine
        impl<'a, #(#bt),*> Eq for (dyn #key_ident<#(#bt),*> + 'a)
            where #(#bt: ?Sized + Eq,)*
        {
        }
        #NewLine
        impl<'a, #(#bt),*> PartialEq for (dyn #key_ident<#(#bt),*> + 'a)
            where #(#bt: ?Sized + PartialEq,)*
        {
            fn eq(&self, other: &dyn #key_ident<#(#bt),*>) -> bool {
                self.key() == other.key()
            }
        }
        #NewLine
        impl<'a, #(#bt),*> Hash for (dyn #key_ident<#(#bt),*> + 'a)
            where #(#bt: ?Sized + Hash,)*
        {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.key().hash(state)
            }
        }
        #NewLine
        /// Create an owned key. Example:
        ///
        /// ```rust
        /// use std::collections::HashMap;
        /// use tuple_keys::{OwnedKey2, owned_key2, lookup_key2};
        /// let mut map: HashMap<OwnedKey2<String, String>, String> = HashMap::new();
        /// map.insert(owned_key2("hello", "bye"), "what?".into());
        /// assert_eq!(map.get(lookup_key2(&("hello", "bye"))).unwrap(), "what?");
        /// ```
        pub fn #owned_func<#(#bt),*>(#(#vars: &#bt),*) -> #owned_ident<#(#bt::Owned),*>
            where #(#bt: ?Sized + ToOwned),*
        {
            #owned_ident(#(#vars.to_owned()),*)
        }
        #NewLine
        /// Create a lookup key. Example:
        ///
        /// ```rust
        /// use std::collections::HashMap;
        /// use tuple_keys::{OwnedKey2, owned_key2, lookup_key2};
        /// let mut map: HashMap<OwnedKey2<String, String>, String> = HashMap::new();
        /// map.insert(owned_key2("hello", "bye"), "what?".into());
        /// assert_eq!(map.get(lookup_key2(&("hello", "bye"))).unwrap(), "what?");
        /// ```
        pub fn #lookup_func<'a, 'b,  #(#bt: ?Sized),*>(key: &'b (#(&'a #bt),*)) -> &'b (dyn #key_ident<#(#bt),*> + 'a) {
            #PlainComment "We use #[repr(transparent)], so this is fine"
            unsafe {
                &*(key as *const _ as *const #lookup_ident<#(#bt),*>)
            }
        }
        #NewLine
        impl<'a, #(#bt: ?Sized),*> #key_ident<#(#bt),*> for #lookup_ident<'a, #(#bt),*> {
            fn key(&self) -> (#(&#bt),*) {
                (#((self.0).#indices),*)
            }
        }
        #NewLine
        impl<#(#bt),*, #(#ot),*> #key_ident<#(#bt),*> for #owned_ident<#(#ot),*>
            where
                #(#bt: ?Sized,)*
                #(#ot: Borrow<#bt>,)*
        {
            fn key(&self) -> (#(&#bt),*) {
                (#(self.#indices.borrow()),*)
            }
        }
        #NewLine
        impl<'a, #(#bt),*, #(#ot: 'a),*> Borrow<dyn #key_ident<#(#bt),*> + 'a> for #owned_ident<#(#ot),*>
            where
                #(#bt: ?Sized,)*
                #(#ot: Borrow<#bt>,)*
        {
            fn borrow(&self) -> &(dyn #key_ident<#(#bt),*> + 'a) {
                self
            }
        }
    }
}


fn main() {
    let tokens = generate_tokens(2);
    sourcegen_cli::process_single_file(Path::new("src/arity2.rs"), tokens).unwrap();

    let tokens = generate_tokens(3);
    sourcegen_cli::process_single_file(Path::new("src/arity3.rs"), tokens).unwrap();
}