use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigatorUABrandVersion {
    inner: Any,
}
impl FromVal for NavigatorUABrandVersion {
    fn from_val(v: &Any) -> Self {
        NavigatorUABrandVersion { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NavigatorUABrandVersion {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigatorUABrandVersion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NavigatorUABrandVersion {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NavigatorUABrandVersion {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NavigatorUABrandVersion> for Any {
    fn from(s: NavigatorUABrandVersion) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NavigatorUABrandVersion> for Any {
    fn from(s: &NavigatorUABrandVersion) -> Any {
        s.inner.clone()
    }
}

impl NavigatorUABrandVersion {
    pub fn brand(&self) -> DOMString {
        self.inner.get("brand").as_::<DOMString>()
    }

    pub fn set_brand(&mut self, value: &DOMString) {
        self.inner.set("brand", value);
    }
}
impl NavigatorUABrandVersion {
    pub fn version(&self) -> DOMString {
        self.inner.get("version").as_::<DOMString>()
    }

    pub fn set_version(&mut self, value: &DOMString) {
        self.inner.set("version", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct UADataValues {
    inner: Any,
}
impl FromVal for UADataValues {
    fn from_val(v: &Any) -> Self {
        UADataValues { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for UADataValues {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for UADataValues {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for UADataValues {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for UADataValues {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<UADataValues> for Any {
    fn from(s: UADataValues) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&UADataValues> for Any {
    fn from(s: &UADataValues) -> Any {
        s.inner.clone()
    }
}

impl UADataValues {
    pub fn architecture(&self) -> DOMString {
        self.inner.get("architecture").as_::<DOMString>()
    }

    pub fn set_architecture(&mut self, value: &DOMString) {
        self.inner.set("architecture", value);
    }
}
impl UADataValues {
    pub fn bitness(&self) -> DOMString {
        self.inner.get("bitness").as_::<DOMString>()
    }

    pub fn set_bitness(&mut self, value: &DOMString) {
        self.inner.set("bitness", value);
    }
}
impl UADataValues {
    pub fn brands(&self) -> Sequence<NavigatorUABrandVersion> {
        self.inner
            .get("brands")
            .as_::<Sequence<NavigatorUABrandVersion>>()
    }

    pub fn set_brands(&mut self, value: &Sequence<NavigatorUABrandVersion>) {
        self.inner.set("brands", value);
    }
}
impl UADataValues {
    pub fn form_factors(&self) -> Sequence<DOMString> {
        self.inner.get("formFactors").as_::<Sequence<DOMString>>()
    }

    pub fn set_form_factors(&mut self, value: &Sequence<DOMString>) {
        self.inner.set("formFactors", value);
    }
}
impl UADataValues {
    pub fn full_version_list(&self) -> Sequence<NavigatorUABrandVersion> {
        self.inner
            .get("fullVersionList")
            .as_::<Sequence<NavigatorUABrandVersion>>()
    }

    pub fn set_full_version_list(&mut self, value: &Sequence<NavigatorUABrandVersion>) {
        self.inner.set("fullVersionList", value);
    }
}
impl UADataValues {
    pub fn model(&self) -> DOMString {
        self.inner.get("model").as_::<DOMString>()
    }

    pub fn set_model(&mut self, value: &DOMString) {
        self.inner.set("model", value);
    }
}
impl UADataValues {
    pub fn mobile(&self) -> bool {
        self.inner.get("mobile").as_::<bool>()
    }

    pub fn set_mobile(&mut self, value: bool) {
        self.inner.set("mobile", value);
    }
}
impl UADataValues {
    pub fn platform(&self) -> DOMString {
        self.inner.get("platform").as_::<DOMString>()
    }

    pub fn set_platform(&mut self, value: &DOMString) {
        self.inner.set("platform", value);
    }
}
impl UADataValues {
    pub fn platform_version(&self) -> DOMString {
        self.inner.get("platformVersion").as_::<DOMString>()
    }

    pub fn set_platform_version(&mut self, value: &DOMString) {
        self.inner.set("platformVersion", value);
    }
}
impl UADataValues {
    pub fn ua_full_version(&self) -> DOMString {
        self.inner.get("uaFullVersion").as_::<DOMString>()
    }

    pub fn set_ua_full_version(&mut self, value: &DOMString) {
        self.inner.set("uaFullVersion", value);
    }
}
impl UADataValues {
    pub fn wow64(&self) -> bool {
        self.inner.get("wow64").as_::<bool>()
    }

    pub fn set_wow64(&mut self, value: bool) {
        self.inner.set("wow64", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct UALowEntropyJSON {
    inner: Any,
}
impl FromVal for UALowEntropyJSON {
    fn from_val(v: &Any) -> Self {
        UALowEntropyJSON { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for UALowEntropyJSON {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for UALowEntropyJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for UALowEntropyJSON {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for UALowEntropyJSON {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<UALowEntropyJSON> for Any {
    fn from(s: UALowEntropyJSON) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&UALowEntropyJSON> for Any {
    fn from(s: &UALowEntropyJSON) -> Any {
        s.inner.clone()
    }
}

impl UALowEntropyJSON {
    pub fn brands(&self) -> Sequence<NavigatorUABrandVersion> {
        self.inner
            .get("brands")
            .as_::<Sequence<NavigatorUABrandVersion>>()
    }

    pub fn set_brands(&mut self, value: &Sequence<NavigatorUABrandVersion>) {
        self.inner.set("brands", value);
    }
}
impl UALowEntropyJSON {
    pub fn mobile(&self) -> bool {
        self.inner.get("mobile").as_::<bool>()
    }

    pub fn set_mobile(&mut self, value: bool) {
        self.inner.set("mobile", value);
    }
}
impl UALowEntropyJSON {
    pub fn platform(&self) -> DOMString {
        self.inner.get("platform").as_::<DOMString>()
    }

    pub fn set_platform(&mut self, value: &DOMString) {
        self.inner.set("platform", value);
    }
}
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
    pub fn brands(&self) -> FrozenArray<NavigatorUABrandVersion> {
        self.inner
            .get("brands")
            .as_::<FrozenArray<NavigatorUABrandVersion>>()
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
    pub fn platform(&self) -> DOMString {
        self.inner.get("platform").as_::<DOMString>()
    }
}
impl NavigatorUAData {
    /// The getHighEntropyValues method.
    /// [`NavigatorUAData.getHighEntropyValues`](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorUAData/getHighEntropyValues)
    pub fn get_high_entropy_values(&self, hints: &Sequence<DOMString>) -> Promise<UADataValues> {
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
