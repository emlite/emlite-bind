use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HIDDeviceFilter {
    inner: Any,
}
impl FromVal for HIDDeviceFilter {
    fn from_val(v: &Any) -> Self {
        HIDDeviceFilter { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HIDDeviceFilter {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HIDDeviceFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HIDDeviceFilter {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HIDDeviceFilter {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HIDDeviceFilter> for Any {
    fn from(s: HIDDeviceFilter) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HIDDeviceFilter> for Any {
    fn from(s: &HIDDeviceFilter) -> Any {
        s.inner.clone()
    }
}

impl HIDDeviceFilter {
    pub fn vendor_id(&self) -> u32 {
        self.inner.get("vendorId").as_::<u32>()
    }

    pub fn set_vendor_id(&mut self, value: u32) {
        self.inner.set("vendorId", value);
    }
}
impl HIDDeviceFilter {
    pub fn product_id(&self) -> u16 {
        self.inner.get("productId").as_::<u16>()
    }

    pub fn set_product_id(&mut self, value: u16) {
        self.inner.set("productId", value);
    }
}
impl HIDDeviceFilter {
    pub fn usage_page(&self) -> u16 {
        self.inner.get("usagePage").as_::<u16>()
    }

    pub fn set_usage_page(&mut self, value: u16) {
        self.inner.set("usagePage", value);
    }
}
impl HIDDeviceFilter {
    pub fn usage(&self) -> u16 {
        self.inner.get("usage").as_::<u16>()
    }

    pub fn set_usage(&mut self, value: u16) {
        self.inner.set("usage", value);
    }
}
