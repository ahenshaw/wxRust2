use super::*;

// wxUIActionSimulator
pub trait UIActionSimulatorMethods: WxRustMethods {
    /// Move the mouse to the specified coordinates.
    fn mouse_move_long(&self, x: c_long, y: c_long) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseMove(self.as_ptr(), x, y) }
    }
    /// Move the mouse to the specified coordinates.
    fn mouse_move_point<P: PointMethods>(&self, point: &P) -> bool {
        unsafe {
            let point = point.as_ptr();
            ffi::wxUIActionSimulator_MouseMove1(self.as_ptr(), point)
        }
    }
    /// Press a mouse button.
    fn mouse_down(&self, button: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseDown(self.as_ptr(), button) }
    }
    /// Release a mouse button.
    fn mouse_up(&self, button: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseUp(self.as_ptr(), button) }
    }
    /// Click a mouse button.
    fn mouse_click(&self, button: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseClick(self.as_ptr(), button) }
    }
    /// Double-click a mouse button.
    fn mouse_dbl_click(&self, button: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseDblClick(self.as_ptr(), button) }
    }
    /// Perform a drag and drop operation.
    fn mouse_drag_drop(
        &self,
        x1: c_long,
        y1: c_long,
        x2: c_long,
        y2: c_long,
        button: c_int,
    ) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseDragDrop(self.as_ptr(), x1, y1, x2, y2, button) }
    }
    /// Press a key.
    fn key_down(&self, keycode: c_int, modifiers: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_KeyDown(self.as_ptr(), keycode, modifiers) }
    }
    /// Release a key.
    fn key_up(&self, keycode: c_int, modifiers: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_KeyUp(self.as_ptr(), keycode, modifiers) }
    }
    /// Press and release a key.
    fn char(&self, keycode: c_int, modifiers: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_Char(self.as_ptr(), keycode, modifiers) }
    }
    /// Simulate selection of an item with the given text.
    fn select(&self, text: &str) -> bool {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxUIActionSimulator_Select(self.as_ptr(), text)
        }
    }
    /// Emulate typing in the keys representing the given string.
    fn text(&self, text: *const c_void) -> bool {
        unsafe { ffi::wxUIActionSimulator_Text(self.as_ptr(), text) }
    }
}

// wxURLDataObject
pub trait URLDataObjectMethods: DataObjectMethods {
    /// Returns the URL stored by this object, as a string.
    fn get_url(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxURLDataObject_GetURL(self.as_ptr())).into() }
    }
    /// Sets the URL stored by this object.
    fn set_url(&self, url: &str) {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            ffi::wxURLDataObject_SetURL(self.as_ptr(), url)
        }
    }
}

// wxUpdateUIEvent
pub trait UpdateUIEventMethods: CommandEventMethods {
    /// Check or uncheck the UI element.
    fn check(&self, check: bool) {
        unsafe { ffi::wxUpdateUIEvent_Check(self.as_ptr(), check) }
    }
    /// Enable or disable the UI element.
    fn enable(&self, enable: bool) {
        unsafe { ffi::wxUpdateUIEvent_Enable(self.as_ptr(), enable) }
    }
    /// Returns true if the UI element should be checked.
    fn get_checked(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetChecked(self.as_ptr()) }
    }
    /// Returns true if the UI element should be enabled.
    fn get_enabled(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetEnabled(self.as_ptr()) }
    }
    /// Returns true if the UI element can be checked.
    fn is_checkable(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_IsCheckable(self.as_ptr()) }
    }
    /// Returns true if the application has called Check().
    fn get_set_checked(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetSetChecked(self.as_ptr()) }
    }
    /// Returns true if the application has called Enable().
    fn get_set_enabled(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetSetEnabled(self.as_ptr()) }
    }
    /// Returns true if the application has called Show().
    fn get_set_shown(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetSetShown(self.as_ptr()) }
    }
    /// Returns true if the application has called SetText().
    fn get_set_text(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetSetText(self.as_ptr()) }
    }
    /// Returns true if the UI element should be shown.
    fn get_shown(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetShown(self.as_ptr()) }
    }
    /// Returns the text that should be set for the UI element.
    fn get_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxUpdateUIEvent_GetText(self.as_ptr())).into() }
    }
    /// Sets the text for this UI element.
    fn set_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxUpdateUIEvent_SetText(self.as_ptr(), text)
        }
    }
    /// Show or hide the UI element.
    fn show(&self, show: bool) {
        unsafe { ffi::wxUpdateUIEvent_Show(self.as_ptr(), show) }
    }
    /// Returns true if it is appropriate to update (send UI update events to) this window.
    fn can_update<W: WindowMethods>(window: Option<&W>) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxUpdateUIEvent_CanUpdate(window)
        }
    }
    // NOT_SUPPORTED: fn GetMode()
    /// Returns the current interval between updates in milliseconds.
    fn get_update_interval() -> c_long {
        unsafe { ffi::wxUpdateUIEvent_GetUpdateInterval() }
    }
    /// Used internally to reset the last-updated time to the current time.
    fn reset_update_time() {
        unsafe { ffi::wxUpdateUIEvent_ResetUpdateTime() }
    }
    // NOT_SUPPORTED: fn SetMode()
    /// Sets the interval between updates in milliseconds.
    fn set_update_interval(update_interval: c_long) {
        unsafe { ffi::wxUpdateUIEvent_SetUpdateInterval(update_interval) }
    }
}
