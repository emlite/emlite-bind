use crate::array::TypedArray;
use crate::error::SyntaxError;
use crate::string::JsString;
use crate::utils::bind;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// JavaScript RegExp object.
///
/// Regular expressions are patterns used to match character combinations in strings.
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct RegExp {
    inner: emlite::Val,
}

bind!(RegExp);

/// Flags that can be used when creating a RegExp.
#[derive(Debug, Clone, Copy)]
pub enum RegExpFlags {
    /// Global flag (g) - find all matches
    Global,
    /// Ignore case flag (i) - case-insensitive matching
    IgnoreCase,
    /// Multiline flag (m) - ^ and $ match line breaks
    Multiline,
    /// Dotall flag (s) - . matches any character including newline
    DotAll,
    /// Unicode flag (u) - enables Unicode features
    Unicode,
    /// Sticky flag (y) - matches only from lastIndex
    Sticky,
}

impl RegExpFlags {
    /// Convert flags to their string representation.
    fn to_string(flags: &[RegExpFlags]) -> String {
        let mut result = String::new();
        for flag in flags {
            match flag {
                RegExpFlags::Global => result.push('g'),
                RegExpFlags::IgnoreCase => result.push('i'),
                RegExpFlags::Multiline => result.push('m'),
                RegExpFlags::DotAll => result.push('s'),
                RegExpFlags::Unicode => result.push('u'),
                RegExpFlags::Sticky => result.push('y'),
            }
        }
        result
    }
}

fn new_syntax_error(msg: &str) -> SyntaxError {
    let ctor = emlite::Val::global("SyntaxError");
    ctor.invoke(&[msg.into()]).as_::<SyntaxError>()
}

impl RegExp {
    /// Gets the JS `RegExp` constructor (parity with C++ `instance()`).
    pub fn instance() -> emlite::Val {
        emlite::Val::global("RegExp")
    }

    /// Creates a new RegExp from a pattern string and optional flags.
    ///
    /// ```
    /// let regex = RegExp::new(r"\d+", Some(&[RegExpFlags::Global]))?;
    /// let simple_regex = RegExp::new(r"hello", None)?;
    /// # Ok::<(), SyntaxError>(())
    /// ```
    pub fn new(pattern: &str, flags: Option<&[RegExpFlags]>) -> Result<Self, SyntaxError> {
        let ctor = emlite::Val::global("RegExp");
        let result = match flags {
            Some(f) => {
                let flags_str = RegExpFlags::to_string(f);
                ctor.invoke(&[pattern.into(), flags_str.into()])
            }
            None => ctor.invoke(&[pattern.into()]),
        };
        result.as_::<Result<Self, SyntaxError>>()
    }

    /// Tests whether a string matches this regular expression.
    pub fn test(&self, text: &str) -> bool {
        self.inner.call("test", &[text.into()]).as_::<bool>()
    }

    /// Executes the regular expression against a string and returns the match array or `None`.
    ///
    /// The returned array is like JS `RegExp#exec` (full match + captures). Properties like
    /// `index`/`input` are on the array object in JS.
    pub fn exec(&self, text: &str) -> Option<TypedArray<JsString>> {
        let result = self.inner.call("exec", &[text.into()]);
        if result.is_null() {
            None
        } else {
            Some(result.as_::<TypedArray<JsString>>())
        }
    }

    /// STL-like contains check (parity with C++ `contains`).
    pub fn contains(&self, text: &str) -> bool {
        self.test(text)
    }

    /// STL-like find operation (first match) (parity with C++ `find`).
    pub fn find(&self, text: &str) -> Option<TypedArray<JsString>> {
        self.exec(text)
    }

    /// Gets the source pattern of this RegExp.
    pub fn source(&self) -> Option<String> {
        self.inner.get("source").as_::<Option<String>>()
    }

    /// Gets the flags string of this RegExp.
    pub fn flags(&self) -> Option<String> {
        self.inner.get("flags").as_::<Option<String>>()
    }

    /// Properties (booleans)
    pub fn global(&self) -> bool {
        self.inner.get("global").as_::<bool>()
    }
    pub fn ignore_case(&self) -> bool {
        self.inner.get("ignoreCase").as_::<bool>()
    }
    pub fn multiline(&self) -> bool {
        self.inner.get("multiline").as_::<bool>()
    }
    pub fn dot_all(&self) -> bool {
        self.inner.get("dotAll").as_::<bool>()
    }
    pub fn unicode(&self) -> bool {
        self.inner.get("unicode").as_::<bool>()
    }
    pub fn sticky(&self) -> bool {
        self.inner.get("sticky").as_::<bool>()
    }

    /// Gets or sets the `lastIndex` property.
    pub fn last_index(&self) -> i32 {
        self.inner.get("lastIndex").as_::<i32>()
    }
    pub fn set_last_index(&self, index: i32) {
        self.inner.set("lastIndex", index);
    }

    /// String representation like `/pattern/flags`.
    pub fn to_string_repr(&self) -> Option<String> {
        self.inner.call("toString", &[]).as_::<Option<String>>()
    }

    // ---------- Static convenience constructors (parity with C++) ----------

    /// Creates RegExp that matches literal text (parity with C++ `literal`).
    pub fn literal(text: &str) -> Result<Self, SyntaxError> {
        let escaped = Self::escape(text);
        Self::new(&escaped, None)
    }

    /// Case-insensitive RegExp (`/pattern/i`).
    pub fn new_case_insensitive(pattern: &str) -> Result<Self, SyntaxError> {
        Self::new(pattern, Some(&[RegExpFlags::IgnoreCase]))
    }

    /// Global RegExp (`/pattern/g`).
    pub fn new_global(pattern: &str) -> Result<Self, SyntaxError> {
        Self::new(pattern, Some(&[RegExpFlags::Global]))
    }

    /// Global + ignore case (`/pattern/gi`).
    pub fn new_global_ignore_case(pattern: &str) -> Result<Self, SyntaxError> {
        Self::new(
            pattern,
            Some(&[RegExpFlags::Global, RegExpFlags::IgnoreCase]),
        )
    }

    /// Multiline RegExp (`/pattern/m`).
    pub fn new_multiline(pattern: &str) -> Result<Self, SyntaxError> {
        Self::new(pattern, Some(&[RegExpFlags::Multiline]))
    }

    // ---------- Bulk operations ----------

    /// Gets **all** matches in a string (requires the global flag).
    ///
    /// Returns a `Vec` of match arrays in the same shape as `exec`.
    /// (C++ version uses `str.matchAll(this)` and returns an Array; here we iterate with `exec`.)
    pub fn find_all(&self, text: &str) -> Result<Vec<TypedArray<JsString>>, SyntaxError> {
        if !self.global() {
            return Err(new_syntax_error(
                "RegExp must have global flag for find_all",
            ));
        }
        let mut out = Vec::new();
        // Reset start position as in C++ iterator begin()
        self.set_last_index(0);

        loop {
            let result = self.inner.call("exec", &[text.into()]);
            if result.is_null() {
                break;
            }
            out.push(result.as_::<TypedArray<JsString>>());
        }
        Ok(out)
    }

    // ---------- Iteration over global matches (parity with C++ MatchIterator) ----------

    /// Creates an iterator over **global** matches (like `begin(text)` in C++).
    ///
    /// Use as: `for m in regex.begin("...")? { ... }`
    pub fn begin(&self, text: &str) -> Result<MatchIter, SyntaxError> {
        if !self.global() {
            return Err(new_syntax_error(
                "RegExp must have global flag for iteration",
            ));
        }
        let it = MatchIter {
            regexp: self.inner.clone(),
            text: text.to_string(),
            at_end: false,
        };
        // Ensure iteration starts at index 0 (parity with C++)
        it.regexp.set("lastIndex", 0);
        Ok(it)
    }

    // ---------- Utilities ----------

    /// Escapes special regular expression characters.
    pub fn escape(text: &str) -> String {
        text.chars()
            .map(|c| match c {
                '\\' | '[' | ']' | '(' | ')' | '{' | '}' | '?' | '+' | '*' | '|' | '^' | '$'
                | '.' => {
                    format!("\\{}", c)
                }
                _ => c.to_string(),
            })
            .collect()
    }
}

/// Iterator over matches for a **global** RegExp.
pub struct MatchIter {
    regexp: emlite::Val,
    text: String,
    at_end: bool,
}

impl Iterator for MatchIter {
    type Item = TypedArray<JsString>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at_end {
            return None;
        }
        let result = self.regexp.call("exec", &[self.text.as_str().into()]);
        if result.is_null() {
            self.at_end = true;
            None
        } else {
            Some(result.as_::<TypedArray<JsString>>())
        }
    }
}

impl crate::prelude::DynCast for RegExp {
    fn instanceof(val: &emlite::Val) -> bool {
        let regexp_ctor = emlite::Val::global("RegExp");
        val.instanceof(regexp_ctor)
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

impl core::fmt::Display for RegExp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_string_repr().unwrap_or_default())
    }
}
