use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WritableStreamDefaultWriter {
    inner: emlite::Val,
}
impl FromVal for WritableStreamDefaultWriter {
    fn from_val(v: &emlite::Val) -> Self {
        WritableStreamDefaultWriter {
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
impl core::ops::Deref for WritableStreamDefaultWriter {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WritableStreamDefaultWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WritableStreamDefaultWriter {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WritableStreamDefaultWriter {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WritableStreamDefaultWriter> for emlite::Val {
    fn from(s: WritableStreamDefaultWriter) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(WritableStreamDefaultWriter);

impl WritableStreamDefaultWriter {
    pub fn new(stream: WritableStream) -> WritableStreamDefaultWriter {
        Self {
            inner: emlite::Val::global("WritableStreamDefaultWriter")
                .new(&[stream.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl WritableStreamDefaultWriter {
    pub fn closed(&self) -> jsbind::Promise {
        self.inner.get("closed").as_::<jsbind::Promise>()
    }
}
impl WritableStreamDefaultWriter {
    pub fn desired_size(&self) -> f64 {
        self.inner.get("desiredSize").as_::<f64>()
    }
}
impl WritableStreamDefaultWriter {
    pub fn ready(&self) -> jsbind::Promise {
        self.inner.get("ready").as_::<jsbind::Promise>()
    }
}
impl WritableStreamDefaultWriter {
    pub fn abort0(&self) -> jsbind::Promise {
        self.inner.call("abort", &[]).as_::<jsbind::Promise>()
    }

    pub fn abort1(&self, reason: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("abort", &[reason.into()])
            .as_::<jsbind::Promise>()
    }
}
impl WritableStreamDefaultWriter {
    pub fn close(&self) -> jsbind::Promise {
        self.inner.call("close", &[]).as_::<jsbind::Promise>()
    }
}
impl WritableStreamDefaultWriter {
    pub fn release_lock(&self) -> jsbind::Undefined {
        self.inner
            .call("releaseLock", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl WritableStreamDefaultWriter {
    pub fn write0(&self) -> jsbind::Promise {
        self.inner.call("write", &[]).as_::<jsbind::Promise>()
    }

    pub fn write1(&self, chunk: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("write", &[chunk.into()])
            .as_::<jsbind::Promise>()
    }
}
