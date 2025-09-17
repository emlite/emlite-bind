use super::*;

/// Callback interface EventListener
/// Generated wrapper for WebIDL `callback interface EventListener`
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EventListener {
    inner: Any,
}

impl FromVal for EventListener {
    fn from_val(v: &Any) -> Self {
        EventListener { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        EventListener {
            inner: Any::take_ownership(v),
        }
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for EventListener {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EventListener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<EventListener> for Any {
    fn from(s: EventListener) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EventListener> for Any {
    fn from(s: &EventListener) -> Any {
        s.inner.clone().into()
    }
}

impl EventListener {
    /// Wrap a JavaScript function as a EventListener
    pub fn from_function(f: &Function) -> EventListener {
        EventListener {
            inner: Any::from(f.clone()),
        }
    }
}

impl EventListener {
    /// Build a EventListener from a typed Rust closure matching `handleEvent`
    pub fn from_closure<F>(mut cb: F) -> EventListener
    where
        F: FnMut(Event) -> Undefined + 'static,
    {
        let c = Closure::bind1(move |a1: Event| cb(a1));
        EventListener::from_function(&Function::from(&c))
    }
}

impl EventListener {
    pub fn handle_event(&self, event: &Event) -> Undefined {
        if self.inner.is_function() {
            // Call as a bare function
            self.inner.invoke(&[event.into()]).as_::<Undefined>()
        } else {
            // Call the named method on the object
            self.inner
                .call("handleEvent", &[event.into()])
                .as_::<Undefined>()
        }
    }
}
