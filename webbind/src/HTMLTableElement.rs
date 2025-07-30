use super::*;

/// The HTMLTableElement class.
/// [`HTMLTableElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLTableElement {
    inner: HTMLElement,
}
impl FromVal for HTMLTableElement {
    fn from_val(v: &Any) -> Self {
        HTMLTableElement {
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
impl AsRef<Any> for HTMLTableElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLTableElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLTableElement> for Any {
    fn from(s: HTMLTableElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLTableElement> for Any {
    fn from(s: &HTMLTableElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLTableElement);

impl HTMLTableElement {
    /// The `new HTMLTableElement(..)` constructor, creating a new HTMLTableElement instance
    pub fn new() -> HTMLTableElement {
        Self {
            inner: Any::global("HTMLTableElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLTableElement {
    /// Getter of the `caption` attribute.
    /// [`HTMLTableElement.caption`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/caption)
    pub fn caption(&self) -> HTMLTableCaptionElement {
        self.inner.get("caption").as_::<HTMLTableCaptionElement>()
    }

    /// Setter of the `caption` attribute.
    /// [`HTMLTableElement.caption`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/caption)
    pub fn set_caption(&mut self, value: &HTMLTableCaptionElement) {
        self.inner.set("caption", value);
    }
}
impl HTMLTableElement {
    /// The createCaption method.
    /// [`HTMLTableElement.createCaption`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createCaption)
    pub fn create_caption(&self) -> HTMLTableCaptionElement {
        self.inner
            .call("createCaption", &[])
            .as_::<HTMLTableCaptionElement>()
    }
}
impl HTMLTableElement {
    /// The deleteCaption method.
    /// [`HTMLTableElement.deleteCaption`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteCaption)
    pub fn delete_caption(&self) -> Undefined {
        self.inner.call("deleteCaption", &[]).as_::<Undefined>()
    }
}
impl HTMLTableElement {
    /// Getter of the `tHead` attribute.
    /// [`HTMLTableElement.tHead`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tHead)
    pub fn t_head(&self) -> HTMLTableSectionElement {
        self.inner.get("tHead").as_::<HTMLTableSectionElement>()
    }

    /// Setter of the `tHead` attribute.
    /// [`HTMLTableElement.tHead`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tHead)
    pub fn set_t_head(&mut self, value: &HTMLTableSectionElement) {
        self.inner.set("tHead", value);
    }
}
impl HTMLTableElement {
    /// The createTHead method.
    /// [`HTMLTableElement.createTHead`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTHead)
    pub fn create_t_head(&self) -> HTMLTableSectionElement {
        self.inner
            .call("createTHead", &[])
            .as_::<HTMLTableSectionElement>()
    }
}
impl HTMLTableElement {
    /// The deleteTHead method.
    /// [`HTMLTableElement.deleteTHead`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteTHead)
    pub fn delete_t_head(&self) -> Undefined {
        self.inner.call("deleteTHead", &[]).as_::<Undefined>()
    }
}
impl HTMLTableElement {
    /// Getter of the `tFoot` attribute.
    /// [`HTMLTableElement.tFoot`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tFoot)
    pub fn t_foot(&self) -> HTMLTableSectionElement {
        self.inner.get("tFoot").as_::<HTMLTableSectionElement>()
    }

    /// Setter of the `tFoot` attribute.
    /// [`HTMLTableElement.tFoot`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tFoot)
    pub fn set_t_foot(&mut self, value: &HTMLTableSectionElement) {
        self.inner.set("tFoot", value);
    }
}
impl HTMLTableElement {
    /// The createTFoot method.
    /// [`HTMLTableElement.createTFoot`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTFoot)
    pub fn create_t_foot(&self) -> HTMLTableSectionElement {
        self.inner
            .call("createTFoot", &[])
            .as_::<HTMLTableSectionElement>()
    }
}
impl HTMLTableElement {
    /// The deleteTFoot method.
    /// [`HTMLTableElement.deleteTFoot`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteTFoot)
    pub fn delete_t_foot(&self) -> Undefined {
        self.inner.call("deleteTFoot", &[]).as_::<Undefined>()
    }
}
impl HTMLTableElement {
    /// Getter of the `tBodies` attribute.
    /// [`HTMLTableElement.tBodies`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tBodies)
    pub fn t_bodies(&self) -> HTMLCollection {
        self.inner.get("tBodies").as_::<HTMLCollection>()
    }
}
impl HTMLTableElement {
    /// The createTBody method.
    /// [`HTMLTableElement.createTBody`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTBody)
    pub fn create_t_body(&self) -> HTMLTableSectionElement {
        self.inner
            .call("createTBody", &[])
            .as_::<HTMLTableSectionElement>()
    }
}
impl HTMLTableElement {
    /// Getter of the `rows` attribute.
    /// [`HTMLTableElement.rows`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/rows)
    pub fn rows(&self) -> HTMLCollection {
        self.inner.get("rows").as_::<HTMLCollection>()
    }
}
impl HTMLTableElement {
    /// The insertRow method.
    /// [`HTMLTableElement.insertRow`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/insertRow)
    pub fn insert_row0(&self) -> HTMLTableRowElement {
        self.inner
            .call("insertRow", &[])
            .as_::<HTMLTableRowElement>()
    }
    /// The insertRow method.
    /// [`HTMLTableElement.insertRow`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/insertRow)
    pub fn insert_row1(&self, index: i32) -> HTMLTableRowElement {
        self.inner
            .call("insertRow", &[index.into()])
            .as_::<HTMLTableRowElement>()
    }
}
impl HTMLTableElement {
    /// The deleteRow method.
    /// [`HTMLTableElement.deleteRow`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteRow)
    pub fn delete_row(&self, index: i32) -> Undefined {
        self.inner
            .call("deleteRow", &[index.into()])
            .as_::<Undefined>()
    }
}
impl HTMLTableElement {
    /// Getter of the `align` attribute.
    /// [`HTMLTableElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/align)
    pub fn align(&self) -> JsString {
        self.inner.get("align").as_::<JsString>()
    }

    /// Setter of the `align` attribute.
    /// [`HTMLTableElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/align)
    pub fn set_align(&mut self, value: &JsString) {
        self.inner.set("align", value);
    }
}
impl HTMLTableElement {
    /// Getter of the `border` attribute.
    /// [`HTMLTableElement.border`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/border)
    pub fn border(&self) -> JsString {
        self.inner.get("border").as_::<JsString>()
    }

    /// Setter of the `border` attribute.
    /// [`HTMLTableElement.border`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/border)
    pub fn set_border(&mut self, value: &JsString) {
        self.inner.set("border", value);
    }
}
impl HTMLTableElement {
    /// Getter of the `frame` attribute.
    /// [`HTMLTableElement.frame`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/frame)
    pub fn frame(&self) -> JsString {
        self.inner.get("frame").as_::<JsString>()
    }

    /// Setter of the `frame` attribute.
    /// [`HTMLTableElement.frame`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/frame)
    pub fn set_frame(&mut self, value: &JsString) {
        self.inner.set("frame", value);
    }
}
impl HTMLTableElement {
    /// Getter of the `rules` attribute.
    /// [`HTMLTableElement.rules`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/rules)
    pub fn rules(&self) -> JsString {
        self.inner.get("rules").as_::<JsString>()
    }

    /// Setter of the `rules` attribute.
    /// [`HTMLTableElement.rules`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/rules)
    pub fn set_rules(&mut self, value: &JsString) {
        self.inner.set("rules", value);
    }
}
impl HTMLTableElement {
    /// Getter of the `summary` attribute.
    /// [`HTMLTableElement.summary`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/summary)
    pub fn summary(&self) -> JsString {
        self.inner.get("summary").as_::<JsString>()
    }

    /// Setter of the `summary` attribute.
    /// [`HTMLTableElement.summary`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/summary)
    pub fn set_summary(&mut self, value: &JsString) {
        self.inner.set("summary", value);
    }
}
impl HTMLTableElement {
    /// Getter of the `width` attribute.
    /// [`HTMLTableElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/width)
    pub fn width(&self) -> JsString {
        self.inner.get("width").as_::<JsString>()
    }

    /// Setter of the `width` attribute.
    /// [`HTMLTableElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/width)
    pub fn set_width(&mut self, value: &JsString) {
        self.inner.set("width", value);
    }
}
impl HTMLTableElement {
    /// Getter of the `bgColor` attribute.
    /// [`HTMLTableElement.bgColor`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/bgColor)
    pub fn bg_color(&self) -> JsString {
        self.inner.get("bgColor").as_::<JsString>()
    }

    /// Setter of the `bgColor` attribute.
    /// [`HTMLTableElement.bgColor`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/bgColor)
    pub fn set_bg_color(&mut self, value: &JsString) {
        self.inner.set("bgColor", value);
    }
}
impl HTMLTableElement {
    /// Getter of the `cellPadding` attribute.
    /// [`HTMLTableElement.cellPadding`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellPadding)
    pub fn cell_padding(&self) -> JsString {
        self.inner.get("cellPadding").as_::<JsString>()
    }

    /// Setter of the `cellPadding` attribute.
    /// [`HTMLTableElement.cellPadding`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellPadding)
    pub fn set_cell_padding(&mut self, value: &JsString) {
        self.inner.set("cellPadding", value);
    }
}
impl HTMLTableElement {
    /// Getter of the `cellSpacing` attribute.
    /// [`HTMLTableElement.cellSpacing`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellSpacing)
    pub fn cell_spacing(&self) -> JsString {
        self.inner.get("cellSpacing").as_::<JsString>()
    }

    /// Setter of the `cellSpacing` attribute.
    /// [`HTMLTableElement.cellSpacing`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellSpacing)
    pub fn set_cell_spacing(&mut self, value: &JsString) {
        self.inner.set("cellSpacing", value);
    }
}
