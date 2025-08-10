use super::*;

/// The NavigatorUAData class.
/// [`NavigatorUAData`](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorUAData)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigatorUAData {
    inner: Any,
}

impl FromVal for NavigatorUAData {
    fn from_val(v: &Any) -> Self {
        NavigatorUAData {
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

impl core::ops::Deref for NavigatorUAData {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NavigatorUAData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NavigatorUAData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NavigatorUAData {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<NavigatorUAData> for Any {
    fn from(s: NavigatorUAData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NavigatorUAData> for Any {
    fn from(s: &NavigatorUAData) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(NavigatorUAData);

impl NavigatorUAData {
    /// Getter of the `brands` attribute.
    /// [`NavigatorUAData.brands`](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorUAData/brands)
    pub fn brands(&self) -> TypedArray<NavigatorUABrandVersion> {
        self.inner
            .get("brands")
            .as_::<TypedArray<NavigatorUABrandVersion>>()
    }
}
impl NavigatorUAData {
    /// Getter of the `mobile` attribute.
    /// [`NavigatorUAData.mobile`](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorUAData/mobile)
    pub fn mobile(&self) -> bool {
        self.inner.get("mobile").as_::<bool>()
    }
}
impl NavigatorUAData {
    /// Getter of the `platform` attribute.
    /// [`NavigatorUAData.platform`](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorUAData/platform)
    pub fn platform(&self) -> JsString {
        self.inner.get("platform").as_::<JsString>()
    }
}
impl NavigatorUAData {
    /// The getHighEntropyValues method.
    /// [`NavigatorUAData.getHighEntropyValues`](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorUAData/getHighEntropyValues)
    pub fn get_high_entropy_values(&self, hints: &TypedArray<JsString>) -> Promise<UADataValues> {
        self.inner
            .call("getHighEntropyValues", &[hints.into()])
            .as_::<Promise<UADataValues>>()
    }
}
impl NavigatorUAData {
    /// The toJSON method.
    /// [`NavigatorUAData.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorUAData/toJSON)
    pub fn to_json(&self) -> UALowEntropyJSON {
        self.inner.call("toJSON", &[]).as_::<UALowEntropyJSON>()
    }
}
