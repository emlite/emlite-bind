use crate::utils::bind;
use alloc::string::String;
use emlite::FromVal;

/// JavaScript Symbol type.
///
/// Symbols are a primitive data type in JavaScript that are guaranteed to be unique.
/// They are often used as object keys to avoid naming collisions.
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Symbol {
    inner: emlite::Val,
}

bind!(Symbol);

impl Symbol {
    /// Creates a new Symbol with an optional description.
    ///
    /// # Arguments
    /// * `description` - Optional description for the symbol
    ///
    /// # Examples
    /// ```rust
    /// use jsbind::prelude::*;
    ///
    /// let sym1 = Symbol::new(None);
    /// let sym2 = Symbol::new(Some("my symbol"));
    /// ```
    pub fn new(description: Option<&str>) -> Self {
        let ctor = emlite::Val::global("Symbol");
        let val = match description {
            Some(desc) => ctor.invoke(&[desc.into()]),
            None => ctor.invoke(&[]),
        };
        Self { inner: val }
    }

    /// Creates a symbol from the global Symbol registry.
    ///
    /// # Arguments
    /// * `key` - The key to lookup/register in the global symbol registry
    ///
    /// # Returns
    /// The symbol associated with the key
    ///
    /// # Examples
    /// ```rust
    /// use jsbind::prelude::*;
    ///
    /// let sym1 = Symbol::for_key("myKey");
    /// let sym2 = Symbol::for_key("myKey");
    /// // sym1 and sym2 refer to the same symbol
    /// ```
    pub fn for_key(key: &str) -> Self {
        let val = emlite::Val::global("Symbol").call("for", &[key.into()]);
        Self { inner: val }
    }

    /// Gets the key for this symbol from the global symbol registry.
    ///
    /// # Returns
    /// The key if this symbol is in the global registry, `None` otherwise
    ///
    /// # Examples
    /// ```rust
    /// use jsbind::prelude::*;
    ///
    /// let sym = Symbol::for_key("myKey");
    /// assert_eq!(sym.key_for(), Some("myKey".to_string()));
    ///
    /// let local_sym = Symbol::new(None);
    /// assert_eq!(local_sym.key_for(), None);
    /// ```
    pub fn key_for(&self) -> Option<String> {
        let result = emlite::Val::global("Symbol").call("keyFor", &[self.inner.clone()]);

        if result.is_undefined() {
            None
        } else {
            result.as_::<Option<String>>()
        }
    }

    /// Gets the description of this symbol.
    ///
    /// # Returns
    /// The description if available
    ///
    /// # Examples
    /// ```rust
    /// use jsbind::prelude::*;
    ///
    /// let sym = Symbol::new(Some("test symbol"));
    /// assert_eq!(sym.description(), Some("test symbol".to_string()));
    /// ```
    pub fn description(&self) -> Option<String> {
        let desc = self.inner.get("description");
        if desc.is_undefined() || desc.is_null() {
            None
        } else {
            desc.as_::<Option<String>>()
        }
    }

    /// Returns the string representation of this symbol.
    ///
    /// # Returns
    /// String representation like "Symbol(description)"
    ///
    /// # Examples
    /// ```rust
    /// use jsbind::prelude::*;
    ///
    /// let sym = Symbol::new(Some("test"));
    /// let str_repr = sym.to_string_repr();
    /// // str_repr will be something like "Symbol(test)"
    /// ```
    pub fn to_string_repr(&self) -> String {
        self.inner
            .call("toString", &[])
            .as_::<Option<String>>()
            .unwrap_or_default()
    }

    /// Gets primitive symbol value as string.
    ///
    /// # Returns
    /// String representation of the symbol
    ///
    /// # Examples
    /// ```rust
    /// use jsbind::prelude::*;
    ///
    /// let sym = Symbol::new(Some("test"));
    /// let value = sym.value_of();
    /// ```
    pub fn value_of(&self) -> String {
        self.inner
            .call("valueOf", &[])
            .as_::<Option<String>>()
            .unwrap_or_default()
    }

    /// Checks if symbol has no description.
    ///
    /// # Returns
    /// `true` if description is undefined
    ///
    /// # Examples
    /// ```rust
    /// use jsbind::prelude::*;
    ///
    /// let sym1 = Symbol::new(None);
    /// assert!(sym1.is_empty());
    ///
    /// let sym2 = Symbol::new(Some("test"));
    /// assert!(!sym2.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.description().is_none()
    }

    /// Gets hash code for symbol.
    ///
    /// # Returns
    /// Hash value based on unique handle
    ///
    /// # Examples
    /// ```rust
    /// use jsbind::prelude::*;
    ///
    /// let sym = Symbol::new(Some("test"));
    /// let hash_val = sym.get_hash();
    /// ```
    pub fn get_hash(&self) -> u64 {
        // Simple hash based on the handle value
        self.inner.as_handle() as u64
    }

    // Well-known symbols

    /// Symbol.asyncIterator
    pub fn async_iterator() -> Self {
        Self {
            inner: emlite::Val::global("Symbol").get("asyncIterator"),
        }
    }

    /// Symbol.hasInstance  
    pub fn has_instance() -> Self {
        Self {
            inner: emlite::Val::global("Symbol").get("hasInstance"),
        }
    }

    /// Symbol.isConcatSpreadable
    pub fn is_concat_spreadable() -> Self {
        Self {
            inner: emlite::Val::global("Symbol").get("isConcatSpreadable"),
        }
    }

    /// Symbol.iterator
    pub fn iterator() -> Self {
        Self {
            inner: emlite::Val::global("Symbol").get("iterator"),
        }
    }

    /// Symbol.match
    pub fn match_() -> Self {
        Self {
            inner: emlite::Val::global("Symbol").get("match"),
        }
    }

    /// Symbol.matchAll
    pub fn match_all() -> Self {
        Self {
            inner: emlite::Val::global("Symbol").get("matchAll"),
        }
    }

    /// Symbol.replace
    pub fn replace() -> Self {
        Self {
            inner: emlite::Val::global("Symbol").get("replace"),
        }
    }

    /// Symbol.search
    pub fn search() -> Self {
        Self {
            inner: emlite::Val::global("Symbol").get("search"),
        }
    }

    /// Symbol.species
    pub fn species() -> Self {
        Self {
            inner: emlite::Val::global("Symbol").get("species"),
        }
    }

    /// Symbol.split
    pub fn split() -> Self {
        Self {
            inner: emlite::Val::global("Symbol").get("split"),
        }
    }

    /// Symbol.toPrimitive
    pub fn to_primitive() -> Self {
        Self {
            inner: emlite::Val::global("Symbol").get("toPrimitive"),
        }
    }

    /// Symbol.toStringTag
    pub fn to_string_tag() -> Self {
        Self {
            inner: emlite::Val::global("Symbol").get("toStringTag"),
        }
    }

    /// Symbol.unscopables
    pub fn unscopables() -> Self {
        Self {
            inner: emlite::Val::global("Symbol").get("unscopables"),
        }
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.inner.as_handle() == other.inner.as_handle()
    }
}

impl Eq for Symbol {}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Symbol {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.inner.as_handle().cmp(&other.inner.as_handle())
    }
}

impl core::hash::Hash for Symbol {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.inner.as_handle().hash(state);
    }
}

impl crate::prelude::DynCast for Symbol {
    fn instanceof(val: &emlite::Val) -> bool {
        val.type_of() == "symbol"
    }

    fn unchecked_from_val(v: emlite::Val) -> Self {
        Self { inner: v }
    }

    fn unchecked_from_val_ref(v: &emlite::Val) -> &Self {
        unsafe { &*(v as *const emlite::Val as *const Self) }
    }

    fn unchecked_from_val_mut(v: &mut emlite::Val) -> &mut Self {
        unsafe { &mut *(v as *mut emlite::Val as *mut Self) }
    }
}
