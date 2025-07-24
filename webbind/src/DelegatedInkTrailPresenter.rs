use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InkTrailStyle {
    inner: Any,
}
impl FromVal for InkTrailStyle {
    fn from_val(v: &Any) -> Self {
        InkTrailStyle { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for InkTrailStyle {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for InkTrailStyle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for InkTrailStyle {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for InkTrailStyle {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<InkTrailStyle> for Any {
    fn from(s: InkTrailStyle) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&InkTrailStyle> for Any {
    fn from(s: &InkTrailStyle) -> Any {
        s.inner.clone()
    }
}

impl InkTrailStyle {
    pub fn color(&self) -> DOMString {
        self.inner.get("color").as_::<DOMString>()
    }

    pub fn set_color(&mut self, value: &DOMString) {
        self.inner.set("color", value);
    }
}
impl InkTrailStyle {
    pub fn diameter(&self) -> f64 {
        self.inner.get("diameter").as_::<f64>()
    }

    pub fn set_diameter(&mut self, value: f64) {
        self.inner.set("diameter", value);
    }
}
/// The DelegatedInkTrailPresenter class.
/// [`DelegatedInkTrailPresenter`](https://developer.mozilla.org/en-US/docs/Web/API/DelegatedInkTrailPresenter)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DelegatedInkTrailPresenter {
    inner: Any,
}
impl FromVal for DelegatedInkTrailPresenter {
    fn from_val(v: &Any) -> Self {
        DelegatedInkTrailPresenter {
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
impl core::ops::Deref for DelegatedInkTrailPresenter {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DelegatedInkTrailPresenter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DelegatedInkTrailPresenter {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DelegatedInkTrailPresenter {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DelegatedInkTrailPresenter> for Any {
    fn from(s: DelegatedInkTrailPresenter) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DelegatedInkTrailPresenter> for Any {
    fn from(s: &DelegatedInkTrailPresenter) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DelegatedInkTrailPresenter);

impl DelegatedInkTrailPresenter {
    /// Getter of the `presentationArea` attribute.
    /// [`DelegatedInkTrailPresenter.presentationArea`](https://developer.mozilla.org/en-US/docs/Web/API/DelegatedInkTrailPresenter/presentationArea)
    pub fn presentation_area(&self) -> Element {
        self.inner.get("presentationArea").as_::<Element>()
    }
}
impl DelegatedInkTrailPresenter {
    /// The updateInkTrailStartPoint method.
    /// [`DelegatedInkTrailPresenter.updateInkTrailStartPoint`](https://developer.mozilla.org/en-US/docs/Web/API/DelegatedInkTrailPresenter/updateInkTrailStartPoint)
    pub fn update_ink_trail_start_point(
        &self,
        event: &PointerEvent,
        style: &InkTrailStyle,
    ) -> Undefined {
        self.inner
            .call("updateInkTrailStartPoint", &[event.into(), style.into()])
            .as_::<Undefined>()
    }
}
