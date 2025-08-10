use super::*;

/// The PlaneLayout dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PlaneLayout {
    inner: Any,
}

impl FromVal for PlaneLayout {
    fn from_val(v: &Any) -> Self {
        PlaneLayout { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PlaneLayout {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PlaneLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PlaneLayout {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PlaneLayout {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PlaneLayout> for Any {
    fn from(s: PlaneLayout) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PlaneLayout> for Any {
    fn from(s: &PlaneLayout) -> Any {
        s.inner.clone()
    }
}

impl PlaneLayout {
    /// Getter of the `offset` attribute.
    pub fn offset(&self) -> u32 {
        self.inner.get("offset").as_::<u32>()
    }

    /// Setter of the `offset` attribute.
    pub fn set_offset(&mut self, value: u32) {
        self.inner.set("offset", value);
    }
}
impl PlaneLayout {
    /// Getter of the `stride` attribute.
    pub fn stride(&self) -> u32 {
        self.inner.get("stride").as_::<u32>()
    }

    /// Setter of the `stride` attribute.
    pub fn set_stride(&mut self, value: u32) {
        self.inner.set("stride", value);
    }
}
