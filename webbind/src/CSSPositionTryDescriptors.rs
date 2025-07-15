use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSPositionTryDescriptors {
    inner: CSSStyleDeclaration,
}

jsbind::utils::impl_dyn_cast!(CSSPositionTryDescriptors);

impl AsRef<emlite::Val> for CSSPositionTryDescriptors {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}

impl AsMut<emlite::Val> for CSSPositionTryDescriptors {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
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

impl From<&CSSPositionTryDescriptors> for emlite::Val {
    fn from(x: &CSSPositionTryDescriptors) -> emlite::Val {
        x.inner.clone().into()
    }
}

impl CSSPositionTryDescriptors {
    pub fn margin(&self) -> String {
        self.inner.get("margin").as_::<String>()
    }

    pub fn set_margin(&mut self, value: &str) {
        self.inner.set("margin", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn margin_top(&self) -> String {
        self.inner.get("marginTop").as_::<String>()
    }

    pub fn set_margin_top(&mut self, value: &str) {
        self.inner.set("marginTop", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn margin_right(&self) -> String {
        self.inner.get("marginRight").as_::<String>()
    }

    pub fn set_margin_right(&mut self, value: &str) {
        self.inner.set("marginRight", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn margin_bottom(&self) -> String {
        self.inner.get("marginBottom").as_::<String>()
    }

    pub fn set_margin_bottom(&mut self, value: &str) {
        self.inner.set("marginBottom", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn margin_left(&self) -> String {
        self.inner.get("marginLeft").as_::<String>()
    }

    pub fn set_margin_left(&mut self, value: &str) {
        self.inner.set("marginLeft", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn margin_block(&self) -> String {
        self.inner.get("marginBlock").as_::<String>()
    }

    pub fn set_margin_block(&mut self, value: &str) {
        self.inner.set("marginBlock", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn margin_block_start(&self) -> String {
        self.inner.get("marginBlockStart").as_::<String>()
    }

    pub fn set_margin_block_start(&mut self, value: &str) {
        self.inner.set("marginBlockStart", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn margin_block_end(&self) -> String {
        self.inner.get("marginBlockEnd").as_::<String>()
    }

    pub fn set_margin_block_end(&mut self, value: &str) {
        self.inner.set("marginBlockEnd", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn margin_inline(&self) -> String {
        self.inner.get("marginInline").as_::<String>()
    }

    pub fn set_margin_inline(&mut self, value: &str) {
        self.inner.set("marginInline", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn margin_inline_start(&self) -> String {
        self.inner.get("marginInlineStart").as_::<String>()
    }

    pub fn set_margin_inline_start(&mut self, value: &str) {
        self.inner.set("marginInlineStart", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn margin_inline_end(&self) -> String {
        self.inner.get("marginInlineEnd").as_::<String>()
    }

    pub fn set_margin_inline_end(&mut self, value: &str) {
        self.inner.set("marginInlineEnd", value);
    }
}

impl CSSPositionTryDescriptors {
    pub fn inset(&self) -> String {
        self.inner.get("inset").as_::<String>()
    }

    pub fn set_inset(&mut self, value: &str) {
        self.inner.set("inset", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn inset_block(&self) -> String {
        self.inner.get("insetBlock").as_::<String>()
    }

    pub fn set_inset_block(&mut self, value: &str) {
        self.inner.set("insetBlock", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn inset_block_start(&self) -> String {
        self.inner.get("insetBlockStart").as_::<String>()
    }

    pub fn set_inset_block_start(&mut self, value: &str) {
        self.inner.set("insetBlockStart", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn inset_block_end(&self) -> String {
        self.inner.get("insetBlockEnd").as_::<String>()
    }

    pub fn set_inset_block_end(&mut self, value: &str) {
        self.inner.set("insetBlockEnd", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn inset_inline(&self) -> String {
        self.inner.get("insetInline").as_::<String>()
    }

    pub fn set_inset_inline(&mut self, value: &str) {
        self.inner.set("insetInline", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn inset_inline_start(&self) -> String {
        self.inner.get("insetInlineStart").as_::<String>()
    }

    pub fn set_inset_inline_start(&mut self, value: &str) {
        self.inner.set("insetInlineStart", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn inset_inline_end(&self) -> String {
        self.inner.get("insetInlineEnd").as_::<String>()
    }

    pub fn set_inset_inline_end(&mut self, value: &str) {
        self.inner.set("insetInlineEnd", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn top(&self) -> String {
        self.inner.get("top").as_::<String>()
    }

    pub fn set_top(&mut self, value: &str) {
        self.inner.set("top", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn left(&self) -> String {
        self.inner.get("left").as_::<String>()
    }

    pub fn set_left(&mut self, value: &str) {
        self.inner.set("left", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn right(&self) -> String {
        self.inner.get("right").as_::<String>()
    }

    pub fn set_right(&mut self, value: &str) {
        self.inner.set("right", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn bottom(&self) -> String {
        self.inner.get("bottom").as_::<String>()
    }

    pub fn set_bottom(&mut self, value: &str) {
        self.inner.set("bottom", value);
    }
}

impl CSSPositionTryDescriptors {
    pub fn width(&self) -> String {
        self.inner.get("width").as_::<String>()
    }

    pub fn set_width(&mut self, value: &str) {
        self.inner.set("width", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn min_width(&self) -> String {
        self.inner.get("minWidth").as_::<String>()
    }

    pub fn set_min_width(&mut self, value: &str) {
        self.inner.set("minWidth", value);
    }
}

impl CSSPositionTryDescriptors {
    pub fn height(&self) -> String {
        self.inner.get("height").as_::<String>()
    }

    pub fn set_height(&mut self, value: &str) {
        self.inner.set("height", value);
    }
}

impl CSSPositionTryDescriptors {
    pub fn max_height(&self) -> String {
        self.inner.get("maxHeight").as_::<String>()
    }

    pub fn set_max_height(&mut self, value: &str) {
        self.inner.set("maxHeight", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn block_size(&self) -> String {
        self.inner.get("blockSize").as_::<String>()
    }

    pub fn set_block_size(&mut self, value: &str) {
        self.inner.set("blockSize", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn min_block_size(&self) -> String {
        self.inner.get("minBlockSize").as_::<String>()
    }

    pub fn set_min_block_size(&mut self, value: &str) {
        self.inner.set("minBlockSize", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn max_block_size(&self) -> String {
        self.inner.get("maxBlockSize").as_::<String>()
    }

    pub fn set_max_block_size(&mut self, value: &str) {
        self.inner.set("maxBlockSize", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn inline_size(&self) -> String {
        self.inner.get("inlineSize").as_::<String>()
    }

    pub fn set_inline_size(&mut self, value: &str) {
        self.inner.set("inlineSize", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn min_inline_size(&self) -> String {
        self.inner.get("minInlineSize").as_::<String>()
    }

    pub fn set_min_inline_size(&mut self, value: &str) {
        self.inner.set("minInlineSize", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn max_inline_size(&self) -> String {
        self.inner.get("maxInlineSize").as_::<String>()
    }

    pub fn set_max_inline_size(&mut self, value: &str) {
        self.inner.set("maxInlineSize", value);
    }
}

impl CSSPositionTryDescriptors {
    pub fn min_height(&self) -> String {
        self.inner.get("min-height").as_::<String>()
    }

    pub fn set_min_height(&mut self, value: &str) {
        self.inner.set("min-height", value);
    }
}

impl CSSPositionTryDescriptors {
    pub fn place_self(&self) -> String {
        self.inner.get("placeSelf").as_::<String>()
    }

    pub fn set_place_self(&mut self, value: &str) {
        self.inner.set("placeSelf", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn align_self(&self) -> String {
        self.inner.get("alignSelf").as_::<String>()
    }

    pub fn set_align_self(&mut self, value: &str) {
        self.inner.set("alignSelf", value);
    }
}
impl CSSPositionTryDescriptors {
    pub fn justify_self(&self) -> String {
        self.inner.get("justifySelf").as_::<String>()
    }

    pub fn set_justify_self(&mut self, value: &str) {
        self.inner.set("justifySelf", value);
    }
}

impl CSSPositionTryDescriptors {
    pub fn position_anchor(&self) -> String {
        self.inner.get("positionAnchor").as_::<String>()
    }

    pub fn set_position_anchor(&mut self, value: &str) {
        self.inner.set("positionAnchor", value);
    }
}

impl CSSPositionTryDescriptors {
    pub fn position_area(&self) -> String {
        self.inner.get("positionArea").as_::<String>()
    }

    pub fn set_position_area(&mut self, value: &str) {
        self.inner.set("positionArea", value);
    }
}
