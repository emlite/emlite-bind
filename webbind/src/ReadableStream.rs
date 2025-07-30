use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadableStreamGetReaderOptions {
    inner: Any,
}
impl FromVal for ReadableStreamGetReaderOptions {
    fn from_val(v: &Any) -> Self {
        ReadableStreamGetReaderOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ReadableStreamGetReaderOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReadableStreamGetReaderOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ReadableStreamGetReaderOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ReadableStreamGetReaderOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ReadableStreamGetReaderOptions> for Any {
    fn from(s: ReadableStreamGetReaderOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ReadableStreamGetReaderOptions> for Any {
    fn from(s: &ReadableStreamGetReaderOptions) -> Any {
        s.inner.clone()
    }
}

impl ReadableStreamGetReaderOptions {
    pub fn mode(&self) -> ReadableStreamReaderMode {
        self.inner.get("mode").as_::<ReadableStreamReaderMode>()
    }

    pub fn set_mode(&mut self, value: &ReadableStreamReaderMode) {
        self.inner.set("mode", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadableWritablePair {
    inner: Any,
}
impl FromVal for ReadableWritablePair {
    fn from_val(v: &Any) -> Self {
        ReadableWritablePair { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ReadableWritablePair {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReadableWritablePair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ReadableWritablePair {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ReadableWritablePair {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ReadableWritablePair> for Any {
    fn from(s: ReadableWritablePair) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ReadableWritablePair> for Any {
    fn from(s: &ReadableWritablePair) -> Any {
        s.inner.clone()
    }
}

impl ReadableWritablePair {
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }

    pub fn set_readable(&mut self, value: &ReadableStream) {
        self.inner.set("readable", value);
    }
}
impl ReadableWritablePair {
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }

    pub fn set_writable(&mut self, value: &WritableStream) {
        self.inner.set("writable", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StreamPipeOptions {
    inner: Any,
}
impl FromVal for StreamPipeOptions {
    fn from_val(v: &Any) -> Self {
        StreamPipeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for StreamPipeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StreamPipeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for StreamPipeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for StreamPipeOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<StreamPipeOptions> for Any {
    fn from(s: StreamPipeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&StreamPipeOptions> for Any {
    fn from(s: &StreamPipeOptions) -> Any {
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

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
/// The ReadableStream class.
/// [`ReadableStream`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadableStream {
    inner: Any,
}
impl FromVal for ReadableStream {
    fn from_val(v: &Any) -> Self {
        ReadableStream {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ReadableStream {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReadableStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ReadableStream {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ReadableStream {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ReadableStream> for Any {
    fn from(s: ReadableStream) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ReadableStream> for Any {
    fn from(s: &ReadableStream) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ReadableStream);

impl ReadableStream {
    /// The `new ReadableStream(..)` constructor, creating a new ReadableStream instance
    pub fn new0() -> ReadableStream {
        Self {
            inner: Any::global("ReadableStream").new(&[]).as_::<Any>(),
        }
    }

    /// The `new ReadableStream(..)` constructor, creating a new ReadableStream instance
    pub fn new1(underlying_source: &Object) -> ReadableStream {
        Self {
            inner: Any::global("ReadableStream")
                .new(&[underlying_source.into()])
                .as_::<Any>(),
        }
    }

    /// The `new ReadableStream(..)` constructor, creating a new ReadableStream instance
    pub fn new2(underlying_source: &Object, strategy: &Any) -> ReadableStream {
        Self {
            inner: Any::global("ReadableStream")
                .new(&[underlying_source.into(), strategy.into()])
                .as_::<Any>(),
        }
    }
}
impl ReadableStream {
    /// The from method.
    /// [`ReadableStream.from`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/from)
    pub fn from(async_iterable: &Any) -> ReadableStream {
        Any::global("ReadableStream")
            .call("from", &[async_iterable.into()])
            .as_::<ReadableStream>()
    }
}
impl ReadableStream {
    /// Getter of the `locked` attribute.
    /// [`ReadableStream.locked`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/locked)
    pub fn locked(&self) -> bool {
        self.inner.get("locked").as_::<bool>()
    }
}
impl ReadableStream {
    /// The cancel method.
    /// [`ReadableStream.cancel`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/cancel)
    pub fn cancel0(&self) -> Promise<Undefined> {
        self.inner.call("cancel", &[]).as_::<Promise<Undefined>>()
    }
    /// The cancel method.
    /// [`ReadableStream.cancel`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/cancel)
    pub fn cancel1(&self, reason: &Any) -> Promise<Undefined> {
        self.inner
            .call("cancel", &[reason.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl ReadableStream {
    /// The getReader method.
    /// [`ReadableStream.getReader`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/getReader)
    pub fn get_reader0(&self) -> Any {
        self.inner.call("getReader", &[]).as_::<Any>()
    }
    /// The getReader method.
    /// [`ReadableStream.getReader`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/getReader)
    pub fn get_reader1(&self, options: &ReadableStreamGetReaderOptions) -> Any {
        self.inner.call("getReader", &[options.into()]).as_::<Any>()
    }
}
impl ReadableStream {
    /// The pipeThrough method.
    /// [`ReadableStream.pipeThrough`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeThrough)
    pub fn pipe_through0(&self, transform: &ReadableWritablePair) -> ReadableStream {
        self.inner
            .call("pipeThrough", &[transform.into()])
            .as_::<ReadableStream>()
    }
    /// The pipeThrough method.
    /// [`ReadableStream.pipeThrough`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeThrough)
    pub fn pipe_through1(
        &self,
        transform: &ReadableWritablePair,
        options: &StreamPipeOptions,
    ) -> ReadableStream {
        self.inner
            .call("pipeThrough", &[transform.into(), options.into()])
            .as_::<ReadableStream>()
    }
}
impl ReadableStream {
    /// The pipeTo method.
    /// [`ReadableStream.pipeTo`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeTo)
    pub fn pipe_to0(&self, destination: &WritableStream) -> Promise<Undefined> {
        self.inner
            .call("pipeTo", &[destination.into()])
            .as_::<Promise<Undefined>>()
    }
    /// The pipeTo method.
    /// [`ReadableStream.pipeTo`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeTo)
    pub fn pipe_to1(
        &self,
        destination: &WritableStream,
        options: &StreamPipeOptions,
    ) -> Promise<Undefined> {
        self.inner
            .call("pipeTo", &[destination.into(), options.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl ReadableStream {
    /// The tee method.
    /// [`ReadableStream.tee`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/tee)
    pub fn tee(&self) -> TypedArray<ReadableStream> {
        self.inner
            .call("tee", &[])
            .as_::<TypedArray<ReadableStream>>()
    }
}
