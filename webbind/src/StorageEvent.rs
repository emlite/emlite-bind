use super::*;

/// The StorageEvent class.
/// [`StorageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StorageEvent {
    inner: Event,
}

impl FromVal for StorageEvent {
    fn from_val(v: &Any) -> Self {
        StorageEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for StorageEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for StorageEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for StorageEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for StorageEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<StorageEvent> for Any {
    fn from(s: StorageEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&StorageEvent> for Any {
    fn from(s: &StorageEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(StorageEvent);

impl StorageEvent {
    /// Getter of the `key` attribute.
    /// [`StorageEvent.key`](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/key)
    pub fn key(&self) -> JsString {
        self.inner.get("key").as_::<JsString>()
    }
}
impl StorageEvent {
    /// Getter of the `oldValue` attribute.
    /// [`StorageEvent.oldValue`](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/oldValue)
    pub fn old_value(&self) -> JsString {
        self.inner.get("oldValue").as_::<JsString>()
    }
}
impl StorageEvent {
    /// Getter of the `newValue` attribute.
    /// [`StorageEvent.newValue`](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/newValue)
    pub fn new_value(&self) -> JsString {
        self.inner.get("newValue").as_::<JsString>()
    }
}
impl StorageEvent {
    /// Getter of the `url` attribute.
    /// [`StorageEvent.url`](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/url)
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }
}
impl StorageEvent {
    /// Getter of the `storageArea` attribute.
    /// [`StorageEvent.storageArea`](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/storageArea)
    pub fn storage_area(&self) -> Storage {
        self.inner.get("storageArea").as_::<Storage>()
    }
}

impl StorageEvent {
    /// The `new StorageEvent(..)` constructor, creating a new StorageEvent instance
    pub fn new(type_: &JsString) -> StorageEvent {
        Self {
            inner: Any::global("StorageEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }
}

impl StorageEvent {
    /// The `new StorageEvent(..)` constructor, creating a new StorageEvent instance
    pub fn new_with_event_init_dict(
        type_: &JsString,
        event_init_dict: &StorageEventInit,
    ) -> StorageEvent {
        Self {
            inner: Any::global("StorageEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}

impl StorageEvent {
    /// The initStorageEvent method.
    /// [`StorageEvent.initStorageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)
    pub fn init_storage_event(&self, type_: &JsString) -> Undefined {
        self.inner
            .call("initStorageEvent", &[type_.into()])
            .as_::<Undefined>()
    }
}
impl StorageEvent {
    /// The initStorageEvent method.
    /// [`StorageEvent.initStorageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)
    pub fn init_storage_event_with_bubbles(&self, type_: &JsString, bubbles: bool) -> Undefined {
        self.inner
            .call("initStorageEvent", &[type_.into(), bubbles.into()])
            .as_::<Undefined>()
    }
}
impl StorageEvent {
    /// The initStorageEvent method.
    /// [`StorageEvent.initStorageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)
    pub fn init_storage_event_with_bubbles_and_cancelable(
        &self,
        type_: &JsString,
        bubbles: bool,
        cancelable: bool,
    ) -> Undefined {
        self.inner
            .call(
                "initStorageEvent",
                &[type_.into(), bubbles.into(), cancelable.into()],
            )
            .as_::<Undefined>()
    }
}
impl StorageEvent {
    /// The initStorageEvent method.
    /// [`StorageEvent.initStorageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)
    pub fn init_storage_event_with_bubbles_and_cancelable_and_key(
        &self,
        type_: &JsString,
        bubbles: bool,
        cancelable: bool,
        key: &JsString,
    ) -> Undefined {
        self.inner
            .call(
                "initStorageEvent",
                &[type_.into(), bubbles.into(), cancelable.into(), key.into()],
            )
            .as_::<Undefined>()
    }
}
impl StorageEvent {
    /// The initStorageEvent method.
    /// [`StorageEvent.initStorageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)
    pub fn init_storage_event_with_bubbles_and_cancelable_and_key_and_old_value(
        &self,
        type_: &JsString,
        bubbles: bool,
        cancelable: bool,
        key: &JsString,
        old_value: &JsString,
    ) -> Undefined {
        self.inner
            .call(
                "initStorageEvent",
                &[
                    type_.into(),
                    bubbles.into(),
                    cancelable.into(),
                    key.into(),
                    old_value.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl StorageEvent {
    /// The initStorageEvent method.
    /// [`StorageEvent.initStorageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)
    pub fn init_storage_event_with_bubbles_and_cancelable_and_key_and_old_value_and_new_value(
        &self,
        type_: &JsString,
        bubbles: bool,
        cancelable: bool,
        key: &JsString,
        old_value: &JsString,
        new_value: &JsString,
    ) -> Undefined {
        self.inner
            .call(
                "initStorageEvent",
                &[
                    type_.into(),
                    bubbles.into(),
                    cancelable.into(),
                    key.into(),
                    old_value.into(),
                    new_value.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl StorageEvent {
    /// The initStorageEvent method.
    /// [`StorageEvent.initStorageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)
    pub fn init_storage_event_with_bubbles_and_cancelable_and_key_and_old_value_and_new_value_and_url(
        &self,
        type_: &JsString,
        bubbles: bool,
        cancelable: bool,
        key: &JsString,
        old_value: &JsString,
        new_value: &JsString,
        url: &JsString,
    ) -> Undefined {
        self.inner
            .call(
                "initStorageEvent",
                &[
                    type_.into(),
                    bubbles.into(),
                    cancelable.into(),
                    key.into(),
                    old_value.into(),
                    new_value.into(),
                    url.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl StorageEvent {
    /// The initStorageEvent method.
    /// [`StorageEvent.initStorageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)
    pub fn init_storage_event_with_bubbles_and_cancelable_and_key_and_old_value_and_new_value_and_url_and_storage_area(
        &self,
        type_: &JsString,
        bubbles: bool,
        cancelable: bool,
        key: &JsString,
        old_value: &JsString,
        new_value: &JsString,
        url: &JsString,
        storage_area: &Storage,
    ) -> Undefined {
        self.inner
            .call(
                "initStorageEvent",
                &[
                    type_.into(),
                    bubbles.into(),
                    cancelable.into(),
                    key.into(),
                    old_value.into(),
                    new_value.into(),
                    url.into(),
                    storage_area.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
