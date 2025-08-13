use super::*;




/// The CSSParserOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSParserOptions {
    inner: Any,
}

impl FromVal for CSSParserOptions {
    fn from_val(v: &Any) -> Self {
        CSSParserOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSParserOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSParserOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSParserOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSParserOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSParserOptions> for Any {
    fn from(s: CSSParserOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSParserOptions> for Any {
    fn from(s: &CSSParserOptions) -> Any {
        s.inner.clone()
    }
}

impl CSSParserOptions {
    /// Getter of the `atRules` attribute.
    pub fn at_rules(&self) -> Object {
        self.inner.get("atRules").as_::<Object>()
    }

    /// Setter of the `atRules` attribute.
    pub fn set_at_rules(&mut self, value: &Object) {
        self.inner.set("atRules", value);
    }
}
