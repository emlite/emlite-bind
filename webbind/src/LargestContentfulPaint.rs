use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LargestContentfulPaint {
    inner: PerformanceEntry,
}
impl FromVal for LargestContentfulPaint {
    fn from_val(v: &emlite::Val) -> Self {
        LargestContentfulPaint { inner: PerformanceEntry::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for LargestContentfulPaint {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LargestContentfulPaint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for LargestContentfulPaint {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for LargestContentfulPaint {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<LargestContentfulPaint> for emlite::Val {
    fn from(s: LargestContentfulPaint) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(LargestContentfulPaint);


impl LargestContentfulPaint {
    pub fn load_time(&self) -> Any {
        self.inner.get("loadTime").as_::<Any>()
    }

}
impl LargestContentfulPaint {
    pub fn size(&self) -> u32 {
        self.inner.get("size").as_::<u32>()
    }

}
impl LargestContentfulPaint {
    pub fn id(&self) -> DOMString {
        self.inner.get("id").as_::<DOMString>()
    }

}
impl LargestContentfulPaint {
    pub fn url(&self) -> DOMString {
        self.inner.get("url").as_::<DOMString>()
    }

}
impl LargestContentfulPaint {
    pub fn element(&self) -> Element {
        self.inner.get("element").as_::<Element>()
    }

}
impl LargestContentfulPaint {
    pub fn to_json(&self, ) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }

}
impl LargestContentfulPaint {
    pub fn paint_time(&self) -> Any {
        self.inner.get("paintTime").as_::<Any>()
    }

}
impl LargestContentfulPaint {
    pub fn presentation_time(&self) -> Any {
        self.inner.get("presentationTime").as_::<Any>()
    }

}
