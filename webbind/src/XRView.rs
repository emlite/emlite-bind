use super::*;

/// The XRView class.
/// [`XRView`](https://developer.mozilla.org/en-US/docs/Web/API/XRView)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRView {
    inner: Any,
}

impl FromVal for XRView {
    fn from_val(v: &Any) -> Self {
        XRView {
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

impl core::ops::Deref for XRView {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRView {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRView {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRView> for Any {
    fn from(s: XRView) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRView> for Any {
    fn from(s: &XRView) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRView);

impl XRView {
    /// Getter of the `eye` attribute.
    /// [`XRView.eye`](https://developer.mozilla.org/en-US/docs/Web/API/XRView/eye)
    pub fn eye(&self) -> XREye {
        self.inner.get("eye").as_::<XREye>()
    }
}
impl XRView {
    /// Getter of the `recommendedViewportScale` attribute.
    /// [`XRView.recommendedViewportScale`](https://developer.mozilla.org/en-US/docs/Web/API/XRView/recommendedViewportScale)
    pub fn recommended_viewport_scale(&self) -> f64 {
        self.inner.get("recommendedViewportScale").as_::<f64>()
    }
}
impl XRView {
    /// Getter of the `camera` attribute.
    /// [`XRView.camera`](https://developer.mozilla.org/en-US/docs/Web/API/XRView/camera)
    pub fn camera(&self) -> XRCamera {
        self.inner.get("camera").as_::<XRCamera>()
    }
}
impl XRView {
    /// Getter of the `isFirstPersonObserver` attribute.
    /// [`XRView.isFirstPersonObserver`](https://developer.mozilla.org/en-US/docs/Web/API/XRView/isFirstPersonObserver)
    pub fn is_first_person_observer(&self) -> bool {
        self.inner.get("isFirstPersonObserver").as_::<bool>()
    }
}
impl XRView {
    /// Getter of the `projectionMatrix` attribute.
    /// [`XRView.projectionMatrix`](https://developer.mozilla.org/en-US/docs/Web/API/XRView/projectionMatrix)
    pub fn projection_matrix(&self) -> Float32Array {
        self.inner.get("projectionMatrix").as_::<Float32Array>()
    }
}
impl XRView {
    /// Getter of the `transform` attribute.
    /// [`XRView.transform`](https://developer.mozilla.org/en-US/docs/Web/API/XRView/transform)
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }
}
impl XRView {
    /// The requestViewportScale method.
    /// [`XRView.requestViewportScale`](https://developer.mozilla.org/en-US/docs/Web/API/XRView/requestViewportScale)
    pub fn request_viewport_scale(&self, scale: f64) -> Undefined {
        self.inner
            .call("requestViewportScale", &[scale.into()])
            .as_::<Undefined>()
    }
}
