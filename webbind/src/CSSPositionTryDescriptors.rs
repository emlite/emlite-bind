use super::*;

/// The CSSPositionTryDescriptors class.
/// [`CSSPositionTryDescriptors`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSPositionTryDescriptors {
    inner: CSSStyleDeclaration,
}
impl FromVal for CSSPositionTryDescriptors {
    fn from_val(v: &Any) -> Self {
        CSSPositionTryDescriptors {
            inner: CSSStyleDeclaration::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSPositionTryDescriptors {
    type Target = CSSStyleDeclaration;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSPositionTryDescriptors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSPositionTryDescriptors {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSPositionTryDescriptors {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSPositionTryDescriptors> for Any {
    fn from(s: CSSPositionTryDescriptors) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSPositionTryDescriptors> for Any {
    fn from(s: &CSSPositionTryDescriptors) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSPositionTryDescriptors);

impl CSSPositionTryDescriptors {
    /// Getter of the `margin` attribute.
    /// [`CSSPositionTryDescriptors.margin`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/margin)
    pub fn margin(&self) -> JsString {
        self.inner.get("margin").as_::<JsString>()
    }

    /// Setter of the `margin` attribute.
    /// [`CSSPositionTryDescriptors.margin`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/margin)
    pub fn set_margin(&mut self, value: &JsString) {
        self.inner.set("margin", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `marginTop` attribute.
    /// [`CSSPositionTryDescriptors.marginTop`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/marginTop)
    pub fn margin_top(&self) -> JsString {
        self.inner.get("marginTop").as_::<JsString>()
    }

    /// Setter of the `marginTop` attribute.
    /// [`CSSPositionTryDescriptors.marginTop`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/marginTop)
    pub fn set_margin_top(&mut self, value: &JsString) {
        self.inner.set("marginTop", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `marginRight` attribute.
    /// [`CSSPositionTryDescriptors.marginRight`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/marginRight)
    pub fn margin_right(&self) -> JsString {
        self.inner.get("marginRight").as_::<JsString>()
    }

    /// Setter of the `marginRight` attribute.
    /// [`CSSPositionTryDescriptors.marginRight`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/marginRight)
    pub fn set_margin_right(&mut self, value: &JsString) {
        self.inner.set("marginRight", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `marginBottom` attribute.
    /// [`CSSPositionTryDescriptors.marginBottom`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/marginBottom)
    pub fn margin_bottom(&self) -> JsString {
        self.inner.get("marginBottom").as_::<JsString>()
    }

    /// Setter of the `marginBottom` attribute.
    /// [`CSSPositionTryDescriptors.marginBottom`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/marginBottom)
    pub fn set_margin_bottom(&mut self, value: &JsString) {
        self.inner.set("marginBottom", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `marginLeft` attribute.
    /// [`CSSPositionTryDescriptors.marginLeft`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/marginLeft)
    pub fn margin_left(&self) -> JsString {
        self.inner.get("marginLeft").as_::<JsString>()
    }

    /// Setter of the `marginLeft` attribute.
    /// [`CSSPositionTryDescriptors.marginLeft`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/marginLeft)
    pub fn set_margin_left(&mut self, value: &JsString) {
        self.inner.set("marginLeft", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `marginBlock` attribute.
    /// [`CSSPositionTryDescriptors.marginBlock`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/marginBlock)
    pub fn margin_block(&self) -> JsString {
        self.inner.get("marginBlock").as_::<JsString>()
    }

    /// Setter of the `marginBlock` attribute.
    /// [`CSSPositionTryDescriptors.marginBlock`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/marginBlock)
    pub fn set_margin_block(&mut self, value: &JsString) {
        self.inner.set("marginBlock", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `marginBlockStart` attribute.
    /// [`CSSPositionTryDescriptors.marginBlockStart`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/marginBlockStart)
    pub fn margin_block_start(&self) -> JsString {
        self.inner.get("marginBlockStart").as_::<JsString>()
    }

    /// Setter of the `marginBlockStart` attribute.
    /// [`CSSPositionTryDescriptors.marginBlockStart`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/marginBlockStart)
    pub fn set_margin_block_start(&mut self, value: &JsString) {
        self.inner.set("marginBlockStart", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `marginBlockEnd` attribute.
    /// [`CSSPositionTryDescriptors.marginBlockEnd`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/marginBlockEnd)
    pub fn margin_block_end(&self) -> JsString {
        self.inner.get("marginBlockEnd").as_::<JsString>()
    }

    /// Setter of the `marginBlockEnd` attribute.
    /// [`CSSPositionTryDescriptors.marginBlockEnd`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/marginBlockEnd)
    pub fn set_margin_block_end(&mut self, value: &JsString) {
        self.inner.set("marginBlockEnd", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `marginInline` attribute.
    /// [`CSSPositionTryDescriptors.marginInline`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/marginInline)
    pub fn margin_inline(&self) -> JsString {
        self.inner.get("marginInline").as_::<JsString>()
    }

    /// Setter of the `marginInline` attribute.
    /// [`CSSPositionTryDescriptors.marginInline`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/marginInline)
    pub fn set_margin_inline(&mut self, value: &JsString) {
        self.inner.set("marginInline", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `marginInlineStart` attribute.
    /// [`CSSPositionTryDescriptors.marginInlineStart`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/marginInlineStart)
    pub fn margin_inline_start(&self) -> JsString {
        self.inner.get("marginInlineStart").as_::<JsString>()
    }

    /// Setter of the `marginInlineStart` attribute.
    /// [`CSSPositionTryDescriptors.marginInlineStart`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/marginInlineStart)
    pub fn set_margin_inline_start(&mut self, value: &JsString) {
        self.inner.set("marginInlineStart", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `marginInlineEnd` attribute.
    /// [`CSSPositionTryDescriptors.marginInlineEnd`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/marginInlineEnd)
    pub fn margin_inline_end(&self) -> JsString {
        self.inner.get("marginInlineEnd").as_::<JsString>()
    }

    /// Setter of the `marginInlineEnd` attribute.
    /// [`CSSPositionTryDescriptors.marginInlineEnd`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/marginInlineEnd)
    pub fn set_margin_inline_end(&mut self, value: &JsString) {
        self.inner.set("marginInlineEnd", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `inset` attribute.
    /// [`CSSPositionTryDescriptors.inset`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/inset)
    pub fn inset(&self) -> JsString {
        self.inner.get("inset").as_::<JsString>()
    }

    /// Setter of the `inset` attribute.
    /// [`CSSPositionTryDescriptors.inset`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/inset)
    pub fn set_inset(&mut self, value: &JsString) {
        self.inner.set("inset", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `insetBlock` attribute.
    /// [`CSSPositionTryDescriptors.insetBlock`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/insetBlock)
    pub fn inset_block(&self) -> JsString {
        self.inner.get("insetBlock").as_::<JsString>()
    }

    /// Setter of the `insetBlock` attribute.
    /// [`CSSPositionTryDescriptors.insetBlock`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/insetBlock)
    pub fn set_inset_block(&mut self, value: &JsString) {
        self.inner.set("insetBlock", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `insetBlockStart` attribute.
    /// [`CSSPositionTryDescriptors.insetBlockStart`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/insetBlockStart)
    pub fn inset_block_start(&self) -> JsString {
        self.inner.get("insetBlockStart").as_::<JsString>()
    }

    /// Setter of the `insetBlockStart` attribute.
    /// [`CSSPositionTryDescriptors.insetBlockStart`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/insetBlockStart)
    pub fn set_inset_block_start(&mut self, value: &JsString) {
        self.inner.set("insetBlockStart", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `insetBlockEnd` attribute.
    /// [`CSSPositionTryDescriptors.insetBlockEnd`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/insetBlockEnd)
    pub fn inset_block_end(&self) -> JsString {
        self.inner.get("insetBlockEnd").as_::<JsString>()
    }

    /// Setter of the `insetBlockEnd` attribute.
    /// [`CSSPositionTryDescriptors.insetBlockEnd`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/insetBlockEnd)
    pub fn set_inset_block_end(&mut self, value: &JsString) {
        self.inner.set("insetBlockEnd", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `insetInline` attribute.
    /// [`CSSPositionTryDescriptors.insetInline`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/insetInline)
    pub fn inset_inline(&self) -> JsString {
        self.inner.get("insetInline").as_::<JsString>()
    }

    /// Setter of the `insetInline` attribute.
    /// [`CSSPositionTryDescriptors.insetInline`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/insetInline)
    pub fn set_inset_inline(&mut self, value: &JsString) {
        self.inner.set("insetInline", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `insetInlineStart` attribute.
    /// [`CSSPositionTryDescriptors.insetInlineStart`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/insetInlineStart)
    pub fn inset_inline_start(&self) -> JsString {
        self.inner.get("insetInlineStart").as_::<JsString>()
    }

    /// Setter of the `insetInlineStart` attribute.
    /// [`CSSPositionTryDescriptors.insetInlineStart`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/insetInlineStart)
    pub fn set_inset_inline_start(&mut self, value: &JsString) {
        self.inner.set("insetInlineStart", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `insetInlineEnd` attribute.
    /// [`CSSPositionTryDescriptors.insetInlineEnd`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/insetInlineEnd)
    pub fn inset_inline_end(&self) -> JsString {
        self.inner.get("insetInlineEnd").as_::<JsString>()
    }

    /// Setter of the `insetInlineEnd` attribute.
    /// [`CSSPositionTryDescriptors.insetInlineEnd`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/insetInlineEnd)
    pub fn set_inset_inline_end(&mut self, value: &JsString) {
        self.inner.set("insetInlineEnd", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `top` attribute.
    /// [`CSSPositionTryDescriptors.top`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/top)
    pub fn top(&self) -> JsString {
        self.inner.get("top").as_::<JsString>()
    }

    /// Setter of the `top` attribute.
    /// [`CSSPositionTryDescriptors.top`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/top)
    pub fn set_top(&mut self, value: &JsString) {
        self.inner.set("top", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `left` attribute.
    /// [`CSSPositionTryDescriptors.left`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/left)
    pub fn left(&self) -> JsString {
        self.inner.get("left").as_::<JsString>()
    }

    /// Setter of the `left` attribute.
    /// [`CSSPositionTryDescriptors.left`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/left)
    pub fn set_left(&mut self, value: &JsString) {
        self.inner.set("left", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `right` attribute.
    /// [`CSSPositionTryDescriptors.right`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/right)
    pub fn right(&self) -> JsString {
        self.inner.get("right").as_::<JsString>()
    }

    /// Setter of the `right` attribute.
    /// [`CSSPositionTryDescriptors.right`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/right)
    pub fn set_right(&mut self, value: &JsString) {
        self.inner.set("right", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `bottom` attribute.
    /// [`CSSPositionTryDescriptors.bottom`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/bottom)
    pub fn bottom(&self) -> JsString {
        self.inner.get("bottom").as_::<JsString>()
    }

    /// Setter of the `bottom` attribute.
    /// [`CSSPositionTryDescriptors.bottom`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/bottom)
    pub fn set_bottom(&mut self, value: &JsString) {
        self.inner.set("bottom", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `width` attribute.
    /// [`CSSPositionTryDescriptors.width`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/width)
    pub fn width(&self) -> JsString {
        self.inner.get("width").as_::<JsString>()
    }

    /// Setter of the `width` attribute.
    /// [`CSSPositionTryDescriptors.width`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/width)
    pub fn set_width(&mut self, value: &JsString) {
        self.inner.set("width", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `minWidth` attribute.
    /// [`CSSPositionTryDescriptors.minWidth`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/minWidth)
    pub fn min_width(&self) -> JsString {
        self.inner.get("minWidth").as_::<JsString>()
    }

    /// Setter of the `minWidth` attribute.
    /// [`CSSPositionTryDescriptors.minWidth`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/minWidth)
    pub fn set_min_width(&mut self, value: &JsString) {
        self.inner.set("minWidth", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `maxWidth` attribute.
    /// [`CSSPositionTryDescriptors.maxWidth`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/maxWidth)
    pub fn max_width(&self) -> JsString {
        self.inner.get("maxWidth").as_::<JsString>()
    }

    /// Setter of the `maxWidth` attribute.
    /// [`CSSPositionTryDescriptors.maxWidth`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/maxWidth)
    pub fn set_max_width(&mut self, value: &JsString) {
        self.inner.set("maxWidth", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `height` attribute.
    /// [`CSSPositionTryDescriptors.height`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/height)
    pub fn height(&self) -> JsString {
        self.inner.get("height").as_::<JsString>()
    }

    /// Setter of the `height` attribute.
    /// [`CSSPositionTryDescriptors.height`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/height)
    pub fn set_height(&mut self, value: &JsString) {
        self.inner.set("height", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `minHeight` attribute.
    /// [`CSSPositionTryDescriptors.minHeight`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/minHeight)
    pub fn min_height(&self) -> JsString {
        self.inner.get("minHeight").as_::<JsString>()
    }

    /// Setter of the `minHeight` attribute.
    /// [`CSSPositionTryDescriptors.minHeight`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/minHeight)
    pub fn set_min_height(&mut self, value: &JsString) {
        self.inner.set("minHeight", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `maxHeight` attribute.
    /// [`CSSPositionTryDescriptors.maxHeight`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/maxHeight)
    pub fn max_height(&self) -> JsString {
        self.inner.get("maxHeight").as_::<JsString>()
    }

    /// Setter of the `maxHeight` attribute.
    /// [`CSSPositionTryDescriptors.maxHeight`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/maxHeight)
    pub fn set_max_height(&mut self, value: &JsString) {
        self.inner.set("maxHeight", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `blockSize` attribute.
    /// [`CSSPositionTryDescriptors.blockSize`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/blockSize)
    pub fn block_size(&self) -> JsString {
        self.inner.get("blockSize").as_::<JsString>()
    }

    /// Setter of the `blockSize` attribute.
    /// [`CSSPositionTryDescriptors.blockSize`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/blockSize)
    pub fn set_block_size(&mut self, value: &JsString) {
        self.inner.set("blockSize", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `minBlockSize` attribute.
    /// [`CSSPositionTryDescriptors.minBlockSize`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/minBlockSize)
    pub fn min_block_size(&self) -> JsString {
        self.inner.get("minBlockSize").as_::<JsString>()
    }

    /// Setter of the `minBlockSize` attribute.
    /// [`CSSPositionTryDescriptors.minBlockSize`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/minBlockSize)
    pub fn set_min_block_size(&mut self, value: &JsString) {
        self.inner.set("minBlockSize", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `maxBlockSize` attribute.
    /// [`CSSPositionTryDescriptors.maxBlockSize`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/maxBlockSize)
    pub fn max_block_size(&self) -> JsString {
        self.inner.get("maxBlockSize").as_::<JsString>()
    }

    /// Setter of the `maxBlockSize` attribute.
    /// [`CSSPositionTryDescriptors.maxBlockSize`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/maxBlockSize)
    pub fn set_max_block_size(&mut self, value: &JsString) {
        self.inner.set("maxBlockSize", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `inlineSize` attribute.
    /// [`CSSPositionTryDescriptors.inlineSize`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/inlineSize)
    pub fn inline_size(&self) -> JsString {
        self.inner.get("inlineSize").as_::<JsString>()
    }

    /// Setter of the `inlineSize` attribute.
    /// [`CSSPositionTryDescriptors.inlineSize`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/inlineSize)
    pub fn set_inline_size(&mut self, value: &JsString) {
        self.inner.set("inlineSize", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `minInlineSize` attribute.
    /// [`CSSPositionTryDescriptors.minInlineSize`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/minInlineSize)
    pub fn min_inline_size(&self) -> JsString {
        self.inner.get("minInlineSize").as_::<JsString>()
    }

    /// Setter of the `minInlineSize` attribute.
    /// [`CSSPositionTryDescriptors.minInlineSize`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/minInlineSize)
    pub fn set_min_inline_size(&mut self, value: &JsString) {
        self.inner.set("minInlineSize", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `maxInlineSize` attribute.
    /// [`CSSPositionTryDescriptors.maxInlineSize`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/maxInlineSize)
    pub fn max_inline_size(&self) -> JsString {
        self.inner.get("maxInlineSize").as_::<JsString>()
    }

    /// Setter of the `maxInlineSize` attribute.
    /// [`CSSPositionTryDescriptors.maxInlineSize`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/maxInlineSize)
    pub fn set_max_inline_size(&mut self, value: &JsString) {
        self.inner.set("maxInlineSize", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `placeSelf` attribute.
    /// [`CSSPositionTryDescriptors.placeSelf`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/placeSelf)
    pub fn place_self(&self) -> JsString {
        self.inner.get("placeSelf").as_::<JsString>()
    }

    /// Setter of the `placeSelf` attribute.
    /// [`CSSPositionTryDescriptors.placeSelf`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/placeSelf)
    pub fn set_place_self(&mut self, value: &JsString) {
        self.inner.set("placeSelf", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `alignSelf` attribute.
    /// [`CSSPositionTryDescriptors.alignSelf`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/alignSelf)
    pub fn align_self(&self) -> JsString {
        self.inner.get("alignSelf").as_::<JsString>()
    }

    /// Setter of the `alignSelf` attribute.
    /// [`CSSPositionTryDescriptors.alignSelf`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/alignSelf)
    pub fn set_align_self(&mut self, value: &JsString) {
        self.inner.set("alignSelf", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `justifySelf` attribute.
    /// [`CSSPositionTryDescriptors.justifySelf`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/justifySelf)
    pub fn justify_self(&self) -> JsString {
        self.inner.get("justifySelf").as_::<JsString>()
    }

    /// Setter of the `justifySelf` attribute.
    /// [`CSSPositionTryDescriptors.justifySelf`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/justifySelf)
    pub fn set_justify_self(&mut self, value: &JsString) {
        self.inner.set("justifySelf", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `positionAnchor` attribute.
    /// [`CSSPositionTryDescriptors.positionAnchor`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/positionAnchor)
    pub fn position_anchor(&self) -> JsString {
        self.inner.get("positionAnchor").as_::<JsString>()
    }

    /// Setter of the `positionAnchor` attribute.
    /// [`CSSPositionTryDescriptors.positionAnchor`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/positionAnchor)
    pub fn set_position_anchor(&mut self, value: &JsString) {
        self.inner.set("positionAnchor", value);
    }
}
impl CSSPositionTryDescriptors {
    /// Getter of the `positionArea` attribute.
    /// [`CSSPositionTryDescriptors.positionArea`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/positionArea)
    pub fn position_area(&self) -> JsString {
        self.inner.get("positionArea").as_::<JsString>()
    }

    /// Setter of the `positionArea` attribute.
    /// [`CSSPositionTryDescriptors.positionArea`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryDescriptors/positionArea)
    pub fn set_position_area(&mut self, value: &JsString) {
        self.inner.set("positionArea", value);
    }
}
