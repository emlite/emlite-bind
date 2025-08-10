use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Report {
    inner: Any,
}
impl FromVal for Report {
    fn from_val(v: &Any) -> Self {
        Report { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Report {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Report {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Report {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Report {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Report> for Any {
    fn from(s: Report) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Report> for Any {
    fn from(s: &Report) -> Any {
        s.inner.clone()
    }
}

impl Report {
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
impl Report {
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }

    pub fn set_url(&mut self, value: &JsString) {
        self.inner.set("url", value);
    }
}
impl Report {
    pub fn body(&self) -> ReportBody {
        self.inner.get("body").as_::<ReportBody>()
    }

    pub fn set_body(&mut self, value: &ReportBody) {
        self.inner.set("body", value);
    }
}
