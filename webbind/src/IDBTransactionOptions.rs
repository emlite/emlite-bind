use super::*;

/// The IDBTransactionOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBTransactionOptions {
    inner: Any,
}

impl FromVal for IDBTransactionOptions {
    fn from_val(v: &Any) -> Self {
        IDBTransactionOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IDBTransactionOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IDBTransactionOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IDBTransactionOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IDBTransactionOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<IDBTransactionOptions> for Any {
    fn from(s: IDBTransactionOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IDBTransactionOptions> for Any {
    fn from(s: &IDBTransactionOptions) -> Any {
        s.inner.clone()
    }
}

impl IDBTransactionOptions {
    /// Getter of the `durability` attribute.
    pub fn durability(&self) -> IDBTransactionDurability {
        self.inner
            .get("durability")
            .as_::<IDBTransactionDurability>()
    }

    /// Setter of the `durability` attribute.
    pub fn set_durability(&mut self, value: &IDBTransactionDurability) {
        self.inner.set("durability", value);
    }
}
