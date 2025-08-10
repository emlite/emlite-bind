use super::*;

/// The RewriterRewriteOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RewriterRewriteOptions {
    inner: Any,
}

impl FromVal for RewriterRewriteOptions {
    fn from_val(v: &Any) -> Self {
        RewriterRewriteOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RewriterRewriteOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RewriterRewriteOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RewriterRewriteOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RewriterRewriteOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RewriterRewriteOptions> for Any {
    fn from(s: RewriterRewriteOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RewriterRewriteOptions> for Any {
    fn from(s: &RewriterRewriteOptions) -> Any {
        s.inner.clone()
    }
}

impl RewriterRewriteOptions {
    /// Getter of the `context` attribute.
    pub fn context(&self) -> JsString {
        self.inner.get("context").as_::<JsString>()
    }

    /// Setter of the `context` attribute.
    pub fn set_context(&mut self, value: &JsString) {
        self.inner.set("context", value);
    }
}
impl RewriterRewriteOptions {
    /// Getter of the `signal` attribute.
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    /// Setter of the `signal` attribute.
    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
