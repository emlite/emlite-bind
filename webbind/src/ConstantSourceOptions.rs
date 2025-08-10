use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ConstantSourceOptions {
    inner: Any,
}
impl FromVal for ConstantSourceOptions {
    fn from_val(v: &Any) -> Self {
        ConstantSourceOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ConstantSourceOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ConstantSourceOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ConstantSourceOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ConstantSourceOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ConstantSourceOptions> for Any {
    fn from(s: ConstantSourceOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ConstantSourceOptions> for Any {
    fn from(s: &ConstantSourceOptions) -> Any {
        s.inner.clone()
    }
}

impl ConstantSourceOptions {
    pub fn offset(&self) -> f32 {
        self.inner.get("offset").as_::<f32>()
    }

    pub fn set_offset(&mut self, value: f32) {
        self.inner.set("offset", value);
    }
}
