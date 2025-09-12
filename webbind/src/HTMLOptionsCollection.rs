use super::*;

/// The HTMLOptionsCollection class.
/// [`HTMLOptionsCollection`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLOptionsCollection {
    inner: HTMLCollection,
}

impl FromVal for HTMLOptionsCollection {
    fn from_val(v: &Any) -> Self {
        HTMLOptionsCollection {
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

impl core::ops::Deref for HTMLOptionsCollection {
    type Target = HTMLCollection;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLOptionsCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLOptionsCollection {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLOptionsCollection {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLOptionsCollection> for Any {
    fn from(s: HTMLOptionsCollection) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLOptionsCollection> for Any {
    fn from(s: &HTMLOptionsCollection) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLOptionsCollection);

impl HTMLOptionsCollection {
    /// Getter of the `length` attribute.
    /// [`HTMLOptionsCollection.length`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

    /// Setter of the `length` attribute.
    /// [`HTMLOptionsCollection.length`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/length)
    pub fn set_length(&mut self, value: u32) {
        self.inner.set("length", value);
    }
}
impl HTMLOptionsCollection {
    /// Getter of the `selectedIndex` attribute.
    /// [`HTMLOptionsCollection.selectedIndex`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/selectedIndex)
    pub fn selected_index(&self) -> i32 {
        self.inner.get("selectedIndex").as_::<i32>()
    }

    /// Setter of the `selectedIndex` attribute.
    /// [`HTMLOptionsCollection.selectedIndex`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/selectedIndex)
    pub fn set_selected_index(&mut self, value: i32) {
        self.inner.set("selectedIndex", value);
    }
}
impl HTMLOptionsCollection {
    /// The add method.
    /// [`HTMLOptionsCollection.add`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)
    pub fn add(&self, element: &Any) -> Undefined {
        self.inner.call("add", &[element.into()]).as_::<Undefined>()
    }
}
impl HTMLOptionsCollection {
    /// The add method.
    /// [`HTMLOptionsCollection.add`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)
    pub fn add_with_before(&self, element: &Any, before: &Any) -> Undefined {
        self.inner
            .call("add", &[element.into(), before.into()])
            .as_::<Undefined>()
    }
}
impl HTMLOptionsCollection {
    /// The remove method.
    /// [`HTMLOptionsCollection.remove`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/remove)
    pub fn remove(&self, index: i32) -> Undefined {
        self.inner
            .call("remove", &[index.into()])
            .as_::<Undefined>()
    }
}
