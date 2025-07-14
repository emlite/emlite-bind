use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FormData {
    inner: emlite::Val,
}
impl FromVal for FormData {
    fn from_val(v: &emlite::Val) -> Self {
        FormData {
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
impl core::ops::Deref for FormData {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FormData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FormData> for emlite::Val {
    fn from(s: FormData) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FormData {
    pub fn new0() -> FormData {
        Self {
            inner: emlite::Val::global("FormData")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(form: HTMLFormElement) -> FormData {
        Self {
            inner: emlite::Val::global("FormData")
                .new(&[form.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(form: HTMLFormElement, submitter: HTMLElement) -> FormData {
        Self {
            inner: emlite::Val::global("FormData")
                .new(&[form.into(), submitter.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl FormData {
    pub fn append0(&self, name: jsbind::USVString, blob_value: Blob) -> jsbind::Undefined {
        self.inner
            .call("append", &[name.into(), blob_value.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn append1(
        &self,
        name: jsbind::USVString,
        blob_value: Blob,
        filename: jsbind::USVString,
    ) -> jsbind::Undefined {
        self.inner
            .call("append", &[name.into(), blob_value.into(), filename.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl FormData {
    pub fn delete(&self, name: jsbind::USVString) -> jsbind::Undefined {
        self.inner
            .call("delete", &[name.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl FormData {
    pub fn get(&self, name: jsbind::USVString) -> jsbind::Any {
        self.inner.call("get", &[name.into()]).as_::<jsbind::Any>()
    }
}
impl FormData {
    pub fn get_all(&self, name: jsbind::USVString) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .call("getAll", &[name.into()])
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }
}
impl FormData {
    pub fn has(&self, name: jsbind::USVString) -> bool {
        self.inner.call("has", &[name.into()]).as_::<bool>()
    }
}
impl FormData {
    pub fn set0(&self, name: jsbind::USVString, blob_value: Blob) -> jsbind::Undefined {
        self.inner
            .call("set", &[name.into(), blob_value.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn set1(
        &self,
        name: jsbind::USVString,
        blob_value: Blob,
        filename: jsbind::USVString,
    ) -> jsbind::Undefined {
        self.inner
            .call("set", &[name.into(), blob_value.into(), filename.into()])
            .as_::<jsbind::Undefined>()
    }
}
