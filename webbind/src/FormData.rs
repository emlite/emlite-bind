use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for FormData {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FormData {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&FormData> for emlite::Val {
    fn from(s: &FormData) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FormData);

impl FormData {
    pub fn new0() -> FormData {
        Self {
            inner: emlite::Val::global("FormData")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(form: &HTMLFormElement) -> FormData {
        Self {
            inner: emlite::Val::global("FormData")
                .new(&[form.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(form: &HTMLFormElement, submitter: &HTMLElement) -> FormData {
        Self {
            inner: emlite::Val::global("FormData")
                .new(&[form.into(), submitter.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl FormData {
    pub fn append0(&self, name: &str, blob_value: &Blob) -> Undefined {
        self.inner
            .call("append", &[name.into(), blob_value.into()])
            .as_::<Undefined>()
    }

    pub fn append1(&self, name: &str, blob_value: &Blob, filename: &str) -> Undefined {
        self.inner
            .call("append", &[name.into(), blob_value.into(), filename.into()])
            .as_::<Undefined>()
    }
}
impl FormData {
    pub fn delete(&self, name: &str) -> Undefined {
        self.inner.call("delete", &[name.into()]).as_::<Undefined>()
    }
}
impl FormData {
    pub fn get(&self, name: &str) -> Any {
        self.inner.call("get", &[name.into()]).as_::<Any>()
    }
}
impl FormData {
    pub fn get_all(&self, name: &str) -> Sequence<Any> {
        self.inner
            .call("getAll", &[name.into()])
            .as_::<Sequence<Any>>()
    }
}
impl FormData {
    pub fn has(&self, name: &str) -> bool {
        self.inner.call("has", &[name.into()]).as_::<bool>()
    }
}
impl FormData {
    pub fn set0(&self, name: &str, blob_value: &Blob) -> Undefined {
        self.inner
            .call("set", &[name.into(), blob_value.into()])
            .as_::<Undefined>()
    }

    pub fn set1(&self, name: &str, blob_value: &Blob, filename: &str) -> Undefined {
        self.inner
            .call("set", &[name.into(), blob_value.into(), filename.into()])
            .as_::<Undefined>()
    }
}
