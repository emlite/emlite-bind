use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Range {
    inner: AbstractRange,
}
impl FromVal for Range {
    fn from_val(v: &emlite::Val) -> Self {
        Range {
            inner: AbstractRange::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for Range {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Range {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Range> for emlite::Val {
    fn from(s: Range) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Range> for emlite::Val {
    fn from(s: &Range) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Range);

impl Range {
    pub fn new() -> Range {
        Self {
            inner: emlite::Val::global("Range").new(&[]).as_::<AbstractRange>(),
        }
    }
}
impl Range {
    pub fn common_ancestor_container(&self) -> Node {
        self.inner.get("commonAncestorContainer").as_::<Node>()
    }
}
impl Range {
    pub fn set_start(&self, node: Node, offset: u32) -> Undefined {
        self.inner
            .call("setStart", &[node.into(), offset.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    pub fn set_end(&self, node: Node, offset: u32) -> Undefined {
        self.inner
            .call("setEnd", &[node.into(), offset.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    pub fn set_start_before(&self, node: Node) -> Undefined {
        self.inner
            .call("setStartBefore", &[node.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    pub fn set_start_after(&self, node: Node) -> Undefined {
        self.inner
            .call("setStartAfter", &[node.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    pub fn set_end_before(&self, node: Node) -> Undefined {
        self.inner
            .call("setEndBefore", &[node.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    pub fn set_end_after(&self, node: Node) -> Undefined {
        self.inner
            .call("setEndAfter", &[node.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    pub fn collapse0(&self) -> Undefined {
        self.inner.call("collapse", &[]).as_::<Undefined>()
    }

    pub fn collapse1(&self, to_start: bool) -> Undefined {
        self.inner
            .call("collapse", &[to_start.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    pub fn select_node(&self, node: Node) -> Undefined {
        self.inner
            .call("selectNode", &[node.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    pub fn select_node_contents(&self, node: Node) -> Undefined {
        self.inner
            .call("selectNodeContents", &[node.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    pub fn compare_boundary_points(&self, how: u16, source_range: Range) -> i16 {
        self.inner
            .call("compareBoundaryPoints", &[how.into(), source_range.into()])
            .as_::<i16>()
    }
}
impl Range {
    pub fn delete_contents(&self) -> Undefined {
        self.inner.call("deleteContents", &[]).as_::<Undefined>()
    }
}
impl Range {
    pub fn extract_contents(&self) -> DocumentFragment {
        self.inner
            .call("extractContents", &[])
            .as_::<DocumentFragment>()
    }
}
impl Range {
    pub fn clone_contents(&self) -> DocumentFragment {
        self.inner
            .call("cloneContents", &[])
            .as_::<DocumentFragment>()
    }
}
impl Range {
    pub fn insert_node(&self, node: Node) -> Undefined {
        self.inner
            .call("insertNode", &[node.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    pub fn surround_contents(&self, new_parent: Node) -> Undefined {
        self.inner
            .call("surroundContents", &[new_parent.into()])
            .as_::<Undefined>()
    }
}
impl Range {
    pub fn clone_range(&self) -> Range {
        self.inner.call("cloneRange", &[]).as_::<Range>()
    }
}
impl Range {
    pub fn detach(&self) -> Undefined {
        self.inner.call("detach", &[]).as_::<Undefined>()
    }
}
impl Range {
    pub fn is_point_in_range(&self, node: Node, offset: u32) -> bool {
        self.inner
            .call("isPointInRange", &[node.into(), offset.into()])
            .as_::<bool>()
    }
}
impl Range {
    pub fn compare_point(&self, node: Node, offset: u32) -> i16 {
        self.inner
            .call("comparePoint", &[node.into(), offset.into()])
            .as_::<i16>()
    }
}
impl Range {
    pub fn intersects_node(&self, node: Node) -> bool {
        self.inner
            .call("intersectsNode", &[node.into()])
            .as_::<bool>()
    }
}
impl Range {
    pub fn get_client_rects(&self) -> DOMRectList {
        self.inner.call("getClientRects", &[]).as_::<DOMRectList>()
    }
}
impl Range {
    pub fn get_bounding_client_rect(&self) -> DOMRect {
        self.inner
            .call("getBoundingClientRect", &[])
            .as_::<DOMRect>()
    }
}
impl Range {
    pub fn create_contextual_fragment(&self, string: Any) -> DocumentFragment {
        self.inner
            .call("createContextualFragment", &[string.into()])
            .as_::<DocumentFragment>()
    }
}
