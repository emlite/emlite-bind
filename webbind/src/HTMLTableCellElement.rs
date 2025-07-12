use super::*;

#[derive(Clone, Debug)]
pub struct HTMLTableCellElement {
    inner: HTMLElement,
}
impl FromVal for HTMLTableCellElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLTableCellElement {
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
impl std::ops::Deref for HTMLTableCellElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLTableCellElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLTableCellElement> for emlite::Val {
    fn from(s: HTMLTableCellElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLTableCellElement {
    pub fn new() -> HTMLTableCellElement {
        Self {
            inner: emlite::Val::global("HTMLTableCellElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLTableCellElement {
    pub fn col_span(&self) -> u32 {
        self.inner.get("colSpan").as_::<u32>()
    }

    pub fn set_col_span(&mut self, value: u32) {
        self.inner.set("colSpan", value);
    }
}
impl HTMLTableCellElement {
    pub fn row_span(&self) -> u32 {
        self.inner.get("rowSpan").as_::<u32>()
    }

    pub fn set_row_span(&mut self, value: u32) {
        self.inner.set("rowSpan", value);
    }
}
impl HTMLTableCellElement {
    pub fn headers(&self) -> jsbind::DOMString {
        self.inner.get("headers").as_::<jsbind::DOMString>()
    }

    pub fn set_headers(&mut self, value: jsbind::DOMString) {
        self.inner.set("headers", value);
    }
}
impl HTMLTableCellElement {
    pub fn cell_index(&self) -> i32 {
        self.inner.get("cellIndex").as_::<i32>()
    }
}
impl HTMLTableCellElement {
    pub fn scope(&self) -> jsbind::DOMString {
        self.inner.get("scope").as_::<jsbind::DOMString>()
    }

    pub fn set_scope(&mut self, value: jsbind::DOMString) {
        self.inner.set("scope", value);
    }
}
impl HTMLTableCellElement {
    pub fn abbr(&self) -> jsbind::DOMString {
        self.inner.get("abbr").as_::<jsbind::DOMString>()
    }

    pub fn set_abbr(&mut self, value: jsbind::DOMString) {
        self.inner.set("abbr", value);
    }
}
impl HTMLTableCellElement {
    pub fn align(&self) -> jsbind::DOMString {
        self.inner.get("align").as_::<jsbind::DOMString>()
    }

    pub fn set_align(&mut self, value: jsbind::DOMString) {
        self.inner.set("align", value);
    }
}
impl HTMLTableCellElement {
    pub fn axis(&self) -> jsbind::DOMString {
        self.inner.get("axis").as_::<jsbind::DOMString>()
    }

    pub fn set_axis(&mut self, value: jsbind::DOMString) {
        self.inner.set("axis", value);
    }
}
impl HTMLTableCellElement {
    pub fn height(&self) -> jsbind::DOMString {
        self.inner.get("height").as_::<jsbind::DOMString>()
    }

    pub fn set_height(&mut self, value: jsbind::DOMString) {
        self.inner.set("height", value);
    }
}
impl HTMLTableCellElement {
    pub fn width(&self) -> jsbind::DOMString {
        self.inner.get("width").as_::<jsbind::DOMString>()
    }

    pub fn set_width(&mut self, value: jsbind::DOMString) {
        self.inner.set("width", value);
    }
}
impl HTMLTableCellElement {
    pub fn ch(&self) -> jsbind::DOMString {
        self.inner.get("ch").as_::<jsbind::DOMString>()
    }

    pub fn set_ch(&mut self, value: jsbind::DOMString) {
        self.inner.set("ch", value);
    }
}
impl HTMLTableCellElement {
    pub fn ch_off(&self) -> jsbind::DOMString {
        self.inner.get("chOff").as_::<jsbind::DOMString>()
    }

    pub fn set_ch_off(&mut self, value: jsbind::DOMString) {
        self.inner.set("chOff", value);
    }
}
impl HTMLTableCellElement {
    pub fn no_wrap(&self) -> bool {
        self.inner.get("noWrap").as_::<bool>()
    }

    pub fn set_no_wrap(&mut self, value: bool) {
        self.inner.set("noWrap", value);
    }
}
impl HTMLTableCellElement {
    pub fn v_align(&self) -> jsbind::DOMString {
        self.inner.get("vAlign").as_::<jsbind::DOMString>()
    }

    pub fn set_v_align(&mut self, value: jsbind::DOMString) {
        self.inner.set("vAlign", value);
    }
}
impl HTMLTableCellElement {
    pub fn bg_color(&self) -> jsbind::DOMString {
        self.inner.get("bgColor").as_::<jsbind::DOMString>()
    }

    pub fn set_bg_color(&mut self, value: jsbind::DOMString) {
        self.inner.set("bgColor", value);
    }
}
