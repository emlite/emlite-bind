use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WritableStream {
    inner: emlite::Val,
}
impl FromVal for WritableStream {
    fn from_val(v: &emlite::Val) -> Self {
        WritableStream {
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
impl core::ops::Deref for WritableStream {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WritableStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WritableStream> for emlite::Val {
    fn from(s: WritableStream) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WritableStream {
    pub fn new0() -> WritableStream {
        Self {
            inner: emlite::Val::global("WritableStream")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(underlying_sink: jsbind::Object) -> WritableStream {
        Self {
            inner: emlite::Val::global("WritableStream")
                .new(&[underlying_sink.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(underlying_sink: jsbind::Object, strategy: jsbind::Any) -> WritableStream {
        Self {
            inner: emlite::Val::global("WritableStream")
                .new(&[underlying_sink.into(), strategy.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl WritableStream {
    pub fn locked(&self) -> bool {
        self.inner.get("locked").as_::<bool>()
    }
}
impl WritableStream {
    pub fn abort0(&self) -> jsbind::Promise {
        self.inner.call("abort", &[]).as_::<jsbind::Promise>()
    }

    pub fn abort1(&self, reason: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("abort", &[reason.into()])
            .as_::<jsbind::Promise>()
    }
}
impl WritableStream {
    pub fn close(&self) -> jsbind::Promise {
        self.inner.call("close", &[]).as_::<jsbind::Promise>()
    }
}
impl WritableStream {
    pub fn get_writer(&self) -> WritableStreamDefaultWriter {
        self.inner
            .call("getWriter", &[])
            .as_::<WritableStreamDefaultWriter>()
    }
}
