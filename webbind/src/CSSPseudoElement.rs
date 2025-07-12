use super::*;

#[derive(Clone, Debug)]
pub struct CSSPseudoElement {
    inner: EventTarget,
}
impl FromVal for CSSPseudoElement {
    fn from_val(v: &emlite::Val) -> Self {
        CSSPseudoElement {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CSSPseudoElement {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSPseudoElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSPseudoElement> for emlite::Val {
    fn from(s: CSSPseudoElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSPseudoElement {
    pub fn type_(&self) -> jsbind::CSSOMString {
        self.inner.get("type").as_::<jsbind::CSSOMString>()
    }
}
impl CSSPseudoElement {
    pub fn element(&self) -> Element {
        self.inner.get("element").as_::<Element>()
    }
}
impl CSSPseudoElement {
    pub fn parent(&self) -> jsbind::Any {
        self.inner.get("parent").as_::<jsbind::Any>()
    }
}
impl CSSPseudoElement {
    pub fn pseudo(&self, type_: jsbind::CSSOMString) -> CSSPseudoElement {
        self.inner
            .call("pseudo", &[type_.into()])
            .as_::<CSSPseudoElement>()
    }
}
impl CSSPseudoElement {
    pub fn get_box_quads0(&self) -> jsbind::Sequence<DOMQuad> {
        self.inner
            .call("getBoxQuads", &[])
            .as_::<jsbind::Sequence<DOMQuad>>()
    }

    pub fn get_box_quads1(&self, options: BoxQuadOptions) -> jsbind::Sequence<DOMQuad> {
        self.inner
            .call("getBoxQuads", &[options.into()])
            .as_::<jsbind::Sequence<DOMQuad>>()
    }
}
impl CSSPseudoElement {
    pub fn convert_quad_from_node0(&self, quad: DOMQuadInit, from: jsbind::Any) -> DOMQuad {
        self.inner
            .call("convertQuadFromNode", &[quad.into(), from.into()])
            .as_::<DOMQuad>()
    }

    pub fn convert_quad_from_node1(
        &self,
        quad: DOMQuadInit,
        from: jsbind::Any,
        options: ConvertCoordinateOptions,
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
    pub fn convert_rect_from_node0(&self, rect: DOMRectReadOnly, from: jsbind::Any) -> DOMQuad {
        self.inner
            .call("convertRectFromNode", &[rect.into(), from.into()])
            .as_::<DOMQuad>()
    }

    pub fn convert_rect_from_node1(
        &self,
        rect: DOMRectReadOnly,
        from: jsbind::Any,
        options: ConvertCoordinateOptions,
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
    pub fn convert_point_from_node0(&self, point: DOMPointInit, from: jsbind::Any) -> DOMPoint {
        self.inner
            .call("convertPointFromNode", &[point.into(), from.into()])
            .as_::<DOMPoint>()
    }

    pub fn convert_point_from_node1(
        &self,
        point: DOMPointInit,
        from: jsbind::Any,
        options: ConvertCoordinateOptions,
    ) -> DOMPoint {
        self.inner
            .call(
                "convertPointFromNode",
                &[point.into(), from.into(), options.into()],
            )
            .as_::<DOMPoint>()
    }
}
