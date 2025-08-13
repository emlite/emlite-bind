use super::*;




/// The URLPatternComponentResult dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct URLPatternComponentResult {
    inner: Any,
}

impl FromVal for URLPatternComponentResult {
    fn from_val(v: &Any) -> Self {
        URLPatternComponentResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for URLPatternComponentResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for URLPatternComponentResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for URLPatternComponentResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for URLPatternComponentResult {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<URLPatternComponentResult> for Any {
    fn from(s: URLPatternComponentResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&URLPatternComponentResult> for Any {
    fn from(s: &URLPatternComponentResult) -> Any {
        s.inner.clone()
    }
}

impl URLPatternComponentResult {
    /// Getter of the `input` attribute.
    pub fn input(&self) -> JsString {
        self.inner.get("input").as_::<JsString>()
    }

    /// Setter of the `input` attribute.
    pub fn set_input(&mut self, value: &JsString) {
        self.inner.set("input", value);
    }
}
impl URLPatternComponentResult {
    /// Getter of the `groups` attribute.
    pub fn groups(&self) -> Record<JsString, Any> {
        self.inner.get("groups").as_::<Record<JsString, Any>>()
    }

    /// Setter of the `groups` attribute.
    pub fn set_groups(&mut self, value: &Record<JsString, Any>) {
        self.inner.set("groups", value);
    }
}
