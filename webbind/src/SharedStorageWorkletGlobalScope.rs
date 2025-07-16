use super::*;

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
    pub fn join_count(&self) -> u64 {
        self.inner.get("joinCount").as_::<u64>()
    }

    pub fn set_join_count(&mut self, value: u64) {
        self.inner.set("joinCount", value);
    }
}
impl StorageInterestGroup {
    pub fn bid_count(&self) -> u64 {
        self.inner.get("bidCount").as_::<u64>()
    }

    pub fn set_bid_count(&mut self, value: u64) {
        self.inner.set("bidCount", value);
    }
}
impl StorageInterestGroup {
    pub fn prev_wins_ms(&self) -> Sequence<Any> {
        self.inner.get("prevWinsMs").as_::<Sequence<Any>>()
    }

    pub fn set_prev_wins_ms(&mut self, value: &Sequence<Any>) {
        self.inner.set("prevWinsMs", value);
    }
}
impl StorageInterestGroup {
    pub fn joining_origin(&self) -> String {
        self.inner.get("joiningOrigin").as_::<String>()
    }

    pub fn set_joining_origin(&mut self, value: &str) {
        self.inner.set("joiningOrigin", value);
    }
}
impl StorageInterestGroup {
    pub fn time_since_group_joined_ms(&self) -> i64 {
        self.inner.get("timeSinceGroupJoinedMs").as_::<i64>()
    }

    pub fn set_time_since_group_joined_ms(&mut self, value: i64) {
        self.inner.set("timeSinceGroupJoinedMs", value);
    }
}
impl StorageInterestGroup {
    pub fn lifetime_remaining_ms(&self) -> i64 {
        self.inner.get("lifetimeRemainingMs").as_::<i64>()
    }

    pub fn set_lifetime_remaining_ms(&mut self, value: i64) {
        self.inner.set("lifetimeRemainingMs", value);
    }
}
impl StorageInterestGroup {
    pub fn time_since_last_update_ms(&self) -> i64 {
        self.inner.get("timeSinceLastUpdateMs").as_::<i64>()
    }

    pub fn set_time_since_last_update_ms(&mut self, value: i64) {
        self.inner.set("timeSinceLastUpdateMs", value);
    }
}
impl StorageInterestGroup {
    pub fn time_until_next_update_ms(&self) -> i64 {
        self.inner.get("timeUntilNextUpdateMs").as_::<i64>()
    }

    pub fn set_time_until_next_update_ms(&mut self, value: i64) {
        self.inner.set("timeUntilNextUpdateMs", value);
    }
}
impl StorageInterestGroup {
    pub fn estimated_size(&self) -> u64 {
        self.inner.get("estimatedSize").as_::<u64>()
    }

    pub fn set_estimated_size(&mut self, value: u64) {
        self.inner.set("estimatedSize", value);
    }
}
/// The SharedStorageWorkletGlobalScope class.
/// [`SharedStorageWorkletGlobalScope`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorkletGlobalScope)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorageWorkletGlobalScope {
    inner: WorkletGlobalScope,
}
impl FromVal for SharedStorageWorkletGlobalScope {
    fn from_val(v: &Any) -> Self {
        SharedStorageWorkletGlobalScope {
            inner: WorkletGlobalScope::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SharedStorageWorkletGlobalScope {
    type Target = WorkletGlobalScope;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedStorageWorkletGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SharedStorageWorkletGlobalScope {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SharedStorageWorkletGlobalScope {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SharedStorageWorkletGlobalScope> for Any {
    fn from(s: SharedStorageWorkletGlobalScope) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SharedStorageWorkletGlobalScope> for Any {
    fn from(s: &SharedStorageWorkletGlobalScope) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SharedStorageWorkletGlobalScope);

impl SharedStorageWorkletGlobalScope {
    /// The register method.
    /// [`SharedStorageWorkletGlobalScope.register`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorkletGlobalScope/register)
    pub fn register(&self, name: &str, operation_ctor: &Function) -> Undefined {
        self.inner
            .call("register", &[name.into(), operation_ctor.into()])
            .as_::<Undefined>()
    }
}
impl SharedStorageWorkletGlobalScope {
    /// Getter of the `sharedStorage` attribute.
    /// [`SharedStorageWorkletGlobalScope.sharedStorage`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorkletGlobalScope/sharedStorage)
    pub fn shared_storage(&self) -> SharedStorage {
        self.inner.get("sharedStorage").as_::<SharedStorage>()
    }
}
impl SharedStorageWorkletGlobalScope {
    /// Getter of the `privateAggregation` attribute.
    /// [`SharedStorageWorkletGlobalScope.privateAggregation`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorkletGlobalScope/privateAggregation)
    pub fn private_aggregation(&self) -> PrivateAggregation {
        self.inner
            .get("privateAggregation")
            .as_::<PrivateAggregation>()
    }
}
impl SharedStorageWorkletGlobalScope {
    /// The interestGroups method.
    /// [`SharedStorageWorkletGlobalScope.interestGroups`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorkletGlobalScope/interestGroups)
    pub fn interest_groups(&self) -> Promise {
        self.inner.call("interestGroups", &[]).as_::<Promise>()
    }
}
impl SharedStorageWorkletGlobalScope {
    /// Getter of the `navigator` attribute.
    /// [`SharedStorageWorkletGlobalScope.navigator`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorkletGlobalScope/navigator)
    pub fn navigator(&self) -> SharedStorageWorkletNavigator {
        self.inner
            .get("navigator")
            .as_::<SharedStorageWorkletNavigator>()
    }
}
