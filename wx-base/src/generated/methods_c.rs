use super::*;

// wxClassInfo
pub trait ClassInfoMethods: WxRustMethods {
    /// Creates an object of the appropriate kind.
    fn create_object(&self) -> Option<ObjectIsOwned<false>> {
        unsafe { Object::option_from(ffi::wxClassInfo_CreateObject(self.as_ptr())) }
    }
    /// Returns the name of the first base class (NULL if none).
    fn get_base_class_name1(&self) -> *const c_void {
        unsafe { ffi::wxClassInfo_GetBaseClassName1(self.as_ptr()) }
    }
    /// Returns the name of the second base class (NULL if none).
    fn get_base_class_name2(&self) -> *const c_void {
        unsafe { ffi::wxClassInfo_GetBaseClassName2(self.as_ptr()) }
    }
    /// Returns the string form of the class name.
    fn get_class_name(&self) -> *const c_void {
        unsafe { ffi::wxClassInfo_GetClassName(self.as_ptr()) }
    }
    /// Returns the size of the class.
    fn get_size(&self) -> c_int {
        unsafe { ffi::wxClassInfo_GetSize(self.as_ptr()) }
    }
    /// Returns true if this class info can create objects of the associated class.
    fn is_dynamic(&self) -> bool {
        unsafe { ffi::wxClassInfo_IsDynamic(self.as_ptr()) }
    }
    /// Returns true if this class is a kind of (inherits from) the given class.
    fn is_kind_of<C: ClassInfoMethods>(&self, info: Option<&C>) -> bool {
        unsafe {
            let info = match info {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxClassInfo_IsKindOf(self.as_ptr(), info)
        }
    }
    /// Finds the wxClassInfo object for a class with the given name.
    fn find_class(class_name: &str) -> Option<ClassInfoIsOwned<false>> {
        unsafe {
            let class_name = WxString::from(class_name);
            let class_name = class_name.as_ptr();
            ClassInfo::option_from(ffi::wxClassInfo_FindClass(class_name))
        }
    }
}

// wxClientData
pub trait ClientDataMethods: WxRustMethods {
    // DTOR: fn ~wxClientData()
}
