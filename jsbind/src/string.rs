use crate::any::Any;
use crate::utils::*;
use alloc::string::String;

macro_rules! declare_string {
    ($name:ident) => {
        #[doc = concat!("Wrapper for WebIDL `", stringify!($name), "`.\n\n")]
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        #[repr(transparent)]
        pub struct $name {
            inner: emlite::Val,
        }

        bind!($name);

        impl From<$name> for Option<String> {
            fn from(s: $name) -> Self {
                s.as_::<Self>()
            }
        }

        impl From<$name> for Option<&str> {
            fn from(s: $name) -> Self {
                s.as_::<Self>()
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                if core::any::TypeId::of::<$name>() == core::any::TypeId::of::<ByteString>() {
                    assert!(s.is_ascii(), "ByteString must be ASCII/Latin-1");
                }
                emlite::Val::from(s).as_::<Self>()
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                emlite::Val::from(&s).as_::<Self>()
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str().unwrap()
            }
        }

        impl crate::prelude::DynCast for $name {
            #[inline]
            fn instanceof(val: &Any) -> bool {
                let ctor = emlite::Val::global("String");
                val.instanceof(ctor)
            }
            #[inline]
            fn unchecked_from_val(v: emlite::Val) -> Self {
                v.as_::<Self>() // zero-cost new-type cast
            }
            #[inline]
            fn unchecked_from_val_ref(v: &emlite::Val) -> &Self {
                unsafe { &*(v as *const emlite::Val as *const Self) }
            }

            #[inline]
            fn unchecked_from_val_mut(v: &mut emlite::Val) -> &mut Self {
                unsafe { &mut *(v as *mut emlite::Val as *mut Self) }
            }
        }

        impl $name {
            /// `len() == 0` convenience.
            pub fn is_empty(&self) -> bool {
                self.length() == 0
            }
            /// Borrow the JavaScript string as `&str` (UTF‑8 view).
            pub fn as_str(&self) -> Option<&str> {
                self.inner.as_::<Option<&str>>()
            }

            /// Borrow the JavaScript string as `&str` (UTF‑8 view).
            pub fn to_string(&self) -> Option<String> {
                self.inner.as_::<Option<String>>()
            }

            /// Number of UTF-16 code units (`JSString.length`).
            pub fn length(&self) -> usize {
                self.inner.get("length").as_::<usize>()
            }

            /// Returns the 16-bit code unit at `idx` (like `charCodeAt`).
            pub fn char_code_at(&self, idx: usize) -> Option<u16> {
                let v = self.inner.call("charCodeAt", &[idx.into()]);
                if v.is_undefined() {
                    None
                } else {
                    Some(v.as_::<u16>())
                }
            }

            pub fn set(&self, idx: usize, val: char) {
                self.inner.set(idx, val as u8);
            }

            /// [`String.prototype.at`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/at)
            pub fn at(&self, idx: isize) -> Option<Self> {
                let v = self.inner.call("at", &[idx.into()]);
                if v.is_undefined() {
                    None
                } else {
                    Some(v.as_::<Self>())
                }
            }
            /// [`String.prototype.codePointAt`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/codePointAt)
            pub fn code_point_at(&self, idx: usize) -> Option<u32> {
                let v = self.inner.call("codePointAt", &[idx.into()]);
                if v.is_undefined() {
                    None
                } else {
                    Some(v.as_::<u32>())
                }
            }
            /// [`String.prototype.concat`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/concat)
            pub fn concat(&self, rhs: &Self) -> Self {
                self.inner
                    .call("concat", &[rhs.clone().into()])
                    .as_::<Self>()
            }
            /// [`String.prototype.endsWith`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/endsWith)
            pub fn ends_with(&self, pat: &str) -> bool {
                self.inner.call("endsWith", &[pat.into()]).as_::<bool>()
            }
            /// [`String.prototype.includes`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/includes)
            pub fn includes(&self, pat: &str) -> bool {
                self.inner.call("includes", &[pat.into()]).as_::<bool>()
            }
            /// [`String.prototype.indexOf`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/indexOf)
            /// Returns `None` when not found.
            pub fn index_of(&self, pat: &str) -> Option<usize> {
                let n = self.inner.call("indexOf", &[pat.into()]).as_::<i32>();
                if n == -1 { None } else { Some(n as usize) }
            }
            /// [`String.prototype.isWellFormed`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/isWellFormed)
            pub fn is_well_formed(&self) -> bool {
                self.inner.call("isWellFormed", &[]).as_::<bool>()
            }
            /// [`String.prototype.lastIndexOf`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/lastIndexOf)
            pub fn last_index_of(&self, pat: &str) -> Option<usize> {
                let n = self.inner.call("lastIndexOf", &[pat.into()]).as_::<i32>();
                if n == -1 { None } else { Some(n as usize) }
            }
            /// [`String.prototype.localeCompare`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/localeCompare)
            pub fn locale_compare(&self, other: &str) -> i32 {
                self.inner
                    .call("localeCompare", &[other.into()])
                    .as_::<i32>()
            }
            /// [`String.prototype.match`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/match)
            pub fn match_(&self, pat: &Any) -> Any {
                self.inner.call("match", &[pat.clone()])
            }
            /// [`String.prototype.matchAll`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/matchAll)
            pub fn match_all(&self, pat: &Any) -> Any {
                self.inner.call("matchAll", &[pat.clone()])
            }
            /// [`String.prototype.normalize`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/normalize)
            pub fn normalize(&self, form: Option<&str>) -> Self {
                match form {
                    Some(f) => self.inner.call("normalize", &[f.into()]),
                    None => self.inner.call("normalize", &[]),
                }
                .as_::<Self>()
            }
            /// [`String.prototype.padEnd`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/padEnd)
            pub fn pad_end(&self, target_len: usize, pad: Option<&str>) -> Self {
                match pad {
                    Some(p) => self.inner.call("padEnd", &[target_len.into(), p.into()]),
                    None => self.inner.call("padEnd", &[target_len.into()]),
                }
                .as_::<Self>()
            }
            /// [`String.prototype.padStart`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/padStart)
            pub fn pad_start(&self, target_len: usize, pad: Option<&str>) -> Self {
                match pad {
                    Some(p) => self.inner.call("padStart", &[target_len.into(), p.into()]),
                    None => self.inner.call("padStart", &[target_len.into()]),
                }
                .as_::<Self>()
            }
            /// [`String.prototype.repeat`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/repeat)
            pub fn repeat(&self, count: usize) -> Self {
                self.inner.call("repeat", &[count.into()]).as_::<Self>()
            }
            /// [`String.prototype.replace`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/replace)
            pub fn replace(&self, pat: &Any, repl: &Any) -> Self {
                self.inner
                    .call("replace", &[pat.clone(), repl.clone()])
                    .as_::<Self>()
            }
            /// [`String.prototype.replaceAll`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/replaceAll)
            pub fn replace_all(&self, pat: &Any, repl: &Any) -> Self {
                self.inner
                    .call("replaceAll", &[pat.clone(), repl.clone()])
                    .as_::<Self>()
            }
            /// [`String.prototype.search`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/search)
            pub fn search(&self, pat: &Any) -> isize {
                self.inner.call("search", &[pat.clone()]).as_::<i32>() as isize
            }
            /// [`String.prototype.slice`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/slice)
            pub fn slice(&self, start: isize, end: Option<isize>) -> Self {
                match end {
                    Some(e) => self.inner.call("slice", &[start.into(), e.into()]),
                    None => self.inner.call("slice", &[start.into()]),
                }
                .as_::<Self>()
            }
            /// [`String.prototype.split`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/split)
            pub fn split(&self, sep: &str) -> crate::sequence::Sequence<Self> {
                self.inner
                    .call("split", &[sep.into()])
                    .as_::<crate::sequence::Sequence<Self>>()
            }
            /// [`String.prototype.startsWith`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/startsWith)
            pub fn starts_with(&self, pat: &str) -> bool {
                self.inner.call("startsWith", &[pat.into()]).as_::<bool>()
            }
            /// [`String.prototype.substring`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/substring)
            pub fn substring(&self, start: usize, end: Option<usize>) -> Self {
                match end {
                    Some(e) => self.inner.call("substring", &[start.into(), e.into()]),
                    None => self.inner.call("substring", &[start.into()]),
                }
                .as_::<Self>()
            }
            /// [`String.prototype.toLocaleLowerCase`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/toLocaleLowerCase)
            pub fn to_locale_lower_case(&self) -> Self {
                self.inner.call("toLocaleLowerCase", &[]).as_::<Self>()
            }
            /// [`String.prototype.toLocaleUpperCase`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/toLocaleUpperCase)
            pub fn to_locale_upper_case(&self) -> Self {
                self.inner.call("toLocaleUpperCase", &[]).as_::<Self>()
            }
            /// [`String.prototype.toLowerCase`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/toLowerCase)
            pub fn to_lower_case(&self) -> Self {
                self.inner.call("toLowerCase", &[]).as_::<Self>()
            }
            /// [`String.prototype.toUpperCase`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/toUpperCase)
            pub fn to_upper_case(&self) -> Self {
                self.inner.call("toUpperCase", &[]).as_::<Self>()
            }
            /// [`String.prototype.toWellFormed`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/toWellFormed)
            pub fn to_well_formed(&self) -> Self {
                self.inner.call("toWellFormed", &[]).as_::<Self>()
            }
            /// [`String.prototype.trim`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/trim)
            pub fn trim(&self) -> Self {
                self.inner.call("trim", &[]).as_::<Self>()
            }
            /// [`String.prototype.trimEnd`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/trimEnd)
            pub fn trim_end(&self) -> Self {
                self.inner.call("trimEnd", &[]).as_::<Self>()
            }
            /// [`String.prototype.trimStart`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/String/trimStart)
            pub fn trim_start(&self) -> Self {
                self.inner.call("trimStart", &[]).as_::<Self>()
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if let Some(s) = self.as_str() {
                    f.write_str(s)
                } else {
                    f.write_str("undefined")
                }
            }
        }

        impl core::ops::Add for $name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                self.call("concat", &[rhs.into()]).as_::<Self>()
            }
        }

        impl PartialEq<str> for $name {
            fn eq(&self, other: &str) -> bool {
                self.as_str() == Some(other)
            }
        }
    };
}

declare_string!(ByteString);
declare_string!(DOMString);
declare_string!(CSSOMString);
declare_string!(USVString);
