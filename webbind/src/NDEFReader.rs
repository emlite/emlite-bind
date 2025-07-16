use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NDEFScanOptions {
    inner: Any,
}
impl FromVal for NDEFScanOptions {
    fn from_val(v: &Any) -> Self {
        NDEFScanOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NDEFScanOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NDEFScanOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NDEFScanOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NDEFScanOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NDEFScanOptions> for Any {
    fn from(s: NDEFScanOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NDEFScanOptions> for Any {
    fn from(s: &NDEFScanOptions) -> Any {
        s.inner.clone()
    }
}

impl NDEFScanOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NDEFWriteOptions {
    inner: Any,
}
impl FromVal for NDEFWriteOptions {
    fn from_val(v: &Any) -> Self {
        NDEFWriteOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NDEFWriteOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NDEFWriteOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NDEFWriteOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NDEFWriteOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NDEFWriteOptions> for Any {
    fn from(s: NDEFWriteOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NDEFWriteOptions> for Any {
    fn from(s: &NDEFWriteOptions) -> Any {
        s.inner.clone()
    }
}

impl NDEFWriteOptions {
    pub fn overwrite(&self) -> bool {
        self.inner.get("overwrite").as_::<bool>()
    }

    pub fn set_overwrite(&mut self, value: bool) {
        self.inner.set("overwrite", value);
    }
}
impl NDEFWriteOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NDEFMakeReadOnlyOptions {
    inner: Any,
}
impl FromVal for NDEFMakeReadOnlyOptions {
    fn from_val(v: &Any) -> Self {
        NDEFMakeReadOnlyOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NDEFMakeReadOnlyOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NDEFMakeReadOnlyOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NDEFMakeReadOnlyOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NDEFMakeReadOnlyOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NDEFMakeReadOnlyOptions> for Any {
    fn from(s: NDEFMakeReadOnlyOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NDEFMakeReadOnlyOptions> for Any {
    fn from(s: &NDEFMakeReadOnlyOptions) -> Any {
        s.inner.clone()
    }
}

impl NDEFMakeReadOnlyOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
/// The NDEFReader class.
/// [`NDEFReader`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFReader)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NDEFReader {
    inner: EventTarget,
}
impl FromVal for NDEFReader {
    fn from_val(v: &Any) -> Self {
        NDEFReader {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NDEFReader {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NDEFReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NDEFReader {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NDEFReader {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NDEFReader> for Any {
    fn from(s: NDEFReader) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NDEFReader> for Any {
    fn from(s: &NDEFReader) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(NDEFReader);

impl NDEFReader {
    /// The `new NDEFReader(..)` constructor, creating a new NDEFReader instance
    pub fn new() -> NDEFReader {
        Self {
            inner: Any::global("NDEFReader").new(&[]).as_::<EventTarget>(),
        }
    }
}
impl NDEFReader {
    /// Getter of the `onreading` attribute.
    /// [`NDEFReader.onreading`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFReader/onreading)
    pub fn onreading(&self) -> Any {
        self.inner.get("onreading").as_::<Any>()
    }

    /// Setter of the `onreading` attribute.
    /// [`NDEFReader.onreading`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFReader/onreading)
    pub fn set_onreading(&mut self, value: &Any) {
        self.inner.set("onreading", value);
    }
}
impl NDEFReader {
    /// Getter of the `onreadingerror` attribute.
    /// [`NDEFReader.onreadingerror`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFReader/onreadingerror)
    pub fn onreadingerror(&self) -> Any {
        self.inner.get("onreadingerror").as_::<Any>()
    }

    /// Setter of the `onreadingerror` attribute.
    /// [`NDEFReader.onreadingerror`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFReader/onreadingerror)
    pub fn set_onreadingerror(&mut self, value: &Any) {
        self.inner.set("onreadingerror", value);
    }
}
impl NDEFReader {
    /// The scan method.
    /// [`NDEFReader.scan`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFReader/scan)
    pub fn scan0(&self) -> Promise {
        self.inner.call("scan", &[]).as_::<Promise>()
    }
    /// The scan method.
    /// [`NDEFReader.scan`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFReader/scan)
    pub fn scan1(&self, options: &NDEFScanOptions) -> Promise {
        self.inner.call("scan", &[options.into()]).as_::<Promise>()
    }
}
impl NDEFReader {
    /// The write method.
    /// [`NDEFReader.write`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFReader/write)
    pub fn write0(&self, message: &Any) -> Promise {
        self.inner.call("write", &[message.into()]).as_::<Promise>()
    }
    /// The write method.
    /// [`NDEFReader.write`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFReader/write)
    pub fn write1(&self, message: &Any, options: &NDEFWriteOptions) -> Promise {
        self.inner
            .call("write", &[message.into(), options.into()])
            .as_::<Promise>()
    }
}
impl NDEFReader {
    /// The makeReadOnly method.
    /// [`NDEFReader.makeReadOnly`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFReader/makeReadOnly)
    pub fn make_read_only0(&self) -> Promise {
        self.inner.call("makeReadOnly", &[]).as_::<Promise>()
    }
    /// The makeReadOnly method.
    /// [`NDEFReader.makeReadOnly`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFReader/makeReadOnly)
    pub fn make_read_only1(&self, options: &NDEFMakeReadOnlyOptions) -> Promise {
        self.inner
            .call("makeReadOnly", &[options.into()])
            .as_::<Promise>()
    }
}
