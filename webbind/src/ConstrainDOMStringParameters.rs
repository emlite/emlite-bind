use super::*;

/// The ConstrainDOMStringParameters dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ConstrainDOMStringParameters {
    inner: Any,
}

impl FromVal for ConstrainDOMStringParameters {
    fn from_val(v: &Any) -> Self {
        ConstrainDOMStringParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ConstrainDOMStringParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ConstrainDOMStringParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ConstrainDOMStringParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ConstrainDOMStringParameters {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ConstrainDOMStringParameters> for Any {
    fn from(s: ConstrainDOMStringParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ConstrainDOMStringParameters> for Any {
    fn from(s: &ConstrainDOMStringParameters) -> Any {
        s.inner.clone()
    }
}

impl ConstrainDOMStringParameters {
    /// Getter of the `exact` attribute.
    pub fn exact(&self) -> Any {
        self.inner.get("exact").as_::<Any>()
    }

    /// Setter of the `exact` attribute.
    pub fn set_exact(&mut self, value: &Any) {
        self.inner.set("exact", value);
    }
}
impl ConstrainDOMStringParameters {
    /// Getter of the `ideal` attribute.
    pub fn ideal(&self) -> Any {
        self.inner.get("ideal").as_::<Any>()
    }

    /// Setter of the `ideal` attribute.
    pub fn set_ideal(&mut self, value: &Any) {
        self.inner.set("ideal", value);
    }
}
