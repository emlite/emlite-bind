use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ConstrainBooleanParameters {
    inner: Any,
}
impl FromVal for ConstrainBooleanParameters {
    fn from_val(v: &Any) -> Self {
        ConstrainBooleanParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ConstrainBooleanParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ConstrainBooleanParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ConstrainBooleanParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ConstrainBooleanParameters {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ConstrainBooleanParameters> for Any {
    fn from(s: ConstrainBooleanParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ConstrainBooleanParameters> for Any {
    fn from(s: &ConstrainBooleanParameters) -> Any {
        s.inner.clone()
    }
}

impl ConstrainBooleanParameters {
    pub fn exact(&self) -> bool {
        self.inner.get("exact").as_::<bool>()
    }

    pub fn set_exact(&mut self, value: bool) {
        self.inner.set("exact", value);
    }
}
impl ConstrainBooleanParameters {
    pub fn ideal(&self) -> bool {
        self.inner.get("ideal").as_::<bool>()
    }

    pub fn set_ideal(&mut self, value: bool) {
        self.inner.set("ideal", value);
    }
}
