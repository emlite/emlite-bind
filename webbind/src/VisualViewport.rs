use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VisualViewport {
    inner: EventTarget,
}
impl FromVal for VisualViewport {
    fn from_val(v: &emlite::Val) -> Self {
        VisualViewport { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VisualViewport {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VisualViewport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for VisualViewport {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for VisualViewport {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<VisualViewport> for emlite::Val {
    fn from(s: VisualViewport) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(VisualViewport);


impl VisualViewport {
    pub fn offset_left(&self) -> f64 {
        self.inner.get("offsetLeft").as_::<f64>()
    }

}
impl VisualViewport {
    pub fn offset_top(&self) -> f64 {
        self.inner.get("offsetTop").as_::<f64>()
    }

}
impl VisualViewport {
    pub fn page_left(&self) -> f64 {
        self.inner.get("pageLeft").as_::<f64>()
    }

}
impl VisualViewport {
    pub fn page_top(&self) -> f64 {
        self.inner.get("pageTop").as_::<f64>()
    }

}
impl VisualViewport {
    pub fn width(&self) -> f64 {
        self.inner.get("width").as_::<f64>()
    }

}
impl VisualViewport {
    pub fn height(&self) -> f64 {
        self.inner.get("height").as_::<f64>()
    }

}
impl VisualViewport {
    pub fn scale(&self) -> f64 {
        self.inner.get("scale").as_::<f64>()
    }

}
impl VisualViewport {
    pub fn onresize(&self) -> Any {
        self.inner.get("onresize").as_::<Any>()
    }

    pub fn set_onresize(&mut self, value: Any) {
        self.inner.set("onresize", value);
    }

}
impl VisualViewport {
    pub fn onscroll(&self) -> Any {
        self.inner.get("onscroll").as_::<Any>()
    }

    pub fn set_onscroll(&mut self, value: Any) {
        self.inner.set("onscroll", value);
    }

}
impl VisualViewport {
    pub fn onscrollend(&self) -> Any {
        self.inner.get("onscrollend").as_::<Any>()
    }

    pub fn set_onscrollend(&mut self, value: Any) {
        self.inner.set("onscrollend", value);
    }

}
