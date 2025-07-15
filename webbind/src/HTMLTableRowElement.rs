use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLTableRowElement {
    inner: HTMLElement,
}
impl FromVal for HTMLTableRowElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLTableRowElement { inner: HTMLElement::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLTableRowElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLTableRowElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLTableRowElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLTableRowElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<HTMLTableRowElement> for emlite::Val {
    fn from(s: HTMLTableRowElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLTableRowElement);



impl HTMLTableRowElement {
    pub fn new() -> HTMLTableRowElement {
        Self {
            inner: emlite::Val::global("HTMLTableRowElement").new(&[]).as_::<HTMLElement>(),
        }
    }

}
impl HTMLTableRowElement {
    pub fn row_index(&self) -> i32 {
        self.inner.get("rowIndex").as_::<i32>()
    }

}
impl HTMLTableRowElement {
    pub fn section_row_index(&self) -> i32 {
        self.inner.get("sectionRowIndex").as_::<i32>()
    }

}
impl HTMLTableRowElement {
    pub fn cells(&self) -> HTMLCollection {
        self.inner.get("cells").as_::<HTMLCollection>()
    }

}
impl HTMLTableRowElement {
    pub fn insert_cell0(&self, ) -> HTMLTableCellElement {
        self.inner.call("insertCell", &[]).as_::<HTMLTableCellElement>()
    }

    pub fn insert_cell1(&self, index: i32) -> HTMLTableCellElement {
        self.inner.call("insertCell", &[index.into(), ]).as_::<HTMLTableCellElement>()
    }

}
impl HTMLTableRowElement {
    pub fn delete_cell(&self, index: i32) -> Undefined {
        self.inner.call("deleteCell", &[index.into(), ]).as_::<Undefined>()
    }

}
impl HTMLTableRowElement {
    pub fn align(&self) -> DOMString {
        self.inner.get("align").as_::<DOMString>()
    }

    pub fn set_align(&mut self, value: DOMString) {
        self.inner.set("align", value);
    }

}
impl HTMLTableRowElement {
    pub fn ch(&self) -> DOMString {
        self.inner.get("ch").as_::<DOMString>()
    }

    pub fn set_ch(&mut self, value: DOMString) {
        self.inner.set("ch", value);
    }

}
impl HTMLTableRowElement {
    pub fn ch_off(&self) -> DOMString {
        self.inner.get("chOff").as_::<DOMString>()
    }

    pub fn set_ch_off(&mut self, value: DOMString) {
        self.inner.set("chOff", value);
    }

}
impl HTMLTableRowElement {
    pub fn v_align(&self) -> DOMString {
        self.inner.get("vAlign").as_::<DOMString>()
    }

    pub fn set_v_align(&mut self, value: DOMString) {
        self.inner.set("vAlign", value);
    }

}
impl HTMLTableRowElement {
    pub fn bg_color(&self) -> DOMString {
        self.inner.get("bgColor").as_::<DOMString>()
    }

    pub fn set_bg_color(&mut self, value: DOMString) {
        self.inner.set("bgColor", value);
    }

}
