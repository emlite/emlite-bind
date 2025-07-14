use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CSSPositionTryDescriptors {
    inner: CSSStyleDeclaration,
}
impl FromVal for CSSPositionTryDescriptors {
    fn from_val(v: &emlite::Val) -> Self {
        CSSPositionTryDescriptors {
            inner: CSSStyleDeclaration::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl From<CSSPositionTryDescriptors> for emlite::Val {
    fn from(x: CSSPositionTryDescriptors) -> emlite::Val {
        let handle = x.inner.as_handle();
        core::mem::forget(x);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSPositionTryDescriptors {
    pub fn margin(&self) -> jsbind::CSSOMString {
        self.inner.get("margin").as_::<jsbind::CSSOMString>()
    }

    pub fn set_margin(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("margin", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn margin_top(&self) -> jsbind::CSSOMString {
        self.inner.get("marginTop").as_::<jsbind::CSSOMString>()
    }

    pub fn set_margin_top(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("marginTop", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn margin_right(&self) -> jsbind::CSSOMString {
        self.inner.get("marginRight").as_::<jsbind::CSSOMString>()
    }

    pub fn set_margin_right(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("marginRight", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn margin_bottom(&self) -> jsbind::CSSOMString {
        self.inner.get("marginBottom").as_::<jsbind::CSSOMString>()
    }

    pub fn set_margin_bottom(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("marginBottom", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn margin_left(&self) -> jsbind::CSSOMString {
        self.inner.get("marginLeft").as_::<jsbind::CSSOMString>()
    }

    pub fn set_margin_left(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("marginLeft", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn margin_block(&self) -> jsbind::CSSOMString {
        self.inner.get("marginBlock").as_::<jsbind::CSSOMString>()
    }

    pub fn set_margin_block(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("marginBlock", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn margin_block_start(&self) -> jsbind::CSSOMString {
        self.inner
            .get("marginBlockStart")
            .as_::<jsbind::CSSOMString>()
    }

    pub fn set_margin_block_start(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("marginBlockStart", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn margin_block_end(&self) -> jsbind::CSSOMString {
        self.inner
            .get("marginBlockEnd")
            .as_::<jsbind::CSSOMString>()
    }

    pub fn set_margin_block_end(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("marginBlockEnd", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn margin_inline(&self) -> jsbind::CSSOMString {
        self.inner.get("marginInline").as_::<jsbind::CSSOMString>()
    }

    pub fn set_margin_inline(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("marginInline", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn margin_inline_start(&self) -> jsbind::CSSOMString {
        self.inner
            .get("marginInlineStart")
            .as_::<jsbind::CSSOMString>()
    }

    pub fn set_margin_inline_start(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("marginInlineStart", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn margin_inline_end(&self) -> jsbind::CSSOMString {
        self.inner
            .get("marginInlineEnd")
            .as_::<jsbind::CSSOMString>()
    }

    pub fn set_margin_inline_end(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("marginInlineEnd", value);
    }
}

impl CSSPositionTryDescriptors {
    pub fn inset(&self) -> jsbind::CSSOMString {
        self.inner.get("inset").as_::<jsbind::CSSOMString>()
    }

    pub fn set_inset(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("inset", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn inset_block(&self) -> jsbind::CSSOMString {
        self.inner.get("insetBlock").as_::<jsbind::CSSOMString>()
    }

    pub fn set_inset_block(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("insetBlock", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn inset_block_start(&self) -> jsbind::CSSOMString {
        self.inner
            .get("insetBlockStart")
            .as_::<jsbind::CSSOMString>()
    }

    pub fn set_inset_block_start(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("insetBlockStart", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn inset_block_end(&self) -> jsbind::CSSOMString {
        self.inner.get("insetBlockEnd").as_::<jsbind::CSSOMString>()
    }

    pub fn set_inset_block_end(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("insetBlockEnd", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn inset_inline(&self) -> jsbind::CSSOMString {
        self.inner.get("insetInline").as_::<jsbind::CSSOMString>()
    }

    pub fn set_inset_inline(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("insetInline", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn inset_inline_start(&self) -> jsbind::CSSOMString {
        self.inner
            .get("insetInlineStart")
            .as_::<jsbind::CSSOMString>()
    }

    pub fn set_inset_inline_start(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("insetInlineStart", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn inset_inline_end(&self) -> jsbind::CSSOMString {
        self.inner
            .get("insetInlineEnd")
            .as_::<jsbind::CSSOMString>()
    }

    pub fn set_inset_inline_end(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("insetInlineEnd", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn top(&self) -> jsbind::CSSOMString {
        self.inner.get("top").as_::<jsbind::CSSOMString>()
    }

    pub fn set_top(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("top", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn left(&self) -> jsbind::CSSOMString {
        self.inner.get("left").as_::<jsbind::CSSOMString>()
    }

    pub fn set_left(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("left", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn right(&self) -> jsbind::CSSOMString {
        self.inner.get("right").as_::<jsbind::CSSOMString>()
    }

    pub fn set_right(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("right", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn bottom(&self) -> jsbind::CSSOMString {
        self.inner.get("bottom").as_::<jsbind::CSSOMString>()
    }

    pub fn set_bottom(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("bottom", value);
    }
}

impl CSSPositionTryDescriptors {
    pub fn width(&self) -> jsbind::CSSOMString {
        self.inner.get("width").as_::<jsbind::CSSOMString>()
    }

    pub fn set_width(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("width", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn min_width(&self) -> jsbind::CSSOMString {
        self.inner.get("minWidth").as_::<jsbind::CSSOMString>()
    }

    pub fn set_min_width(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("minWidth", value);
    }
}

impl CSSPositionTryDescriptors {
    pub fn height(&self) -> jsbind::CSSOMString {
        self.inner.get("height").as_::<jsbind::CSSOMString>()
    }

    pub fn set_height(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("height", value);
    }
}

impl CSSPositionTryDescriptors {
    pub fn max_height(&self) -> jsbind::CSSOMString {
        self.inner.get("maxHeight").as_::<jsbind::CSSOMString>()
    }

    pub fn set_max_height(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("maxHeight", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn block_size(&self) -> jsbind::CSSOMString {
        self.inner.get("blockSize").as_::<jsbind::CSSOMString>()
    }

    pub fn set_block_size(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("blockSize", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn min_block_size(&self) -> jsbind::CSSOMString {
        self.inner.get("minBlockSize").as_::<jsbind::CSSOMString>()
    }

    pub fn set_min_block_size(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("minBlockSize", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn max_block_size(&self) -> jsbind::CSSOMString {
        self.inner.get("maxBlockSize").as_::<jsbind::CSSOMString>()
    }

    pub fn set_max_block_size(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("maxBlockSize", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn inline_size(&self) -> jsbind::CSSOMString {
        self.inner.get("inlineSize").as_::<jsbind::CSSOMString>()
    }

    pub fn set_inline_size(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("inlineSize", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn min_inline_size(&self) -> jsbind::CSSOMString {
        self.inner.get("minInlineSize").as_::<jsbind::CSSOMString>()
    }

    pub fn set_min_inline_size(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("minInlineSize", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn max_inline_size(&self) -> jsbind::CSSOMString {
        self.inner.get("maxInlineSize").as_::<jsbind::CSSOMString>()
    }

    pub fn set_max_inline_size(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("maxInlineSize", value);
    }
}

impl CSSPositionTryDescriptors {
    pub fn min_height(&self) -> jsbind::CSSOMString {
        self.inner.get("min-height").as_::<jsbind::CSSOMString>()
    }

    pub fn set_min_height(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("min-height", value);
    }
}

impl CSSPositionTryDescriptors {
    pub fn place_self(&self) -> jsbind::CSSOMString {
        self.inner.get("placeSelf").as_::<jsbind::CSSOMString>()
    }

    pub fn set_place_self(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("placeSelf", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn align_self(&self) -> jsbind::CSSOMString {
        self.inner.get("alignSelf").as_::<jsbind::CSSOMString>()
    }

    pub fn set_align_self(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("alignSelf", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn justify_self(&self) -> jsbind::CSSOMString {
        self.inner.get("justifySelf").as_::<jsbind::CSSOMString>()
    }

    pub fn set_justify_self(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("justifySelf", value);
    }
}

impl CSSPositionTryDescriptors {
    pub fn position_anchor(&self) -> jsbind::CSSOMString {
        self.inner
            .get("positionAnchor")
            .as_::<jsbind::CSSOMString>()
    }

    pub fn set_position_anchor(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("positionAnchor", value);
    }
}

impl CSSPositionTryDescriptors {
    pub fn position_area(&self) -> jsbind::CSSOMString {
        self.inner.get("positionArea").as_::<jsbind::CSSOMString>()
    }

    pub fn set_position_area(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("positionArea", value);
    }
}
