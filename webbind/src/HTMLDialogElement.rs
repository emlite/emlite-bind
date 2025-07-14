use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HTMLDialogElement {
    inner: HTMLElement,
}
impl FromVal for HTMLDialogElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLDialogElement {
            inner: HTMLElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLDialogElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLDialogElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLDialogElement> for emlite::Val {
    fn from(s: HTMLDialogElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLDialogElement {
    pub fn new() -> HTMLDialogElement {
        Self {
            inner: emlite::Val::global("HTMLDialogElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLDialogElement {
    pub fn open(&self) -> bool {
        self.inner.get("open").as_::<bool>()
    }

    pub fn set_open(&mut self, value: bool) {
        self.inner.set("open", value);
    }
}
impl HTMLDialogElement {
    pub fn return_value(&self) -> jsbind::DOMString {
        self.inner.get("returnValue").as_::<jsbind::DOMString>()
    }

    pub fn set_return_value(&mut self, value: jsbind::DOMString) {
        self.inner.set("returnValue", value);
    }
}
impl HTMLDialogElement {
    pub fn closed_by(&self) -> jsbind::DOMString {
        self.inner.get("closedBy").as_::<jsbind::DOMString>()
    }

    pub fn set_closed_by(&mut self, value: jsbind::DOMString) {
        self.inner.set("closedBy", value);
    }
}
impl HTMLDialogElement {
    pub fn show(&self) -> jsbind::Undefined {
        self.inner.call("show", &[]).as_::<jsbind::Undefined>()
    }
}
impl HTMLDialogElement {
    pub fn show_modal(&self) -> jsbind::Undefined {
        self.inner.call("showModal", &[]).as_::<jsbind::Undefined>()
    }
}
impl HTMLDialogElement {
    pub fn close0(&self) -> jsbind::Undefined {
        self.inner.call("close", &[]).as_::<jsbind::Undefined>()
    }

    pub fn close1(&self, return_value: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("close", &[return_value.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl HTMLDialogElement {
    pub fn request_close0(&self) -> jsbind::Undefined {
        self.inner
            .call("requestClose", &[])
            .as_::<jsbind::Undefined>()
    }

    pub fn request_close1(&self, return_value: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("requestClose", &[return_value.into()])
            .as_::<jsbind::Undefined>()
    }
}
