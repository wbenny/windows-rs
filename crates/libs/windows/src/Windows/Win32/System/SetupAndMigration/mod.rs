#[doc = "*Required features: `\"Win32_System_SetupAndMigration\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OOBEComplete(isoobecomplete: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "kernel32.dll""system" fn OOBEComplete ( isoobecomplete : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    OOBEComplete(::core::mem::transmute(isoobecomplete))
}
#[doc = "*Required features: `\"Win32_System_SetupAndMigration\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterWaitUntilOOBECompleted(oobecompletedcallback: OOBE_COMPLETED_CALLBACK, callbackcontext: ::core::option::Option<*const ::core::ffi::c_void>, waithandle: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "kernel32.dll""system" fn RegisterWaitUntilOOBECompleted ( oobecompletedcallback : * mut::core::ffi::c_void , callbackcontext : *const ::core::ffi::c_void , waithandle : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    RegisterWaitUntilOOBECompleted(::core::mem::transmute(oobecompletedcallback), ::core::mem::transmute(callbackcontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(waithandle))
}
#[doc = "*Required features: `\"Win32_System_SetupAndMigration\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterWaitUntilOOBECompleted(waithandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "kernel32.dll""system" fn UnregisterWaitUntilOOBECompleted ( waithandle : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    UnregisterWaitUntilOOBECompleted(::core::mem::transmute(waithandle))
}
#[doc = "*Required features: `\"Win32_System_SetupAndMigration\"`*"]
pub type OOBE_COMPLETED_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackcontext: *const ::core::ffi::c_void) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
