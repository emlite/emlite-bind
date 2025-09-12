use super::*;

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
    /// The `new NDEFReader(..)` constructor, creating a new NDEFReader instance
    pub fn new() -> NDEFReader {
        Self {
            inner: Any::global("NDEFReader").new(&[]).as_::<EventTarget>(),
        }
    }
}
impl NDEFReader {
    /// The scan method.
    /// [`NDEFReader.scan`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFReader/scan)
    pub fn scan0(&self) -> Promise<Undefined> {
        self.inner.call("scan", &[]).as_::<Promise<Undefined>>()
    }
    /// The scan method.
    /// [`NDEFReader.scan`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFReader/scan)
    pub fn scan1(&self, options: &NDEFScanOptions) -> Promise<Undefined> {
        self.inner
            .call("scan", &[options.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl NDEFReader {
    /// The write method.
    /// [`NDEFReader.write`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFReader/write)
    pub fn write0(&self, message: &Any) -> Promise<Undefined> {
        self.inner
            .call("write", &[message.into()])
            .as_::<Promise<Undefined>>()
    }
    /// The write method.
    /// [`NDEFReader.write`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFReader/write)
    pub fn write1(&self, message: &Any, options: &NDEFWriteOptions) -> Promise<Undefined> {
        self.inner
            .call("write", &[message.into(), options.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl NDEFReader {
    /// The makeReadOnly method.
    /// [`NDEFReader.makeReadOnly`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFReader/makeReadOnly)
    pub fn make_read_only0(&self) -> Promise<Undefined> {
        self.inner
            .call("makeReadOnly", &[])
            .as_::<Promise<Undefined>>()
    }
    /// The makeReadOnly method.
    /// [`NDEFReader.makeReadOnly`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFReader/makeReadOnly)
    pub fn make_read_only1(&self, options: &NDEFMakeReadOnlyOptions) -> Promise<Undefined> {
        self.inner
            .call("makeReadOnly", &[options.into()])
            .as_::<Promise<Undefined>>()
    }
}
