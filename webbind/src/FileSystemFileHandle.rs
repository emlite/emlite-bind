use super::*;

#[derive(Clone, Debug)]
pub struct FileSystemCreateWritableOptions {
    inner: emlite::Val,
}
impl FromVal for FileSystemCreateWritableOptions {
    fn from_val(v: &emlite::Val) -> Self {
        FileSystemCreateWritableOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for FileSystemCreateWritableOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FileSystemCreateWritableOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FileSystemCreateWritableOptions> for emlite::Val {
    fn from(s: FileSystemCreateWritableOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FileSystemCreateWritableOptions {
    pub fn keep_existing_data(&self) -> bool {
        self.inner.get("keepExistingData").as_::<bool>()
    }

    pub fn set_keep_existing_data(&mut self, value: bool) {
        self.inner.set("keepExistingData", value);
    }
}
#[derive(Clone, Debug)]
pub struct FileSystemFileHandle {
    inner: FileSystemHandle,
}
impl FromVal for FileSystemFileHandle {
    fn from_val(v: &emlite::Val) -> Self {
        FileSystemFileHandle {
            inner: FileSystemHandle::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for FileSystemFileHandle {
    type Target = FileSystemHandle;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FileSystemFileHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FileSystemFileHandle> for emlite::Val {
    fn from(s: FileSystemFileHandle) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FileSystemFileHandle {
    pub fn get_file(&self) -> jsbind::Promise {
        self.inner.call("getFile", &[]).as_::<jsbind::Promise>()
    }
}
impl FileSystemFileHandle {
    pub fn create_writable0(&self) -> jsbind::Promise {
        self.inner
            .call("createWritable", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn create_writable1(&self, options: FileSystemCreateWritableOptions) -> jsbind::Promise {
        self.inner
            .call("createWritable", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl FileSystemFileHandle {
    pub fn create_sync_access_handle(&self) -> jsbind::Promise {
        self.inner
            .call("createSyncAccessHandle", &[])
            .as_::<jsbind::Promise>()
    }
}
