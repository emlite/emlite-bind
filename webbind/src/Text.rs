use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Text {
    inner: CharacterData,
}
impl FromVal for Text {
    fn from_val(v: &emlite::Val) -> Self {
        Text {
            inner: CharacterData::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Text {
    type Target = CharacterData;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Text {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Text {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Text {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Text> for emlite::Val {
    fn from(s: Text) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Text> for emlite::Val {
    fn from(s: &Text) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Text);

impl Text {
    pub fn new0() -> Text {
        Self {
            inner: emlite::Val::global("Text").new(&[]).as_::<CharacterData>(),
        }
    }

    pub fn new1(data: &str) -> Text {
        Self {
            inner: emlite::Val::global("Text")
                .new(&[data.into()])
                .as_::<CharacterData>(),
        }
    }
}
impl Text {
    pub fn split_text(&self, offset: u32) -> Text {
        self.inner.call("splitText", &[offset.into()]).as_::<Text>()
    }
}
impl Text {
    pub fn whole_text(&self) -> String {
        self.inner.get("wholeText").as_::<String>()
    }
}
impl Text {
    pub fn get_box_quads0(&self) -> Sequence<DOMQuad> {
        self.inner
            .call("getBoxQuads", &[])
            .as_::<Sequence<DOMQuad>>()
    }

    pub fn get_box_quads1(&self, options: &BoxQuadOptions) -> Sequence<DOMQuad> {
        self.inner
            .call("getBoxQuads", &[options.into()])
            .as_::<Sequence<DOMQuad>>()
    }
}
impl Text {
    pub fn convert_quad_from_node0(&self, quad: &DOMQuadInit, from: &Any) -> DOMQuad {
        self.inner
            .call("convertQuadFromNode", &[quad.into(), from.into()])
            .as_::<DOMQuad>()
    }

    pub fn convert_quad_from_node1(
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
impl Text {
    pub fn convert_rect_from_node0(&self, rect: &DOMRectReadOnly, from: &Any) -> DOMQuad {
        self.inner
            .call("convertRectFromNode", &[rect.into(), from.into()])
            .as_::<DOMQuad>()
    }

    pub fn convert_rect_from_node1(
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
impl Text {
    pub fn convert_point_from_node0(&self, point: &DOMPointInit, from: &Any) -> DOMPoint {
        self.inner
            .call("convertPointFromNode", &[point.into(), from.into()])
            .as_::<DOMPoint>()
    }

    pub fn convert_point_from_node1(
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
impl Text {
    pub fn assigned_slot(&self) -> HTMLSlotElement {
        self.inner.get("assignedSlot").as_::<HTMLSlotElement>()
    }
}
