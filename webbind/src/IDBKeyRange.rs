use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBKeyRange {
    inner: emlite::Val,
}
impl FromVal for IDBKeyRange {
    fn from_val(v: &emlite::Val) -> Self {
        IDBKeyRange {
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
impl core::ops::Deref for IDBKeyRange {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IDBKeyRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for IDBKeyRange {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IDBKeyRange {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<IDBKeyRange> for emlite::Val {
    fn from(s: IDBKeyRange) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&IDBKeyRange> for emlite::Val {
    fn from(s: &IDBKeyRange) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(IDBKeyRange);

impl IDBKeyRange {
    pub fn lower(&self) -> Any {
        self.inner.get("lower").as_::<Any>()
    }
}
impl IDBKeyRange {
    pub fn upper(&self) -> Any {
        self.inner.get("upper").as_::<Any>()
    }
}
impl IDBKeyRange {
    pub fn lower_open(&self) -> bool {
        self.inner.get("lowerOpen").as_::<bool>()
    }
}
impl IDBKeyRange {
    pub fn upper_open(&self) -> bool {
        self.inner.get("upperOpen").as_::<bool>()
    }
}
impl IDBKeyRange {
    pub fn only(value: Any) -> IDBKeyRange {
        emlite::Val::global("IDBKeyRange")
            .call("only", &[value.into()])
            .as_::<IDBKeyRange>()
    }
}
impl IDBKeyRange {
    pub fn lower_bound0(lower: Any) -> IDBKeyRange {
        emlite::Val::global("IDBKeyRange")
            .call("lowerBound", &[lower.into()])
            .as_::<IDBKeyRange>()
    }

    pub fn lower_bound1(lower: Any, open: bool) -> IDBKeyRange {
        emlite::Val::global("IDBKeyRange")
            .call("lowerBound", &[lower.into(), open.into()])
            .as_::<IDBKeyRange>()
    }
}
impl IDBKeyRange {
    pub fn upper_bound0(upper: Any) -> IDBKeyRange {
        emlite::Val::global("IDBKeyRange")
            .call("upperBound", &[upper.into()])
            .as_::<IDBKeyRange>()
    }

    pub fn upper_bound1(upper: Any, open: bool) -> IDBKeyRange {
        emlite::Val::global("IDBKeyRange")
            .call("upperBound", &[upper.into(), open.into()])
            .as_::<IDBKeyRange>()
    }
}
impl IDBKeyRange {
    pub fn bound0(lower: Any, upper: Any) -> IDBKeyRange {
        emlite::Val::global("IDBKeyRange")
            .call("bound", &[lower.into(), upper.into()])
            .as_::<IDBKeyRange>()
    }

    pub fn bound1(lower: Any, upper: Any, lower_open: bool) -> IDBKeyRange {
        emlite::Val::global("IDBKeyRange")
            .call("bound", &[lower.into(), upper.into(), lower_open.into()])
            .as_::<IDBKeyRange>()
    }

    pub fn bound2(lower: Any, upper: Any, lower_open: bool, upper_open: bool) -> IDBKeyRange {
        emlite::Val::global("IDBKeyRange")
            .call(
                "bound",
                &[
                    lower.into(),
                    upper.into(),
                    lower_open.into(),
                    upper_open.into(),
                ],
            )
            .as_::<IDBKeyRange>()
    }
}
impl IDBKeyRange {
    pub fn includes(&self, key: Any) -> bool {
        self.inner.call("includes", &[key.into()]).as_::<bool>()
    }
}
