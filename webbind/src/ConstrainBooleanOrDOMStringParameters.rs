use super::*;

/// The ConstrainBooleanOrDOMStringParameters dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ConstrainBooleanOrDOMStringParameters {
    inner: Any,
}

impl FromVal for ConstrainBooleanOrDOMStringParameters {
    fn from_val(v: &Any) -> Self {
        ConstrainBooleanOrDOMStringParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ConstrainBooleanOrDOMStringParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ConstrainBooleanOrDOMStringParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ConstrainBooleanOrDOMStringParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ConstrainBooleanOrDOMStringParameters {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ConstrainBooleanOrDOMStringParameters> for Any {
    fn from(s: ConstrainBooleanOrDOMStringParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ConstrainBooleanOrDOMStringParameters> for Any {
    fn from(s: &ConstrainBooleanOrDOMStringParameters) -> Any {
        s.inner.clone()
    }
}

impl ConstrainBooleanOrDOMStringParameters {
    /// Getter of the `exact` attribute.
    pub fn exact(&self) -> Any {
        self.inner.get("exact").as_::<Any>()
    }

    /// Setter of the `exact` attribute.
    pub fn set_exact(&mut self, value: &Any) {
        self.inner.set("exact", value);
    }
}
impl ConstrainBooleanOrDOMStringParameters {
    /// Getter of the `ideal` attribute.
    pub fn ideal(&self) -> Any {
        self.inner.get("ideal").as_::<Any>()
    }

    /// Setter of the `ideal` attribute.
    pub fn set_ideal(&mut self, value: &Any) {
        self.inner.set("ideal", value);
    }
}
