use super::*;

/// The Path2D class.
/// [`Path2D`](https://developer.mozilla.org/en-US/docs/Web/API/Path2D)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Path2D {
    inner: Any,
}

impl FromVal for Path2D {
    fn from_val(v: &Any) -> Self {
        Path2D {
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

impl core::ops::Deref for Path2D {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Path2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Path2D {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Path2D {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Path2D> for Any {
    fn from(s: Path2D) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Path2D> for Any {
    fn from(s: &Path2D) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Path2D);

impl Path2D {
    /// The `new Path2D(..)` constructor, creating a new Path2D instance
    pub fn new() -> Path2D {
        Self {
            inner: Any::global("Path2D").new(&[]).as_::<Any>(),
        }
    }
}

impl Path2D {
    /// The `new Path2D(..)` constructor, creating a new Path2D instance
    pub fn new_with_path(path: &Any) -> Path2D {
        Self {
            inner: Any::global("Path2D").new(&[path.into()]).as_::<Any>(),
        }
    }
}

impl Path2D {
    /// The addPath method.
    /// [`Path2D.addPath`](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/addPath)
    pub fn add_path(&self, path: &Path2D) -> Undefined {
        self.inner
            .call("addPath", &[path.into()])
            .as_::<Undefined>()
    }
}
impl Path2D {
    /// The addPath method.
    /// [`Path2D.addPath`](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/addPath)
    pub fn add_path_with_transform(&self, path: &Path2D, transform: &DOMMatrix2DInit) -> Undefined {
        self.inner
            .call("addPath", &[path.into(), transform.into()])
            .as_::<Undefined>()
    }
}
impl Path2D {
    /// The closePath method.
    /// [`Path2D.closePath`](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/closePath)
    pub fn close_path(&self) -> Undefined {
        self.inner.call("closePath", &[]).as_::<Undefined>()
    }
}
impl Path2D {
    /// The moveTo method.
    /// [`Path2D.moveTo`](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/moveTo)
    pub fn move_to(&self, x: f64, y: f64) -> Undefined {
        self.inner
            .call("moveTo", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl Path2D {
    /// The lineTo method.
    /// [`Path2D.lineTo`](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/lineTo)
    pub fn line_to(&self, x: f64, y: f64) -> Undefined {
        self.inner
            .call("lineTo", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl Path2D {
    /// The quadraticCurveTo method.
    /// [`Path2D.quadraticCurveTo`](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/quadraticCurveTo)
    pub fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) -> Undefined {
        self.inner
            .call(
                "quadraticCurveTo",
                &[cpx.into(), cpy.into(), x.into(), y.into()],
            )
            .as_::<Undefined>()
    }
}
impl Path2D {
    /// The bezierCurveTo method.
    /// [`Path2D.bezierCurveTo`](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/bezierCurveTo)
    pub fn bezier_curve_to(
        &self,
        cp1x: f64,
        cp1y: f64,
        cp2x: f64,
        cp2y: f64,
        x: f64,
        y: f64,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
}
impl Path2D {
    /// The arcTo method.
    /// [`Path2D.arcTo`](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/arcTo)
    pub fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) -> Undefined {
        self.inner
            .call(
                "arcTo",
                &[x1.into(), y1.into(), x2.into(), y2.into(), radius.into()],
            )
            .as_::<Undefined>()
    }
}
impl Path2D {
    /// The rect method.
    /// [`Path2D.rect`](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/rect)
    pub fn rect(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner
            .call("rect", &[x.into(), y.into(), w.into(), h.into()])
            .as_::<Undefined>()
    }
}
impl Path2D {
    /// The roundRect method.
    /// [`Path2D.roundRect`](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/roundRect)
    pub fn round_rect(&self, x: f64, y: f64, w: f64, h: f64) -> Undefined {
        self.inner
            .call("roundRect", &[x.into(), y.into(), w.into(), h.into()])
            .as_::<Undefined>()
    }
}
impl Path2D {
    /// The roundRect method.
    /// [`Path2D.roundRect`](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/roundRect)
    pub fn round_rect_with_radii(&self, x: f64, y: f64, w: f64, h: f64, radii: &Any) -> Undefined {
        self.inner
            .call(
                "roundRect",
                &[x.into(), y.into(), w.into(), h.into(), radii.into()],
            )
            .as_::<Undefined>()
    }
}
impl Path2D {
    /// The arc method.
    /// [`Path2D.arc`](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/arc)
    pub fn arc(&self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) -> Undefined {
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
            .as_::<Undefined>()
    }
}
impl Path2D {
    /// The arc method.
    /// [`Path2D.arc`](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/arc)
    pub fn arc_with_counterclockwise(
        &self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        counterclockwise: bool,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
}
impl Path2D {
    /// The ellipse method.
    /// [`Path2D.ellipse`](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/ellipse)
    pub fn ellipse(
        &self,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
}
impl Path2D {
    /// The ellipse method.
    /// [`Path2D.ellipse`](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/ellipse)
    pub fn ellipse_with_counterclockwise(
        &self,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
        counterclockwise: bool,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
}
