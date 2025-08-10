use super::*;

/// The BiddingBrowserSignals dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BiddingBrowserSignals {
    inner: Any,
}

impl FromVal for BiddingBrowserSignals {
    fn from_val(v: &Any) -> Self {
        BiddingBrowserSignals { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BiddingBrowserSignals {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BiddingBrowserSignals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BiddingBrowserSignals {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BiddingBrowserSignals {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BiddingBrowserSignals> for Any {
    fn from(s: BiddingBrowserSignals) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BiddingBrowserSignals> for Any {
    fn from(s: &BiddingBrowserSignals) -> Any {
        s.inner.clone()
    }
}

impl BiddingBrowserSignals {
    /// Getter of the `topWindowHostname` attribute.
    pub fn top_window_hostname(&self) -> JsString {
        self.inner.get("topWindowHostname").as_::<JsString>()
    }

    /// Setter of the `topWindowHostname` attribute.
    pub fn set_top_window_hostname(&mut self, value: &JsString) {
        self.inner.set("topWindowHostname", value);
    }
}
impl BiddingBrowserSignals {
    /// Getter of the `seller` attribute.
    pub fn seller(&self) -> JsString {
        self.inner.get("seller").as_::<JsString>()
    }

    /// Setter of the `seller` attribute.
    pub fn set_seller(&mut self, value: &JsString) {
        self.inner.set("seller", value);
    }
}
impl BiddingBrowserSignals {
    /// Getter of the `joinCount` attribute.
    pub fn join_count(&self) -> i32 {
        self.inner.get("joinCount").as_::<i32>()
    }

    /// Setter of the `joinCount` attribute.
    pub fn set_join_count(&mut self, value: i32) {
        self.inner.set("joinCount", value);
    }
}
impl BiddingBrowserSignals {
    /// Getter of the `bidCount` attribute.
    pub fn bid_count(&self) -> i32 {
        self.inner.get("bidCount").as_::<i32>()
    }

    /// Setter of the `bidCount` attribute.
    pub fn set_bid_count(&mut self, value: i32) {
        self.inner.set("bidCount", value);
    }
}
impl BiddingBrowserSignals {
    /// Getter of the `recency` attribute.
    pub fn recency(&self) -> i32 {
        self.inner.get("recency").as_::<i32>()
    }

    /// Setter of the `recency` attribute.
    pub fn set_recency(&mut self, value: i32) {
        self.inner.set("recency", value);
    }
}
impl BiddingBrowserSignals {
    /// Getter of the `adComponentsLimit` attribute.
    pub fn ad_components_limit(&self) -> i32 {
        self.inner.get("adComponentsLimit").as_::<i32>()
    }

    /// Setter of the `adComponentsLimit` attribute.
    pub fn set_ad_components_limit(&mut self, value: i32) {
        self.inner.set("adComponentsLimit", value);
    }
}
impl BiddingBrowserSignals {
    /// Getter of the `multiBidLimit` attribute.
    pub fn multi_bid_limit(&self) -> u16 {
        self.inner.get("multiBidLimit").as_::<u16>()
    }

    /// Setter of the `multiBidLimit` attribute.
    pub fn set_multi_bid_limit(&mut self, value: u16) {
        self.inner.set("multiBidLimit", value);
    }
}
impl BiddingBrowserSignals {
    /// Getter of the `requestedSize` attribute.
    pub fn requested_size(&self) -> Record<JsString, JsString> {
        self.inner
            .get("requestedSize")
            .as_::<Record<JsString, JsString>>()
    }

    /// Setter of the `requestedSize` attribute.
    pub fn set_requested_size(&mut self, value: &Record<JsString, JsString>) {
        self.inner.set("requestedSize", value);
    }
}
impl BiddingBrowserSignals {
    /// Getter of the `topLevelSeller` attribute.
    pub fn top_level_seller(&self) -> JsString {
        self.inner.get("topLevelSeller").as_::<JsString>()
    }

    /// Setter of the `topLevelSeller` attribute.
    pub fn set_top_level_seller(&mut self, value: &JsString) {
        self.inner.set("topLevelSeller", value);
    }
}
impl BiddingBrowserSignals {
    /// Getter of the `prevWinsMs` attribute.
    pub fn prev_wins_ms(&self) -> TypedArray<Any> {
        self.inner.get("prevWinsMs").as_::<TypedArray<Any>>()
    }

    /// Setter of the `prevWinsMs` attribute.
    pub fn set_prev_wins_ms(&mut self, value: &TypedArray<Any>) {
        self.inner.set("prevWinsMs", value);
    }
}
impl BiddingBrowserSignals {
    /// Getter of the `wasmHelper` attribute.
    pub fn wasm_helper(&self) -> Object {
        self.inner.get("wasmHelper").as_::<Object>()
    }

    /// Setter of the `wasmHelper` attribute.
    pub fn set_wasm_helper(&mut self, value: &Object) {
        self.inner.set("wasmHelper", value);
    }
}
impl BiddingBrowserSignals {
    /// Getter of the `dataVersion` attribute.
    pub fn data_version(&self) -> u32 {
        self.inner.get("dataVersion").as_::<u32>()
    }

    /// Setter of the `dataVersion` attribute.
    pub fn set_data_version(&mut self, value: u32) {
        self.inner.set("dataVersion", value);
    }
}
impl BiddingBrowserSignals {
    /// Getter of the `crossOriginDataVersion` attribute.
    pub fn cross_origin_data_version(&self) -> u32 {
        self.inner.get("crossOriginDataVersion").as_::<u32>()
    }

    /// Setter of the `crossOriginDataVersion` attribute.
    pub fn set_cross_origin_data_version(&mut self, value: u32) {
        self.inner.set("crossOriginDataVersion", value);
    }
}
impl BiddingBrowserSignals {
    /// Getter of the `forDebuggingOnlyInCooldownOrLockout` attribute.
    pub fn for_debugging_only_in_cooldown_or_lockout(&self) -> bool {
        self.inner
            .get("forDebuggingOnlyInCooldownOrLockout")
            .as_::<bool>()
    }

    /// Setter of the `forDebuggingOnlyInCooldownOrLockout` attribute.
    pub fn set_for_debugging_only_in_cooldown_or_lockout(&mut self, value: bool) {
        self.inner.set("forDebuggingOnlyInCooldownOrLockout", value);
    }
}
