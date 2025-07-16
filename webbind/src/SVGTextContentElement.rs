use super::*;

/// The SVGTextContentElement class.
/// [`SVGTextContentElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGTextContentElement {
    inner: SVGGraphicsElement,
}
impl FromVal for SVGTextContentElement {
    fn from_val(v: &Any) -> Self {
        SVGTextContentElement {
            inner: SVGGraphicsElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGTextContentElement {
    type Target = SVGGraphicsElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGTextContentElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGTextContentElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGTextContentElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGTextContentElement> for Any {
    fn from(s: SVGTextContentElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGTextContentElement> for Any {
    fn from(s: &SVGTextContentElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGTextContentElement);

impl SVGTextContentElement {
    /// Getter of the `textLength` attribute.
    /// [`SVGTextContentElement.textLength`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/textLength)
    pub fn text_length(&self) -> SVGAnimatedLength {
        self.inner.get("textLength").as_::<SVGAnimatedLength>()
    }
}
impl SVGTextContentElement {
    /// Getter of the `lengthAdjust` attribute.
    /// [`SVGTextContentElement.lengthAdjust`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/lengthAdjust)
    pub fn length_adjust(&self) -> SVGAnimatedEnumeration {
        self.inner
            .get("lengthAdjust")
            .as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGTextContentElement {
    /// The getNumberOfChars method.
    /// [`SVGTextContentElement.getNumberOfChars`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getNumberOfChars)
    pub fn get_number_of_chars(&self) -> i32 {
        self.inner.call("getNumberOfChars", &[]).as_::<i32>()
    }
}
impl SVGTextContentElement {
    /// The getComputedTextLength method.
    /// [`SVGTextContentElement.getComputedTextLength`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getComputedTextLength)
    pub fn get_computed_text_length(&self) -> f32 {
        self.inner.call("getComputedTextLength", &[]).as_::<f32>()
    }
}
impl SVGTextContentElement {
    /// The getSubStringLength method.
    /// [`SVGTextContentElement.getSubStringLength`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getSubStringLength)
    pub fn get_sub_string_length(&self, charnum: u32, nchars: u32) -> f32 {
        self.inner
            .call("getSubStringLength", &[charnum.into(), nchars.into()])
            .as_::<f32>()
    }
}
impl SVGTextContentElement {
    /// The getStartPositionOfChar method.
    /// [`SVGTextContentElement.getStartPositionOfChar`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getStartPositionOfChar)
    pub fn get_start_position_of_char(&self, charnum: u32) -> DOMPoint {
        self.inner
            .call("getStartPositionOfChar", &[charnum.into()])
            .as_::<DOMPoint>()
    }
}
impl SVGTextContentElement {
    /// The getEndPositionOfChar method.
    /// [`SVGTextContentElement.getEndPositionOfChar`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getEndPositionOfChar)
    pub fn get_end_position_of_char(&self, charnum: u32) -> DOMPoint {
        self.inner
            .call("getEndPositionOfChar", &[charnum.into()])
            .as_::<DOMPoint>()
    }
}
impl SVGTextContentElement {
    /// The getExtentOfChar method.
    /// [`SVGTextContentElement.getExtentOfChar`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getExtentOfChar)
    pub fn get_extent_of_char(&self, charnum: u32) -> DOMRect {
        self.inner
            .call("getExtentOfChar", &[charnum.into()])
            .as_::<DOMRect>()
    }
}
impl SVGTextContentElement {
    /// The getRotationOfChar method.
    /// [`SVGTextContentElement.getRotationOfChar`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getRotationOfChar)
    pub fn get_rotation_of_char(&self, charnum: u32) -> f32 {
        self.inner
            .call("getRotationOfChar", &[charnum.into()])
            .as_::<f32>()
    }
}
impl SVGTextContentElement {
    /// The getCharNumAtPosition method.
    /// [`SVGTextContentElement.getCharNumAtPosition`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getCharNumAtPosition)
    pub fn get_char_num_at_position0(&self) -> i32 {
        self.inner.call("getCharNumAtPosition", &[]).as_::<i32>()
    }
    /// The getCharNumAtPosition method.
    /// [`SVGTextContentElement.getCharNumAtPosition`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getCharNumAtPosition)
    pub fn get_char_num_at_position1(&self, point: &DOMPointInit) -> i32 {
        self.inner
            .call("getCharNumAtPosition", &[point.into()])
            .as_::<i32>()
    }
}
impl SVGTextContentElement {
    /// The selectSubString method.
    /// [`SVGTextContentElement.selectSubString`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/selectSubString)
    pub fn select_sub_string(&self, charnum: u32, nchars: u32) -> Undefined {
        self.inner
            .call("selectSubString", &[charnum.into(), nchars.into()])
            .as_::<Undefined>()
    }
}
