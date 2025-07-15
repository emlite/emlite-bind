use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadableStreamGetReaderOptions {
    inner: emlite::Val,
}
impl FromVal for ReadableStreamGetReaderOptions {
    fn from_val(v: &emlite::Val) -> Self {
        ReadableStreamGetReaderOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ReadableStreamGetReaderOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReadableStreamGetReaderOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ReadableStreamGetReaderOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ReadableStreamGetReaderOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ReadableStreamGetReaderOptions> for emlite::Val {
    fn from(s: ReadableStreamGetReaderOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ReadableStreamGetReaderOptions> for emlite::Val {
    fn from(s: &ReadableStreamGetReaderOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl ReadableStreamGetReaderOptions {
    pub fn mode(&self) -> ReadableStreamReaderMode {
        self.inner.get("mode").as_::<ReadableStreamReaderMode>()
    }

    pub fn set_mode(&mut self, value: ReadableStreamReaderMode) {
        self.inner.set("mode", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadableWritablePair {
    inner: emlite::Val,
}
impl FromVal for ReadableWritablePair {
    fn from_val(v: &emlite::Val) -> Self {
        ReadableWritablePair { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ReadableWritablePair {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReadableWritablePair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ReadableWritablePair {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ReadableWritablePair {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ReadableWritablePair> for emlite::Val {
    fn from(s: ReadableWritablePair) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ReadableWritablePair> for emlite::Val {
    fn from(s: &ReadableWritablePair) -> emlite::Val {
        s.inner.clone()
    }
}

impl ReadableWritablePair {
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }

    pub fn set_readable(&mut self, value: ReadableStream) {
        self.inner.set("readable", value);
    }
}
impl ReadableWritablePair {
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }

    pub fn set_writable(&mut self, value: WritableStream) {
        self.inner.set("writable", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StreamPipeOptions {
    inner: emlite::Val,
}
impl FromVal for StreamPipeOptions {
    fn from_val(v: &emlite::Val) -> Self {
        StreamPipeOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for StreamPipeOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StreamPipeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for StreamPipeOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for StreamPipeOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<StreamPipeOptions> for emlite::Val {
    fn from(s: StreamPipeOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&StreamPipeOptions> for emlite::Val {
    fn from(s: &StreamPipeOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl StreamPipeOptions {
    pub fn prevent_close(&self) -> bool {
        self.inner.get("preventClose").as_::<bool>()
    }

    pub fn set_prevent_close(&mut self, value: bool) {
        self.inner.set("preventClose", value);
    }
}
impl StreamPipeOptions {
    pub fn prevent_abort(&self) -> bool {
        self.inner.get("preventAbort").as_::<bool>()
    }

    pub fn set_prevent_abort(&mut self, value: bool) {
        self.inner.set("preventAbort", value);
    }
}
impl StreamPipeOptions {
    pub fn prevent_cancel(&self) -> bool {
        self.inner.get("preventCancel").as_::<bool>()
    }

    pub fn set_prevent_cancel(&mut self, value: bool) {
        self.inner.set("preventCancel", value);
    }
}
impl StreamPipeOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: AbortSignal) {
        self.inner.set("signal", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadableStream {
    inner: emlite::Val,
}
impl FromVal for ReadableStream {
    fn from_val(v: &emlite::Val) -> Self {
        ReadableStream {
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
impl core::ops::Deref for ReadableStream {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReadableStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ReadableStream {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ReadableStream {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ReadableStream> for emlite::Val {
    fn from(s: ReadableStream) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ReadableStream> for emlite::Val {
    fn from(s: &ReadableStream) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ReadableStream);

impl ReadableStream {
    pub fn new0() -> ReadableStream {
        Self {
            inner: emlite::Val::global("ReadableStream")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(underlying_source: Object) -> ReadableStream {
        Self {
            inner: emlite::Val::global("ReadableStream")
                .new(&[underlying_source.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(underlying_source: Object, strategy: Any) -> ReadableStream {
        Self {
            inner: emlite::Val::global("ReadableStream")
                .new(&[underlying_source.into(), strategy.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl ReadableStream {
    pub fn from(async_iterable: Any) -> ReadableStream {
        emlite::Val::global("ReadableStream")
            .call("from", &[async_iterable.into()])
            .as_::<ReadableStream>()
    }
}
impl ReadableStream {
    pub fn locked(&self) -> bool {
        self.inner.get("locked").as_::<bool>()
    }
}
impl ReadableStream {
    pub fn cancel0(&self) -> Promise {
        self.inner.call("cancel", &[]).as_::<Promise>()
    }

    pub fn cancel1(&self, reason: Any) -> Promise {
        self.inner.call("cancel", &[reason.into()]).as_::<Promise>()
    }
}
impl ReadableStream {
    pub fn get_reader0(&self) -> Any {
        self.inner.call("getReader", &[]).as_::<Any>()
    }

    pub fn get_reader1(&self, options: ReadableStreamGetReaderOptions) -> Any {
        self.inner.call("getReader", &[options.into()]).as_::<Any>()
    }
}
impl ReadableStream {
    pub fn pipe_through0(&self, transform: ReadableWritablePair) -> ReadableStream {
        self.inner
            .call("pipeThrough", &[transform.into()])
            .as_::<ReadableStream>()
    }

    pub fn pipe_through1(
        &self,
        transform: ReadableWritablePair,
        options: StreamPipeOptions,
    ) -> ReadableStream {
        self.inner
            .call("pipeThrough", &[transform.into(), options.into()])
            .as_::<ReadableStream>()
    }
}
impl ReadableStream {
    pub fn pipe_to0(&self, destination: WritableStream) -> Promise {
        self.inner
            .call("pipeTo", &[destination.into()])
            .as_::<Promise>()
    }

    pub fn pipe_to1(&self, destination: WritableStream, options: StreamPipeOptions) -> Promise {
        self.inner
            .call("pipeTo", &[destination.into(), options.into()])
            .as_::<Promise>()
    }
}
impl ReadableStream {
    pub fn tee(&self) -> Sequence<ReadableStream> {
        self.inner
            .call("tee", &[])
            .as_::<Sequence<ReadableStream>>()
    }
}
