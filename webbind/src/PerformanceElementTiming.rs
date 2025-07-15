use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceElementTiming {
    inner: PerformanceEntry,
}
impl FromVal for PerformanceElementTiming {
    fn from_val(v: &emlite::Val) -> Self {
        PerformanceElementTiming { inner: PerformanceEntry::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PerformanceElementTiming {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceElementTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PerformanceElementTiming {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PerformanceElementTiming {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<PerformanceElementTiming> for emlite::Val {
    fn from(s: PerformanceElementTiming) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PerformanceElementTiming);


impl PerformanceElementTiming {
    pub fn render_time(&self) -> Any {
        self.inner.get("renderTime").as_::<Any>()
    }

}
impl PerformanceElementTiming {
    pub fn load_time(&self) -> Any {
        self.inner.get("loadTime").as_::<Any>()
    }

}
impl PerformanceElementTiming {
    pub fn intersection_rect(&self) -> DOMRectReadOnly {
        self.inner.get("intersectionRect").as_::<DOMRectReadOnly>()
    }

}
impl PerformanceElementTiming {
    pub fn identifier(&self) -> DOMString {
        self.inner.get("identifier").as_::<DOMString>()
    }

}
impl PerformanceElementTiming {
    pub fn natural_width(&self) -> u32 {
        self.inner.get("naturalWidth").as_::<u32>()
    }

}
impl PerformanceElementTiming {
    pub fn natural_height(&self) -> u32 {
        self.inner.get("naturalHeight").as_::<u32>()
    }

}
impl PerformanceElementTiming {
    pub fn id(&self) -> DOMString {
        self.inner.get("id").as_::<DOMString>()
    }

}
impl PerformanceElementTiming {
    pub fn element(&self) -> Element {
        self.inner.get("element").as_::<Element>()
    }

}
impl PerformanceElementTiming {
    pub fn url(&self) -> USVString {
        self.inner.get("url").as_::<USVString>()
    }

}
impl PerformanceElementTiming {
    pub fn to_json(&self, ) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }

}
impl PerformanceElementTiming {
    pub fn paint_time(&self) -> Any {
        self.inner.get("paintTime").as_::<Any>()
    }

}
impl PerformanceElementTiming {
    pub fn presentation_time(&self) -> Any {
        self.inner.get("presentationTime").as_::<Any>()
    }

}
