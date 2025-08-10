use super::*;

/// The HTMLFormControlsCollection class.
/// [`HTMLFormControlsCollection`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormControlsCollection)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLFormControlsCollection {
    inner: HTMLCollection,
}

impl FromVal for HTMLFormControlsCollection {
    fn from_val(v: &Any) -> Self {
        HTMLFormControlsCollection {
            inner: HTMLCollection::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HTMLFormControlsCollection {
    type Target = HTMLCollection;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLFormControlsCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLFormControlsCollection {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLFormControlsCollection {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLFormControlsCollection> for Any {
    fn from(s: HTMLFormControlsCollection) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLFormControlsCollection> for Any {
    fn from(s: &HTMLFormControlsCollection) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLFormControlsCollection);

impl HTMLFormControlsCollection {
    /// The namedItem method.
    /// [`HTMLFormControlsCollection.namedItem`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormControlsCollection/namedItem)
    pub fn named_item(&self, name: &JsString) -> Any {
        self.inner.call("namedItem", &[name.into()]).as_::<Any>()
    }
}
