use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StorageEvent {
    inner: Event,
}
impl FromVal for StorageEvent {
    fn from_val(v: &emlite::Val) -> Self {
        StorageEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl From<StorageEvent> for emlite::Val {
    fn from(s: StorageEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl StorageEvent {
    pub fn new0(type_: jsbind::DOMString) -> StorageEvent {
        Self {
            inner: emlite::Val::global("StorageEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> StorageEvent {
        Self {
            inner: emlite::Val::global("StorageEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl StorageEvent {
    pub fn key(&self) -> jsbind::DOMString {
        self.inner.get("key").as_::<jsbind::DOMString>()
    }
}
impl StorageEvent {
    pub fn old_value(&self) -> jsbind::DOMString {
        self.inner.get("oldValue").as_::<jsbind::DOMString>()
    }
}
impl StorageEvent {
    pub fn new_value(&self) -> jsbind::DOMString {
        self.inner.get("newValue").as_::<jsbind::DOMString>()
    }
}
impl StorageEvent {
    pub fn url(&self) -> jsbind::USVString {
        self.inner.get("url").as_::<jsbind::USVString>()
    }
}
impl StorageEvent {
    pub fn storage_area(&self) -> Storage {
        self.inner.get("storageArea").as_::<Storage>()
    }
}
impl StorageEvent {
    pub fn init_storage_event0(&self, type_: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("initStorageEvent", &[type_.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn init_storage_event1(
        &self,
        type_: jsbind::DOMString,
        bubbles: bool,
    ) -> jsbind::Undefined {
        self.inner
            .call("initStorageEvent", &[type_.into(), bubbles.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn init_storage_event2(
        &self,
        type_: jsbind::DOMString,
        bubbles: bool,
        cancelable: bool,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initStorageEvent",
                &[type_.into(), bubbles.into(), cancelable.into()],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn init_storage_event3(
        &self,
        type_: jsbind::DOMString,
        bubbles: bool,
        cancelable: bool,
        key: jsbind::DOMString,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initStorageEvent",
                &[type_.into(), bubbles.into(), cancelable.into(), key.into()],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn init_storage_event4(
        &self,
        type_: jsbind::DOMString,
        bubbles: bool,
        cancelable: bool,
        key: jsbind::DOMString,
        old_value: jsbind::DOMString,
    ) -> jsbind::Undefined {
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
            .as_::<jsbind::Undefined>()
    }

    pub fn init_storage_event5(
        &self,
        type_: jsbind::DOMString,
        bubbles: bool,
        cancelable: bool,
        key: jsbind::DOMString,
        old_value: jsbind::DOMString,
        new_value: jsbind::DOMString,
    ) -> jsbind::Undefined {
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
            .as_::<jsbind::Undefined>()
    }

    pub fn init_storage_event6(
        &self,
        type_: jsbind::DOMString,
        bubbles: bool,
        cancelable: bool,
        key: jsbind::DOMString,
        old_value: jsbind::DOMString,
        new_value: jsbind::DOMString,
        url: jsbind::USVString,
    ) -> jsbind::Undefined {
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
            .as_::<jsbind::Undefined>()
    }

    pub fn init_storage_event7(
        &self,
        type_: jsbind::DOMString,
        bubbles: bool,
        cancelable: bool,
        key: jsbind::DOMString,
        old_value: jsbind::DOMString,
        new_value: jsbind::DOMString,
        url: jsbind::USVString,
        storage_area: Storage,
    ) -> jsbind::Undefined {
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
            .as_::<jsbind::Undefined>()
    }
}
