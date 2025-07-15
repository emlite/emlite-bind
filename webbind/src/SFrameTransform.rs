use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SFrameTransform {
    inner: EventTarget,
}
impl FromVal for SFrameTransform {
    fn from_val(v: &emlite::Val) -> Self {
        SFrameTransform { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SFrameTransform {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SFrameTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SFrameTransform {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SFrameTransform {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<SFrameTransform> for emlite::Val {
    fn from(s: SFrameTransform) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SFrameTransform);



impl SFrameTransform {
    pub fn new0() -> SFrameTransform {
        Self {
            inner: emlite::Val::global("SFrameTransform").new(&[]).as_::<EventTarget>(),
        }
    }

    pub fn new1(options: Any) -> SFrameTransform {
        Self {
            inner: emlite::Val::global("SFrameTransform").new(&[options.into()]).as_::<EventTarget>(),
        }
    }

}
impl SFrameTransform {
    pub fn set_encryption_key0(&self, key: CryptoKey) -> Promise {
        self.inner.call("setEncryptionKey", &[key.into(), ]).as_::<Promise>()
    }

    pub fn set_encryption_key1(&self, key: CryptoKey, key_id: Any) -> Promise {
        self.inner.call("setEncryptionKey", &[key.into(), key_id.into(), ]).as_::<Promise>()
    }

}
impl SFrameTransform {
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    pub fn set_onerror(&mut self, value: Any) {
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
