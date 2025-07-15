use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemReadWriteOptions {
    inner: emlite::Val,
}
impl FromVal for FileSystemReadWriteOptions {
    fn from_val(v: &emlite::Val) -> Self {
        FileSystemReadWriteOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FileSystemReadWriteOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemReadWriteOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FileSystemReadWriteOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FileSystemReadWriteOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<FileSystemReadWriteOptions> for emlite::Val {
    fn from(s: FileSystemReadWriteOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&FileSystemReadWriteOptions> for emlite::Val {
    fn from(s: &FileSystemReadWriteOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl FileSystemReadWriteOptions {
    pub fn at(&self) -> u64 {
        self.inner.get("at").as_::<u64>()
    }

    pub fn set_at(&mut self, value: u64) {
        self.inner.set("at", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemSyncAccessHandle {
    inner: emlite::Val,
}
impl FromVal for FileSystemSyncAccessHandle {
    fn from_val(v: &emlite::Val) -> Self {
        FileSystemSyncAccessHandle {
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
impl core::ops::Deref for FileSystemSyncAccessHandle {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemSyncAccessHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FileSystemSyncAccessHandle {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FileSystemSyncAccessHandle {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<FileSystemSyncAccessHandle> for emlite::Val {
    fn from(s: FileSystemSyncAccessHandle) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&FileSystemSyncAccessHandle> for emlite::Val {
    fn from(s: &FileSystemSyncAccessHandle) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FileSystemSyncAccessHandle);

impl FileSystemSyncAccessHandle {
    pub fn read0(&self, buffer: &Any) -> u64 {
        self.inner.call("read", &[buffer.into()]).as_::<u64>()
    }

    pub fn read1(&self, buffer: &Any, options: &FileSystemReadWriteOptions) -> u64 {
        self.inner
            .call("read", &[buffer.into(), options.into()])
            .as_::<u64>()
    }
}
impl FileSystemSyncAccessHandle {
    pub fn write0(&self, buffer: &Any) -> u64 {
        self.inner.call("write", &[buffer.into()]).as_::<u64>()
    }

    pub fn write1(&self, buffer: &Any, options: &FileSystemReadWriteOptions) -> u64 {
        self.inner
            .call("write", &[buffer.into(), options.into()])
            .as_::<u64>()
    }
}
impl FileSystemSyncAccessHandle {
    pub fn truncate(&self, new_size: u64) -> Undefined {
        self.inner
            .call("truncate", &[new_size.into()])
            .as_::<Undefined>()
    }
}
impl FileSystemSyncAccessHandle {
    pub fn get_size(&self) -> u64 {
        self.inner.call("getSize", &[]).as_::<u64>()
    }
}
impl FileSystemSyncAccessHandle {
    pub fn flush(&self) -> Undefined {
        self.inner.call("flush", &[]).as_::<Undefined>()
    }
}
impl FileSystemSyncAccessHandle {
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
