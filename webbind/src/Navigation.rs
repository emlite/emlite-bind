use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationUpdateCurrentEntryOptions {
    inner: Any,
}
impl FromVal for NavigationUpdateCurrentEntryOptions {
    fn from_val(v: &Any) -> Self {
        NavigationUpdateCurrentEntryOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NavigationUpdateCurrentEntryOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationUpdateCurrentEntryOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NavigationUpdateCurrentEntryOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NavigationUpdateCurrentEntryOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NavigationUpdateCurrentEntryOptions> for Any {
    fn from(s: NavigationUpdateCurrentEntryOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NavigationUpdateCurrentEntryOptions> for Any {
    fn from(s: &NavigationUpdateCurrentEntryOptions) -> Any {
        s.inner.clone()
    }
}

impl NavigationUpdateCurrentEntryOptions {
    pub fn state(&self) -> Any {
        self.inner.get("state").as_::<Any>()
    }

    pub fn set_state(&mut self, value: &Any) {
        self.inner.set("state", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationResult {
    inner: Any,
}
impl FromVal for NavigationResult {
    fn from_val(v: &Any) -> Self {
        NavigationResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NavigationResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NavigationResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NavigationResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NavigationResult> for Any {
    fn from(s: NavigationResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NavigationResult> for Any {
    fn from(s: &NavigationResult) -> Any {
        s.inner.clone()
    }
}

impl NavigationResult {
    pub fn committed(&self) -> Promise<NavigationHistoryEntry> {
        self.inner
            .get("committed")
            .as_::<Promise<NavigationHistoryEntry>>()
    }

    pub fn set_committed(&mut self, value: &Promise<NavigationHistoryEntry>) {
        self.inner.set("committed", value);
    }
}
impl NavigationResult {
    pub fn finished(&self) -> Promise<NavigationHistoryEntry> {
        self.inner
            .get("finished")
            .as_::<Promise<NavigationHistoryEntry>>()
    }

    pub fn set_finished(&mut self, value: &Promise<NavigationHistoryEntry>) {
        self.inner.set("finished", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationNavigateOptions {
    inner: Any,
}
impl FromVal for NavigationNavigateOptions {
    fn from_val(v: &Any) -> Self {
        NavigationNavigateOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NavigationNavigateOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationNavigateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NavigationNavigateOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NavigationNavigateOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NavigationNavigateOptions> for Any {
    fn from(s: NavigationNavigateOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NavigationNavigateOptions> for Any {
    fn from(s: &NavigationNavigateOptions) -> Any {
        s.inner.clone()
    }
}

impl NavigationNavigateOptions {
    pub fn state(&self) -> Any {
        self.inner.get("state").as_::<Any>()
    }

    pub fn set_state(&mut self, value: &Any) {
        self.inner.set("state", value);
    }
}
impl NavigationNavigateOptions {
    pub fn history(&self) -> NavigationHistoryBehavior {
        self.inner.get("history").as_::<NavigationHistoryBehavior>()
    }

    pub fn set_history(&mut self, value: &NavigationHistoryBehavior) {
        self.inner.set("history", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationReloadOptions {
    inner: Any,
}
impl FromVal for NavigationReloadOptions {
    fn from_val(v: &Any) -> Self {
        NavigationReloadOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NavigationReloadOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationReloadOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NavigationReloadOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NavigationReloadOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NavigationReloadOptions> for Any {
    fn from(s: NavigationReloadOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NavigationReloadOptions> for Any {
    fn from(s: &NavigationReloadOptions) -> Any {
        s.inner.clone()
    }
}

impl NavigationReloadOptions {
    pub fn state(&self) -> Any {
        self.inner.get("state").as_::<Any>()
    }

    pub fn set_state(&mut self, value: &Any) {
        self.inner.set("state", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationOptions {
    inner: Any,
}
impl FromVal for NavigationOptions {
    fn from_val(v: &Any) -> Self {
        NavigationOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NavigationOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NavigationOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NavigationOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NavigationOptions> for Any {
    fn from(s: NavigationOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NavigationOptions> for Any {
    fn from(s: &NavigationOptions) -> Any {
        s.inner.clone()
    }
}

impl NavigationOptions {
    pub fn info(&self) -> Any {
        self.inner.get("info").as_::<Any>()
    }

    pub fn set_info(&mut self, value: &Any) {
        self.inner.set("info", value);
    }
}
/// The Navigation class.
/// [`Navigation`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Navigation {
    inner: EventTarget,
}
impl FromVal for Navigation {
    fn from_val(v: &Any) -> Self {
        Navigation {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for Navigation {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Navigation {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Navigation> for Any {
    fn from(s: Navigation) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Navigation> for Any {
    fn from(s: &Navigation) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Navigation);

impl Navigation {
    /// The entries method.
    /// [`Navigation.entries`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/entries)
    pub fn entries(&self) -> Sequence<NavigationHistoryEntry> {
        self.inner
            .call("entries", &[])
            .as_::<Sequence<NavigationHistoryEntry>>()
    }
}
impl Navigation {
    /// Getter of the `currentEntry` attribute.
    /// [`Navigation.currentEntry`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/currentEntry)
    pub fn current_entry(&self) -> NavigationHistoryEntry {
        self.inner
            .get("currentEntry")
            .as_::<NavigationHistoryEntry>()
    }
}
impl Navigation {
    /// The updateCurrentEntry method.
    /// [`Navigation.updateCurrentEntry`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/updateCurrentEntry)
    pub fn update_current_entry(&self, options: &NavigationUpdateCurrentEntryOptions) -> Undefined {
        self.inner
            .call("updateCurrentEntry", &[options.into()])
            .as_::<Undefined>()
    }
}
impl Navigation {
    /// Getter of the `transition` attribute.
    /// [`Navigation.transition`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/transition)
    pub fn transition(&self) -> NavigationTransition {
        self.inner.get("transition").as_::<NavigationTransition>()
    }
}
impl Navigation {
    /// Getter of the `activation` attribute.
    /// [`Navigation.activation`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/activation)
    pub fn activation(&self) -> NavigationActivation {
        self.inner.get("activation").as_::<NavigationActivation>()
    }
}
impl Navigation {
    /// Getter of the `canGoBack` attribute.
    /// [`Navigation.canGoBack`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/canGoBack)
    pub fn can_go_back(&self) -> bool {
        self.inner.get("canGoBack").as_::<bool>()
    }
}
impl Navigation {
    /// Getter of the `canGoForward` attribute.
    /// [`Navigation.canGoForward`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/canGoForward)
    pub fn can_go_forward(&self) -> bool {
        self.inner.get("canGoForward").as_::<bool>()
    }
}
impl Navigation {
    /// The navigate method.
    /// [`Navigation.navigate`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/navigate)
    pub fn navigate0(&self, url: &str) -> NavigationResult {
        self.inner
            .call("navigate", &[url.into()])
            .as_::<NavigationResult>()
    }
    /// The navigate method.
    /// [`Navigation.navigate`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/navigate)
    pub fn navigate1(&self, url: &str, options: &NavigationNavigateOptions) -> NavigationResult {
        self.inner
            .call("navigate", &[url.into(), options.into()])
            .as_::<NavigationResult>()
    }
}
impl Navigation {
    /// The reload method.
    /// [`Navigation.reload`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/reload)
    pub fn reload0(&self) -> NavigationResult {
        self.inner.call("reload", &[]).as_::<NavigationResult>()
    }
    /// The reload method.
    /// [`Navigation.reload`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/reload)
    pub fn reload1(&self, options: &NavigationReloadOptions) -> NavigationResult {
        self.inner
            .call("reload", &[options.into()])
            .as_::<NavigationResult>()
    }
}
impl Navigation {
    /// The traverseTo method.
    /// [`Navigation.traverseTo`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/traverseTo)
    pub fn traverse_to0(&self, key: &str) -> NavigationResult {
        self.inner
            .call("traverseTo", &[key.into()])
            .as_::<NavigationResult>()
    }
    /// The traverseTo method.
    /// [`Navigation.traverseTo`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/traverseTo)
    pub fn traverse_to1(&self, key: &str, options: &NavigationOptions) -> NavigationResult {
        self.inner
            .call("traverseTo", &[key.into(), options.into()])
            .as_::<NavigationResult>()
    }
}
impl Navigation {
    /// The back method.
    /// [`Navigation.back`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/back)
    pub fn back0(&self) -> NavigationResult {
        self.inner.call("back", &[]).as_::<NavigationResult>()
    }
    /// The back method.
    /// [`Navigation.back`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/back)
    pub fn back1(&self, options: &NavigationOptions) -> NavigationResult {
        self.inner
            .call("back", &[options.into()])
            .as_::<NavigationResult>()
    }
}
impl Navigation {
    /// The forward method.
    /// [`Navigation.forward`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/forward)
    pub fn forward0(&self) -> NavigationResult {
        self.inner.call("forward", &[]).as_::<NavigationResult>()
    }
    /// The forward method.
    /// [`Navigation.forward`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/forward)
    pub fn forward1(&self, options: &NavigationOptions) -> NavigationResult {
        self.inner
            .call("forward", &[options.into()])
            .as_::<NavigationResult>()
    }
}
impl Navigation {
    /// Getter of the `onnavigate` attribute.
    /// [`Navigation.onnavigate`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/onnavigate)
    pub fn onnavigate(&self) -> Any {
        self.inner.get("onnavigate").as_::<Any>()
    }

    /// Setter of the `onnavigate` attribute.
    /// [`Navigation.onnavigate`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/onnavigate)
    pub fn set_onnavigate(&mut self, value: &Any) {
        self.inner.set("onnavigate", value);
    }
}
impl Navigation {
    /// Getter of the `onnavigatesuccess` attribute.
    /// [`Navigation.onnavigatesuccess`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/onnavigatesuccess)
    pub fn onnavigatesuccess(&self) -> Any {
        self.inner.get("onnavigatesuccess").as_::<Any>()
    }

    /// Setter of the `onnavigatesuccess` attribute.
    /// [`Navigation.onnavigatesuccess`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/onnavigatesuccess)
    pub fn set_onnavigatesuccess(&mut self, value: &Any) {
        self.inner.set("onnavigatesuccess", value);
    }
}
impl Navigation {
    /// Getter of the `onnavigateerror` attribute.
    /// [`Navigation.onnavigateerror`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/onnavigateerror)
    pub fn onnavigateerror(&self) -> Any {
        self.inner.get("onnavigateerror").as_::<Any>()
    }

    /// Setter of the `onnavigateerror` attribute.
    /// [`Navigation.onnavigateerror`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/onnavigateerror)
    pub fn set_onnavigateerror(&mut self, value: &Any) {
        self.inner.set("onnavigateerror", value);
    }
}
impl Navigation {
    /// Getter of the `oncurrententrychange` attribute.
    /// [`Navigation.oncurrententrychange`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/oncurrententrychange)
    pub fn oncurrententrychange(&self) -> Any {
        self.inner.get("oncurrententrychange").as_::<Any>()
    }

    /// Setter of the `oncurrententrychange` attribute.
    /// [`Navigation.oncurrententrychange`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/oncurrententrychange)
    pub fn set_oncurrententrychange(&mut self, value: &Any) {
        self.inner.set("oncurrententrychange", value);
    }
}
