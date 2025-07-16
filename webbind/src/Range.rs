use super::*;

/// The Range class.
/// [`Range`](https://developer.mozilla.org/en-US/docs/Web/API/Range)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Range {
    inner: AbstractRange,
}
impl FromVal for Range {
    fn from_val(v: &Any) -> Self {
        Range {
            inner: AbstractRange::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Range {
    type Target = AbstractRange;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Range {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Range {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Range {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Range> for Any {
    fn from(s: Range) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Range> for Any {
    fn from(s: &Range) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Range);

impl Range {
    /// The `new Range(..)` constructor, creating a new Range instance
    pub fn new() -> Range {
        Self {
            inner: Any::global("Range").new(&[]).as_::<AbstractRange>(),
        }
    }
}
impl Range {
    /// Getter of the `commonAncestorContainer` attribute.
    /// [`Range.commonAncestorContainer`](https://developer.mozilla.org/en-US/docs/Web/API/Range/commonAncestorContainer)
    pub fn common_ancestor_container(&self) -> Node {
        self.inner.get("commonAncestorContainer").as_::<Node>()
    }
}
impl Range {
    /// The setStart method.
    /// [`Range.setStart`](https://developer.mozilla.org/en-US/docs/Web/API/Range/setStart)
    pub fn set_start(&self, node: &Node, offset: u32) -> Undefined {
        self.inner
            .call("setStart", &[node.into(), offset.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    /// The setEnd method.
    /// [`Range.setEnd`](https://developer.mozilla.org/en-US/docs/Web/API/Range/setEnd)
    pub fn set_end(&self, node: &Node, offset: u32) -> Undefined {
        self.inner
            .call("setEnd", &[node.into(), offset.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    /// The setStartBefore method.
    /// [`Range.setStartBefore`](https://developer.mozilla.org/en-US/docs/Web/API/Range/setStartBefore)
    pub fn set_start_before(&self, node: &Node) -> Undefined {
        self.inner
            .call("setStartBefore", &[node.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    /// The setStartAfter method.
    /// [`Range.setStartAfter`](https://developer.mozilla.org/en-US/docs/Web/API/Range/setStartAfter)
    pub fn set_start_after(&self, node: &Node) -> Undefined {
        self.inner
            .call("setStartAfter", &[node.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    /// The setEndBefore method.
    /// [`Range.setEndBefore`](https://developer.mozilla.org/en-US/docs/Web/API/Range/setEndBefore)
    pub fn set_end_before(&self, node: &Node) -> Undefined {
        self.inner
            .call("setEndBefore", &[node.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    /// The setEndAfter method.
    /// [`Range.setEndAfter`](https://developer.mozilla.org/en-US/docs/Web/API/Range/setEndAfter)
    pub fn set_end_after(&self, node: &Node) -> Undefined {
        self.inner
            .call("setEndAfter", &[node.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    /// The collapse method.
    /// [`Range.collapse`](https://developer.mozilla.org/en-US/docs/Web/API/Range/collapse)
    pub fn collapse0(&self) -> Undefined {
        self.inner.call("collapse", &[]).as_::<Undefined>()
    }
    /// The collapse method.
    /// [`Range.collapse`](https://developer.mozilla.org/en-US/docs/Web/API/Range/collapse)
    pub fn collapse1(&self, to_start: bool) -> Undefined {
        self.inner
            .call("collapse", &[to_start.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    /// The selectNode method.
    /// [`Range.selectNode`](https://developer.mozilla.org/en-US/docs/Web/API/Range/selectNode)
    pub fn select_node(&self, node: &Node) -> Undefined {
        self.inner
            .call("selectNode", &[node.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    /// The selectNodeContents method.
    /// [`Range.selectNodeContents`](https://developer.mozilla.org/en-US/docs/Web/API/Range/selectNodeContents)
    pub fn select_node_contents(&self, node: &Node) -> Undefined {
        self.inner
            .call("selectNodeContents", &[node.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    /// The compareBoundaryPoints method.
    /// [`Range.compareBoundaryPoints`](https://developer.mozilla.org/en-US/docs/Web/API/Range/compareBoundaryPoints)
    pub fn compare_boundary_points(&self, how: u16, source_range: &Range) -> i16 {
        self.inner
            .call("compareBoundaryPoints", &[how.into(), source_range.into()])
            .as_::<i16>()
    }
}
impl Range {
    /// The deleteContents method.
    /// [`Range.deleteContents`](https://developer.mozilla.org/en-US/docs/Web/API/Range/deleteContents)
    pub fn delete_contents(&self) -> Undefined {
        self.inner.call("deleteContents", &[]).as_::<Undefined>()
    }
}
impl Range {
    /// The extractContents method.
    /// [`Range.extractContents`](https://developer.mozilla.org/en-US/docs/Web/API/Range/extractContents)
    pub fn extract_contents(&self) -> DocumentFragment {
        self.inner
            .call("extractContents", &[])
            .as_::<DocumentFragment>()
    }
}
impl Range {
    /// The cloneContents method.
    /// [`Range.cloneContents`](https://developer.mozilla.org/en-US/docs/Web/API/Range/cloneContents)
    pub fn clone_contents(&self) -> DocumentFragment {
        self.inner
            .call("cloneContents", &[])
            .as_::<DocumentFragment>()
    }
}
impl Range {
    /// The insertNode method.
    /// [`Range.insertNode`](https://developer.mozilla.org/en-US/docs/Web/API/Range/insertNode)
    pub fn insert_node(&self, node: &Node) -> Undefined {
        self.inner
            .call("insertNode", &[node.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    /// The surroundContents method.
    /// [`Range.surroundContents`](https://developer.mozilla.org/en-US/docs/Web/API/Range/surroundContents)
    pub fn surround_contents(&self, new_parent: &Node) -> Undefined {
        self.inner
            .call("surroundContents", &[new_parent.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    /// The cloneRange method.
    /// [`Range.cloneRange`](https://developer.mozilla.org/en-US/docs/Web/API/Range/cloneRange)
    pub fn clone_range(&self) -> Range {
        self.inner.call("cloneRange", &[]).as_::<Range>()
    }
}
impl Range {
    /// The detach method.
    /// [`Range.detach`](https://developer.mozilla.org/en-US/docs/Web/API/Range/detach)
    pub fn detach(&self) -> Undefined {
        self.inner.call("detach", &[]).as_::<Undefined>()
    }
}
impl Range {
    /// The isPointInRange method.
    /// [`Range.isPointInRange`](https://developer.mozilla.org/en-US/docs/Web/API/Range/isPointInRange)
    pub fn is_point_in_range(&self, node: &Node, offset: u32) -> bool {
        self.inner
            .call("isPointInRange", &[node.into(), offset.into()])
            .as_::<bool>()
    }
}
impl Range {
    /// The comparePoint method.
    /// [`Range.comparePoint`](https://developer.mozilla.org/en-US/docs/Web/API/Range/comparePoint)
    pub fn compare_point(&self, node: &Node, offset: u32) -> i16 {
        self.inner
            .call("comparePoint", &[node.into(), offset.into()])
            .as_::<i16>()
    }
}
impl Range {
    /// The intersectsNode method.
    /// [`Range.intersectsNode`](https://developer.mozilla.org/en-US/docs/Web/API/Range/intersectsNode)
    pub fn intersects_node(&self, node: &Node) -> bool {
        self.inner
            .call("intersectsNode", &[node.into()])
            .as_::<bool>()
    }
}
impl Range {
    /// The getClientRects method.
    /// [`Range.getClientRects`](https://developer.mozilla.org/en-US/docs/Web/API/Range/getClientRects)
    pub fn get_client_rects(&self) -> DOMRectList {
        self.inner.call("getClientRects", &[]).as_::<DOMRectList>()
    }
}
impl Range {
    /// The getBoundingClientRect method.
    /// [`Range.getBoundingClientRect`](https://developer.mozilla.org/en-US/docs/Web/API/Range/getBoundingClientRect)
    pub fn get_bounding_client_rect(&self) -> DOMRect {
        self.inner
            .call("getBoundingClientRect", &[])
            .as_::<DOMRect>()
    }
}
impl Range {
    /// The createContextualFragment method.
    /// [`Range.createContextualFragment`](https://developer.mozilla.org/en-US/docs/Web/API/Range/createContextualFragment)
    pub fn create_contextual_fragment(&self, string: &Any) -> DocumentFragment {
        self.inner
            .call("createContextualFragment", &[string.into()])
            .as_::<DocumentFragment>()
    }
}
