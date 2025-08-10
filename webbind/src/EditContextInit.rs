use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EditContextInit {
    inner: Any,
}
impl FromVal for EditContextInit {
    fn from_val(v: &Any) -> Self {
        EditContextInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for EditContextInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for EditContextInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for EditContextInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for EditContextInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<EditContextInit> for Any {
    fn from(s: EditContextInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&EditContextInit> for Any {
    fn from(s: &EditContextInit) -> Any {
        s.inner.clone()
    }
}

impl EditContextInit {
    pub fn text(&self) -> JsString {
        self.inner.get("text").as_::<JsString>()
    }

    pub fn set_text(&mut self, value: &JsString) {
        self.inner.set("text", value);
    }
}
impl EditContextInit {
    pub fn selection_start(&self) -> u32 {
        self.inner.get("selectionStart").as_::<u32>()
    }

    pub fn set_selection_start(&mut self, value: u32) {
        self.inner.set("selectionStart", value);
    }
}
impl EditContextInit {
    pub fn selection_end(&self) -> u32 {
        self.inner.get("selectionEnd").as_::<u32>()
    }

    pub fn set_selection_end(&mut self, value: u32) {
        self.inner.set("selectionEnd", value);
    }
}
