use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DataTransferItem {
    inner: emlite::Val,
}
impl FromVal for DataTransferItem {
    fn from_val(v: &emlite::Val) -> Self {
        DataTransferItem {
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
impl core::ops::Deref for DataTransferItem {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DataTransferItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DataTransferItem {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DataTransferItem {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DataTransferItem> for emlite::Val {
    fn from(s: DataTransferItem) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&DataTransferItem> for emlite::Val {
    fn from(s: &DataTransferItem) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DataTransferItem);

impl DataTransferItem {
    pub fn kind(&self) -> String {
        self.inner.get("kind").as_::<String>()
    }
}
impl DataTransferItem {
    pub fn type_(&self) -> String {
        self.inner.get("type").as_::<String>()
    }
}
impl DataTransferItem {
    pub fn get_as_string(&self, callback: &Any) -> Undefined {
        self.inner
            .call("getAsString", &[callback.into()])
            .as_::<Undefined>()
    }
}
impl DataTransferItem {
    pub fn get_as_file(&self) -> File {
        self.inner.call("getAsFile", &[]).as_::<File>()
    }
}
impl DataTransferItem {
    pub fn webkit_get_as_entry(&self) -> FileSystemEntry {
        self.inner
            .call("webkitGetAsEntry", &[])
            .as_::<FileSystemEntry>()
    }
}
impl DataTransferItem {
    pub fn get_as_file_system_handle(&self) -> Promise {
        self.inner
            .call("getAsFileSystemHandle", &[])
            .as_::<Promise>()
    }
}
