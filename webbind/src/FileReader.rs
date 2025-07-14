use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FileReader {
    inner: EventTarget,
}
impl FromVal for FileReader {
    fn from_val(v: &emlite::Val) -> Self {
        FileReader {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl From<FileReader> for emlite::Val {
    fn from(s: FileReader) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FileReader {
    pub fn new() -> FileReader {
        Self {
            inner: emlite::Val::global("FileReader")
                .new(&[])
                .as_::<EventTarget>(),
        }
    }
}
impl FileReader {
    pub fn read_as_array_buffer(&self, blob: Blob) -> jsbind::Undefined {
        self.inner
            .call("readAsArrayBuffer", &[blob.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl FileReader {
    pub fn read_as_binary_string(&self, blob: Blob) -> jsbind::Undefined {
        self.inner
            .call("readAsBinaryString", &[blob.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl FileReader {
    pub fn read_as_text0(&self, blob: Blob) -> jsbind::Undefined {
        self.inner
            .call("readAsText", &[blob.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn read_as_text1(&self, blob: Blob, encoding: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("readAsText", &[blob.into(), encoding.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl FileReader {
    pub fn read_as_data_url(&self, blob: Blob) -> jsbind::Undefined {
        self.inner
            .call("readAsDataURL", &[blob.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl FileReader {
    pub fn abort(&self) -> jsbind::Undefined {
        self.inner.call("abort", &[]).as_::<jsbind::Undefined>()
    }
}
impl FileReader {
    pub fn ready_state(&self) -> u16 {
        self.inner.get("readyState").as_::<u16>()
    }
}
impl FileReader {
    pub fn result(&self) -> jsbind::Any {
        self.inner.get("result").as_::<jsbind::Any>()
    }
}
impl FileReader {
    pub fn error(&self) -> DOMException {
        self.inner.get("error").as_::<DOMException>()
    }
}
impl FileReader {
    pub fn onloadstart(&self) -> jsbind::Any {
        self.inner.get("onloadstart").as_::<jsbind::Any>()
    }

    pub fn set_onloadstart(&mut self, value: jsbind::Any) {
        self.inner.set("onloadstart", value);
    }
}
impl FileReader {
    pub fn onprogress(&self) -> jsbind::Any {
        self.inner.get("onprogress").as_::<jsbind::Any>()
    }

    pub fn set_onprogress(&mut self, value: jsbind::Any) {
        self.inner.set("onprogress", value);
    }
}
impl FileReader {
    pub fn onload(&self) -> jsbind::Any {
        self.inner.get("onload").as_::<jsbind::Any>()
    }

    pub fn set_onload(&mut self, value: jsbind::Any) {
        self.inner.set("onload", value);
    }
}
impl FileReader {
    pub fn onabort(&self) -> jsbind::Any {
        self.inner.get("onabort").as_::<jsbind::Any>()
    }

    pub fn set_onabort(&mut self, value: jsbind::Any) {
        self.inner.set("onabort", value);
    }
}
impl FileReader {
    pub fn onerror(&self) -> jsbind::Any {
        self.inner.get("onerror").as_::<jsbind::Any>()
    }

    pub fn set_onerror(&mut self, value: jsbind::Any) {
        self.inner.set("onerror", value);
    }
}
impl FileReader {
    pub fn onloadend(&self) -> jsbind::Any {
        self.inner.get("onloadend").as_::<jsbind::Any>()
    }

    pub fn set_onloadend(&mut self, value: jsbind::Any) {
        self.inner.set("onloadend", value);
    }
}
