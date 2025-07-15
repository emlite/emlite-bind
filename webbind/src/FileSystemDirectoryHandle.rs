use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemGetFileOptions {
    inner: emlite::Val,
}
impl FromVal for FileSystemGetFileOptions {
    fn from_val(v: &emlite::Val) -> Self {
        FileSystemGetFileOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FileSystemGetFileOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemGetFileOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FileSystemGetFileOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FileSystemGetFileOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<FileSystemGetFileOptions> for emlite::Val {
    fn from(s: FileSystemGetFileOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FileSystemGetFileOptions {
    pub fn create(&self) -> bool {
        self.inner.get("create").as_::<bool>()
    }

    pub fn set_create(&mut self, value: bool) {
        self.inner.set("create", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemGetDirectoryOptions {
    inner: emlite::Val,
}
impl FromVal for FileSystemGetDirectoryOptions {
    fn from_val(v: &emlite::Val) -> Self {
        FileSystemGetDirectoryOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FileSystemGetDirectoryOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemGetDirectoryOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FileSystemGetDirectoryOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FileSystemGetDirectoryOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<FileSystemGetDirectoryOptions> for emlite::Val {
    fn from(s: FileSystemGetDirectoryOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FileSystemGetDirectoryOptions {
    pub fn create(&self) -> bool {
        self.inner.get("create").as_::<bool>()
    }

    pub fn set_create(&mut self, value: bool) {
        self.inner.set("create", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemRemoveOptions {
    inner: emlite::Val,
}
impl FromVal for FileSystemRemoveOptions {
    fn from_val(v: &emlite::Val) -> Self {
        FileSystemRemoveOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FileSystemRemoveOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemRemoveOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FileSystemRemoveOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FileSystemRemoveOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<FileSystemRemoveOptions> for emlite::Val {
    fn from(s: FileSystemRemoveOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FileSystemRemoveOptions {
    pub fn recursive(&self) -> bool {
        self.inner.get("recursive").as_::<bool>()
    }

    pub fn set_recursive(&mut self, value: bool) {
        self.inner.set("recursive", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemDirectoryHandle {
    inner: FileSystemHandle,
}
impl FromVal for FileSystemDirectoryHandle {
    fn from_val(v: &emlite::Val) -> Self {
        FileSystemDirectoryHandle {
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
impl core::ops::Deref for FileSystemDirectoryHandle {
    type Target = FileSystemHandle;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemDirectoryHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FileSystemDirectoryHandle {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FileSystemDirectoryHandle {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<FileSystemDirectoryHandle> for emlite::Val {
    fn from(s: FileSystemDirectoryHandle) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(FileSystemDirectoryHandle);

impl FileSystemDirectoryHandle {
    pub fn get_file_handle0(&self, name: USVString) -> Promise {
        self.inner
            .call("getFileHandle", &[name.into()])
            .as_::<Promise>()
    }

    pub fn get_file_handle1(&self, name: USVString, options: FileSystemGetFileOptions) -> Promise {
        self.inner
            .call("getFileHandle", &[name.into(), options.into()])
            .as_::<Promise>()
    }
}
impl FileSystemDirectoryHandle {
    pub fn get_directory_handle0(&self, name: USVString) -> Promise {
        self.inner
            .call("getDirectoryHandle", &[name.into()])
            .as_::<Promise>()
    }

    pub fn get_directory_handle1(
        &self,
        name: USVString,
        options: FileSystemGetDirectoryOptions,
    ) -> Promise {
        self.inner
            .call("getDirectoryHandle", &[name.into(), options.into()])
            .as_::<Promise>()
    }
}
impl FileSystemDirectoryHandle {
    pub fn remove_entry0(&self, name: USVString) -> Promise {
        self.inner
            .call("removeEntry", &[name.into()])
            .as_::<Promise>()
    }

    pub fn remove_entry1(&self, name: USVString, options: FileSystemRemoveOptions) -> Promise {
        self.inner
            .call("removeEntry", &[name.into(), options.into()])
            .as_::<Promise>()
    }
}
impl FileSystemDirectoryHandle {
    pub fn resolve(&self, possible_descendant: FileSystemHandle) -> Promise {
        self.inner
            .call("resolve", &[possible_descendant.into()])
            .as_::<Promise>()
    }
}
