use super::*;

/// The LayoutShiftAttribution class.
/// [`LayoutShiftAttribution`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutShiftAttribution)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LayoutShiftAttribution {
    inner: Any,
}
impl FromVal for LayoutShiftAttribution {
    fn from_val(v: &Any) -> Self {
        LayoutShiftAttribution {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for LayoutShiftAttribution {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LayoutShiftAttribution {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for LayoutShiftAttribution {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for LayoutShiftAttribution {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<LayoutShiftAttribution> for Any {
    fn from(s: LayoutShiftAttribution) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&LayoutShiftAttribution> for Any {
    fn from(s: &LayoutShiftAttribution) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(LayoutShiftAttribution);

impl LayoutShiftAttribution {
    /// Getter of the `node` attribute.
    /// [`LayoutShiftAttribution.node`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutShiftAttribution/node)
    pub fn node(&self) -> Node {
        self.inner.get("node").as_::<Node>()
    }
}
impl LayoutShiftAttribution {
    /// Getter of the `previousRect` attribute.
    /// [`LayoutShiftAttribution.previousRect`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutShiftAttribution/previousRect)
    pub fn previous_rect(&self) -> DOMRectReadOnly {
        self.inner.get("previousRect").as_::<DOMRectReadOnly>()
    }
}
impl LayoutShiftAttribution {
    /// Getter of the `currentRect` attribute.
    /// [`LayoutShiftAttribution.currentRect`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutShiftAttribution/currentRect)
    pub fn current_rect(&self) -> DOMRectReadOnly {
        self.inner.get("currentRect").as_::<DOMRectReadOnly>()
    }
}
