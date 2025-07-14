use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FileSystem {
    inner: emlite::Val,
}
impl FromVal for FileSystem {
    fn from_val(v: &emlite::Val) -> Self {
        FileSystem {
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
impl core::ops::Deref for FileSystem {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FileSystem> for emlite::Val {
    fn from(s: FileSystem) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FileSystem {
    pub fn name(&self) -> jsbind::USVString {
        self.inner.get("name").as_::<jsbind::USVString>()
    }
}
impl FileSystem {
    pub fn root(&self) -> FileSystemDirectoryEntry {
        self.inner.get("root").as_::<FileSystemDirectoryEntry>()
    }
}
