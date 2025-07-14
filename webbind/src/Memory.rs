use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Memory {
    inner: emlite::Val,
}
impl FromVal for Memory {
    fn from_val(v: &emlite::Val) -> Self {
        Memory {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Memory {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Memory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Memory> for emlite::Val {
    fn from(s: Memory) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Memory {
    pub fn new(descriptor: jsbind::Any) -> Memory {
        Self {
            inner: emlite::Val::global("Memory")
                .new(&[descriptor.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl Memory {
    pub fn grow(&self, delta: u32) -> u32 {
        self.inner.call("grow", &[delta.into()]).as_::<u32>()
    }
}
impl Memory {
    pub fn to_fixed_length_buffer(&self) -> jsbind::ArrayBuffer {
        self.inner
            .call("toFixedLengthBuffer", &[])
            .as_::<jsbind::ArrayBuffer>()
    }
}
impl Memory {
    pub fn to_resizable_buffer(&self) -> jsbind::ArrayBuffer {
        self.inner
            .call("toResizableBuffer", &[])
            .as_::<jsbind::ArrayBuffer>()
    }
}
impl Memory {
    pub fn buffer(&self) -> jsbind::ArrayBuffer {
        self.inner.get("buffer").as_::<jsbind::ArrayBuffer>()
    }
}
