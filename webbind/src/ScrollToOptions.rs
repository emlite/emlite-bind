use super::*;

/// The ScrollToOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ScrollToOptions {
    inner: Any,
}

impl FromVal for ScrollToOptions {
    fn from_val(v: &Any) -> Self {
        ScrollToOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ScrollToOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ScrollToOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ScrollToOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ScrollToOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ScrollToOptions> for Any {
    fn from(s: ScrollToOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ScrollToOptions> for Any {
    fn from(s: &ScrollToOptions) -> Any {
        s.inner.clone()
    }
}

impl ScrollToOptions {
    /// Getter of the `left` attribute.
    pub fn left(&self) -> f64 {
        self.inner.get("left").as_::<f64>()
    }

    /// Setter of the `left` attribute.
    pub fn set_left(&mut self, value: f64) {
        self.inner.set("left", value);
    }
}
impl ScrollToOptions {
    /// Getter of the `top` attribute.
    pub fn top(&self) -> f64 {
        self.inner.get("top").as_::<f64>()
    }

    /// Setter of the `top` attribute.
    pub fn set_top(&mut self, value: f64) {
        self.inner.set("top", value);
    }
}
