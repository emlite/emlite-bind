use super::*;

/// The DataTransferItem class.
/// [`DataTransferItem`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DataTransferItem {
    inner: Any,
}
impl FromVal for DataTransferItem {
    fn from_val(v: &Any) -> Self {
        DataTransferItem {
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
impl core::ops::Deref for DataTransferItem {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DataTransferItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DataTransferItem {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DataTransferItem {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DataTransferItem> for Any {
    fn from(s: DataTransferItem) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DataTransferItem> for Any {
    fn from(s: &DataTransferItem) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DataTransferItem);

impl DataTransferItem {
    /// Getter of the `kind` attribute.
    /// [`DataTransferItem.kind`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/kind)
    pub fn kind(&self) -> JsString {
        self.inner.get("kind").as_::<JsString>()
    }
}
impl DataTransferItem {
    /// Getter of the `type` attribute.
    /// [`DataTransferItem.type`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/type)
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }
}
impl DataTransferItem {
    /// The getAsString method.
    /// [`DataTransferItem.getAsString`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/getAsString)
    pub fn get_as_string(&self, callback: &Function) -> Undefined {
        self.inner
            .call("getAsString", &[callback.into()])
            .as_::<Undefined>()
    }
}
impl DataTransferItem {
    /// The getAsFile method.
    /// [`DataTransferItem.getAsFile`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/getAsFile)
    pub fn get_as_file(&self) -> File {
        self.inner.call("getAsFile", &[]).as_::<File>()
    }
}
impl DataTransferItem {
    /// The webkitGetAsEntry method.
    /// [`DataTransferItem.webkitGetAsEntry`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/webkitGetAsEntry)
    pub fn webkit_get_as_entry(&self) -> FileSystemEntry {
        self.inner
            .call("webkitGetAsEntry", &[])
            .as_::<FileSystemEntry>()
    }
}
impl DataTransferItem {
    /// The getAsFileSystemHandle method.
    /// [`DataTransferItem.getAsFileSystemHandle`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/getAsFileSystemHandle)
    pub fn get_as_file_system_handle(&self) -> Promise<FileSystemHandle> {
        self.inner
            .call("getAsFileSystemHandle", &[])
            .as_::<Promise<FileSystemHandle>>()
    }
}
