use super::*;

/// The XRMediaBinding class.
/// [`XRMediaBinding`](https://developer.mozilla.org/en-US/docs/Web/API/XRMediaBinding)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRMediaBinding {
    inner: Any,
}

impl FromVal for XRMediaBinding {
    fn from_val(v: &Any) -> Self {
        XRMediaBinding {
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

impl core::ops::Deref for XRMediaBinding {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRMediaBinding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRMediaBinding {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRMediaBinding {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRMediaBinding> for Any {
    fn from(s: XRMediaBinding) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRMediaBinding> for Any {
    fn from(s: &XRMediaBinding) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRMediaBinding);

impl XRMediaBinding {
    /// The `new XRMediaBinding(..)` constructor, creating a new XRMediaBinding instance
    pub fn new(session: &XRSession) -> XRMediaBinding {
        Self {
            inner: Any::global("XRMediaBinding")
                .new(&[session.into()])
                .as_::<Any>(),
        }
    }
}
impl XRMediaBinding {
    /// The createQuadLayer method.
    /// [`XRMediaBinding.createQuadLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRMediaBinding/createQuadLayer)
    pub fn create_quad_layer0(&self, video: &HTMLVideoElement) -> XRQuadLayer {
        self.inner
            .call("createQuadLayer", &[video.into()])
            .as_::<XRQuadLayer>()
    }
    /// The createQuadLayer method.
    /// [`XRMediaBinding.createQuadLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRMediaBinding/createQuadLayer)
    pub fn create_quad_layer1(
        &self,
        video: &HTMLVideoElement,
        init: &XRMediaQuadLayerInit,
    ) -> XRQuadLayer {
        self.inner
            .call("createQuadLayer", &[video.into(), init.into()])
            .as_::<XRQuadLayer>()
    }
}
impl XRMediaBinding {
    /// The createCylinderLayer method.
    /// [`XRMediaBinding.createCylinderLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRMediaBinding/createCylinderLayer)
    pub fn create_cylinder_layer0(&self, video: &HTMLVideoElement) -> XRCylinderLayer {
        self.inner
            .call("createCylinderLayer", &[video.into()])
            .as_::<XRCylinderLayer>()
    }
    /// The createCylinderLayer method.
    /// [`XRMediaBinding.createCylinderLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRMediaBinding/createCylinderLayer)
    pub fn create_cylinder_layer1(
        &self,
        video: &HTMLVideoElement,
        init: &XRMediaCylinderLayerInit,
    ) -> XRCylinderLayer {
        self.inner
            .call("createCylinderLayer", &[video.into(), init.into()])
            .as_::<XRCylinderLayer>()
    }
}
impl XRMediaBinding {
    /// The createEquirectLayer method.
    /// [`XRMediaBinding.createEquirectLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRMediaBinding/createEquirectLayer)
    pub fn create_equirect_layer0(&self, video: &HTMLVideoElement) -> XREquirectLayer {
        self.inner
            .call("createEquirectLayer", &[video.into()])
            .as_::<XREquirectLayer>()
    }
    /// The createEquirectLayer method.
    /// [`XRMediaBinding.createEquirectLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRMediaBinding/createEquirectLayer)
    pub fn create_equirect_layer1(
        &self,
        video: &HTMLVideoElement,
        init: &XRMediaEquirectLayerInit,
    ) -> XREquirectLayer {
        self.inner
            .call("createEquirectLayer", &[video.into(), init.into()])
            .as_::<XREquirectLayer>()
    }
}
