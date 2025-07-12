use super::*;

#[derive(Clone, Debug)]
pub struct Path2D {
    inner: emlite::Val,
}
impl FromVal for Path2D {
    fn from_val(v: &emlite::Val) -> Self {
        Path2D {
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
impl std::ops::Deref for Path2D {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Path2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Path2D> for emlite::Val {
    fn from(s: Path2D) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Path2D {
    pub fn new0() -> Path2D {
        Self {
            inner: emlite::Val::global("Path2D").new(&[]).as_::<emlite::Val>(),
        }
    }

    pub fn new1(path: jsbind::Any) -> Path2D {
        Self {
            inner: emlite::Val::global("Path2D")
                .new(&[path.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl Path2D {
    pub fn add_path0(&self, path: Path2D) -> jsbind::Undefined {
        self.inner
            .call("addPath", &[path.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn add_path1(&self, path: Path2D, transform: DOMMatrix2DInit) -> jsbind::Undefined {
        self.inner
            .call("addPath", &[path.into(), transform.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Path2D {
    pub fn close_path(&self) -> jsbind::Undefined {
        self.inner.call("closePath", &[]).as_::<jsbind::Undefined>()
    }
}
impl Path2D {
    pub fn move_to(&self, x: f64, y: f64) -> jsbind::Undefined {
        self.inner
            .call("moveTo", &[x.into(), y.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Path2D {
    pub fn line_to(&self, x: f64, y: f64) -> jsbind::Undefined {
        self.inner
            .call("lineTo", &[x.into(), y.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Path2D {
    pub fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) -> jsbind::Undefined {
        self.inner
            .call(
                "quadraticCurveTo",
                &[cpx.into(), cpy.into(), x.into(), y.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl Path2D {
    pub fn bezier_curve_to(
        &self,
        cp1x: f64,
        cp1y: f64,
        cp2x: f64,
        cp2y: f64,
        x: f64,
        y: f64,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "bezierCurveTo",
                &[
                    cp1x.into(),
                    cp1y.into(),
                    cp2x.into(),
                    cp2y.into(),
                    x.into(),
                    y.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl Path2D {
    pub fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) -> jsbind::Undefined {
        self.inner
            .call(
                "arcTo",
                &[x1.into(), y1.into(), x2.into(), y2.into(), radius.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl Path2D {
    pub fn rect(&self, x: f64, y: f64, w: f64, h: f64) -> jsbind::Undefined {
        self.inner
            .call("rect", &[x.into(), y.into(), w.into(), h.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Path2D {
    pub fn round_rect0(&self, x: f64, y: f64, w: f64, h: f64) -> jsbind::Undefined {
        self.inner
            .call("roundRect", &[x.into(), y.into(), w.into(), h.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn round_rect1(
        &self,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        radii: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "roundRect",
                &[x.into(), y.into(), w.into(), h.into(), radii.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl Path2D {
    pub fn arc0(
        &self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "arc",
                &[
                    x.into(),
                    y.into(),
                    radius.into(),
                    start_angle.into(),
                    end_angle.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn arc1(
        &self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        counterclockwise: bool,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "arc",
                &[
                    x.into(),
                    y.into(),
                    radius.into(),
                    start_angle.into(),
                    end_angle.into(),
                    counterclockwise.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl Path2D {
    pub fn ellipse0(
        &self,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "ellipse",
                &[
                    x.into(),
                    y.into(),
                    radius_x.into(),
                    radius_y.into(),
                    rotation.into(),
                    start_angle.into(),
                    end_angle.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn ellipse1(
        &self,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
        counterclockwise: bool,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "ellipse",
                &[
                    x.into(),
                    y.into(),
                    radius_x.into(),
                    radius_y.into(),
                    rotation.into(),
                    start_angle.into(),
                    end_angle.into(),
                    counterclockwise.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
