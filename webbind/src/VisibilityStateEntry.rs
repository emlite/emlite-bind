use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VisibilityStateEntry {
    inner: PerformanceEntry,
}
impl FromVal for VisibilityStateEntry {
    fn from_val(v: &emlite::Val) -> Self {
        VisibilityStateEntry { inner: PerformanceEntry::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VisibilityStateEntry {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VisibilityStateEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for VisibilityStateEntry {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for VisibilityStateEntry {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<VisibilityStateEntry> for emlite::Val {
    fn from(s: VisibilityStateEntry) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(VisibilityStateEntry);


impl VisibilityStateEntry {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

}
impl VisibilityStateEntry {
    pub fn entry_type(&self) -> DOMString {
        self.inner.get("entryType").as_::<DOMString>()
    }

}
impl VisibilityStateEntry {
    pub fn start_time(&self) -> Any {
        self.inner.get("startTime").as_::<Any>()
    }

}
impl VisibilityStateEntry {
    pub fn duration(&self) -> u32 {
        self.inner.get("duration").as_::<u32>()
    }

}
