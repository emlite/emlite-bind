use super::*;

/// The BreakTokenOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BreakTokenOptions {
    inner: Any,
}

impl FromVal for BreakTokenOptions {
    fn from_val(v: &Any) -> Self {
        BreakTokenOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BreakTokenOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BreakTokenOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BreakTokenOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BreakTokenOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BreakTokenOptions> for Any {
    fn from(s: BreakTokenOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BreakTokenOptions> for Any {
    fn from(s: &BreakTokenOptions) -> Any {
        s.inner.clone()
    }
}

impl BreakTokenOptions {
    /// Getter of the `childBreakTokens` attribute.
    pub fn child_break_tokens(&self) -> TypedArray<ChildBreakToken> {
        self.inner
            .get("childBreakTokens")
            .as_::<TypedArray<ChildBreakToken>>()
    }

    /// Setter of the `childBreakTokens` attribute.
    pub fn set_child_break_tokens(&mut self, value: &TypedArray<ChildBreakToken>) {
        self.inner.set("childBreakTokens", value);
    }
}
impl BreakTokenOptions {
    /// Getter of the `data` attribute.
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }

    /// Setter of the `data` attribute.
    pub fn set_data(&mut self, value: &Any) {
        self.inner.set("data", value);
    }
}
