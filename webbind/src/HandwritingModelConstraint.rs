use super::*;

/// The HandwritingModelConstraint dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HandwritingModelConstraint {
    inner: Any,
}

impl FromVal for HandwritingModelConstraint {
    fn from_val(v: &Any) -> Self {
        HandwritingModelConstraint { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HandwritingModelConstraint {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HandwritingModelConstraint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HandwritingModelConstraint {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HandwritingModelConstraint {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HandwritingModelConstraint> for Any {
    fn from(s: HandwritingModelConstraint) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HandwritingModelConstraint> for Any {
    fn from(s: &HandwritingModelConstraint) -> Any {
        s.inner.clone()
    }
}

impl HandwritingModelConstraint {
    /// Getter of the `languages` attribute.
    pub fn languages(&self) -> TypedArray<JsString> {
        self.inner.get("languages").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `languages` attribute.
    pub fn set_languages(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("languages", value);
    }
}
