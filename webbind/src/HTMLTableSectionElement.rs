use super::*;

#[derive(Clone, Debug)]
pub struct HTMLTableSectionElement {
    inner: HTMLElement,
}
impl FromVal for HTMLTableSectionElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLTableSectionElement {
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
impl std::ops::Deref for HTMLTableSectionElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLTableSectionElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLTableSectionElement> for emlite::Val {
    fn from(s: HTMLTableSectionElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLTableSectionElement {
    pub fn new() -> HTMLTableSectionElement {
        Self {
            inner: emlite::Val::global("HTMLTableSectionElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLTableSectionElement {
    pub fn rows(&self) -> HTMLCollection {
        self.inner.get("rows").as_::<HTMLCollection>()
    }
}
impl HTMLTableSectionElement {
    pub fn insert_row0(&self) -> HTMLTableRowElement {
        self.inner
            .call("insertRow", &[])
            .as_::<HTMLTableRowElement>()
    }

    pub fn insert_row1(&self, index: i32) -> HTMLTableRowElement {
        self.inner
            .call("insertRow", &[index.into()])
            .as_::<HTMLTableRowElement>()
    }
}
impl HTMLTableSectionElement {
    pub fn delete_row(&self, index: i32) -> jsbind::Undefined {
        self.inner
            .call("deleteRow", &[index.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl HTMLTableSectionElement {
    pub fn align(&self) -> jsbind::DOMString {
        self.inner.get("align").as_::<jsbind::DOMString>()
    }

    pub fn set_align(&mut self, value: jsbind::DOMString) {
        self.inner.set("align", value);
    }
}
impl HTMLTableSectionElement {
    pub fn ch(&self) -> jsbind::DOMString {
        self.inner.get("ch").as_::<jsbind::DOMString>()
    }

    pub fn set_ch(&mut self, value: jsbind::DOMString) {
        self.inner.set("ch", value);
    }
}
impl HTMLTableSectionElement {
    pub fn ch_off(&self) -> jsbind::DOMString {
        self.inner.get("chOff").as_::<jsbind::DOMString>()
    }

    pub fn set_ch_off(&mut self, value: jsbind::DOMString) {
        self.inner.set("chOff", value);
    }
}
impl HTMLTableSectionElement {
    pub fn v_align(&self) -> jsbind::DOMString {
        self.inner.get("vAlign").as_::<jsbind::DOMString>()
    }

    pub fn set_v_align(&mut self, value: jsbind::DOMString) {
        self.inner.set("vAlign", value);
    }
}
