use super::*;




/// The StorageInterestGroup dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StorageInterestGroup {
    inner: Any,
}

impl FromVal for StorageInterestGroup {
    fn from_val(v: &Any) -> Self {
        StorageInterestGroup { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for StorageInterestGroup {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for StorageInterestGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for StorageInterestGroup {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for StorageInterestGroup {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<StorageInterestGroup> for Any {
    fn from(s: StorageInterestGroup) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&StorageInterestGroup> for Any {
    fn from(s: &StorageInterestGroup) -> Any {
        s.inner.clone()
    }
}

impl StorageInterestGroup {
    /// Getter of the `joinCount` attribute.
    pub fn join_count(&self) -> u64 {
        self.inner.get("joinCount").as_::<u64>()
    }

    /// Setter of the `joinCount` attribute.
    pub fn set_join_count(&mut self, value: u64) {
        self.inner.set("joinCount", value);
    }
}
impl StorageInterestGroup {
    /// Getter of the `bidCount` attribute.
    pub fn bid_count(&self) -> u64 {
        self.inner.get("bidCount").as_::<u64>()
    }

    /// Setter of the `bidCount` attribute.
    pub fn set_bid_count(&mut self, value: u64) {
        self.inner.set("bidCount", value);
    }
}
impl StorageInterestGroup {
    /// Getter of the `prevWinsMs` attribute.
    pub fn prev_wins_ms(&self) -> TypedArray<Any> {
        self.inner.get("prevWinsMs").as_::<TypedArray<Any>>()
    }

    /// Setter of the `prevWinsMs` attribute.
    pub fn set_prev_wins_ms(&mut self, value: &TypedArray<Any>) {
        self.inner.set("prevWinsMs", value);
    }
}
impl StorageInterestGroup {
    /// Getter of the `joiningOrigin` attribute.
    pub fn joining_origin(&self) -> JsString {
        self.inner.get("joiningOrigin").as_::<JsString>()
    }

    /// Setter of the `joiningOrigin` attribute.
    pub fn set_joining_origin(&mut self, value: &JsString) {
        self.inner.set("joiningOrigin", value);
    }
}
impl StorageInterestGroup {
    /// Getter of the `timeSinceGroupJoinedMs` attribute.
    pub fn time_since_group_joined_ms(&self) -> i64 {
        self.inner.get("timeSinceGroupJoinedMs").as_::<i64>()
    }

    /// Setter of the `timeSinceGroupJoinedMs` attribute.
    pub fn set_time_since_group_joined_ms(&mut self, value: i64) {
        self.inner.set("timeSinceGroupJoinedMs", value);
    }
}
impl StorageInterestGroup {
    /// Getter of the `lifetimeRemainingMs` attribute.
    pub fn lifetime_remaining_ms(&self) -> i64 {
        self.inner.get("lifetimeRemainingMs").as_::<i64>()
    }

    /// Setter of the `lifetimeRemainingMs` attribute.
    pub fn set_lifetime_remaining_ms(&mut self, value: i64) {
        self.inner.set("lifetimeRemainingMs", value);
    }
}
impl StorageInterestGroup {
    /// Getter of the `timeSinceLastUpdateMs` attribute.
    pub fn time_since_last_update_ms(&self) -> i64 {
        self.inner.get("timeSinceLastUpdateMs").as_::<i64>()
    }

    /// Setter of the `timeSinceLastUpdateMs` attribute.
    pub fn set_time_since_last_update_ms(&mut self, value: i64) {
        self.inner.set("timeSinceLastUpdateMs", value);
    }
}
impl StorageInterestGroup {
    /// Getter of the `timeUntilNextUpdateMs` attribute.
    pub fn time_until_next_update_ms(&self) -> i64 {
        self.inner.get("timeUntilNextUpdateMs").as_::<i64>()
    }

    /// Setter of the `timeUntilNextUpdateMs` attribute.
    pub fn set_time_until_next_update_ms(&mut self, value: i64) {
        self.inner.set("timeUntilNextUpdateMs", value);
    }
}
impl StorageInterestGroup {
    /// Getter of the `estimatedSize` attribute.
    pub fn estimated_size(&self) -> u64 {
        self.inner.get("estimatedSize").as_::<u64>()
    }

    /// Setter of the `estimatedSize` attribute.
    pub fn set_estimated_size(&mut self, value: u64) {
        self.inner.set("estimatedSize", value);
    }
}
