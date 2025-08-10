use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaintRenderingContext2DSettings {
    inner: Any,
}
impl FromVal for PaintRenderingContext2DSettings {
    fn from_val(v: &Any) -> Self {
        PaintRenderingContext2DSettings { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PaintRenderingContext2DSettings {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaintRenderingContext2DSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PaintRenderingContext2DSettings {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PaintRenderingContext2DSettings {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PaintRenderingContext2DSettings> for Any {
    fn from(s: PaintRenderingContext2DSettings) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PaintRenderingContext2DSettings> for Any {
    fn from(s: &PaintRenderingContext2DSettings) -> Any {
        s.inner.clone()
    }
}

impl PaintRenderingContext2DSettings {
    pub fn alpha(&self) -> bool {
        self.inner.get("alpha").as_::<bool>()
    }

    pub fn set_alpha(&mut self, value: bool) {
        self.inner.set("alpha", value);
    }
}
