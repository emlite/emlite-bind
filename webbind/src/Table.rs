use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Table {
    inner: emlite::Val,
}
impl FromVal for Table {
    fn from_val(v: &emlite::Val) -> Self {
        Table {
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
impl core::ops::Deref for Table {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Table {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Table {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Table {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Table> for emlite::Val {
    fn from(s: Table) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Table);

impl Table {
    pub fn new0(descriptor: Any) -> Table {
        Self {
            inner: emlite::Val::global("Table")
                .new(&[descriptor.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(descriptor: Any, value: Any) -> Table {
        Self {
            inner: emlite::Val::global("Table")
                .new(&[descriptor.into(), value.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl Table {
    pub fn grow0(&self, delta: u32) -> u32 {
        self.inner.call("grow", &[delta.into()]).as_::<u32>()
    }

    pub fn grow1(&self, delta: u32, value: Any) -> u32 {
        self.inner
            .call("grow", &[delta.into(), value.into()])
            .as_::<u32>()
    }
}
impl Table {
    pub fn get(&self, index: u32) -> Any {
        self.inner.call("get", &[index.into()]).as_::<Any>()
    }
}
impl Table {
    pub fn set0(&self, index: u32) -> Undefined {
        self.inner.call("set", &[index.into()]).as_::<Undefined>()
    }

    pub fn set1(&self, index: u32, value: Any) -> Undefined {
        self.inner
            .call("set", &[index.into(), value.into()])
            .as_::<Undefined>()
    }
}
impl Table {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
