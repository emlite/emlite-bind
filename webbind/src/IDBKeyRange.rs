use super::*;

/// The IDBKeyRange class.
/// [`IDBKeyRange`](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBKeyRange {
    inner: Any,
}

impl FromVal for IDBKeyRange {
    fn from_val(v: &Any) -> Self {
        IDBKeyRange {
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

impl core::ops::Deref for IDBKeyRange {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IDBKeyRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IDBKeyRange {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IDBKeyRange {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<IDBKeyRange> for Any {
    fn from(s: IDBKeyRange) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IDBKeyRange> for Any {
    fn from(s: &IDBKeyRange) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(IDBKeyRange);

impl IDBKeyRange {
    /// Getter of the `lower` attribute.
    /// [`IDBKeyRange.lower`](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lower)
    pub fn lower(&self) -> Any {
        self.inner.get("lower").as_::<Any>()
    }
}
impl IDBKeyRange {
    /// Getter of the `upper` attribute.
    /// [`IDBKeyRange.upper`](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upper)
    pub fn upper(&self) -> Any {
        self.inner.get("upper").as_::<Any>()
    }
}
impl IDBKeyRange {
    /// Getter of the `lowerOpen` attribute.
    /// [`IDBKeyRange.lowerOpen`](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lowerOpen)
    pub fn lower_open(&self) -> bool {
        self.inner.get("lowerOpen").as_::<bool>()
    }
}
impl IDBKeyRange {
    /// Getter of the `upperOpen` attribute.
    /// [`IDBKeyRange.upperOpen`](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upperOpen)
    pub fn upper_open(&self) -> bool {
        self.inner.get("upperOpen").as_::<bool>()
    }
}
impl IDBKeyRange {
    /// The only method.
    /// [`IDBKeyRange.only`](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/only)
    pub fn only(value: &Any) -> IDBKeyRange {
        Any::global("IDBKeyRange")
            .call("only", &[value.into()])
            .as_::<IDBKeyRange>()
    }
}
impl IDBKeyRange {
    /// The lowerBound method.
    /// [`IDBKeyRange.lowerBound`](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lowerBound)
    pub fn lower_bound0(lower: &Any) -> IDBKeyRange {
        Any::global("IDBKeyRange")
            .call("lowerBound", &[lower.into()])
            .as_::<IDBKeyRange>()
    }
    /// The lowerBound method.
    /// [`IDBKeyRange.lowerBound`](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lowerBound)
    pub fn lower_bound1(lower: &Any, open: bool) -> IDBKeyRange {
        Any::global("IDBKeyRange")
            .call("lowerBound", &[lower.into(), open.into()])
            .as_::<IDBKeyRange>()
    }
}
impl IDBKeyRange {
    /// The upperBound method.
    /// [`IDBKeyRange.upperBound`](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upperBound)
    pub fn upper_bound0(upper: &Any) -> IDBKeyRange {
        Any::global("IDBKeyRange")
            .call("upperBound", &[upper.into()])
            .as_::<IDBKeyRange>()
    }
    /// The upperBound method.
    /// [`IDBKeyRange.upperBound`](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upperBound)
    pub fn upper_bound1(upper: &Any, open: bool) -> IDBKeyRange {
        Any::global("IDBKeyRange")
            .call("upperBound", &[upper.into(), open.into()])
            .as_::<IDBKeyRange>()
    }
}
impl IDBKeyRange {
    /// The bound method.
    /// [`IDBKeyRange.bound`](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/bound)
    pub fn bound0(lower: &Any, upper: &Any) -> IDBKeyRange {
        Any::global("IDBKeyRange")
            .call("bound", &[lower.into(), upper.into()])
            .as_::<IDBKeyRange>()
    }
    /// The bound method.
    /// [`IDBKeyRange.bound`](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/bound)
    pub fn bound1(lower: &Any, upper: &Any, lower_open: bool) -> IDBKeyRange {
        Any::global("IDBKeyRange")
            .call("bound", &[lower.into(), upper.into(), lower_open.into()])
            .as_::<IDBKeyRange>()
    }
    /// The bound method.
    /// [`IDBKeyRange.bound`](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/bound)
    pub fn bound2(lower: &Any, upper: &Any, lower_open: bool, upper_open: bool) -> IDBKeyRange {
        Any::global("IDBKeyRange")
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
    /// The includes method.
    /// [`IDBKeyRange.includes`](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/includes)
    pub fn includes(&self, key: &Any) -> bool {
        self.inner.call("includes", &[key.into()]).as_::<bool>()
    }
}
