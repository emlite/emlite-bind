use super::*;

/// The HTMLTableSectionElement class.
/// [`HTMLTableSectionElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLTableSectionElement {
    inner: HTMLElement,
}
impl FromVal for HTMLTableSectionElement {
    fn from_val(v: &Any) -> Self {
        HTMLTableSectionElement {
            inner: HTMLElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLTableSectionElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLTableSectionElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLTableSectionElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLTableSectionElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLTableSectionElement> for Any {
    fn from(s: HTMLTableSectionElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLTableSectionElement> for Any {
    fn from(s: &HTMLTableSectionElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLTableSectionElement);

impl HTMLTableSectionElement {
    /// The `new HTMLTableSectionElement(..)` constructor, creating a new HTMLTableSectionElement instance
    pub fn new() -> HTMLTableSectionElement {
        Self {
            inner: Any::global("HTMLTableSectionElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLTableSectionElement {
    /// Getter of the `rows` attribute.
    /// [`HTMLTableSectionElement.rows`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/rows)
    pub fn rows(&self) -> HTMLCollection {
        self.inner.get("rows").as_::<HTMLCollection>()
    }
}
impl HTMLTableSectionElement {
    /// The insertRow method.
    /// [`HTMLTableSectionElement.insertRow`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/insertRow)
    pub fn insert_row0(&self) -> HTMLTableRowElement {
        self.inner
            .call("insertRow", &[])
            .as_::<HTMLTableRowElement>()
    }
    /// The insertRow method.
    /// [`HTMLTableSectionElement.insertRow`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/insertRow)
    pub fn insert_row1(&self, index: i32) -> HTMLTableRowElement {
        self.inner
            .call("insertRow", &[index.into()])
            .as_::<HTMLTableRowElement>()
    }
}
impl HTMLTableSectionElement {
    /// The deleteRow method.
    /// [`HTMLTableSectionElement.deleteRow`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/deleteRow)
    pub fn delete_row(&self, index: i32) -> Undefined {
        self.inner
            .call("deleteRow", &[index.into()])
            .as_::<Undefined>()
    }
}
impl HTMLTableSectionElement {
    /// Getter of the `align` attribute.
    /// [`HTMLTableSectionElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/align)
    pub fn align(&self) -> String {
        self.inner.get("align").as_::<String>()
    }

    /// Setter of the `align` attribute.
    /// [`HTMLTableSectionElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/align)
    pub fn set_align(&mut self, value: &str) {
        self.inner.set("align", value);
    }
}
impl HTMLTableSectionElement {
    /// Getter of the `ch` attribute.
    /// [`HTMLTableSectionElement.ch`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/ch)
    pub fn ch(&self) -> String {
        self.inner.get("ch").as_::<String>()
    }

    /// Setter of the `ch` attribute.
    /// [`HTMLTableSectionElement.ch`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/ch)
    pub fn set_ch(&mut self, value: &str) {
        self.inner.set("ch", value);
    }
}
impl HTMLTableSectionElement {
    /// Getter of the `chOff` attribute.
    /// [`HTMLTableSectionElement.chOff`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/chOff)
    pub fn ch_off(&self) -> String {
        self.inner.get("chOff").as_::<String>()
    }

    /// Setter of the `chOff` attribute.
    /// [`HTMLTableSectionElement.chOff`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/chOff)
    pub fn set_ch_off(&mut self, value: &str) {
        self.inner.set("chOff", value);
    }
}
impl HTMLTableSectionElement {
    /// Getter of the `vAlign` attribute.
    /// [`HTMLTableSectionElement.vAlign`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/vAlign)
    pub fn v_align(&self) -> String {
        self.inner.get("vAlign").as_::<String>()
    }

    /// Setter of the `vAlign` attribute.
    /// [`HTMLTableSectionElement.vAlign`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/vAlign)
    pub fn set_v_align(&mut self, value: &str) {
        self.inner.set("vAlign", value);
    }
}
