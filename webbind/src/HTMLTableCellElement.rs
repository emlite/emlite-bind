use super::*;

/// The HTMLTableCellElement class.
/// [`HTMLTableCellElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLTableCellElement {
    inner: HTMLElement,
}
impl FromVal for HTMLTableCellElement {
    fn from_val(v: &Any) -> Self {
        HTMLTableCellElement {
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
impl core::ops::Deref for HTMLTableCellElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLTableCellElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLTableCellElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLTableCellElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLTableCellElement> for Any {
    fn from(s: HTMLTableCellElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLTableCellElement> for Any {
    fn from(s: &HTMLTableCellElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLTableCellElement);

impl HTMLTableCellElement {
    /// The `new HTMLTableCellElement(..)` constructor, creating a new HTMLTableCellElement instance
    pub fn new() -> HTMLTableCellElement {
        Self {
            inner: Any::global("HTMLTableCellElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLTableCellElement {
    /// Getter of the `colSpan` attribute.
    /// [`HTMLTableCellElement.colSpan`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/colSpan)
    pub fn col_span(&self) -> u32 {
        self.inner.get("colSpan").as_::<u32>()
    }

    /// Setter of the `colSpan` attribute.
    /// [`HTMLTableCellElement.colSpan`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/colSpan)
    pub fn set_col_span(&mut self, value: u32) {
        self.inner.set("colSpan", value);
    }
}
impl HTMLTableCellElement {
    /// Getter of the `rowSpan` attribute.
    /// [`HTMLTableCellElement.rowSpan`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/rowSpan)
    pub fn row_span(&self) -> u32 {
        self.inner.get("rowSpan").as_::<u32>()
    }

    /// Setter of the `rowSpan` attribute.
    /// [`HTMLTableCellElement.rowSpan`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/rowSpan)
    pub fn set_row_span(&mut self, value: u32) {
        self.inner.set("rowSpan", value);
    }
}
impl HTMLTableCellElement {
    /// Getter of the `headers` attribute.
    /// [`HTMLTableCellElement.headers`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/headers)
    pub fn headers(&self) -> String {
        self.inner.get("headers").as_::<String>()
    }

    /// Setter of the `headers` attribute.
    /// [`HTMLTableCellElement.headers`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/headers)
    pub fn set_headers(&mut self, value: &str) {
        self.inner.set("headers", value);
    }
}
impl HTMLTableCellElement {
    /// Getter of the `cellIndex` attribute.
    /// [`HTMLTableCellElement.cellIndex`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/cellIndex)
    pub fn cell_index(&self) -> i32 {
        self.inner.get("cellIndex").as_::<i32>()
    }
}
impl HTMLTableCellElement {
    /// Getter of the `scope` attribute.
    /// [`HTMLTableCellElement.scope`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/scope)
    pub fn scope(&self) -> String {
        self.inner.get("scope").as_::<String>()
    }

    /// Setter of the `scope` attribute.
    /// [`HTMLTableCellElement.scope`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/scope)
    pub fn set_scope(&mut self, value: &str) {
        self.inner.set("scope", value);
    }
}
impl HTMLTableCellElement {
    /// Getter of the `abbr` attribute.
    /// [`HTMLTableCellElement.abbr`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/abbr)
    pub fn abbr(&self) -> String {
        self.inner.get("abbr").as_::<String>()
    }

    /// Setter of the `abbr` attribute.
    /// [`HTMLTableCellElement.abbr`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/abbr)
    pub fn set_abbr(&mut self, value: &str) {
        self.inner.set("abbr", value);
    }
}
impl HTMLTableCellElement {
    /// Getter of the `align` attribute.
    /// [`HTMLTableCellElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/align)
    pub fn align(&self) -> String {
        self.inner.get("align").as_::<String>()
    }

    /// Setter of the `align` attribute.
    /// [`HTMLTableCellElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/align)
    pub fn set_align(&mut self, value: &str) {
        self.inner.set("align", value);
    }
}
impl HTMLTableCellElement {
    /// Getter of the `axis` attribute.
    /// [`HTMLTableCellElement.axis`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/axis)
    pub fn axis(&self) -> String {
        self.inner.get("axis").as_::<String>()
    }

    /// Setter of the `axis` attribute.
    /// [`HTMLTableCellElement.axis`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/axis)
    pub fn set_axis(&mut self, value: &str) {
        self.inner.set("axis", value);
    }
}
impl HTMLTableCellElement {
    /// Getter of the `height` attribute.
    /// [`HTMLTableCellElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/height)
    pub fn height(&self) -> String {
        self.inner.get("height").as_::<String>()
    }

    /// Setter of the `height` attribute.
    /// [`HTMLTableCellElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/height)
    pub fn set_height(&mut self, value: &str) {
        self.inner.set("height", value);
    }
}
impl HTMLTableCellElement {
    /// Getter of the `width` attribute.
    /// [`HTMLTableCellElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/width)
    pub fn width(&self) -> String {
        self.inner.get("width").as_::<String>()
    }

    /// Setter of the `width` attribute.
    /// [`HTMLTableCellElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/width)
    pub fn set_width(&mut self, value: &str) {
        self.inner.set("width", value);
    }
}
impl HTMLTableCellElement {
    /// Getter of the `ch` attribute.
    /// [`HTMLTableCellElement.ch`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/ch)
    pub fn ch(&self) -> String {
        self.inner.get("ch").as_::<String>()
    }

    /// Setter of the `ch` attribute.
    /// [`HTMLTableCellElement.ch`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/ch)
    pub fn set_ch(&mut self, value: &str) {
        self.inner.set("ch", value);
    }
}
impl HTMLTableCellElement {
    /// Getter of the `chOff` attribute.
    /// [`HTMLTableCellElement.chOff`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/chOff)
    pub fn ch_off(&self) -> String {
        self.inner.get("chOff").as_::<String>()
    }

    /// Setter of the `chOff` attribute.
    /// [`HTMLTableCellElement.chOff`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/chOff)
    pub fn set_ch_off(&mut self, value: &str) {
        self.inner.set("chOff", value);
    }
}
impl HTMLTableCellElement {
    /// Getter of the `noWrap` attribute.
    /// [`HTMLTableCellElement.noWrap`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/noWrap)
    pub fn no_wrap(&self) -> bool {
        self.inner.get("noWrap").as_::<bool>()
    }

    /// Setter of the `noWrap` attribute.
    /// [`HTMLTableCellElement.noWrap`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/noWrap)
    pub fn set_no_wrap(&mut self, value: bool) {
        self.inner.set("noWrap", value);
    }
}
impl HTMLTableCellElement {
    /// Getter of the `vAlign` attribute.
    /// [`HTMLTableCellElement.vAlign`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/vAlign)
    pub fn v_align(&self) -> String {
        self.inner.get("vAlign").as_::<String>()
    }

    /// Setter of the `vAlign` attribute.
    /// [`HTMLTableCellElement.vAlign`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/vAlign)
    pub fn set_v_align(&mut self, value: &str) {
        self.inner.set("vAlign", value);
    }
}
impl HTMLTableCellElement {
    /// Getter of the `bgColor` attribute.
    /// [`HTMLTableCellElement.bgColor`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/bgColor)
    pub fn bg_color(&self) -> String {
        self.inner.get("bgColor").as_::<String>()
    }

    /// Setter of the `bgColor` attribute.
    /// [`HTMLTableCellElement.bgColor`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/bgColor)
    pub fn set_bg_color(&mut self, value: &str) {
        self.inner.set("bgColor", value);
    }
}
