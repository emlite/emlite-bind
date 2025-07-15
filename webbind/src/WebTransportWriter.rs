use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for WebTransportWriter {
    type Target = WritableStreamDefaultWriter;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransportWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WebTransportWriter {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WebTransportWriter {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WebTransportWriter> for emlite::Val {
    fn from(s: WebTransportWriter) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&WebTransportWriter> for emlite::Val {
    fn from(s: &WebTransportWriter) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WebTransportWriter);

impl WebTransportWriter {
    pub fn atomic_write0(&self) -> Promise {
        self.inner.call("atomicWrite", &[]).as_::<Promise>()
    }

    pub fn atomic_write1(&self, chunk: &Any) -> Promise {
        self.inner
            .call("atomicWrite", &[chunk.into()])
            .as_::<Promise>()
    }
}
impl WebTransportWriter {
    pub fn commit(&self) -> Undefined {
        self.inner.call("commit", &[]).as_::<Undefined>()
    }
}
