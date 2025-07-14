use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StorageInterestGroup {
    inner: emlite::Val,
}
impl FromVal for StorageInterestGroup {
    fn from_val(v: &emlite::Val) -> Self {
        StorageInterestGroup { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for StorageInterestGroup {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StorageInterestGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for StorageInterestGroup {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for StorageInterestGroup {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<StorageInterestGroup> for emlite::Val {
    fn from(s: StorageInterestGroup) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
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
    pub fn prev_wins_ms(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("prevWinsMs")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_prev_wins_ms(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("prevWinsMs", value);
    }
}
impl StorageInterestGroup {
    pub fn joining_origin(&self) -> jsbind::USVString {
        self.inner.get("joiningOrigin").as_::<jsbind::USVString>()
    }

    pub fn set_joining_origin(&mut self, value: jsbind::USVString) {
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorageWorkletGlobalScope {
    inner: WorkletGlobalScope,
}
impl FromVal for SharedStorageWorkletGlobalScope {
    fn from_val(v: &emlite::Val) -> Self {
        SharedStorageWorkletGlobalScope {
            inner: WorkletGlobalScope::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for SharedStorageWorkletGlobalScope {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SharedStorageWorkletGlobalScope {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SharedStorageWorkletGlobalScope> for emlite::Val {
    fn from(s: SharedStorageWorkletGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SharedStorageWorkletGlobalScope);

impl SharedStorageWorkletGlobalScope {
    pub fn register(
        &self,
        name: jsbind::DOMString,
        operation_ctor: jsbind::Function,
    ) -> jsbind::Undefined {
        self.inner
            .call("register", &[name.into(), operation_ctor.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl SharedStorageWorkletGlobalScope {
    pub fn shared_storage(&self) -> SharedStorage {
        self.inner.get("sharedStorage").as_::<SharedStorage>()
    }
}
impl SharedStorageWorkletGlobalScope {
    pub fn private_aggregation(&self) -> PrivateAggregation {
        self.inner
            .get("privateAggregation")
            .as_::<PrivateAggregation>()
    }
}
impl SharedStorageWorkletGlobalScope {
    pub fn interest_groups(&self) -> jsbind::Promise {
        self.inner
            .call("interestGroups", &[])
            .as_::<jsbind::Promise>()
    }
}
impl SharedStorageWorkletGlobalScope {
    pub fn navigator(&self) -> SharedStorageWorkletNavigator {
        self.inner
            .get("navigator")
            .as_::<SharedStorageWorkletNavigator>()
    }
}
