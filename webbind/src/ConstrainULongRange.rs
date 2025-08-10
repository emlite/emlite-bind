use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ConstrainULongRange {
    inner: Any,
}
impl FromVal for ConstrainULongRange {
    fn from_val(v: &Any) -> Self {
        ConstrainULongRange { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ConstrainULongRange {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ConstrainULongRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ConstrainULongRange {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ConstrainULongRange {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ConstrainULongRange> for Any {
    fn from(s: ConstrainULongRange) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ConstrainULongRange> for Any {
    fn from(s: &ConstrainULongRange) -> Any {
        s.inner.clone()
    }
}

impl ConstrainULongRange {
    pub fn exact(&self) -> u32 {
        self.inner.get("exact").as_::<u32>()
    }

    pub fn set_exact(&mut self, value: u32) {
        self.inner.set("exact", value);
    }
}
impl ConstrainULongRange {
    pub fn ideal(&self) -> u32 {
        self.inner.get("ideal").as_::<u32>()
    }

    pub fn set_ideal(&mut self, value: u32) {
        self.inner.set("ideal", value);
    }
}
