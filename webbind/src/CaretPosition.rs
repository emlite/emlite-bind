use super::*;

/// The CaretPosition class.
/// [`CaretPosition`](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CaretPosition {
    inner: Any,
}

impl FromVal for CaretPosition {
    fn from_val(v: &Any) -> Self {
        CaretPosition {
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

impl core::ops::Deref for CaretPosition {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CaretPosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CaretPosition {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CaretPosition {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CaretPosition> for Any {
    fn from(s: CaretPosition) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CaretPosition> for Any {
    fn from(s: &CaretPosition) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CaretPosition);

impl CaretPosition {
    /// Getter of the `offsetNode` attribute.
    /// [`CaretPosition.offsetNode`](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition/offsetNode)
    pub fn offset_node(&self) -> Node {
        self.inner.get("offsetNode").as_::<Node>()
    }
}
impl CaretPosition {
    /// Getter of the `offset` attribute.
    /// [`CaretPosition.offset`](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition/offset)
    pub fn offset(&self) -> u32 {
        self.inner.get("offset").as_::<u32>()
    }
}
impl CaretPosition {
    /// The getClientRect method.
    /// [`CaretPosition.getClientRect`](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition/getClientRect)
    pub fn get_client_rect(&self) -> DOMRect {
        self.inner.call("getClientRect", &[]).as_::<DOMRect>()
    }
}
