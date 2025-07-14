use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigationUpdateCurrentEntryOptions {
    inner: emlite::Val,
}
impl FromVal for NavigationUpdateCurrentEntryOptions {
    fn from_val(v: &emlite::Val) -> Self {
        NavigationUpdateCurrentEntryOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NavigationUpdateCurrentEntryOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationUpdateCurrentEntryOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NavigationUpdateCurrentEntryOptions> for emlite::Val {
    fn from(s: NavigationUpdateCurrentEntryOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NavigationUpdateCurrentEntryOptions {
    pub fn state(&self) -> jsbind::Any {
        self.inner.get("state").as_::<jsbind::Any>()
    }

    pub fn set_state(&mut self, value: jsbind::Any) {
        self.inner.set("state", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigationResult {
    inner: emlite::Val,
}
impl FromVal for NavigationResult {
    fn from_val(v: &emlite::Val) -> Self {
        NavigationResult { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NavigationResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NavigationResult> for emlite::Val {
    fn from(s: NavigationResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NavigationResult {
    pub fn committed(&self) -> jsbind::Promise {
        self.inner.get("committed").as_::<jsbind::Promise>()
    }

    pub fn set_committed(&mut self, value: jsbind::Promise) {
        self.inner.set("committed", value);
    }
}
impl NavigationResult {
    pub fn finished(&self) -> jsbind::Promise {
        self.inner.get("finished").as_::<jsbind::Promise>()
    }

    pub fn set_finished(&mut self, value: jsbind::Promise) {
        self.inner.set("finished", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigationNavigateOptions {
    inner: emlite::Val,
}
impl FromVal for NavigationNavigateOptions {
    fn from_val(v: &emlite::Val) -> Self {
        NavigationNavigateOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NavigationNavigateOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationNavigateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NavigationNavigateOptions> for emlite::Val {
    fn from(s: NavigationNavigateOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NavigationNavigateOptions {
    pub fn state(&self) -> jsbind::Any {
        self.inner.get("state").as_::<jsbind::Any>()
    }

    pub fn set_state(&mut self, value: jsbind::Any) {
        self.inner.set("state", value);
    }
}
impl NavigationNavigateOptions {
    pub fn history(&self) -> NavigationHistoryBehavior {
        self.inner.get("history").as_::<NavigationHistoryBehavior>()
    }

    pub fn set_history(&mut self, value: NavigationHistoryBehavior) {
        self.inner.set("history", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigationReloadOptions {
    inner: emlite::Val,
}
impl FromVal for NavigationReloadOptions {
    fn from_val(v: &emlite::Val) -> Self {
        NavigationReloadOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NavigationReloadOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationReloadOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NavigationReloadOptions> for emlite::Val {
    fn from(s: NavigationReloadOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NavigationReloadOptions {
    pub fn state(&self) -> jsbind::Any {
        self.inner.get("state").as_::<jsbind::Any>()
    }

    pub fn set_state(&mut self, value: jsbind::Any) {
        self.inner.set("state", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigationOptions {
    inner: emlite::Val,
}
impl FromVal for NavigationOptions {
    fn from_val(v: &emlite::Val) -> Self {
        NavigationOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NavigationOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NavigationOptions> for emlite::Val {
    fn from(s: NavigationOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NavigationOptions {
    pub fn info(&self) -> jsbind::Any {
        self.inner.get("info").as_::<jsbind::Any>()
    }

    pub fn set_info(&mut self, value: jsbind::Any) {
        self.inner.set("info", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Navigation {
    inner: EventTarget,
}
impl FromVal for Navigation {
    fn from_val(v: &emlite::Val) -> Self {
        Navigation {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Navigation {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Navigation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Navigation> for emlite::Val {
    fn from(s: Navigation) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Navigation {
    pub fn entries(&self) -> jsbind::Sequence<NavigationHistoryEntry> {
        self.inner
            .call("entries", &[])
            .as_::<jsbind::Sequence<NavigationHistoryEntry>>()
    }
}
impl Navigation {
    pub fn current_entry(&self) -> NavigationHistoryEntry {
        self.inner
            .get("currentEntry")
            .as_::<NavigationHistoryEntry>()
    }
}
impl Navigation {
    pub fn update_current_entry(
        &self,
        options: NavigationUpdateCurrentEntryOptions,
    ) -> jsbind::Undefined {
        self.inner
            .call("updateCurrentEntry", &[options.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Navigation {
    pub fn transition(&self) -> NavigationTransition {
        self.inner.get("transition").as_::<NavigationTransition>()
    }
}
impl Navigation {
    pub fn activation(&self) -> NavigationActivation {
        self.inner.get("activation").as_::<NavigationActivation>()
    }
}
impl Navigation {
    pub fn can_go_back(&self) -> bool {
        self.inner.get("canGoBack").as_::<bool>()
    }
}
impl Navigation {
    pub fn can_go_forward(&self) -> bool {
        self.inner.get("canGoForward").as_::<bool>()
    }
}
impl Navigation {
    pub fn navigate0(&self, url: jsbind::USVString) -> NavigationResult {
        self.inner
            .call("navigate", &[url.into()])
            .as_::<NavigationResult>()
    }

    pub fn navigate1(
        &self,
        url: jsbind::USVString,
        options: NavigationNavigateOptions,
    ) -> NavigationResult {
        self.inner
            .call("navigate", &[url.into(), options.into()])
            .as_::<NavigationResult>()
    }
}
impl Navigation {
    pub fn reload0(&self) -> NavigationResult {
        self.inner.call("reload", &[]).as_::<NavigationResult>()
    }

    pub fn reload1(&self, options: NavigationReloadOptions) -> NavigationResult {
        self.inner
            .call("reload", &[options.into()])
            .as_::<NavigationResult>()
    }
}
impl Navigation {
    pub fn traverse_to0(&self, key: jsbind::DOMString) -> NavigationResult {
        self.inner
            .call("traverseTo", &[key.into()])
            .as_::<NavigationResult>()
    }

    pub fn traverse_to1(
        &self,
        key: jsbind::DOMString,
        options: NavigationOptions,
    ) -> NavigationResult {
        self.inner
            .call("traverseTo", &[key.into(), options.into()])
            .as_::<NavigationResult>()
    }
}
impl Navigation {
    pub fn back0(&self) -> NavigationResult {
        self.inner.call("back", &[]).as_::<NavigationResult>()
    }

    pub fn back1(&self, options: NavigationOptions) -> NavigationResult {
        self.inner
            .call("back", &[options.into()])
            .as_::<NavigationResult>()
    }
}
impl Navigation {
    pub fn forward0(&self) -> NavigationResult {
        self.inner.call("forward", &[]).as_::<NavigationResult>()
    }

    pub fn forward1(&self, options: NavigationOptions) -> NavigationResult {
        self.inner
            .call("forward", &[options.into()])
            .as_::<NavigationResult>()
    }
}
impl Navigation {
    pub fn onnavigate(&self) -> jsbind::Any {
        self.inner.get("onnavigate").as_::<jsbind::Any>()
    }

    pub fn set_onnavigate(&mut self, value: jsbind::Any) {
        self.inner.set("onnavigate", value);
    }
}
impl Navigation {
    pub fn onnavigatesuccess(&self) -> jsbind::Any {
        self.inner.get("onnavigatesuccess").as_::<jsbind::Any>()
    }

    pub fn set_onnavigatesuccess(&mut self, value: jsbind::Any) {
        self.inner.set("onnavigatesuccess", value);
    }
}
impl Navigation {
    pub fn onnavigateerror(&self) -> jsbind::Any {
        self.inner.get("onnavigateerror").as_::<jsbind::Any>()
    }

    pub fn set_onnavigateerror(&mut self, value: jsbind::Any) {
        self.inner.set("onnavigateerror", value);
    }
}
impl Navigation {
    pub fn oncurrententrychange(&self) -> jsbind::Any {
        self.inner.get("oncurrententrychange").as_::<jsbind::Any>()
    }

    pub fn set_oncurrententrychange(&mut self, value: jsbind::Any) {
        self.inner.set("oncurrententrychange", value);
    }
}
