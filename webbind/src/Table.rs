use super::*;

/// The Table class.
/// [`Table`](https://developer.mozilla.org/en-US/docs/Web/API/Table)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Table {
    inner: Any,
}

impl FromVal for Table {
    fn from_val(v: &Any) -> Self {
        Table {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Table {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Table {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Table {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Table {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Table> for Any {
    fn from(s: Table) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Table> for Any {
    fn from(s: &Table) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Table);

impl Table {
    /// Getter of the `length` attribute.
    /// [`Table.length`](https://developer.mozilla.org/en-US/docs/Web/API/Table/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}

impl Table {
    /// The `new Table(..)` constructor, creating a new Table instance
    pub fn new(descriptor: &TableDescriptor) -> Table {
        Self {
            inner: Any::global("Table").new(&[descriptor.into()]).as_::<Any>(),
        }
    }
}

impl Table {
    /// The `new Table(..)` constructor, creating a new Table instance
    pub fn new_with_value(descriptor: &TableDescriptor, value: &Any) -> Table {
        Self {
            inner: Any::global("Table")
                .new(&[descriptor.into(), value.into()])
                .as_::<Any>(),
        }
    }
}

impl Table {
    /// The grow method.
    /// [`Table.grow`](https://developer.mozilla.org/en-US/docs/Web/API/Table/grow)
    pub fn grow(&self, delta: u32) -> u32 {
        self.inner.call("grow", &[delta.into()]).as_::<u32>()
    }
}
impl Table {
    /// The grow method.
    /// [`Table.grow`](https://developer.mozilla.org/en-US/docs/Web/API/Table/grow)
    pub fn grow_with_value(&self, delta: u32, value: &Any) -> u32 {
        self.inner
            .call("grow", &[delta.into(), value.into()])
            .as_::<u32>()
    }
}
impl Table {
    /// The get method.
    /// [`Table.get`](https://developer.mozilla.org/en-US/docs/Web/API/Table/get)
    pub fn get(&self, index: u32) -> Any {
        self.inner.call("get", &[index.into()]).as_::<Any>()
    }
}
impl Table {
    /// The set method.
    /// [`Table.set`](https://developer.mozilla.org/en-US/docs/Web/API/Table/set)
    pub fn set(&self, index: u32) -> Undefined {
        self.inner.call("set", &[index.into()]).as_::<Undefined>()
    }
}
impl Table {
    /// The set method.
    /// [`Table.set`](https://developer.mozilla.org/en-US/docs/Web/API/Table/set)
    pub fn set_with_value(&self, index: u32, value: &Any) -> Undefined {
        self.inner
            .call("set", &[index.into(), value.into()])
            .as_::<Undefined>()
    }
}
