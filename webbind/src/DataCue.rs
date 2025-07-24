use super::*;

/// The DataCue class.
/// [`DataCue`](https://developer.mozilla.org/en-US/docs/Web/API/DataCue)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DataCue {
    inner: TextTrackCue,
}
impl FromVal for DataCue {
    fn from_val(v: &Any) -> Self {
        DataCue {
            inner: TextTrackCue::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DataCue {
    type Target = TextTrackCue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DataCue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DataCue {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DataCue {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DataCue> for Any {
    fn from(s: DataCue) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DataCue> for Any {
    fn from(s: &DataCue) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DataCue);

impl DataCue {
    /// The `new DataCue(..)` constructor, creating a new DataCue instance
    pub fn new0(start_time: f64, end_time: f64, value: &Any) -> DataCue {
        Self {
            inner: Any::global("DataCue")
                .new(&[start_time.into(), end_time.into(), value.into()])
                .as_::<TextTrackCue>(),
        }
    }

    /// The `new DataCue(..)` constructor, creating a new DataCue instance
    pub fn new1(start_time: f64, end_time: f64, value: &Any, type_: &DOMString) -> DataCue {
        Self {
            inner: Any::global("DataCue")
                .new(&[
                    start_time.into(),
                    end_time.into(),
                    value.into(),
                    type_.into(),
                ])
                .as_::<TextTrackCue>(),
        }
    }
}
impl DataCue {
    /// Getter of the `value` attribute.
    /// [`DataCue.value`](https://developer.mozilla.org/en-US/docs/Web/API/DataCue/value)
    pub fn value(&self) -> Any {
        self.inner.get("value").as_::<Any>()
    }

    /// Setter of the `value` attribute.
    /// [`DataCue.value`](https://developer.mozilla.org/en-US/docs/Web/API/DataCue/value)
    pub fn set_value(&mut self, value: &Any) {
        self.inner.set("value", value);
    }
}
impl DataCue {
    /// Getter of the `type` attribute.
    /// [`DataCue.type`](https://developer.mozilla.org/en-US/docs/Web/API/DataCue/type)
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }
}
