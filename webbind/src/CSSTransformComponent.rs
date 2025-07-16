use super::*;

/// The CSSTransformComponent class.
/// [`CSSTransformComponent`](https://developer.mozilla.org/en-US/docs/Web/API/CSSTransformComponent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSTransformComponent {
    inner: Any,
}
impl FromVal for CSSTransformComponent {
    fn from_val(v: &Any) -> Self {
        CSSTransformComponent {
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
impl core::ops::Deref for CSSTransformComponent {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSTransformComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSTransformComponent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSTransformComponent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSTransformComponent> for Any {
    fn from(s: CSSTransformComponent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSTransformComponent> for Any {
    fn from(s: &CSSTransformComponent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSTransformComponent);

impl CSSTransformComponent {
    /// Getter of the `is2D` attribute.
    /// [`CSSTransformComponent.is2D`](https://developer.mozilla.org/en-US/docs/Web/API/CSSTransformComponent/is2D)
    pub fn is2_d(&self) -> bool {
        self.inner.get("is2D").as_::<bool>()
    }

    /// Setter of the `is2D` attribute.
    /// [`CSSTransformComponent.is2D`](https://developer.mozilla.org/en-US/docs/Web/API/CSSTransformComponent/is2D)
    pub fn set_is2_d(&mut self, value: bool) {
        self.inner.set("is2D", value);
    }
}
impl CSSTransformComponent {
    /// The toMatrix method.
    /// [`CSSTransformComponent.toMatrix`](https://developer.mozilla.org/en-US/docs/Web/API/CSSTransformComponent/toMatrix)
    pub fn to_matrix(&self) -> DOMMatrix {
        self.inner.call("toMatrix", &[]).as_::<DOMMatrix>()
    }
}
