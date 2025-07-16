use super::*;

/// The LayoutEdges class.
/// [`LayoutEdges`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutEdges)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LayoutEdges {
    inner: Any,
}
impl FromVal for LayoutEdges {
    fn from_val(v: &Any) -> Self {
        LayoutEdges {
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
impl core::ops::Deref for LayoutEdges {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LayoutEdges {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for LayoutEdges {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for LayoutEdges {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<LayoutEdges> for Any {
    fn from(s: LayoutEdges) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&LayoutEdges> for Any {
    fn from(s: &LayoutEdges) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(LayoutEdges);

impl LayoutEdges {
    /// Getter of the `inlineStart` attribute.
    /// [`LayoutEdges.inlineStart`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutEdges/inlineStart)
    pub fn inline_start(&self) -> f64 {
        self.inner.get("inlineStart").as_::<f64>()
    }
}
impl LayoutEdges {
    /// Getter of the `inlineEnd` attribute.
    /// [`LayoutEdges.inlineEnd`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutEdges/inlineEnd)
    pub fn inline_end(&self) -> f64 {
        self.inner.get("inlineEnd").as_::<f64>()
    }
}
impl LayoutEdges {
    /// Getter of the `blockStart` attribute.
    /// [`LayoutEdges.blockStart`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutEdges/blockStart)
    pub fn block_start(&self) -> f64 {
        self.inner.get("blockStart").as_::<f64>()
    }
}
impl LayoutEdges {
    /// Getter of the `blockEnd` attribute.
    /// [`LayoutEdges.blockEnd`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutEdges/blockEnd)
    pub fn block_end(&self) -> f64 {
        self.inner.get("blockEnd").as_::<f64>()
    }
}
impl LayoutEdges {
    /// Getter of the `inline` attribute.
    /// [`LayoutEdges.inline`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutEdges/inline)
    pub fn inline(&self) -> f64 {
        self.inner.get("inline").as_::<f64>()
    }
}
impl LayoutEdges {
    /// Getter of the `block` attribute.
    /// [`LayoutEdges.block`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutEdges/block)
    pub fn block(&self) -> f64 {
        self.inner.get("block").as_::<f64>()
    }
}
