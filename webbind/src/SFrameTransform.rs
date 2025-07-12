use super::*;

#[derive(Clone, Debug)]
pub struct SFrameTransform {
    inner: EventTarget,
}
impl FromVal for SFrameTransform {
    fn from_val(v: &emlite::Val) -> Self {
        SFrameTransform {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SFrameTransform {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SFrameTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SFrameTransform> for emlite::Val {
    fn from(s: SFrameTransform) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SFrameTransform {
    pub fn new0() -> SFrameTransform {
        Self {
            inner: emlite::Val::global("SFrameTransform")
                .new(&[])
                .as_::<EventTarget>(),
        }
    }

    pub fn new1(options: jsbind::Any) -> SFrameTransform {
        Self {
            inner: emlite::Val::global("SFrameTransform")
                .new(&[options.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl SFrameTransform {
    pub fn set_encryption_key0(&self, key: CryptoKey) -> jsbind::Promise {
        self.inner
            .call("setEncryptionKey", &[key.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn set_encryption_key1(&self, key: CryptoKey, key_id: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("setEncryptionKey", &[key.into(), key_id.into()])
            .as_::<jsbind::Promise>()
    }
}
impl SFrameTransform {
    pub fn onerror(&self) -> jsbind::Any {
        self.inner.get("onerror").as_::<jsbind::Any>()
    }

    pub fn set_onerror(&mut self, value: jsbind::Any) {
        self.inner.set("onerror", value);
    }
}
impl SFrameTransform {
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }
}
impl SFrameTransform {
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }
}
