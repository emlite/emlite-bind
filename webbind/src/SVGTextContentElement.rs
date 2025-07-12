use super::*;

#[derive(Clone, Debug)]
pub struct SVGTextContentElement {
    inner: SVGGraphicsElement,
}
impl FromVal for SVGTextContentElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGTextContentElement {
            inner: SVGGraphicsElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SVGTextContentElement {
    type Target = SVGGraphicsElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGTextContentElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGTextContentElement> for emlite::Val {
    fn from(s: SVGTextContentElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGTextContentElement {
    pub fn text_length(&self) -> SVGAnimatedLength {
        self.inner.get("textLength").as_::<SVGAnimatedLength>()
    }
}
impl SVGTextContentElement {
    pub fn length_adjust(&self) -> SVGAnimatedEnumeration {
        self.inner
            .get("lengthAdjust")
            .as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGTextContentElement {
    pub fn get_number_of_chars(&self) -> i32 {
        self.inner.call("getNumberOfChars", &[]).as_::<i32>()
    }
}
impl SVGTextContentElement {
    pub fn get_computed_text_length(&self) -> f32 {
        self.inner.call("getComputedTextLength", &[]).as_::<f32>()
    }
}
impl SVGTextContentElement {
    pub fn get_sub_string_length(&self, charnum: u32, nchars: u32) -> f32 {
        self.inner
            .call("getSubStringLength", &[charnum.into(), nchars.into()])
            .as_::<f32>()
    }
}
impl SVGTextContentElement {
    pub fn get_start_position_of_char(&self, charnum: u32) -> DOMPoint {
        self.inner
            .call("getStartPositionOfChar", &[charnum.into()])
            .as_::<DOMPoint>()
    }
}
impl SVGTextContentElement {
    pub fn get_end_position_of_char(&self, charnum: u32) -> DOMPoint {
        self.inner
            .call("getEndPositionOfChar", &[charnum.into()])
            .as_::<DOMPoint>()
    }
}
impl SVGTextContentElement {
    pub fn get_extent_of_char(&self, charnum: u32) -> DOMRect {
        self.inner
            .call("getExtentOfChar", &[charnum.into()])
            .as_::<DOMRect>()
    }
}
impl SVGTextContentElement {
    pub fn get_rotation_of_char(&self, charnum: u32) -> f32 {
        self.inner
            .call("getRotationOfChar", &[charnum.into()])
            .as_::<f32>()
    }
}
impl SVGTextContentElement {
    pub fn get_char_num_at_position0(&self) -> i32 {
        self.inner.call("getCharNumAtPosition", &[]).as_::<i32>()
    }

    pub fn get_char_num_at_position1(&self, point: DOMPointInit) -> i32 {
        self.inner
            .call("getCharNumAtPosition", &[point.into()])
            .as_::<i32>()
    }
}
impl SVGTextContentElement {
    pub fn select_sub_string(&self, charnum: u32, nchars: u32) -> jsbind::Undefined {
        self.inner
            .call("selectSubString", &[charnum.into(), nchars.into()])
            .as_::<jsbind::Undefined>()
    }
}
