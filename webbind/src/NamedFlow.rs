use super::*;

/// The NamedFlow class.
/// [`NamedFlow`](https://developer.mozilla.org/en-US/docs/Web/API/NamedFlow)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NamedFlow {
    inner: EventTarget,
}
impl FromVal for NamedFlow {
    fn from_val(v: &Any) -> Self {
        NamedFlow {
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
impl core::ops::Deref for NamedFlow {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NamedFlow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NamedFlow {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NamedFlow {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NamedFlow> for Any {
    fn from(s: NamedFlow) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NamedFlow> for Any {
    fn from(s: &NamedFlow) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(NamedFlow);

impl NamedFlow {
    /// Getter of the `name` attribute.
    /// [`NamedFlow.name`](https://developer.mozilla.org/en-US/docs/Web/API/NamedFlow/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }
}
impl NamedFlow {
    /// Getter of the `overset` attribute.
    /// [`NamedFlow.overset`](https://developer.mozilla.org/en-US/docs/Web/API/NamedFlow/overset)
    pub fn overset(&self) -> bool {
        self.inner.get("overset").as_::<bool>()
    }
}
impl NamedFlow {
    /// The getRegions method.
    /// [`NamedFlow.getRegions`](https://developer.mozilla.org/en-US/docs/Web/API/NamedFlow/getRegions)
    pub fn get_regions(&self) -> TypedArray<Element> {
        self.inner
            .call("getRegions", &[])
            .as_::<TypedArray<Element>>()
    }
}
impl NamedFlow {
    /// Getter of the `firstEmptyRegionIndex` attribute.
    /// [`NamedFlow.firstEmptyRegionIndex`](https://developer.mozilla.org/en-US/docs/Web/API/NamedFlow/firstEmptyRegionIndex)
    pub fn first_empty_region_index(&self) -> i16 {
        self.inner.get("firstEmptyRegionIndex").as_::<i16>()
    }
}
impl NamedFlow {
    /// The getContent method.
    /// [`NamedFlow.getContent`](https://developer.mozilla.org/en-US/docs/Web/API/NamedFlow/getContent)
    pub fn get_content(&self) -> TypedArray<Node> {
        self.inner.call("getContent", &[]).as_::<TypedArray<Node>>()
    }
}
impl NamedFlow {
    /// The getRegionsByContent method.
    /// [`NamedFlow.getRegionsByContent`](https://developer.mozilla.org/en-US/docs/Web/API/NamedFlow/getRegionsByContent)
    pub fn get_regions_by_content(&self, node: &Node) -> TypedArray<Element> {
        self.inner
            .call("getRegionsByContent", &[node.into()])
            .as_::<TypedArray<Element>>()
    }
}
