use super::*;

/// The CSSPseudoElement class.
/// [`CSSPseudoElement`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSPseudoElement {
    inner: EventTarget,
}

impl FromVal for CSSPseudoElement {
    fn from_val(v: &Any) -> Self {
        CSSPseudoElement {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSPseudoElement {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSPseudoElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSPseudoElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSPseudoElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSPseudoElement> for Any {
    fn from(s: CSSPseudoElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSPseudoElement> for Any {
    fn from(s: &CSSPseudoElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSPseudoElement);

impl CSSPseudoElement {
    /// Getter of the `type` attribute.
    /// [`CSSPseudoElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/type)
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }
}
impl CSSPseudoElement {
    /// Getter of the `element` attribute.
    /// [`CSSPseudoElement.element`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/element)
    pub fn element(&self) -> Element {
        self.inner.get("element").as_::<Element>()
    }
}
impl CSSPseudoElement {
    /// Getter of the `parent` attribute.
    /// [`CSSPseudoElement.parent`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/parent)
    pub fn parent(&self) -> Any {
        self.inner.get("parent").as_::<Any>()
    }
}
impl CSSPseudoElement {
    /// The pseudo method.
    /// [`CSSPseudoElement.pseudo`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/pseudo)
    pub fn pseudo(&self, type_: &JsString) -> CSSPseudoElement {
        self.inner
            .call("pseudo", &[type_.into()])
            .as_::<CSSPseudoElement>()
    }
}
impl CSSPseudoElement {
    /// The getBoxQuads method.
    /// [`CSSPseudoElement.getBoxQuads`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/getBoxQuads)
    pub fn get_box_quads(&self) -> TypedArray<DOMQuad> {
        self.inner
            .call("getBoxQuads", &[])
            .as_::<TypedArray<DOMQuad>>()
    }
}
impl CSSPseudoElement {
    /// The getBoxQuads method.
    /// [`CSSPseudoElement.getBoxQuads`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/getBoxQuads)
    pub fn get_box_quads_with_options(&self, options: &BoxQuadOptions) -> TypedArray<DOMQuad> {
        self.inner
            .call("getBoxQuads", &[options.into()])
            .as_::<TypedArray<DOMQuad>>()
    }
}
impl CSSPseudoElement {
    /// The convertQuadFromNode method.
    /// [`CSSPseudoElement.convertQuadFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/convertQuadFromNode)
    pub fn convert_quad_from_node(&self, quad: &DOMQuadInit, from: &Any) -> DOMQuad {
        self.inner
            .call("convertQuadFromNode", &[quad.into(), from.into()])
            .as_::<DOMQuad>()
    }
}
impl CSSPseudoElement {
    /// The convertQuadFromNode method.
    /// [`CSSPseudoElement.convertQuadFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/convertQuadFromNode)
    pub fn convert_quad_from_node_with_options(
        &self,
        quad: &DOMQuadInit,
        from: &Any,
        options: &ConvertCoordinateOptions,
    ) -> DOMQuad {
        self.inner
            .call(
                "convertQuadFromNode",
                &[quad.into(), from.into(), options.into()],
            )
            .as_::<DOMQuad>()
    }
}
impl CSSPseudoElement {
    /// The convertRectFromNode method.
    /// [`CSSPseudoElement.convertRectFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/convertRectFromNode)
    pub fn convert_rect_from_node(&self, rect: &DOMRectReadOnly, from: &Any) -> DOMQuad {
        self.inner
            .call("convertRectFromNode", &[rect.into(), from.into()])
            .as_::<DOMQuad>()
    }
}
impl CSSPseudoElement {
    /// The convertRectFromNode method.
    /// [`CSSPseudoElement.convertRectFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/convertRectFromNode)
    pub fn convert_rect_from_node_with_options(
        &self,
        rect: &DOMRectReadOnly,
        from: &Any,
        options: &ConvertCoordinateOptions,
    ) -> DOMQuad {
        self.inner
            .call(
                "convertRectFromNode",
                &[rect.into(), from.into(), options.into()],
            )
            .as_::<DOMQuad>()
    }
}
impl CSSPseudoElement {
    /// The convertPointFromNode method.
    /// [`CSSPseudoElement.convertPointFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/convertPointFromNode)
    pub fn convert_point_from_node(&self, point: &DOMPointInit, from: &Any) -> DOMPoint {
        self.inner
            .call("convertPointFromNode", &[point.into(), from.into()])
            .as_::<DOMPoint>()
    }
}
impl CSSPseudoElement {
    /// The convertPointFromNode method.
    /// [`CSSPseudoElement.convertPointFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/convertPointFromNode)
    pub fn convert_point_from_node_with_options(
        &self,
        point: &DOMPointInit,
        from: &Any,
        options: &ConvertCoordinateOptions,
    ) -> DOMPoint {
        self.inner
            .call(
                "convertPointFromNode",
                &[point.into(), from.into(), options.into()],
            )
            .as_::<DOMPoint>()
    }
}
