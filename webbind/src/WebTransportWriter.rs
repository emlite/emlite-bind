use super::*;

#[derive(Clone, Debug)]
pub struct WebTransportWriter {
    inner: WritableStreamDefaultWriter,
}
impl FromVal for WebTransportWriter {
    fn from_val(v: &emlite::Val) -> Self {
        WebTransportWriter {
            inner: WritableStreamDefaultWriter::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for WebTransportWriter {
    type Target = WritableStreamDefaultWriter;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WebTransportWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WebTransportWriter> for emlite::Val {
    fn from(s: WebTransportWriter) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WebTransportWriter {
    pub fn atomic_write0(&self) -> jsbind::Promise {
        self.inner.call("atomicWrite", &[]).as_::<jsbind::Promise>()
    }

    pub fn atomic_write1(&self, chunk: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("atomicWrite", &[chunk.into()])
            .as_::<jsbind::Promise>()
    }
}
impl WebTransportWriter {
    pub fn commit(&self) -> jsbind::Undefined {
        self.inner.call("commit", &[]).as_::<jsbind::Undefined>()
    }
}
