use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TransformStream {
    inner: emlite::Val,
}
impl FromVal for TransformStream {
    fn from_val(v: &emlite::Val) -> Self {
        TransformStream {
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
impl core::ops::Deref for TransformStream {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TransformStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for TransformStream {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TransformStream {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<TransformStream> for emlite::Val {
    fn from(s: TransformStream) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&TransformStream> for emlite::Val {
    fn from(s: &TransformStream) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TransformStream);

impl TransformStream {
    pub fn new0() -> TransformStream {
        Self {
            inner: emlite::Val::global("TransformStream")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(transformer: &Object) -> TransformStream {
        Self {
            inner: emlite::Val::global("TransformStream")
                .new(&[transformer.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(transformer: &Object, writable_strategy: &Any) -> TransformStream {
        Self {
            inner: emlite::Val::global("TransformStream")
                .new(&[transformer.into(), writable_strategy.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new3(
        transformer: &Object,
        writable_strategy: &Any,
        readable_strategy: &Any,
    ) -> TransformStream {
        Self {
            inner: emlite::Val::global("TransformStream")
                .new(&[
                    transformer.into(),
                    writable_strategy.into(),
                    readable_strategy.into(),
                ])
                .as_::<emlite::Val>(),
        }
    }
}
impl TransformStream {
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }
}
impl TransformStream {
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }
}
