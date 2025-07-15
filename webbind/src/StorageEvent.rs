use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for StorageEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for StorageEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
jsbind::utils::impl_dyn_cast!(StorageEvent);

impl StorageEvent {
    pub fn new0(type_: DOMString) -> StorageEvent {
        Self {
            inner: emlite::Val::global("StorageEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> StorageEvent {
        Self {
            inner: emlite::Val::global("StorageEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl StorageEvent {
    pub fn key(&self) -> DOMString {
        self.inner.get("key").as_::<DOMString>()
    }
}
impl StorageEvent {
    pub fn old_value(&self) -> DOMString {
        self.inner.get("oldValue").as_::<DOMString>()
    }
}
impl StorageEvent {
    pub fn new_value(&self) -> DOMString {
        self.inner.get("newValue").as_::<DOMString>()
    }
}
impl StorageEvent {
    pub fn url(&self) -> USVString {
        self.inner.get("url").as_::<USVString>()
    }
}
impl StorageEvent {
    pub fn storage_area(&self) -> Storage {
        self.inner.get("storageArea").as_::<Storage>()
    }
}
impl StorageEvent {
    pub fn init_storage_event0(&self, type_: DOMString) -> Undefined {
        self.inner
            .call("initStorageEvent", &[type_.into()])
            .as_::<Undefined>()
    }

    pub fn init_storage_event1(&self, type_: DOMString, bubbles: bool) -> Undefined {
        self.inner
            .call("initStorageEvent", &[type_.into(), bubbles.into()])
            .as_::<Undefined>()
    }

    pub fn init_storage_event2(
        &self,
        type_: DOMString,
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

    pub fn init_storage_event3(
        &self,
        type_: DOMString,
        bubbles: bool,
        cancelable: bool,
        key: DOMString,
    ) -> Undefined {
        self.inner
            .call(
                "initStorageEvent",
                &[type_.into(), bubbles.into(), cancelable.into(), key.into()],
            )
            .as_::<Undefined>()
    }

    pub fn init_storage_event4(
        &self,
        type_: DOMString,
        bubbles: bool,
        cancelable: bool,
        key: DOMString,
        old_value: DOMString,
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

    pub fn init_storage_event5(
        &self,
        type_: DOMString,
        bubbles: bool,
        cancelable: bool,
        key: DOMString,
        old_value: DOMString,
        new_value: DOMString,
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

    pub fn init_storage_event6(
        &self,
        type_: DOMString,
        bubbles: bool,
        cancelable: bool,
        key: DOMString,
        old_value: DOMString,
        new_value: DOMString,
        url: USVString,
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

    pub fn init_storage_event7(
        &self,
        type_: DOMString,
        bubbles: bool,
        cancelable: bool,
        key: DOMString,
        old_value: DOMString,
        new_value: DOMString,
        url: USVString,
        storage_area: Storage,
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
