use super::*;

// wxKeyEvent
wxwidgets! {
    /// This event class contains information about key press and release events.
    /// - [`KeyEvent`] represents a C++ `wxKeyEvent` class instance which your code has ownership, [`KeyEventIsOwned`]`<false>` represents one which don't own.
    /// - Use [`KeyEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxKeyEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_key_event.html) for more details.
    #[doc(alias = "wxKeyEvent")]
    #[doc(alias = "KeyEvent")]
    class KeyEvent
        = KeyEventIsOwned<true>(wxKeyEvent) impl
        KeyEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> KeyEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxKeyEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for KeyEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<KeyEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: KeyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<KeyEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: KeyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for KeyEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxKeyEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for KeyEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
