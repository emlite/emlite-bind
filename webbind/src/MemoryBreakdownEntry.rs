use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MemoryBreakdownEntry {
    inner: Any,
}
impl FromVal for MemoryBreakdownEntry {
    fn from_val(v: &Any) -> Self {
        MemoryBreakdownEntry { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MemoryBreakdownEntry {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MemoryBreakdownEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MemoryBreakdownEntry {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MemoryBreakdownEntry {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MemoryBreakdownEntry> for Any {
    fn from(s: MemoryBreakdownEntry) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MemoryBreakdownEntry> for Any {
    fn from(s: &MemoryBreakdownEntry) -> Any {
        s.inner.clone()
    }
}

impl MemoryBreakdownEntry {
    pub fn bytes(&self) -> u64 {
        self.inner.get("bytes").as_::<u64>()
    }

    pub fn set_bytes(&mut self, value: u64) {
        self.inner.set("bytes", value);
    }
}
impl MemoryBreakdownEntry {
    pub fn attribution(&self) -> TypedArray<MemoryAttribution> {
        self.inner
            .get("attribution")
            .as_::<TypedArray<MemoryAttribution>>()
    }

    pub fn set_attribution(&mut self, value: &TypedArray<MemoryAttribution>) {
        self.inner.set("attribution", value);
    }
}
impl MemoryBreakdownEntry {
    pub fn types(&self) -> TypedArray<JsString> {
        self.inner.get("types").as_::<TypedArray<JsString>>()
    }

    pub fn set_types(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("types", value);
    }
}
