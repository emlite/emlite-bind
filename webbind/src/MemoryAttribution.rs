use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MemoryAttribution {
    inner: Any,
}
impl FromVal for MemoryAttribution {
    fn from_val(v: &Any) -> Self {
        MemoryAttribution { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MemoryAttribution {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MemoryAttribution {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MemoryAttribution {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MemoryAttribution {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MemoryAttribution> for Any {
    fn from(s: MemoryAttribution) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MemoryAttribution> for Any {
    fn from(s: &MemoryAttribution) -> Any {
        s.inner.clone()
    }
}

impl MemoryAttribution {
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }

    pub fn set_url(&mut self, value: &JsString) {
        self.inner.set("url", value);
    }
}
impl MemoryAttribution {
    pub fn container(&self) -> MemoryAttributionContainer {
        self.inner
            .get("container")
            .as_::<MemoryAttributionContainer>()
    }

    pub fn set_container(&mut self, value: &MemoryAttributionContainer) {
        self.inner.set("container", value);
    }
}
impl MemoryAttribution {
    pub fn scope(&self) -> JsString {
        self.inner.get("scope").as_::<JsString>()
    }

    pub fn set_scope(&mut self, value: &JsString) {
        self.inner.set("scope", value);
    }
}
