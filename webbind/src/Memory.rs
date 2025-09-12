use super::*;

/// The Memory class.
/// [`Memory`](https://developer.mozilla.org/en-US/docs/Web/API/Memory)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Memory {
    inner: Any,
}

impl FromVal for Memory {
    fn from_val(v: &Any) -> Self {
        Memory {
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

impl core::ops::Deref for Memory {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Memory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Memory {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Memory {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Memory> for Any {
    fn from(s: Memory) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Memory> for Any {
    fn from(s: &Memory) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Memory);

impl Memory {
    /// Getter of the `buffer` attribute.
    /// [`Memory.buffer`](https://developer.mozilla.org/en-US/docs/Web/API/Memory/buffer)
    pub fn buffer(&self) -> ArrayBuffer {
        self.inner.get("buffer").as_::<ArrayBuffer>()
    }
}

impl Memory {
    /// The `new Memory(..)` constructor, creating a new Memory instance
    pub fn new(descriptor: &MemoryDescriptor) -> Memory {
        Self {
            inner: Any::global("Memory").new(&[descriptor.into()]).as_::<Any>(),
        }
    }
}

impl Memory {
    /// The grow method.
    /// [`Memory.grow`](https://developer.mozilla.org/en-US/docs/Web/API/Memory/grow)
    pub fn grow(&self, delta: u32) -> u32 {
        self.inner.call("grow", &[delta.into()]).as_::<u32>()
    }
}
impl Memory {
    /// The toFixedLengthBuffer method.
    /// [`Memory.toFixedLengthBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/Memory/toFixedLengthBuffer)
    pub fn to_fixed_length_buffer(&self) -> ArrayBuffer {
        self.inner
            .call("toFixedLengthBuffer", &[])
            .as_::<ArrayBuffer>()
    }
}
impl Memory {
    /// The toResizableBuffer method.
    /// [`Memory.toResizableBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/Memory/toResizableBuffer)
    pub fn to_resizable_buffer(&self) -> ArrayBuffer {
        self.inner
            .call("toResizableBuffer", &[])
            .as_::<ArrayBuffer>()
    }
}
