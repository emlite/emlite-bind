use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for WritableStream {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WritableStream {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
jsbind::utils::impl_dyn_cast!(WritableStream);

impl WritableStream {
    pub fn new0() -> WritableStream {
        Self {
            inner: emlite::Val::global("WritableStream")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(underlying_sink: Object) -> WritableStream {
        Self {
            inner: emlite::Val::global("WritableStream")
                .new(&[underlying_sink.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(underlying_sink: Object, strategy: Any) -> WritableStream {
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
    pub fn abort0(&self) -> Promise {
        self.inner.call("abort", &[]).as_::<Promise>()
    }

    pub fn abort1(&self, reason: Any) -> Promise {
        self.inner.call("abort", &[reason.into()]).as_::<Promise>()
    }
}
impl WritableStream {
    pub fn close(&self) -> Promise {
        self.inner.call("close", &[]).as_::<Promise>()
    }
}
impl WritableStream {
    pub fn get_writer(&self) -> WritableStreamDefaultWriter {
        self.inner
            .call("getWriter", &[])
            .as_::<WritableStreamDefaultWriter>()
    }
}
