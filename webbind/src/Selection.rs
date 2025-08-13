use super::*;




/// The Selection class.
/// [`Selection`](https://developer.mozilla.org/en-US/docs/Web/API/Selection)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Selection {
    inner: Any,
}

impl FromVal for Selection {
    fn from_val(v: &Any) -> Self {
        Selection { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Selection {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Selection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Selection {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Selection {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<Selection> for Any {
    fn from(s: Selection) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Selection> for Any {
    fn from(s: &Selection) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Selection);


impl Selection {
    /// Getter of the `anchorNode` attribute.
    /// [`Selection.anchorNode`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/anchorNode)
    pub fn anchor_node(&self) -> Node {
        self.inner.get("anchorNode").as_::<Node>()
    }

}
impl Selection {
    /// Getter of the `anchorOffset` attribute.
    /// [`Selection.anchorOffset`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/anchorOffset)
    pub fn anchor_offset(&self) -> u32 {
        self.inner.get("anchorOffset").as_::<u32>()
    }

}
impl Selection {
    /// Getter of the `focusNode` attribute.
    /// [`Selection.focusNode`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/focusNode)
    pub fn focus_node(&self) -> Node {
        self.inner.get("focusNode").as_::<Node>()
    }

}
impl Selection {
    /// Getter of the `focusOffset` attribute.
    /// [`Selection.focusOffset`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/focusOffset)
    pub fn focus_offset(&self) -> u32 {
        self.inner.get("focusOffset").as_::<u32>()
    }

}
impl Selection {
    /// Getter of the `isCollapsed` attribute.
    /// [`Selection.isCollapsed`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/isCollapsed)
    pub fn is_collapsed(&self) -> bool {
        self.inner.get("isCollapsed").as_::<bool>()
    }

}
impl Selection {
    /// Getter of the `rangeCount` attribute.
    /// [`Selection.rangeCount`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/rangeCount)
    pub fn range_count(&self) -> u32 {
        self.inner.get("rangeCount").as_::<u32>()
    }

}
impl Selection {
    /// Getter of the `type` attribute.
    /// [`Selection.type`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/type)
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

}
impl Selection {
    /// Getter of the `direction` attribute.
    /// [`Selection.direction`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/direction)
    pub fn direction(&self) -> JsString {
        self.inner.get("direction").as_::<JsString>()
    }

}
impl Selection {
    /// The getRangeAt method.
    /// [`Selection.getRangeAt`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/getRangeAt)
    pub fn get_range_at(&self, index: u32) -> Range {
        self.inner.call("getRangeAt", &[index.into(), ]).as_::<Range>()
    }
}
impl Selection {
    /// The addRange method.
    /// [`Selection.addRange`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/addRange)
    pub fn add_range(&self, range: &Range) -> Undefined {
        self.inner.call("addRange", &[range.into(), ]).as_::<Undefined>()
    }
}
impl Selection {
    /// The removeRange method.
    /// [`Selection.removeRange`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/removeRange)
    pub fn remove_range(&self, range: &Range) -> Undefined {
        self.inner.call("removeRange", &[range.into(), ]).as_::<Undefined>()
    }
}
impl Selection {
    /// The removeAllRanges method.
    /// [`Selection.removeAllRanges`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/removeAllRanges)
    pub fn remove_all_ranges(&self, ) -> Undefined {
        self.inner.call("removeAllRanges", &[]).as_::<Undefined>()
    }
}
impl Selection {
    /// The empty method.
    /// [`Selection.empty`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/empty)
    pub fn empty(&self, ) -> Undefined {
        self.inner.call("empty", &[]).as_::<Undefined>()
    }
}
impl Selection {
    /// The getComposedRanges method.
    /// [`Selection.getComposedRanges`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/getComposedRanges)
    pub fn get_composed_ranges0(&self, ) -> TypedArray<StaticRange> {
        self.inner.call("getComposedRanges", &[]).as_::<TypedArray<StaticRange>>()
    }
    /// The getComposedRanges method.
    /// [`Selection.getComposedRanges`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/getComposedRanges)
    pub fn get_composed_ranges1(&self, options: &GetComposedRangesOptions) -> TypedArray<StaticRange> {
        self.inner.call("getComposedRanges", &[options.into(), ]).as_::<TypedArray<StaticRange>>()
    }
}
impl Selection {
    /// The collapse method.
    /// [`Selection.collapse`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapse)
    pub fn collapse0(&self, node: &Node) -> Undefined {
        self.inner.call("collapse", &[node.into(), ]).as_::<Undefined>()
    }
    /// The collapse method.
    /// [`Selection.collapse`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapse)
    pub fn collapse1(&self, node: &Node, offset: u32) -> Undefined {
        self.inner.call("collapse", &[node.into(), offset.into(), ]).as_::<Undefined>()
    }
}
impl Selection {
    /// The setPosition method.
    /// [`Selection.setPosition`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/setPosition)
    pub fn set_position0(&self, node: &Node) -> Undefined {
        self.inner.call("setPosition", &[node.into(), ]).as_::<Undefined>()
    }
    /// The setPosition method.
    /// [`Selection.setPosition`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/setPosition)
    pub fn set_position1(&self, node: &Node, offset: u32) -> Undefined {
        self.inner.call("setPosition", &[node.into(), offset.into(), ]).as_::<Undefined>()
    }
}
impl Selection {
    /// The collapseToStart method.
    /// [`Selection.collapseToStart`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapseToStart)
    pub fn collapse_to_start(&self, ) -> Undefined {
        self.inner.call("collapseToStart", &[]).as_::<Undefined>()
    }
}
impl Selection {
    /// The collapseToEnd method.
    /// [`Selection.collapseToEnd`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapseToEnd)
    pub fn collapse_to_end(&self, ) -> Undefined {
        self.inner.call("collapseToEnd", &[]).as_::<Undefined>()
    }
}
impl Selection {
    /// The extend method.
    /// [`Selection.extend`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/extend)
    pub fn extend0(&self, node: &Node) -> Undefined {
        self.inner.call("extend", &[node.into(), ]).as_::<Undefined>()
    }
    /// The extend method.
    /// [`Selection.extend`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/extend)
    pub fn extend1(&self, node: &Node, offset: u32) -> Undefined {
        self.inner.call("extend", &[node.into(), offset.into(), ]).as_::<Undefined>()
    }
}
impl Selection {
    /// The setBaseAndExtent method.
    /// [`Selection.setBaseAndExtent`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/setBaseAndExtent)
    pub fn set_base_and_extent(&self, anchor_node: &Node, anchor_offset: u32, focus_node: &Node, focus_offset: u32) -> Undefined {
        self.inner.call("setBaseAndExtent", &[anchor_node.into(), anchor_offset.into(), focus_node.into(), focus_offset.into(), ]).as_::<Undefined>()
    }
}
impl Selection {
    /// The selectAllChildren method.
    /// [`Selection.selectAllChildren`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/selectAllChildren)
    pub fn select_all_children(&self, node: &Node) -> Undefined {
        self.inner.call("selectAllChildren", &[node.into(), ]).as_::<Undefined>()
    }
}
impl Selection {
    /// The modify method.
    /// [`Selection.modify`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/modify)
    pub fn modify0(&self, ) -> Undefined {
        self.inner.call("modify", &[]).as_::<Undefined>()
    }
    /// The modify method.
    /// [`Selection.modify`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/modify)
    pub fn modify1(&self, alter: &JsString) -> Undefined {
        self.inner.call("modify", &[alter.into(), ]).as_::<Undefined>()
    }
    /// The modify method.
    /// [`Selection.modify`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/modify)
    pub fn modify2(&self, alter: &JsString, direction: &JsString) -> Undefined {
        self.inner.call("modify", &[alter.into(), direction.into(), ]).as_::<Undefined>()
    }
    /// The modify method.
    /// [`Selection.modify`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/modify)
    pub fn modify3(&self, alter: &JsString, direction: &JsString, granularity: &JsString) -> Undefined {
        self.inner.call("modify", &[alter.into(), direction.into(), granularity.into(), ]).as_::<Undefined>()
    }
}
impl Selection {
    /// The deleteFromDocument method.
    /// [`Selection.deleteFromDocument`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/deleteFromDocument)
    pub fn delete_from_document(&self, ) -> Undefined {
        self.inner.call("deleteFromDocument", &[]).as_::<Undefined>()
    }
}
impl Selection {
    /// The containsNode method.
    /// [`Selection.containsNode`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/containsNode)
    pub fn contains_node0(&self, node: &Node) -> bool {
        self.inner.call("containsNode", &[node.into(), ]).as_::<bool>()
    }
    /// The containsNode method.
    /// [`Selection.containsNode`](https://developer.mozilla.org/en-US/docs/Web/API/Selection/containsNode)
    pub fn contains_node1(&self, node: &Node, allow_partial_containment: bool) -> bool {
        self.inner.call("containsNode", &[node.into(), allow_partial_containment.into(), ]).as_::<bool>()
    }
}
