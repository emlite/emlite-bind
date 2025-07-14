use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FileSystemFlags {
    inner: emlite::Val,
}
impl FromVal for FileSystemFlags {
    fn from_val(v: &emlite::Val) -> Self {
        FileSystemFlags { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FileSystemFlags {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemFlags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FileSystemFlags> for emlite::Val {
    fn from(s: FileSystemFlags) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FileSystemFlags {
    pub fn create(&self) -> bool {
        self.inner.get("create").as_::<bool>()
    }

    pub fn set_create(&mut self, value: bool) {
        self.inner.set("create", value);
    }
}
impl FileSystemFlags {
    pub fn exclusive(&self) -> bool {
        self.inner.get("exclusive").as_::<bool>()
    }

    pub fn set_exclusive(&mut self, value: bool) {
        self.inner.set("exclusive", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FileSystemDirectoryEntry {
    inner: FileSystemEntry,
}
impl FromVal for FileSystemDirectoryEntry {
    fn from_val(v: &emlite::Val) -> Self {
        FileSystemDirectoryEntry {
            inner: FileSystemEntry::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FileSystemDirectoryEntry {
    type Target = FileSystemEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemDirectoryEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FileSystemDirectoryEntry> for emlite::Val {
    fn from(s: FileSystemDirectoryEntry) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FileSystemDirectoryEntry {
    pub fn create_reader(&self) -> FileSystemDirectoryReader {
        self.inner
            .call("createReader", &[])
            .as_::<FileSystemDirectoryReader>()
    }
}
impl FileSystemDirectoryEntry {
    pub fn get_file0(&self) -> jsbind::Undefined {
        self.inner.call("getFile", &[]).as_::<jsbind::Undefined>()
    }

    pub fn get_file1(&self, path: jsbind::USVString) -> jsbind::Undefined {
        self.inner
            .call("getFile", &[path.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn get_file2(
        &self,
        path: jsbind::USVString,
        options: FileSystemFlags,
    ) -> jsbind::Undefined {
        self.inner
            .call("getFile", &[path.into(), options.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn get_file3(
        &self,
        path: jsbind::USVString,
        options: FileSystemFlags,
        success_callback: jsbind::Function,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "getFile",
                &[path.into(), options.into(), success_callback.into()],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn get_file4(
        &self,
        path: jsbind::USVString,
        options: FileSystemFlags,
        success_callback: jsbind::Function,
        error_callback: jsbind::Function,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "getFile",
                &[
                    path.into(),
                    options.into(),
                    success_callback.into(),
                    error_callback.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl FileSystemDirectoryEntry {
    pub fn get_directory0(&self) -> jsbind::Undefined {
        self.inner
            .call("getDirectory", &[])
            .as_::<jsbind::Undefined>()
    }

    pub fn get_directory1(&self, path: jsbind::USVString) -> jsbind::Undefined {
        self.inner
            .call("getDirectory", &[path.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn get_directory2(
        &self,
        path: jsbind::USVString,
        options: FileSystemFlags,
    ) -> jsbind::Undefined {
        self.inner
            .call("getDirectory", &[path.into(), options.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn get_directory3(
        &self,
        path: jsbind::USVString,
        options: FileSystemFlags,
        success_callback: jsbind::Function,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "getDirectory",
                &[path.into(), options.into(), success_callback.into()],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn get_directory4(
        &self,
        path: jsbind::USVString,
        options: FileSystemFlags,
        success_callback: jsbind::Function,
        error_callback: jsbind::Function,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "getDirectory",
                &[
                    path.into(),
                    options.into(),
                    success_callback.into(),
                    error_callback.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
