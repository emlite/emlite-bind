use super::*;




/// The Text class.
/// [`Text`](https://developer.mozilla.org/en-US/docs/Web/API/Text)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Text {
    inner: CharacterData,
}

impl FromVal for Text {
    fn from_val(v: &Any) -> Self {
        Text { inner: CharacterData::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for Text {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Text {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<Text> for Any {
    fn from(s: Text) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Text> for Any {
    fn from(s: &Text) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Text);



impl Text {
    /// The `new Text(..)` constructor, creating a new Text instance
    pub fn new0() -> Text {
        Self {
            inner: Any::global("Text").new(&[]).as_::<CharacterData>(),
        }
    }

    /// The `new Text(..)` constructor, creating a new Text instance
    pub fn new1(data: &JsString) -> Text {
        Self {
            inner: Any::global("Text").new(&[data.into()]).as_::<CharacterData>(),
        }
    }

}
impl Text {
    /// The splitText method.
    /// [`Text.splitText`](https://developer.mozilla.org/en-US/docs/Web/API/Text/splitText)
    pub fn split_text(&self, offset: u32) -> Text {
        self.inner.call("splitText", &[offset.into(), ]).as_::<Text>()
    }
}
impl Text {
    /// Getter of the `wholeText` attribute.
    /// [`Text.wholeText`](https://developer.mozilla.org/en-US/docs/Web/API/Text/wholeText)
    pub fn whole_text(&self) -> JsString {
        self.inner.get("wholeText").as_::<JsString>()
    }

}
impl Text {
    /// The getBoxQuads method.
    /// [`Text.getBoxQuads`](https://developer.mozilla.org/en-US/docs/Web/API/Text/getBoxQuads)
    pub fn get_box_quads0(&self, ) -> TypedArray<DOMQuad> {
        self.inner.call("getBoxQuads", &[]).as_::<TypedArray<DOMQuad>>()
    }
    /// The getBoxQuads method.
    /// [`Text.getBoxQuads`](https://developer.mozilla.org/en-US/docs/Web/API/Text/getBoxQuads)
    pub fn get_box_quads1(&self, options: &BoxQuadOptions) -> TypedArray<DOMQuad> {
        self.inner.call("getBoxQuads", &[options.into(), ]).as_::<TypedArray<DOMQuad>>()
    }
}
impl Text {
    /// The convertQuadFromNode method.
    /// [`Text.convertQuadFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertQuadFromNode)
    pub fn convert_quad_from_node0(&self, quad: &DOMQuadInit, from: &Any) -> DOMQuad {
        self.inner.call("convertQuadFromNode", &[quad.into(), from.into(), ]).as_::<DOMQuad>()
    }
    /// The convertQuadFromNode method.
    /// [`Text.convertQuadFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertQuadFromNode)
    pub fn convert_quad_from_node1(&self, quad: &DOMQuadInit, from: &Any, options: &ConvertCoordinateOptions) -> DOMQuad {
        self.inner.call("convertQuadFromNode", &[quad.into(), from.into(), options.into(), ]).as_::<DOMQuad>()
    }
}
impl Text {
    /// The convertRectFromNode method.
    /// [`Text.convertRectFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertRectFromNode)
    pub fn convert_rect_from_node0(&self, rect: &DOMRectReadOnly, from: &Any) -> DOMQuad {
        self.inner.call("convertRectFromNode", &[rect.into(), from.into(), ]).as_::<DOMQuad>()
    }
    /// The convertRectFromNode method.
    /// [`Text.convertRectFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertRectFromNode)
    pub fn convert_rect_from_node1(&self, rect: &DOMRectReadOnly, from: &Any, options: &ConvertCoordinateOptions) -> DOMQuad {
        self.inner.call("convertRectFromNode", &[rect.into(), from.into(), options.into(), ]).as_::<DOMQuad>()
    }
}
impl Text {
    /// The convertPointFromNode method.
    /// [`Text.convertPointFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertPointFromNode)
    pub fn convert_point_from_node0(&self, point: &DOMPointInit, from: &Any) -> DOMPoint {
        self.inner.call("convertPointFromNode", &[point.into(), from.into(), ]).as_::<DOMPoint>()
    }
    /// The convertPointFromNode method.
    /// [`Text.convertPointFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertPointFromNode)
    pub fn convert_point_from_node1(&self, point: &DOMPointInit, from: &Any, options: &ConvertCoordinateOptions) -> DOMPoint {
        self.inner.call("convertPointFromNode", &[point.into(), from.into(), options.into(), ]).as_::<DOMPoint>()
    }
}
impl Text {
    /// Getter of the `assignedSlot` attribute.
    /// [`Text.assignedSlot`](https://developer.mozilla.org/en-US/docs/Web/API/Text/assignedSlot)
    pub fn assigned_slot(&self) -> HTMLSlotElement {
        self.inner.get("assignedSlot").as_::<HTMLSlotElement>()
    }

}
