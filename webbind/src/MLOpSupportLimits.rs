use super::*;

/// The MLOpSupportLimits dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLOpSupportLimits {
    inner: Any,
}

impl FromVal for MLOpSupportLimits {
    fn from_val(v: &Any) -> Self {
        MLOpSupportLimits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLOpSupportLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLOpSupportLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLOpSupportLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLOpSupportLimits {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLOpSupportLimits> for Any {
    fn from(s: MLOpSupportLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLOpSupportLimits> for Any {
    fn from(s: &MLOpSupportLimits) -> Any {
        s.inner.clone()
    }
}

impl MLOpSupportLimits {
    /// Getter of the `where` attribute.
    pub fn where_(&self) -> MLWhereSupportLimits {
        self.inner.get("where").as_::<MLWhereSupportLimits>()
    }

    /// Setter of the `where` attribute.
    pub fn set_where_(&mut self, value: &MLWhereSupportLimits) {
        self.inner.set("where", value);
    }
}
