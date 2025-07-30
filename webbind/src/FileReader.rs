use super::*;

/// The FileReader class.
/// [`FileReader`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileReader {
    inner: EventTarget,
}
impl FromVal for FileReader {
    fn from_val(v: &Any) -> Self {
        FileReader {
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
impl core::ops::Deref for FileReader {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FileReader {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FileReader {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FileReader> for Any {
    fn from(s: FileReader) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FileReader> for Any {
    fn from(s: &FileReader) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FileReader);

impl FileReader {
    /// The `new FileReader(..)` constructor, creating a new FileReader instance
    pub fn new() -> FileReader {
        Self {
            inner: Any::global("FileReader").new(&[]).as_::<EventTarget>(),
        }
    }
}
impl FileReader {
    /// The readAsArrayBuffer method.
    /// [`FileReader.readAsArrayBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsArrayBuffer)
    pub fn read_as_array_buffer(&self, blob: &Blob) -> Undefined {
        self.inner
            .call("readAsArrayBuffer", &[blob.into()])
            .as_::<Undefined>()
    }
}
impl FileReader {
    /// The readAsBinaryString method.
    /// [`FileReader.readAsBinaryString`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsBinaryString)
    pub fn read_as_binary_string(&self, blob: &Blob) -> Undefined {
        self.inner
            .call("readAsBinaryString", &[blob.into()])
            .as_::<Undefined>()
    }
}
impl FileReader {
    /// The readAsText method.
    /// [`FileReader.readAsText`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsText)
    pub fn read_as_text0(&self, blob: &Blob) -> Undefined {
        self.inner
            .call("readAsText", &[blob.into()])
            .as_::<Undefined>()
    }
    /// The readAsText method.
    /// [`FileReader.readAsText`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsText)
    pub fn read_as_text1(&self, blob: &Blob, encoding: &JsString) -> Undefined {
        self.inner
            .call("readAsText", &[blob.into(), encoding.into()])
            .as_::<Undefined>()
    }
}
impl FileReader {
    /// The readAsDataURL method.
    /// [`FileReader.readAsDataURL`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsDataURL)
    pub fn read_as_data_url(&self, blob: &Blob) -> Undefined {
        self.inner
            .call("readAsDataURL", &[blob.into()])
            .as_::<Undefined>()
    }
}
impl FileReader {
    /// The abort method.
    /// [`FileReader.abort`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/abort)
    pub fn abort(&self) -> Undefined {
        self.inner.call("abort", &[]).as_::<Undefined>()
    }
}
impl FileReader {
    /// Getter of the `readyState` attribute.
    /// [`FileReader.readyState`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readyState)
    pub fn ready_state(&self) -> u16 {
        self.inner.get("readyState").as_::<u16>()
    }
}
impl FileReader {
    /// Getter of the `result` attribute.
    /// [`FileReader.result`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/result)
    pub fn result(&self) -> Any {
        self.inner.get("result").as_::<Any>()
    }
}
impl FileReader {
    /// Getter of the `error` attribute.
    /// [`FileReader.error`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/error)
    pub fn error(&self) -> DOMException {
        self.inner.get("error").as_::<DOMException>()
    }
}
impl FileReader {
    /// Getter of the `onloadstart` attribute.
    /// [`FileReader.onloadstart`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadstart)
    pub fn onloadstart(&self) -> Any {
        self.inner.get("onloadstart").as_::<Any>()
    }

    /// Setter of the `onloadstart` attribute.
    /// [`FileReader.onloadstart`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadstart)
    pub fn set_onloadstart(&mut self, value: &Any) {
        self.inner.set("onloadstart", value);
    }
}
impl FileReader {
    /// Getter of the `onprogress` attribute.
    /// [`FileReader.onprogress`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onprogress)
    pub fn onprogress(&self) -> Any {
        self.inner.get("onprogress").as_::<Any>()
    }

    /// Setter of the `onprogress` attribute.
    /// [`FileReader.onprogress`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onprogress)
    pub fn set_onprogress(&mut self, value: &Any) {
        self.inner.set("onprogress", value);
    }
}
impl FileReader {
    /// Getter of the `onload` attribute.
    /// [`FileReader.onload`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onload)
    pub fn onload(&self) -> Any {
        self.inner.get("onload").as_::<Any>()
    }

    /// Setter of the `onload` attribute.
    /// [`FileReader.onload`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onload)
    pub fn set_onload(&mut self, value: &Any) {
        self.inner.set("onload", value);
    }
}
impl FileReader {
    /// Getter of the `onabort` attribute.
    /// [`FileReader.onabort`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onabort)
    pub fn onabort(&self) -> Any {
        self.inner.get("onabort").as_::<Any>()
    }

    /// Setter of the `onabort` attribute.
    /// [`FileReader.onabort`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onabort)
    pub fn set_onabort(&mut self, value: &Any) {
        self.inner.set("onabort", value);
    }
}
impl FileReader {
    /// Getter of the `onerror` attribute.
    /// [`FileReader.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`FileReader.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
impl FileReader {
    /// Getter of the `onloadend` attribute.
    /// [`FileReader.onloadend`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadend)
    pub fn onloadend(&self) -> Any {
        self.inner.get("onloadend").as_::<Any>()
    }

    /// Setter of the `onloadend` attribute.
    /// [`FileReader.onloadend`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadend)
    pub fn set_onloadend(&mut self, value: &Any) {
        self.inner.set("onloadend", value);
    }
}
