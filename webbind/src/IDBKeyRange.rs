use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for IDBKeyRange {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for IDBKeyRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IDBKeyRange> for emlite::Val {
    fn from(s: IDBKeyRange) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IDBKeyRange {
    pub fn lower(&self) -> jsbind::Any {
        self.inner.get("lower").as_::<jsbind::Any>()
    }
}
impl IDBKeyRange {
    pub fn upper(&self) -> jsbind::Any {
        self.inner.get("upper").as_::<jsbind::Any>()
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
    pub fn only(value: jsbind::Any) -> IDBKeyRange {
        emlite::Val::global("idbkeyrange")
            .call("only", &[value.into()])
            .as_::<IDBKeyRange>()
    }
}
impl IDBKeyRange {
    pub fn lower_bound0(lower: jsbind::Any) -> IDBKeyRange {
        emlite::Val::global("idbkeyrange")
            .call("lowerBound", &[lower.into()])
            .as_::<IDBKeyRange>()
    }

    pub fn lower_bound1(lower: jsbind::Any, open: bool) -> IDBKeyRange {
        emlite::Val::global("idbkeyrange")
            .call("lowerBound", &[lower.into(), open.into()])
            .as_::<IDBKeyRange>()
    }
}
impl IDBKeyRange {
    pub fn upper_bound0(upper: jsbind::Any) -> IDBKeyRange {
        emlite::Val::global("idbkeyrange")
            .call("upperBound", &[upper.into()])
            .as_::<IDBKeyRange>()
    }

    pub fn upper_bound1(upper: jsbind::Any, open: bool) -> IDBKeyRange {
        emlite::Val::global("idbkeyrange")
            .call("upperBound", &[upper.into(), open.into()])
            .as_::<IDBKeyRange>()
    }
}
impl IDBKeyRange {
    pub fn bound0(lower: jsbind::Any, upper: jsbind::Any) -> IDBKeyRange {
        emlite::Val::global("idbkeyrange")
            .call("bound", &[lower.into(), upper.into()])
            .as_::<IDBKeyRange>()
    }

    pub fn bound1(lower: jsbind::Any, upper: jsbind::Any, lower_open: bool) -> IDBKeyRange {
        emlite::Val::global("idbkeyrange")
            .call("bound", &[lower.into(), upper.into(), lower_open.into()])
            .as_::<IDBKeyRange>()
    }

    pub fn bound2(
        lower: jsbind::Any,
        upper: jsbind::Any,
        lower_open: bool,
        upper_open: bool,
    ) -> IDBKeyRange {
        emlite::Val::global("idbkeyrange")
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
    pub fn includes(&self, key: jsbind::Any) -> bool {
        self.inner.call("includes", &[key.into()]).as_::<bool>()
    }
}
