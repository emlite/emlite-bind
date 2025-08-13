use super::*;




/// The Navigation class.
/// [`Navigation`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Navigation {
    inner: EventTarget,
}

impl FromVal for Navigation {
    fn from_val(v: &Any) -> Self {
        Navigation { inner: EventTarget::from_val(v) }
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
    pub fn entries(&self, ) -> TypedArray<NavigationHistoryEntry> {
        self.inner.call("entries", &[]).as_::<TypedArray<NavigationHistoryEntry>>()
    }
}
impl Navigation {
    /// Getter of the `currentEntry` attribute.
    /// [`Navigation.currentEntry`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/currentEntry)
    pub fn current_entry(&self) -> NavigationHistoryEntry {
        self.inner.get("currentEntry").as_::<NavigationHistoryEntry>()
    }

}
impl Navigation {
    /// The updateCurrentEntry method.
    /// [`Navigation.updateCurrentEntry`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/updateCurrentEntry)
    pub fn update_current_entry(&self, options: &NavigationUpdateCurrentEntryOptions) -> Undefined {
        self.inner.call("updateCurrentEntry", &[options.into(), ]).as_::<Undefined>()
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
    pub fn navigate0(&self, url: &JsString) -> NavigationResult {
        self.inner.call("navigate", &[url.into(), ]).as_::<NavigationResult>()
    }
    /// The navigate method.
    /// [`Navigation.navigate`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/navigate)
    pub fn navigate1(&self, url: &JsString, options: &NavigationNavigateOptions) -> NavigationResult {
        self.inner.call("navigate", &[url.into(), options.into(), ]).as_::<NavigationResult>()
    }
}
impl Navigation {
    /// The reload method.
    /// [`Navigation.reload`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/reload)
    pub fn reload0(&self, ) -> NavigationResult {
        self.inner.call("reload", &[]).as_::<NavigationResult>()
    }
    /// The reload method.
    /// [`Navigation.reload`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/reload)
    pub fn reload1(&self, options: &NavigationReloadOptions) -> NavigationResult {
        self.inner.call("reload", &[options.into(), ]).as_::<NavigationResult>()
    }
}
impl Navigation {
    /// The traverseTo method.
    /// [`Navigation.traverseTo`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/traverseTo)
    pub fn traverse_to0(&self, key: &JsString) -> NavigationResult {
        self.inner.call("traverseTo", &[key.into(), ]).as_::<NavigationResult>()
    }
    /// The traverseTo method.
    /// [`Navigation.traverseTo`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/traverseTo)
    pub fn traverse_to1(&self, key: &JsString, options: &NavigationOptions) -> NavigationResult {
        self.inner.call("traverseTo", &[key.into(), options.into(), ]).as_::<NavigationResult>()
    }
}
impl Navigation {
    /// The back method.
    /// [`Navigation.back`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/back)
    pub fn back0(&self, ) -> NavigationResult {
        self.inner.call("back", &[]).as_::<NavigationResult>()
    }
    /// The back method.
    /// [`Navigation.back`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/back)
    pub fn back1(&self, options: &NavigationOptions) -> NavigationResult {
        self.inner.call("back", &[options.into(), ]).as_::<NavigationResult>()
    }
}
impl Navigation {
    /// The forward method.
    /// [`Navigation.forward`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/forward)
    pub fn forward0(&self, ) -> NavigationResult {
        self.inner.call("forward", &[]).as_::<NavigationResult>()
    }
    /// The forward method.
    /// [`Navigation.forward`](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/forward)
    pub fn forward1(&self, options: &NavigationOptions) -> NavigationResult {
        self.inner.call("forward", &[options.into(), ]).as_::<NavigationResult>()
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
