use super::*;

/// The SVGTransform class.
/// [`SVGTransform`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGTransform {
    inner: Any,
}

impl FromVal for SVGTransform {
    fn from_val(v: &Any) -> Self {
        SVGTransform {
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

impl core::ops::Deref for SVGTransform {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGTransform {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGTransform {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGTransform> for Any {
    fn from(s: SVGTransform) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGTransform> for Any {
    fn from(s: &SVGTransform) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGTransform);

impl SVGTransform {
    /// Getter of the `type` attribute.
    /// [`SVGTransform.type`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/type)
    pub fn type_(&self) -> u16 {
        self.inner.get("type").as_::<u16>()
    }
}
impl SVGTransform {
    /// Getter of the `matrix` attribute.
    /// [`SVGTransform.matrix`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/matrix)
    pub fn matrix(&self) -> DOMMatrix {
        self.inner.get("matrix").as_::<DOMMatrix>()
    }
}
impl SVGTransform {
    /// Getter of the `angle` attribute.
    /// [`SVGTransform.angle`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/angle)
    pub fn angle(&self) -> f32 {
        self.inner.get("angle").as_::<f32>()
    }
}
impl SVGTransform {
    /// The setMatrix method.
    /// [`SVGTransform.setMatrix`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setMatrix)
    pub fn set_matrix0(&self) -> Undefined {
        self.inner.call("setMatrix", &[]).as_::<Undefined>()
    }
    /// The setMatrix method.
    /// [`SVGTransform.setMatrix`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setMatrix)
    pub fn set_matrix1(&self, matrix: &DOMMatrix2DInit) -> Undefined {
        self.inner
            .call("setMatrix", &[matrix.into()])
            .as_::<Undefined>()
    }
}
impl SVGTransform {
    /// The setTranslate method.
    /// [`SVGTransform.setTranslate`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setTranslate)
    pub fn set_translate(&self, tx: f32, ty: f32) -> Undefined {
        self.inner
            .call("setTranslate", &[tx.into(), ty.into()])
            .as_::<Undefined>()
    }
}
impl SVGTransform {
    /// The setScale method.
    /// [`SVGTransform.setScale`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setScale)
    pub fn set_scale(&self, sx: f32, sy: f32) -> Undefined {
        self.inner
            .call("setScale", &[sx.into(), sy.into()])
            .as_::<Undefined>()
    }
}
impl SVGTransform {
    /// The setRotate method.
    /// [`SVGTransform.setRotate`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setRotate)
    pub fn set_rotate(&self, angle: f32, cx: f32, cy: f32) -> Undefined {
        self.inner
            .call("setRotate", &[angle.into(), cx.into(), cy.into()])
            .as_::<Undefined>()
    }
}
impl SVGTransform {
    /// The setSkewX method.
    /// [`SVGTransform.setSkewX`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setSkewX)
    pub fn set_skew_x(&self, angle: f32) -> Undefined {
        self.inner
            .call("setSkewX", &[angle.into()])
            .as_::<Undefined>()
    }
}
impl SVGTransform {
    /// The setSkewY method.
    /// [`SVGTransform.setSkewY`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setSkewY)
    pub fn set_skew_y(&self, angle: f32) -> Undefined {
        self.inner
            .call("setSkewY", &[angle.into()])
            .as_::<Undefined>()
    }
}
