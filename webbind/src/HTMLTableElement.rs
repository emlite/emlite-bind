use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLTableElement {
    inner: HTMLElement,
}
impl FromVal for HTMLTableElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLTableElement {
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
impl core::ops::Deref for HTMLTableElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLTableElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLTableElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLTableElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLTableElement> for emlite::Val {
    fn from(s: HTMLTableElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HTMLTableElement> for emlite::Val {
    fn from(s: &HTMLTableElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLTableElement);

impl HTMLTableElement {
    pub fn new() -> HTMLTableElement {
        Self {
            inner: emlite::Val::global("HTMLTableElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLTableElement {
    pub fn caption(&self) -> HTMLTableCaptionElement {
        self.inner.get("caption").as_::<HTMLTableCaptionElement>()
    }

    pub fn set_caption(&mut self, value: HTMLTableCaptionElement) {
        self.inner.set("caption", value);
    }
}
impl HTMLTableElement {
    pub fn create_caption(&self) -> HTMLTableCaptionElement {
        self.inner
            .call("createCaption", &[])
            .as_::<HTMLTableCaptionElement>()
    }
}
impl HTMLTableElement {
    pub fn delete_caption(&self) -> Undefined {
        self.inner.call("deleteCaption", &[]).as_::<Undefined>()
    }
}
impl HTMLTableElement {
    pub fn t_head(&self) -> HTMLTableSectionElement {
        self.inner.get("tHead").as_::<HTMLTableSectionElement>()
    }

    pub fn set_t_head(&mut self, value: HTMLTableSectionElement) {
        self.inner.set("tHead", value);
    }
}
impl HTMLTableElement {
    pub fn create_t_head(&self) -> HTMLTableSectionElement {
        self.inner
            .call("createTHead", &[])
            .as_::<HTMLTableSectionElement>()
    }
}
impl HTMLTableElement {
    pub fn delete_t_head(&self) -> Undefined {
        self.inner.call("deleteTHead", &[]).as_::<Undefined>()
    }
}
impl HTMLTableElement {
    pub fn t_foot(&self) -> HTMLTableSectionElement {
        self.inner.get("tFoot").as_::<HTMLTableSectionElement>()
    }

    pub fn set_t_foot(&mut self, value: HTMLTableSectionElement) {
        self.inner.set("tFoot", value);
    }
}
impl HTMLTableElement {
    pub fn create_t_foot(&self) -> HTMLTableSectionElement {
        self.inner
            .call("createTFoot", &[])
            .as_::<HTMLTableSectionElement>()
    }
}
impl HTMLTableElement {
    pub fn delete_t_foot(&self) -> Undefined {
        self.inner.call("deleteTFoot", &[]).as_::<Undefined>()
    }
}
impl HTMLTableElement {
    pub fn t_bodies(&self) -> HTMLCollection {
        self.inner.get("tBodies").as_::<HTMLCollection>()
    }
}
impl HTMLTableElement {
    pub fn create_t_body(&self) -> HTMLTableSectionElement {
        self.inner
            .call("createTBody", &[])
            .as_::<HTMLTableSectionElement>()
    }
}
impl HTMLTableElement {
    pub fn rows(&self) -> HTMLCollection {
        self.inner.get("rows").as_::<HTMLCollection>()
    }
}
impl HTMLTableElement {
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
impl HTMLTableElement {
    pub fn delete_row(&self, index: i32) -> Undefined {
        self.inner
            .call("deleteRow", &[index.into()])
            .as_::<Undefined>()
    }
}
impl HTMLTableElement {
    pub fn align(&self) -> DOMString {
        self.inner.get("align").as_::<DOMString>()
    }

    pub fn set_align(&mut self, value: DOMString) {
        self.inner.set("align", value);
    }
}
impl HTMLTableElement {
    pub fn border(&self) -> DOMString {
        self.inner.get("border").as_::<DOMString>()
    }

    pub fn set_border(&mut self, value: DOMString) {
        self.inner.set("border", value);
    }
}
impl HTMLTableElement {
    pub fn frame(&self) -> DOMString {
        self.inner.get("frame").as_::<DOMString>()
    }

    pub fn set_frame(&mut self, value: DOMString) {
        self.inner.set("frame", value);
    }
}
impl HTMLTableElement {
    pub fn rules(&self) -> DOMString {
        self.inner.get("rules").as_::<DOMString>()
    }

    pub fn set_rules(&mut self, value: DOMString) {
        self.inner.set("rules", value);
    }
}
impl HTMLTableElement {
    pub fn summary(&self) -> DOMString {
        self.inner.get("summary").as_::<DOMString>()
    }

    pub fn set_summary(&mut self, value: DOMString) {
        self.inner.set("summary", value);
    }
}
impl HTMLTableElement {
    pub fn width(&self) -> DOMString {
        self.inner.get("width").as_::<DOMString>()
    }

    pub fn set_width(&mut self, value: DOMString) {
        self.inner.set("width", value);
    }
}
impl HTMLTableElement {
    pub fn bg_color(&self) -> DOMString {
        self.inner.get("bgColor").as_::<DOMString>()
    }

    pub fn set_bg_color(&mut self, value: DOMString) {
        self.inner.set("bgColor", value);
    }
}
impl HTMLTableElement {
    pub fn cell_padding(&self) -> DOMString {
        self.inner.get("cellPadding").as_::<DOMString>()
    }

    pub fn set_cell_padding(&mut self, value: DOMString) {
        self.inner.set("cellPadding", value);
    }
}
impl HTMLTableElement {
    pub fn cell_spacing(&self) -> DOMString {
        self.inner.get("cellSpacing").as_::<DOMString>()
    }

    pub fn set_cell_spacing(&mut self, value: DOMString) {
        self.inner.set("cellSpacing", value);
    }
}
