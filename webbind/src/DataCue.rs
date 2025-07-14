use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DataCue {
    inner: TextTrackCue,
}
impl FromVal for DataCue {
    fn from_val(v: &emlite::Val) -> Self {
        DataCue {
            inner: TextTrackCue::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl From<DataCue> for emlite::Val {
    fn from(s: DataCue) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DataCue {
    pub fn new0(start_time: f64, end_time: f64, value: jsbind::Any) -> DataCue {
        Self {
            inner: emlite::Val::global("DataCue")
                .new(&[start_time.into(), end_time.into(), value.into()])
                .as_::<TextTrackCue>(),
        }
    }

    pub fn new1(
        start_time: f64,
        end_time: f64,
        value: jsbind::Any,
        type_: jsbind::DOMString,
    ) -> DataCue {
        Self {
            inner: emlite::Val::global("DataCue")
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
    pub fn value(&self) -> jsbind::Any {
        self.inner.get("value").as_::<jsbind::Any>()
    }

    pub fn set_value(&mut self, value: jsbind::Any) {
        self.inner.set("value", value);
    }
}
impl DataCue {
    pub fn type_(&self) -> jsbind::DOMString {
        self.inner.get("type").as_::<jsbind::DOMString>()
    }
}
