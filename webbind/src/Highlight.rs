use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Highlight {
    inner: emlite::Val,
}
impl FromVal for Highlight {
    fn from_val(v: &emlite::Val) -> Self {
        Highlight {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Highlight {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Highlight {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Highlight {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Highlight {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Highlight> for emlite::Val {
    fn from(s: Highlight) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Highlight);

impl Highlight {
    pub fn new(initial_ranges: AbstractRange) -> Highlight {
        Self {
            inner: emlite::Val::global("Highlight")
                .new(&[initial_ranges.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl Highlight {
    pub fn priority(&self) -> i32 {
        self.inner.get("priority").as_::<i32>()
    }

    pub fn set_priority(&mut self, value: i32) {
        self.inner.set("priority", value);
    }
}
impl Highlight {
    pub fn type_(&self) -> HighlightType {
        self.inner.get("type").as_::<HighlightType>()
    }

    pub fn set_type_(&mut self, value: HighlightType) {
        self.inner.set("type", value);
    }
}
