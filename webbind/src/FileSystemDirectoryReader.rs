use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FileSystemDirectoryReader {
    inner: emlite::Val,
}
impl FromVal for FileSystemDirectoryReader {
    fn from_val(v: &emlite::Val) -> Self {
        FileSystemDirectoryReader {
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
impl core::ops::Deref for FileSystemDirectoryReader {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemDirectoryReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FileSystemDirectoryReader> for emlite::Val {
    fn from(s: FileSystemDirectoryReader) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FileSystemDirectoryReader {
    pub fn read_entries0(&self, success_callback: jsbind::Function) -> jsbind::Undefined {
        self.inner
            .call("readEntries", &[success_callback.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn read_entries1(
        &self,
        success_callback: jsbind::Function,
        error_callback: jsbind::Function,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "readEntries",
                &[success_callback.into(), error_callback.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
