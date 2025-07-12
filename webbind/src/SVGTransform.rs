use super::*;

#[derive(Clone, Debug)]
pub struct SVGTransform {
    inner: emlite::Val,
}
impl FromVal for SVGTransform {
    fn from_val(v: &emlite::Val) -> Self {
        SVGTransform {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SVGTransform {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGTransform> for emlite::Val {
    fn from(s: SVGTransform) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGTransform {
    pub fn type_(&self) -> u16 {
        self.inner.get("type").as_::<u16>()
    }
}
impl SVGTransform {
    pub fn matrix(&self) -> DOMMatrix {
        self.inner.get("matrix").as_::<DOMMatrix>()
    }
}
impl SVGTransform {
    pub fn angle(&self) -> f32 {
        self.inner.get("angle").as_::<f32>()
    }
}
impl SVGTransform {
    pub fn set_matrix0(&self) -> jsbind::Undefined {
        self.inner.call("setMatrix", &[]).as_::<jsbind::Undefined>()
    }

    pub fn set_matrix1(&self, matrix: DOMMatrix2DInit) -> jsbind::Undefined {
        self.inner
            .call("setMatrix", &[matrix.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl SVGTransform {
    pub fn set_translate(&self, tx: f32, ty: f32) -> jsbind::Undefined {
        self.inner
            .call("setTranslate", &[tx.into(), ty.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl SVGTransform {
    pub fn set_scale(&self, sx: f32, sy: f32) -> jsbind::Undefined {
        self.inner
            .call("setScale", &[sx.into(), sy.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl SVGTransform {
    pub fn set_rotate(&self, angle: f32, cx: f32, cy: f32) -> jsbind::Undefined {
        self.inner
            .call("setRotate", &[angle.into(), cx.into(), cy.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl SVGTransform {
    pub fn set_skew_x(&self, angle: f32) -> jsbind::Undefined {
        self.inner
            .call("setSkewX", &[angle.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl SVGTransform {
    pub fn set_skew_y(&self, angle: f32) -> jsbind::Undefined {
        self.inner
            .call("setSkewY", &[angle.into()])
            .as_::<jsbind::Undefined>()
    }
}
