use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemEntry {
    inner: emlite::Val,
}
impl FromVal for FileSystemEntry {
    fn from_val(v: &emlite::Val) -> Self {
        FileSystemEntry {
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
impl core::ops::Deref for FileSystemEntry {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FileSystemEntry {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FileSystemEntry {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<FileSystemEntry> for emlite::Val {
    fn from(s: FileSystemEntry) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(FileSystemEntry);

impl FileSystemEntry {
    pub fn is_file(&self) -> bool {
        self.inner.get("isFile").as_::<bool>()
    }
}
impl FileSystemEntry {
    pub fn is_directory(&self) -> bool {
        self.inner.get("isDirectory").as_::<bool>()
    }
}
impl FileSystemEntry {
    pub fn name(&self) -> USVString {
        self.inner.get("name").as_::<USVString>()
    }
}
impl FileSystemEntry {
    pub fn full_path(&self) -> USVString {
        self.inner.get("fullPath").as_::<USVString>()
    }
}
impl FileSystemEntry {
    pub fn filesystem(&self) -> FileSystem {
        self.inner.get("filesystem").as_::<FileSystem>()
    }
}
impl FileSystemEntry {
    pub fn get_parent0(&self) -> Undefined {
        self.inner.call("getParent", &[]).as_::<Undefined>()
    }

    pub fn get_parent1(&self, success_callback: Function) -> Undefined {
        self.inner
            .call("getParent", &[success_callback.into()])
            .as_::<Undefined>()
    }

    pub fn get_parent2(&self, success_callback: Function, error_callback: Function) -> Undefined {
        self.inner
            .call(
                "getParent",
                &[success_callback.into(), error_callback.into()],
            )
            .as_::<Undefined>()
    }
}
