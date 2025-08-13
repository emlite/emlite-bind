use super::*;




/// The BufferedChangeEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BufferedChangeEventInit {
    inner: Any,
}

impl FromVal for BufferedChangeEventInit {
    fn from_val(v: &Any) -> Self {
        BufferedChangeEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BufferedChangeEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BufferedChangeEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BufferedChangeEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BufferedChangeEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<BufferedChangeEventInit> for Any {
    fn from(s: BufferedChangeEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BufferedChangeEventInit> for Any {
    fn from(s: &BufferedChangeEventInit) -> Any {
        s.inner.clone()
    }
}

impl BufferedChangeEventInit {
    /// Getter of the `addedRanges` attribute.
    pub fn added_ranges(&self) -> TimeRanges {
        self.inner.get("addedRanges").as_::<TimeRanges>()
    }

    /// Setter of the `addedRanges` attribute.
    pub fn set_added_ranges(&mut self, value: &TimeRanges) {
        self.inner.set("addedRanges", value);
    }
}
impl BufferedChangeEventInit {
    /// Getter of the `removedRanges` attribute.
    pub fn removed_ranges(&self) -> TimeRanges {
        self.inner.get("removedRanges").as_::<TimeRanges>()
    }

    /// Setter of the `removedRanges` attribute.
    pub fn set_removed_ranges(&mut self, value: &TimeRanges) {
        self.inner.set("removedRanges", value);
    }
}
