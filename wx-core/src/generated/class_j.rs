use super::*;

// wxJPEGHandler
wxwidgets! {
    /// This is the image handler for the JPEG format.
    /// - [`JPEGHandler`] represents a C++ `wxJPEGHandler` class instance which your code has ownership, [`JPEGHandlerIsOwned`]`<false>` represents one which don't own.
    /// - Use [`JPEGHandler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxJPEGHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_j_p_e_g_handler.html) for more details.
    #[doc(alias = "wxJPEGHandler")]
    #[doc(alias = "JPEGHandler")]
    class JPEGHandler
        = JPEGHandlerIsOwned<true>(wxJPEGHandler) impl
        JPEGHandlerMethods,
        // ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> JPEGHandlerIsOwned<OWNED> {
    /// Default constructor for wxJPEGHandler.
    ///
    /// See [C++ `wxJPEGHandler::wxJPEGHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_j_p_e_g_handler.html#a2935d0b968967a673b0b6cef75dcbc04).
    pub fn new() -> JPEGHandlerIsOwned<OWNED> {
        unsafe { JPEGHandlerIsOwned(ffi::wxJPEGHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for JPEGHandlerIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<JPEGHandlerIsOwned<OWNED>> for ImageHandlerIsOwned<OWNED> {
    fn from(o: JPEGHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<JPEGHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: JPEGHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for JPEGHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxJPEGHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for JPEGHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
impl<const OWNED: bool> ImageHandlerMethods for JPEGHandlerIsOwned<OWNED> {
    // NOT_SUPPORTED: fn GetLibraryVersionInfo()
}

// wxJoystickEvent
wxwidgets! {
    /// This event class contains information about joystick events, particularly events received by windows.
    /// - [`JoystickEvent`] represents a C++ `wxJoystickEvent` class instance which your code has ownership, [`JoystickEventIsOwned`]`<false>` represents one which don't own.
    /// - Use [`JoystickEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxJoystickEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_joystick_event.html) for more details.
    #[doc(alias = "wxJoystickEvent")]
    #[doc(alias = "JoystickEvent")]
    class JoystickEvent
        = JoystickEventIsOwned<true>(wxJoystickEvent) impl
        JoystickEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> JoystickEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxJoystickEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for JoystickEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<JoystickEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: JoystickEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<JoystickEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: JoystickEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for JoystickEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxJoystickEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for JoystickEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
