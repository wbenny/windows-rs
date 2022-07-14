#[cfg(feature = "Win32_System_WinRT_AllJoyn")]
pub mod AllJoyn;
#[cfg(feature = "Win32_System_WinRT_Composition")]
pub mod Composition;
#[cfg(feature = "Win32_System_WinRT_CoreInputView")]
pub mod CoreInputView;
#[cfg(feature = "Win32_System_WinRT_Direct3D11")]
pub mod Direct3D11;
#[cfg(feature = "Win32_System_WinRT_Display")]
pub mod Display;
#[cfg(feature = "Win32_System_WinRT_Graphics")]
pub mod Graphics;
#[cfg(feature = "Win32_System_WinRT_Holographic")]
pub mod Holographic;
#[cfg(feature = "Win32_System_WinRT_Isolation")]
pub mod Isolation;
#[cfg(feature = "Win32_System_WinRT_ML")]
pub mod ML;
#[cfg(feature = "Win32_System_WinRT_Media")]
pub mod Media;
#[cfg(feature = "Win32_System_WinRT_Pdf")]
pub mod Pdf;
#[cfg(feature = "Win32_System_WinRT_Printing")]
pub mod Printing;
#[cfg(feature = "Win32_System_WinRT_Shell")]
pub mod Shell;
#[cfg(feature = "Win32_System_WinRT_Storage")]
pub mod Storage;
#[cfg(feature = "Win32_System_WinRT_Xaml")]
pub mod Xaml;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ACTIVATIONTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const ACTIVATIONTYPE_UNCATEGORIZED: ACTIVATIONTYPE = ACTIVATIONTYPE(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const ACTIVATIONTYPE_FROM_MONIKER: ACTIVATIONTYPE = ACTIVATIONTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const ACTIVATIONTYPE_FROM_DATA: ACTIVATIONTYPE = ACTIVATIONTYPE(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const ACTIVATIONTYPE_FROM_STORAGE: ACTIVATIONTYPE = ACTIVATIONTYPE(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const ACTIVATIONTYPE_FROM_STREAM: ACTIVATIONTYPE = ACTIVATIONTYPE(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const ACTIVATIONTYPE_FROM_FILE: ACTIVATIONTYPE = ACTIVATIONTYPE(16i32);
impl ::core::marker::Copy for ACTIVATIONTYPE {}
impl ::core::clone::Clone for ACTIVATIONTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACTIVATIONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ACTIVATIONTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ACTIVATIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTIVATIONTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APARTMENT_SHUTDOWN_REGISTRATION_COOKIE(pub isize);
impl APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {}
impl ::core::fmt::Debug for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APARTMENT_SHUTDOWN_REGISTRATION_COOKIE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<APARTMENT_SHUTDOWN_REGISTRATION_COOKIE>> for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    fn from(optional: ::core::option::Option<APARTMENT_SHUTDOWN_REGISTRATION_COOKIE>) -> APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AgileReferenceOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const AGILEREFERENCE_DEFAULT: AgileReferenceOptions = AgileReferenceOptions(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const AGILEREFERENCE_DELAYEDMARSHAL: AgileReferenceOptions = AgileReferenceOptions(1i32);
impl ::core::marker::Copy for AgileReferenceOptions {}
impl ::core::clone::Clone for AgileReferenceOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AgileReferenceOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AgileReferenceOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for AgileReferenceOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AgileReferenceOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BSOS_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const BSOS_DEFAULT: BSOS_OPTIONS = BSOS_OPTIONS(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const BSOS_PREFERDESTINATIONSTREAM: BSOS_OPTIONS = BSOS_OPTIONS(1i32);
impl ::core::marker::Copy for BSOS_OPTIONS {}
impl ::core::clone::Clone for BSOS_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BSOS_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BSOS_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for BSOS_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BSOS_OPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CASTING_CONNECTION_ERROR_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_SUCCEEDED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_DID_NOT_RESPOND: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_ERROR: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_LOCKED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_PROTECTED_PLAYBACK_FAILED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_INVALID_CASTING_SOURCE: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(5i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_UNKNOWN: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(6i32);
impl ::core::marker::Copy for CASTING_CONNECTION_ERROR_STATUS {}
impl ::core::clone::Clone for CASTING_CONNECTION_ERROR_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CASTING_CONNECTION_ERROR_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CASTING_CONNECTION_ERROR_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CASTING_CONNECTION_ERROR_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CASTING_CONNECTION_ERROR_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CASTING_CONNECTION_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_STATE_DISCONNECTED: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_STATE_CONNECTED: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_STATE_RENDERING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_STATE_DISCONNECTING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_STATE_CONNECTING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(4i32);
impl ::core::marker::Copy for CASTING_CONNECTION_STATE {}
impl ::core::clone::Clone for CASTING_CONNECTION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CASTING_CONNECTION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CASTING_CONNECTION_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CASTING_CONNECTION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CASTING_CONNECTION_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CastingSourceInfo_Property_CastingTypes: &str = "CastingTypes";
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CastingSourceInfo_Property_PreferredSourceUriScheme: &str = "PreferredSourceUriScheme";
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CastingSourceInfo_Property_ProtectedMedia: &str = "ProtectedMedia";
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn CoDecodeProxy(dwclientpid: u32, ui64proxyaddress: u64) -> ::windows::core::Result<ServerInformation> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CoDecodeProxy(dwclientpid: u32, ui64proxyaddress: u64, pserverinformation: *mut ServerInformation) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CoDecodeProxy(dwclientpid, ui64proxyaddress, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ServerInformation>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn CreateControlInput<T>() -> ::windows::core::Result<T>
where
    T: ::windows::core::Interface,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateControlInput(riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::option::Option::None;
    CreateControlInput(&<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn CreateControlInputEx<'a, P0, T>(pcorewindow: P0) -> ::windows::core::Result<T>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    T: ::windows::core::Interface,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateControlInputEx(pcorewindow: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::option::Option::None;
    CreateControlInputEx(pcorewindow.into().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"System\"`*"]
#[cfg(feature = "System")]
#[inline]
pub unsafe fn CreateDispatcherQueueController(options: DispatcherQueueOptions) -> ::windows::core::Result<super::super::super::System::DispatcherQueueController> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDispatcherQueueController(options: DispatcherQueueOptions, dispatcherqueuecontroller: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateDispatcherQueueController(::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::DispatcherQueueController>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn CreateRandomAccessStreamOnFile<'a, P0, T>(filepath: P0, accessmode: u32) -> ::windows::core::Result<T>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    T: ::windows::core::Interface,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateRandomAccessStreamOnFile(filepath: ::windows::core::PCWSTR, accessmode: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::option::Option::None;
    CreateRandomAccessStreamOnFile(filepath.into(), accessmode, &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateRandomAccessStreamOverStream<'a, P0, T>(stream: P0, options: BSOS_OPTIONS) -> ::windows::core::Result<T>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::IStream>>,
    T: ::windows::core::Interface,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateRandomAccessStreamOverStream(stream: *mut ::core::ffi::c_void, options: BSOS_OPTIONS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::option::Option::None;
    CreateRandomAccessStreamOverStream(stream.into().abi(), options, &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn CreateStreamOverRandomAccessStream<'a, P0, T>(randomaccessstream: P0) -> ::windows::core::Result<T>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    T: ::windows::core::Interface,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateStreamOverRandomAccessStream(randomaccessstream: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::option::Option::None;
    CreateStreamOverRandomAccessStream(randomaccessstream.into().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPATCHERQUEUE_THREAD_APARTMENTTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const DQTAT_COM_NONE: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const DQTAT_COM_ASTA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const DQTAT_COM_STA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(2i32);
impl ::core::marker::Copy for DISPATCHERQUEUE_THREAD_APARTMENTTYPE {}
impl ::core::clone::Clone for DISPATCHERQUEUE_THREAD_APARTMENTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPATCHERQUEUE_THREAD_APARTMENTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DISPATCHERQUEUE_THREAD_APARTMENTTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPATCHERQUEUE_THREAD_APARTMENTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPATCHERQUEUE_THREAD_APARTMENTTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPATCHERQUEUE_THREAD_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const DQTYPE_THREAD_DEDICATED: DISPATCHERQUEUE_THREAD_TYPE = DISPATCHERQUEUE_THREAD_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const DQTYPE_THREAD_CURRENT: DISPATCHERQUEUE_THREAD_TYPE = DISPATCHERQUEUE_THREAD_TYPE(2i32);
impl ::core::marker::Copy for DISPATCHERQUEUE_THREAD_TYPE {}
impl ::core::clone::Clone for DISPATCHERQUEUE_THREAD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPATCHERQUEUE_THREAD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DISPATCHERQUEUE_THREAD_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPATCHERQUEUE_THREAD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPATCHERQUEUE_THREAD_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub struct DispatcherQueueOptions {
    pub dwSize: u32,
    pub threadType: DISPATCHERQUEUE_THREAD_TYPE,
    pub apartmentType: DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
}
impl ::core::marker::Copy for DispatcherQueueOptions {}
impl ::core::clone::Clone for DispatcherQueueOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DispatcherQueueOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DispatcherQueueOptions").field("dwSize", &self.dwSize).field("threadType", &self.threadType).field("apartmentType", &self.apartmentType).finish()
    }
}
unsafe impl ::windows::core::Abi for DispatcherQueueOptions {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DispatcherQueueOptions {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DispatcherQueueOptions>()) == 0 }
    }
}
impl ::core::cmp::Eq for DispatcherQueueOptions {}
impl ::core::default::Default for DispatcherQueueOptions {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub struct EventRegistrationToken {
    pub value: i64,
}
impl ::core::marker::Copy for EventRegistrationToken {}
impl ::core::clone::Clone for EventRegistrationToken {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EventRegistrationToken {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EventRegistrationToken").field("value", &self.value).finish()
    }
}
unsafe impl ::windows::core::Abi for EventRegistrationToken {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EventRegistrationToken {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EventRegistrationToken>()) == 0 }
    }
}
impl ::core::cmp::Eq for EventRegistrationToken {}
impl ::core::default::Default for EventRegistrationToken {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn GetRestrictedErrorInfo() -> ::windows::core::Result<IRestrictedErrorInfo> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetRestrictedErrorInfo(pprestrictederrorinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetRestrictedErrorInfo(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRestrictedErrorInfo>(result__)
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HSTRING_BUFFER(pub isize);
impl HSTRING_BUFFER {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HSTRING_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HSTRING_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HSTRING_BUFFER {}
impl ::core::fmt::Debug for HSTRING_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HSTRING_BUFFER").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HSTRING_BUFFER>> for HSTRING_BUFFER {
    fn from(optional: ::core::option::Option<HSTRING_BUFFER>) -> HSTRING_BUFFER {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HSTRING_BUFFER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub struct HSTRING_HEADER {
    pub flags: u32,
    pub length: u32,
    pub padding1: u32,
    pub padding2: u32,
    pub data: isize,
}
impl ::core::marker::Copy for HSTRING_HEADER {}
impl ::core::clone::Clone for HSTRING_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HSTRING_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSTRING_HEADER").field("flags", &self.flags).field("length", &self.length).field("padding1", &self.padding1).field("padding2", &self.padding2).field("data", &self.data).finish()
    }
}
unsafe impl ::windows::core::Abi for HSTRING_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HSTRING_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HSTRING_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for HSTRING_HEADER {}
impl ::core::default::Default for HSTRING_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn HSTRING_UserFree(param0: *const u32, param1: *const ::windows::core::HSTRING) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HSTRING_UserFree(param0: *const u32, param1: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>);
    }
    HSTRING_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn HSTRING_UserFree64(param0: *const u32, param1: *const ::windows::core::HSTRING) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HSTRING_UserFree64(param0: *const u32, param1: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>);
    }
    HSTRING_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn HSTRING_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::windows::core::HSTRING) -> *mut u8 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HSTRING_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> *mut u8;
    }
    HSTRING_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn HSTRING_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::windows::core::HSTRING) -> *mut u8 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HSTRING_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> *mut u8;
    }
    HSTRING_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn HSTRING_UserSize(param0: *const u32, param1: u32, param2: *const ::windows::core::HSTRING) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HSTRING_UserSize(param0: *const u32, param1: u32, param2: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> u32;
    }
    HSTRING_UserSize(::core::mem::transmute(param0), param1, ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn HSTRING_UserSize64(param0: *const u32, param1: u32, param2: *const ::windows::core::HSTRING) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HSTRING_UserSize64(param0: *const u32, param1: u32, param2: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> u32;
    }
    HSTRING_UserSize64(::core::mem::transmute(param0), param1, ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn HSTRING_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::windows::core::HSTRING) -> *mut u8 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HSTRING_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> *mut u8;
    }
    HSTRING_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn HSTRING_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::windows::core::HSTRING) -> *mut u8 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HSTRING_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> *mut u8;
    }
    HSTRING_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IAccountsSettingsPaneInterop(::windows::core::IUnknown);
impl IAccountsSettingsPaneInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<'a, P0, T>(&self, appwindow: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetForWindow)(::windows::core::Interface::as_raw(self), appwindow.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowManageAccountsForWindowAsync<'a, P0, T>(&self, appwindow: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).ShowManageAccountsForWindowAsync)(::windows::core::Interface::as_raw(self), appwindow.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowAddAccountForWindowAsync<'a, P0, T>(&self, appwindow: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).ShowAddAccountForWindowAsync)(::windows::core::Interface::as_raw(self), appwindow.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IAccountsSettingsPaneInterop> for ::windows::core::IUnknown {
    fn from(value: IAccountsSettingsPaneInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAccountsSettingsPaneInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAccountsSettingsPaneInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAccountsSettingsPaneInterop> for ::windows::core::IUnknown {
    fn from(value: &IAccountsSettingsPaneInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IAccountsSettingsPaneInterop> for ::windows::core::IInspectable {
    fn from(value: IAccountsSettingsPaneInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAccountsSettingsPaneInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IAccountsSettingsPaneInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAccountsSettingsPaneInterop> for ::windows::core::IInspectable {
    fn from(value: &IAccountsSettingsPaneInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAccountsSettingsPaneInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAccountsSettingsPaneInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccountsSettingsPaneInterop {}
impl ::core::fmt::Debug for IAccountsSettingsPaneInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccountsSettingsPaneInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAccountsSettingsPaneInterop {
    type Vtable = IAccountsSettingsPaneInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3ee12ad_3865_4362_9746_b75a682df0e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, accountssettingspane: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowManageAccountsForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowManageAccountsForWindowAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowAddAccountForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowAddAccountForWindowAsync: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IActivationFactory(::windows::core::IUnknown);
impl IActivationFactory {
    pub unsafe fn ActivateInstance(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ActivateInstance)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IInspectable>(result__)
    }
}
impl ::core::convert::From<IActivationFactory> for ::windows::core::IUnknown {
    fn from(value: IActivationFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IActivationFactory> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IActivationFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivationFactory> for ::windows::core::IUnknown {
    fn from(value: &IActivationFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IActivationFactory> for ::windows::core::IInspectable {
    fn from(value: IActivationFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IActivationFactory> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IActivationFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivationFactory> for ::windows::core::IInspectable {
    fn from(value: &IActivationFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IActivationFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActivationFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivationFactory {}
impl ::core::fmt::Debug for IActivationFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivationFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActivationFactory {
    type Vtable = IActivationFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000035_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ActivateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IAgileReference(::windows::core::IUnknown);
impl IAgileReference {
    pub unsafe fn Resolve<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).Resolve)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IAgileReference> for ::windows::core::IUnknown {
    fn from(value: IAgileReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAgileReference> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAgileReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAgileReference> for ::windows::core::IUnknown {
    fn from(value: &IAgileReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAgileReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAgileReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAgileReference {}
impl ::core::fmt::Debug for IAgileReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAgileReference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAgileReference {
    type Vtable = IAgileReference_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc03f6a43_65a4_9818_987e_e0b810d2a6f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileReference_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Resolve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobjectreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IApartmentShutdown(::windows::core::IUnknown);
impl IApartmentShutdown {
    pub unsafe fn OnUninitialize(&self, ui64apartmentidentifier: u64) {
        (::windows::core::Interface::vtable(self).OnUninitialize)(::windows::core::Interface::as_raw(self), ui64apartmentidentifier)
    }
}
impl ::core::convert::From<IApartmentShutdown> for ::windows::core::IUnknown {
    fn from(value: IApartmentShutdown) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IApartmentShutdown> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IApartmentShutdown) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IApartmentShutdown> for ::windows::core::IUnknown {
    fn from(value: &IApartmentShutdown) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IApartmentShutdown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IApartmentShutdown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApartmentShutdown {}
impl ::core::fmt::Debug for IApartmentShutdown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApartmentShutdown").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IApartmentShutdown {
    type Vtable = IApartmentShutdown_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2f05a09_27a2_42b5_bc0e_ac163ef49d9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApartmentShutdown_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnUninitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ui64apartmentidentifier: u64),
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IAppServiceConnectionExtendedExecution(::windows::core::IUnknown);
impl IAppServiceConnectionExtendedExecution {
    pub unsafe fn OpenForExtendedExecutionAsync<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).OpenForExtendedExecutionAsync)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IAppServiceConnectionExtendedExecution> for ::windows::core::IUnknown {
    fn from(value: IAppServiceConnectionExtendedExecution) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAppServiceConnectionExtendedExecution> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAppServiceConnectionExtendedExecution) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppServiceConnectionExtendedExecution> for ::windows::core::IUnknown {
    fn from(value: &IAppServiceConnectionExtendedExecution) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAppServiceConnectionExtendedExecution {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppServiceConnectionExtendedExecution {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppServiceConnectionExtendedExecution {}
impl ::core::fmt::Debug for IAppServiceConnectionExtendedExecution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppServiceConnectionExtendedExecution").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAppServiceConnectionExtendedExecution {
    type Vtable = IAppServiceConnectionExtendedExecution_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65219584_f9cb_4ae3_81f9_a28a6ca450d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceConnectionExtendedExecution_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OpenForExtendedExecutionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, operation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IBufferByteAccess(::windows::core::IUnknown);
impl IBufferByteAccess {
    pub unsafe fn Buffer(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Buffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut u8>(result__)
    }
}
impl ::core::convert::From<IBufferByteAccess> for ::windows::core::IUnknown {
    fn from(value: IBufferByteAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBufferByteAccess> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBufferByteAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBufferByteAccess> for ::windows::core::IUnknown {
    fn from(value: &IBufferByteAccess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBufferByteAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBufferByteAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBufferByteAccess {}
impl ::core::fmt::Debug for IBufferByteAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBufferByteAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBufferByteAccess {
    type Vtable = IBufferByteAccess_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905a0fef_bc53_11df_8c49_001e4fc686da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferByteAccess_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Buffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ICastingController(::windows::core::IUnknown);
impl ICastingController {
    pub unsafe fn Initialize<'a, P0, P1>(&self, castingengine: P0, castingsource: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), castingengine.into().abi(), castingsource.into().abi()).ok()
    }
    pub unsafe fn Connect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Connect)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Disconnect)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Advise<'a, P0>(&self, eventhandler: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ICastingEventHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Advise)(::windows::core::Interface::as_raw(self), eventhandler.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn UnAdvise(&self, cookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnAdvise)(::windows::core::Interface::as_raw(self), cookie).ok()
    }
}
impl ::core::convert::From<ICastingController> for ::windows::core::IUnknown {
    fn from(value: ICastingController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICastingController> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICastingController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICastingController> for ::windows::core::IUnknown {
    fn from(value: &ICastingController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICastingController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICastingController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICastingController {}
impl ::core::fmt::Debug for ICastingController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICastingController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICastingController {
    type Vtable = ICastingController_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0a56423_a664_4fbd_8b43_409a45e8d9a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingController_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, castingengine: *mut ::core::ffi::c_void, castingsource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT,
    pub UnAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ICastingEventHandler(::windows::core::IUnknown);
impl ICastingEventHandler {
    pub unsafe fn OnStateChanged(&self, newstate: CASTING_CONNECTION_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStateChanged)(::windows::core::Interface::as_raw(self), newstate).ok()
    }
    pub unsafe fn OnError<'a, P0>(&self, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnError)(::windows::core::Interface::as_raw(self), errorstatus, errormessage.into()).ok()
    }
}
impl ::core::convert::From<ICastingEventHandler> for ::windows::core::IUnknown {
    fn from(value: ICastingEventHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICastingEventHandler> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICastingEventHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICastingEventHandler> for ::windows::core::IUnknown {
    fn from(value: &ICastingEventHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICastingEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICastingEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICastingEventHandler {}
impl ::core::fmt::Debug for ICastingEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICastingEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICastingEventHandler {
    type Vtable = ICastingEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc79a6cb7_bebd_47a6_a2ad_4d45ad79c7bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstate: CASTING_CONNECTION_STATE) -> ::windows::core::HRESULT,
    pub OnError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ICastingSourceInfo(::windows::core::IUnknown);
impl ICastingSourceInfo {
    pub unsafe fn GetController(&self) -> ::windows::core::Result<ICastingController> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetController)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICastingController>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::INamedPropertyStore> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::UI::Shell::PropertiesSystem::INamedPropertyStore>(result__)
    }
}
impl ::core::convert::From<ICastingSourceInfo> for ::windows::core::IUnknown {
    fn from(value: ICastingSourceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICastingSourceInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICastingSourceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICastingSourceInfo> for ::windows::core::IUnknown {
    fn from(value: &ICastingSourceInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICastingSourceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICastingSourceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICastingSourceInfo {}
impl ::core::fmt::Debug for ICastingSourceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICastingSourceInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICastingSourceInfo {
    type Vtable = ICastingSourceInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45101ab7_7c3a_4bce_9500_12c09024b298);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingSourceInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controller: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, props: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetProperties: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ICoreInputInterop(::windows::core::IUnknown);
impl ICoreInputInterop {
    pub unsafe fn SetInputSource<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).SetInputSource)(::windows::core::Interface::as_raw(self), value.into().abi()).ok()
    }
    pub unsafe fn SetMessageHandled(&self, value: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMessageHandled)(::windows::core::Interface::as_raw(self), value).ok()
    }
}
impl ::core::convert::From<ICoreInputInterop> for ::windows::core::IUnknown {
    fn from(value: ICoreInputInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICoreInputInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICoreInputInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreInputInterop> for ::windows::core::IUnknown {
    fn from(value: &ICoreInputInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICoreInputInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreInputInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreInputInterop {}
impl ::core::fmt::Debug for ICoreInputInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreInputInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICoreInputInterop {
    type Vtable = ICoreInputInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40bfe3e3_b75a_4479_ac96_475365749bb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputInterop_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetInputSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMessageHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ICoreWindowAdapterInterop(::windows::core::IUnknown);
impl ICoreWindowAdapterInterop {
    pub unsafe fn AppActivationClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AppActivationClientAdapter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn ApplicationViewClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ApplicationViewClientAdapter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn CoreApplicationViewClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CoreApplicationViewClientAdapter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn HoloViewClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).HoloViewClientAdapter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn PositionerClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PositionerClientAdapter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn SystemNavigationClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SystemNavigationClientAdapter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn TitleBarClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TitleBarClientAdapter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn SetWindowClientAdapter<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).SetWindowClientAdapter)(::windows::core::Interface::as_raw(self), value.into().abi()).ok()
    }
}
impl ::core::convert::From<ICoreWindowAdapterInterop> for ::windows::core::IUnknown {
    fn from(value: ICoreWindowAdapterInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICoreWindowAdapterInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICoreWindowAdapterInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreWindowAdapterInterop> for ::windows::core::IUnknown {
    fn from(value: &ICoreWindowAdapterInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ICoreWindowAdapterInterop> for ::windows::core::IInspectable {
    fn from(value: ICoreWindowAdapterInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICoreWindowAdapterInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a ICoreWindowAdapterInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreWindowAdapterInterop> for ::windows::core::IInspectable {
    fn from(value: &ICoreWindowAdapterInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICoreWindowAdapterInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreWindowAdapterInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreWindowAdapterInterop {}
impl ::core::fmt::Debug for ICoreWindowAdapterInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreWindowAdapterInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICoreWindowAdapterInterop {
    type Vtable = ICoreWindowAdapterInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a5b6fd1_cd73_4b6c_9cf4_2e869eaf470a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowAdapterInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub AppActivationClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ApplicationViewClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CoreApplicationViewClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HoloViewClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PositionerClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SystemNavigationClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TitleBarClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetWindowClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ICoreWindowComponentInterop(::windows::core::IUnknown);
impl ICoreWindowComponentInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConfigureComponentInput<'a, P0, P1>(&self, hostviewinstanceid: u32, hwndhost: P0, inputsourcevisual: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).ConfigureComponentInput)(::windows::core::Interface::as_raw(self), hostviewinstanceid, hwndhost.into(), inputsourcevisual.into().abi()).ok()
    }
    pub unsafe fn GetViewInstanceId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetViewInstanceId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<ICoreWindowComponentInterop> for ::windows::core::IUnknown {
    fn from(value: ICoreWindowComponentInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICoreWindowComponentInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICoreWindowComponentInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreWindowComponentInterop> for ::windows::core::IUnknown {
    fn from(value: &ICoreWindowComponentInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICoreWindowComponentInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreWindowComponentInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreWindowComponentInterop {}
impl ::core::fmt::Debug for ICoreWindowComponentInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreWindowComponentInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICoreWindowComponentInterop {
    type Vtable = ICoreWindowComponentInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0576ab31_a310_4c40_ba31_fd37e0298dfa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowComponentInterop_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ConfigureComponentInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostviewinstanceid: u32, hwndhost: super::super::Foundation::HWND, inputsourcevisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConfigureComponentInput: usize,
    pub GetViewInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, componentviewinstanceid: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ICoreWindowInterop(::windows::core::IUnknown);
impl ICoreWindowInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).WindowHandle)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HWND>(result__)
    }
    pub unsafe fn SetMessageHandled(&self, value: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMessageHandled)(::windows::core::Interface::as_raw(self), value).ok()
    }
}
impl ::core::convert::From<ICoreWindowInterop> for ::windows::core::IUnknown {
    fn from(value: ICoreWindowInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICoreWindowInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICoreWindowInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreWindowInterop> for ::windows::core::IUnknown {
    fn from(value: &ICoreWindowInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICoreWindowInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreWindowInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreWindowInterop {}
impl ::core::fmt::Debug for ICoreWindowInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreWindowInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICoreWindowInterop {
    type Vtable = ICoreWindowInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45d64a29_a63e_4cb6_b498_5781d298cb4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowInterop_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub WindowHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WindowHandle: usize,
    pub SetMessageHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ICorrelationVectorInformation(::windows::core::IUnknown);
impl ICorrelationVectorInformation {
    pub unsafe fn LastCorrelationVectorForThread(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LastCorrelationVectorForThread)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::HSTRING>(result__)
    }
    pub unsafe fn NextCorrelationVectorForThread(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).NextCorrelationVectorForThread)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::HSTRING>(result__)
    }
    pub unsafe fn SetNextCorrelationVectorForThread<'a, P0>(&self, cv: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
    {
        (::windows::core::Interface::vtable(self).SetNextCorrelationVectorForThread)(::windows::core::Interface::as_raw(self), cv.into().abi()).ok()
    }
}
impl ::core::convert::From<ICorrelationVectorInformation> for ::windows::core::IUnknown {
    fn from(value: ICorrelationVectorInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICorrelationVectorInformation> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICorrelationVectorInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICorrelationVectorInformation> for ::windows::core::IUnknown {
    fn from(value: &ICorrelationVectorInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ICorrelationVectorInformation> for ::windows::core::IInspectable {
    fn from(value: ICorrelationVectorInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICorrelationVectorInformation> for &'a ::windows::core::IInspectable {
    fn from(value: &'a ICorrelationVectorInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICorrelationVectorInformation> for ::windows::core::IInspectable {
    fn from(value: &ICorrelationVectorInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICorrelationVectorInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICorrelationVectorInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorrelationVectorInformation {}
impl ::core::fmt::Debug for ICorrelationVectorInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorrelationVectorInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorrelationVectorInformation {
    type Vtable = ICorrelationVectorInformation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83c78b3c_d88b_4950_aa6e_22b8d22aabd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorrelationVectorInformation_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub LastCorrelationVectorForThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cv: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NextCorrelationVectorForThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cv: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetNextCorrelationVectorForThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cv: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ICorrelationVectorSource(::windows::core::IUnknown);
impl ICorrelationVectorSource {
    pub unsafe fn CorrelationVector(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CorrelationVector)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::HSTRING>(result__)
    }
}
impl ::core::convert::From<ICorrelationVectorSource> for ::windows::core::IUnknown {
    fn from(value: ICorrelationVectorSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICorrelationVectorSource> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICorrelationVectorSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICorrelationVectorSource> for ::windows::core::IUnknown {
    fn from(value: &ICorrelationVectorSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICorrelationVectorSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICorrelationVectorSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorrelationVectorSource {}
impl ::core::fmt::Debug for ICorrelationVectorSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorrelationVectorSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorrelationVectorSource {
    type Vtable = ICorrelationVectorSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x152b8a3b_b9b9_4685_b56e_974847bc7545);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorrelationVectorSource_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub CorrelationVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cv: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IDragDropManagerInterop(::windows::core::IUnknown);
impl IDragDropManagerInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<'a, P0, T>(&self, hwnd: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetForWindow)(::windows::core::Interface::as_raw(self), hwnd.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IDragDropManagerInterop> for ::windows::core::IUnknown {
    fn from(value: IDragDropManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDragDropManagerInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDragDropManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDragDropManagerInterop> for ::windows::core::IUnknown {
    fn from(value: &IDragDropManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IDragDropManagerInterop> for ::windows::core::IInspectable {
    fn from(value: IDragDropManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDragDropManagerInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IDragDropManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDragDropManagerInterop> for ::windows::core::IInspectable {
    fn from(value: &IDragDropManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDragDropManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDragDropManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDragDropManagerInterop {}
impl ::core::fmt::Debug for IDragDropManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDragDropManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDragDropManagerInterop {
    type Vtable = IDragDropManagerInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ad8cba7_4c01_4dac_9074_827894292d63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragDropManagerInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IHolographicSpaceInterop(::windows::core::IUnknown);
impl IHolographicSpaceInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateForWindow<'a, P0, T>(&self, window: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateForWindow)(::windows::core::Interface::as_raw(self), window.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IHolographicSpaceInterop> for ::windows::core::IUnknown {
    fn from(value: IHolographicSpaceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IHolographicSpaceInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IHolographicSpaceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolographicSpaceInterop> for ::windows::core::IUnknown {
    fn from(value: &IHolographicSpaceInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IHolographicSpaceInterop> for ::windows::core::IInspectable {
    fn from(value: IHolographicSpaceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IHolographicSpaceInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IHolographicSpaceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolographicSpaceInterop> for ::windows::core::IInspectable {
    fn from(value: &IHolographicSpaceInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IHolographicSpaceInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHolographicSpaceInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHolographicSpaceInterop {}
impl ::core::fmt::Debug for IHolographicSpaceInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHolographicSpaceInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHolographicSpaceInterop {
    type Vtable = IHolographicSpaceInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c4ee536_6a98_4b86_a170_587013d6fd4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, holographicspace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IInputPaneInterop(::windows::core::IUnknown);
impl IInputPaneInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<'a, P0, T>(&self, appwindow: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetForWindow)(::windows::core::Interface::as_raw(self), appwindow.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IInputPaneInterop> for ::windows::core::IUnknown {
    fn from(value: IInputPaneInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IInputPaneInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IInputPaneInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInputPaneInterop> for ::windows::core::IUnknown {
    fn from(value: &IInputPaneInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IInputPaneInterop> for ::windows::core::IInspectable {
    fn from(value: IInputPaneInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IInputPaneInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IInputPaneInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInputPaneInterop> for ::windows::core::IInspectable {
    fn from(value: &IInputPaneInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IInputPaneInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInputPaneInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInputPaneInterop {}
impl ::core::fmt::Debug for IInputPaneInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInputPaneInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInputPaneInterop {
    type Vtable = IInputPaneInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75cf2c57_9195_4931_8332_f0b409e916af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPaneInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, inputpane: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ILanguageExceptionErrorInfo(::windows::core::IUnknown);
impl ILanguageExceptionErrorInfo {
    pub unsafe fn GetLanguageException(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLanguageException)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<ILanguageExceptionErrorInfo> for ::windows::core::IUnknown {
    fn from(value: ILanguageExceptionErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILanguageExceptionErrorInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ILanguageExceptionErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILanguageExceptionErrorInfo> for ::windows::core::IUnknown {
    fn from(value: &ILanguageExceptionErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ILanguageExceptionErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILanguageExceptionErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILanguageExceptionErrorInfo {}
impl ::core::fmt::Debug for ILanguageExceptionErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILanguageExceptionErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILanguageExceptionErrorInfo {
    type Vtable = ILanguageExceptionErrorInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04a2dbf3_df83_116c_0946_0812abf6e07d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionErrorInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetLanguageException: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languageexception: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ILanguageExceptionErrorInfo2(::windows::core::IUnknown);
impl ILanguageExceptionErrorInfo2 {
    pub unsafe fn GetLanguageException(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetLanguageException)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetPreviousLanguageExceptionErrorInfo(&self) -> ::windows::core::Result<ILanguageExceptionErrorInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPreviousLanguageExceptionErrorInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ILanguageExceptionErrorInfo2>(result__)
    }
    pub unsafe fn CapturePropagationContext<'a, P0>(&self, languageexception: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).CapturePropagationContext)(::windows::core::Interface::as_raw(self), languageexception.into().abi()).ok()
    }
    pub unsafe fn GetPropagationContextHead(&self) -> ::windows::core::Result<ILanguageExceptionErrorInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPropagationContextHead)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ILanguageExceptionErrorInfo2>(result__)
    }
}
impl ::core::convert::From<ILanguageExceptionErrorInfo2> for ::windows::core::IUnknown {
    fn from(value: ILanguageExceptionErrorInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILanguageExceptionErrorInfo2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ILanguageExceptionErrorInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILanguageExceptionErrorInfo2> for ::windows::core::IUnknown {
    fn from(value: &ILanguageExceptionErrorInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ILanguageExceptionErrorInfo2> for ILanguageExceptionErrorInfo {
    fn from(value: ILanguageExceptionErrorInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILanguageExceptionErrorInfo2> for &'a ILanguageExceptionErrorInfo {
    fn from(value: &'a ILanguageExceptionErrorInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILanguageExceptionErrorInfo2> for ILanguageExceptionErrorInfo {
    fn from(value: &ILanguageExceptionErrorInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ILanguageExceptionErrorInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILanguageExceptionErrorInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILanguageExceptionErrorInfo2 {}
impl ::core::fmt::Debug for ILanguageExceptionErrorInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILanguageExceptionErrorInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILanguageExceptionErrorInfo2 {
    type Vtable = ILanguageExceptionErrorInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5746e5c4_5b97_424c_b620_2822915734dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionErrorInfo2_Vtbl {
    pub base__: ILanguageExceptionErrorInfo_Vtbl,
    pub GetPreviousLanguageExceptionErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previouslanguageexceptionerrorinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CapturePropagationContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languageexception: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPropagationContextHead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propagatedlanguageexceptionerrorinfohead: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ILanguageExceptionStackBackTrace(::windows::core::IUnknown);
impl ILanguageExceptionStackBackTrace {
    pub unsafe fn GetStackBackTrace(&self, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStackBackTrace)(::windows::core::Interface::as_raw(self), maxframestocapture, ::core::mem::transmute(stackbacktrace), ::core::mem::transmute(framescaptured)).ok()
    }
}
impl ::core::convert::From<ILanguageExceptionStackBackTrace> for ::windows::core::IUnknown {
    fn from(value: ILanguageExceptionStackBackTrace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILanguageExceptionStackBackTrace> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ILanguageExceptionStackBackTrace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILanguageExceptionStackBackTrace> for ::windows::core::IUnknown {
    fn from(value: &ILanguageExceptionStackBackTrace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ILanguageExceptionStackBackTrace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILanguageExceptionStackBackTrace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILanguageExceptionStackBackTrace {}
impl ::core::fmt::Debug for ILanguageExceptionStackBackTrace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILanguageExceptionStackBackTrace").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILanguageExceptionStackBackTrace {
    type Vtable = ILanguageExceptionStackBackTrace_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbe53fb5_f967_4258_8d34_42f5e25833de);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionStackBackTrace_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetStackBackTrace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ILanguageExceptionTransform(::windows::core::IUnknown);
impl ILanguageExceptionTransform {
    pub unsafe fn GetTransformedRestrictedErrorInfo(&self) -> ::windows::core::Result<IRestrictedErrorInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTransformedRestrictedErrorInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRestrictedErrorInfo>(result__)
    }
}
impl ::core::convert::From<ILanguageExceptionTransform> for ::windows::core::IUnknown {
    fn from(value: ILanguageExceptionTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILanguageExceptionTransform> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ILanguageExceptionTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILanguageExceptionTransform> for ::windows::core::IUnknown {
    fn from(value: &ILanguageExceptionTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ILanguageExceptionTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILanguageExceptionTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILanguageExceptionTransform {}
impl ::core::fmt::Debug for ILanguageExceptionTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILanguageExceptionTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILanguageExceptionTransform {
    type Vtable = ILanguageExceptionTransform_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfeb5a271_a6cd_45ce_880a_696706badc65);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionTransform_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetTransformedRestrictedErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restrictederrorinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IMemoryBufferByteAccess(::windows::core::IUnknown);
impl IMemoryBufferByteAccess {
    pub unsafe fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(value), ::core::mem::transmute(capacity)).ok()
    }
}
impl ::core::convert::From<IMemoryBufferByteAccess> for ::windows::core::IUnknown {
    fn from(value: IMemoryBufferByteAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMemoryBufferByteAccess> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMemoryBufferByteAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMemoryBufferByteAccess> for ::windows::core::IUnknown {
    fn from(value: &IMemoryBufferByteAccess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMemoryBufferByteAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMemoryBufferByteAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMemoryBufferByteAccess {}
impl ::core::fmt::Debug for IMemoryBufferByteAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMemoryBufferByteAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMemoryBufferByteAccess {
    type Vtable = IMemoryBufferByteAccess_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b0d3235_4dba_4d44_865e_8f1d0e4fd04d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryBufferByteAccess_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut u8, capacity: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IMessageDispatcher(::windows::core::IUnknown);
impl IMessageDispatcher {
    pub unsafe fn PumpMessages(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PumpMessages)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IMessageDispatcher> for ::windows::core::IUnknown {
    fn from(value: IMessageDispatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMessageDispatcher> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMessageDispatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMessageDispatcher> for ::windows::core::IUnknown {
    fn from(value: &IMessageDispatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IMessageDispatcher> for ::windows::core::IInspectable {
    fn from(value: IMessageDispatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMessageDispatcher> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IMessageDispatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMessageDispatcher> for ::windows::core::IInspectable {
    fn from(value: &IMessageDispatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMessageDispatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMessageDispatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMessageDispatcher {}
impl ::core::fmt::Debug for IMessageDispatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMessageDispatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMessageDispatcher {
    type Vtable = IMessageDispatcher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5f84c8f_cfd0_4cd6_b66b_c5d26ff1689d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageDispatcher_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PumpMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IPlayToManagerInterop(::windows::core::IUnknown);
impl IPlayToManagerInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<'a, P0, T>(&self, appwindow: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetForWindow)(::windows::core::Interface::as_raw(self), appwindow.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowPlayToUIForWindow<'a, P0>(&self, appwindow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).ShowPlayToUIForWindow)(::windows::core::Interface::as_raw(self), appwindow.into()).ok()
    }
}
impl ::core::convert::From<IPlayToManagerInterop> for ::windows::core::IUnknown {
    fn from(value: IPlayToManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPlayToManagerInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPlayToManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPlayToManagerInterop> for ::windows::core::IUnknown {
    fn from(value: &IPlayToManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IPlayToManagerInterop> for ::windows::core::IInspectable {
    fn from(value: IPlayToManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPlayToManagerInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IPlayToManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPlayToManagerInterop> for ::windows::core::IInspectable {
    fn from(value: &IPlayToManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPlayToManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPlayToManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlayToManagerInterop {}
impl ::core::fmt::Debug for IPlayToManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlayToManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPlayToManagerInterop {
    type Vtable = IPlayToManagerInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24394699_1f2c_4eb3_8cd7_0ec1da42a540);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToManagerInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, playtomanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowPlayToUIForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowPlayToUIForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IRestrictedErrorInfo(::windows::core::IUnknown);
impl IRestrictedErrorInfo {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetErrorDetails(&self, description: *mut super::super::Foundation::BSTR, error: *mut ::windows::core::HRESULT, restricteddescription: *mut super::super::Foundation::BSTR, capabilitysid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetErrorDetails)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(description), ::core::mem::transmute(error), ::core::mem::transmute(restricteddescription), ::core::mem::transmute(capabilitysid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReference(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetReference)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IRestrictedErrorInfo> for ::windows::core::IUnknown {
    fn from(value: IRestrictedErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRestrictedErrorInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRestrictedErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRestrictedErrorInfo> for ::windows::core::IUnknown {
    fn from(value: &IRestrictedErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRestrictedErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRestrictedErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRestrictedErrorInfo {}
impl ::core::fmt::Debug for IRestrictedErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRestrictedErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for IRestrictedErrorInfo {}
unsafe impl ::core::marker::Sync for IRestrictedErrorInfo {}
unsafe impl ::windows::core::Interface for IRestrictedErrorInfo {
    type Vtable = IRestrictedErrorInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82ba7092_4c88_427d_a7bc_16dd93feb67e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRestrictedErrorInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetErrorDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR, error: *mut ::windows::core::HRESULT, restricteddescription: *mut super::super::Foundation::BSTR, capabilitysid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetErrorDetails: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reference: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetReference: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IRoMetaDataLocator(::windows::core::IUnknown);
impl IRoMetaDataLocator {
    pub unsafe fn Locate<'a, P0, P1>(&self, nameelement: P0, metadatadestination: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IRoSimpleMetaDataBuilder>>,
    {
        (::windows::core::Interface::vtable(self).Locate)(::windows::core::Interface::as_raw(self), nameelement.into(), metadatadestination.into().abi()).ok()
    }
}
impl ::core::clone::Clone for IRoMetaDataLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRoMetaDataLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRoMetaDataLocator {}
impl ::core::fmt::Debug for IRoMetaDataLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRoMetaDataLocator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRoMetaDataLocator {
    type Vtable = IRoMetaDataLocator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoMetaDataLocator_Vtbl {
    pub Locate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nameelement: ::windows::core::PCWSTR, metadatadestination: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IRoSimpleMetaDataBuilder(::windows::core::IUnknown);
impl IRoSimpleMetaDataBuilder {
    pub unsafe fn SetWinRtInterface(&self, iid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWinRtInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(iid)).ok()
    }
    pub unsafe fn SetDelegate(&self, iid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDelegate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(iid)).ok()
    }
    pub unsafe fn SetInterfaceGroupSimpleDefault<'a, P0, P1>(&self, name: P0, defaultinterfacename: P1, defaultinterfaceiid: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetInterfaceGroupSimpleDefault)(::windows::core::Interface::as_raw(self), name.into(), defaultinterfacename.into(), ::core::mem::transmute(defaultinterfaceiid)).ok()
    }
    pub unsafe fn SetInterfaceGroupParameterizedDefault<'a, P0>(&self, name: P0, defaultinterfacenameelements: &[::windows::core::PWSTR]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetInterfaceGroupParameterizedDefault)(::windows::core::Interface::as_raw(self), name.into(), defaultinterfacenameelements.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(defaultinterfacenameelements))).ok()
    }
    pub unsafe fn SetRuntimeClassSimpleDefault<'a, P0, P1>(&self, name: P0, defaultinterfacename: P1, defaultinterfaceiid: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetRuntimeClassSimpleDefault)(::windows::core::Interface::as_raw(self), name.into(), defaultinterfacename.into(), ::core::mem::transmute(defaultinterfaceiid)).ok()
    }
    pub unsafe fn SetRuntimeClassParameterizedDefault<'a, P0>(&self, name: P0, defaultinterfacenameelements: &[::windows::core::PWSTR]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetRuntimeClassParameterizedDefault)(::windows::core::Interface::as_raw(self), name.into(), defaultinterfacenameelements.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(defaultinterfacenameelements))).ok()
    }
    pub unsafe fn SetStruct<'a, P0>(&self, name: P0, fieldtypenames: &[::windows::core::PWSTR]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetStruct)(::windows::core::Interface::as_raw(self), name.into(), fieldtypenames.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(fieldtypenames))).ok()
    }
    pub unsafe fn SetEnum<'a, P0, P1>(&self, name: P0, basetype: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetEnum)(::windows::core::Interface::as_raw(self), name.into(), basetype.into()).ok()
    }
    pub unsafe fn SetParameterizedInterface(&self, piid: ::windows::core::GUID, numargs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetParameterizedInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(piid), numargs).ok()
    }
    pub unsafe fn SetParameterizedDelegate(&self, piid: ::windows::core::GUID, numargs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetParameterizedDelegate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(piid), numargs).ok()
    }
}
impl ::core::clone::Clone for IRoSimpleMetaDataBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRoSimpleMetaDataBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRoSimpleMetaDataBuilder {}
impl ::core::fmt::Debug for IRoSimpleMetaDataBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRoSimpleMetaDataBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRoSimpleMetaDataBuilder {
    type Vtable = IRoSimpleMetaDataBuilder_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoSimpleMetaDataBuilder_Vtbl {
    pub SetWinRtInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetDelegate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetInterfaceGroupSimpleDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, defaultinterfacename: ::windows::core::PCWSTR, defaultinterfaceiid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetInterfaceGroupParameterizedDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetRuntimeClassSimpleDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, defaultinterfacename: ::windows::core::PCWSTR, defaultinterfaceiid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetRuntimeClassParameterizedDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetStruct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, numfields: u32, fieldtypenames: *const ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, basetype: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetParameterizedInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: ::windows::core::GUID, numargs: u32) -> ::windows::core::HRESULT,
    pub SetParameterizedDelegate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: ::windows::core::GUID, numargs: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IShareWindowCommandEventArgsInterop(::windows::core::IUnknown);
impl IShareWindowCommandEventArgsInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetWindow)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HWND>(result__)
    }
}
impl ::core::convert::From<IShareWindowCommandEventArgsInterop> for ::windows::core::IUnknown {
    fn from(value: IShareWindowCommandEventArgsInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IShareWindowCommandEventArgsInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IShareWindowCommandEventArgsInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IShareWindowCommandEventArgsInterop> for ::windows::core::IUnknown {
    fn from(value: &IShareWindowCommandEventArgsInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IShareWindowCommandEventArgsInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IShareWindowCommandEventArgsInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShareWindowCommandEventArgsInterop {}
impl ::core::fmt::Debug for IShareWindowCommandEventArgsInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShareWindowCommandEventArgsInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IShareWindowCommandEventArgsInterop {
    type Vtable = IShareWindowCommandEventArgsInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6571a721_643d_43d4_aca4_6b6f5f30f1ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandEventArgsInterop_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IShareWindowCommandSourceInterop(::windows::core::IUnknown);
impl IShareWindowCommandSourceInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<'a, P0, T>(&self, appwindow: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetForWindow)(::windows::core::Interface::as_raw(self), appwindow.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IShareWindowCommandSourceInterop> for ::windows::core::IUnknown {
    fn from(value: IShareWindowCommandSourceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IShareWindowCommandSourceInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IShareWindowCommandSourceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IShareWindowCommandSourceInterop> for ::windows::core::IUnknown {
    fn from(value: &IShareWindowCommandSourceInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IShareWindowCommandSourceInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IShareWindowCommandSourceInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShareWindowCommandSourceInterop {}
impl ::core::fmt::Debug for IShareWindowCommandSourceInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShareWindowCommandSourceInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IShareWindowCommandSourceInterop {
    type Vtable = IShareWindowCommandSourceInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x461a191f_8424_43a6_a0fa_3451a22f56ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandSourceInterop_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, sharewindowcommandsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ISpatialInteractionManagerInterop(::windows::core::IUnknown);
impl ISpatialInteractionManagerInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<'a, P0, T>(&self, window: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetForWindow)(::windows::core::Interface::as_raw(self), window.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ISpatialInteractionManagerInterop> for ::windows::core::IUnknown {
    fn from(value: ISpatialInteractionManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISpatialInteractionManagerInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISpatialInteractionManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialInteractionManagerInterop> for ::windows::core::IUnknown {
    fn from(value: &ISpatialInteractionManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ISpatialInteractionManagerInterop> for ::windows::core::IInspectable {
    fn from(value: ISpatialInteractionManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISpatialInteractionManagerInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a ISpatialInteractionManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialInteractionManagerInterop> for ::windows::core::IInspectable {
    fn from(value: &ISpatialInteractionManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISpatialInteractionManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialInteractionManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialInteractionManagerInterop {}
impl ::core::fmt::Debug for ISpatialInteractionManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialInteractionManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialInteractionManagerInterop {
    type Vtable = ISpatialInteractionManagerInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c4ee536_6a98_4b86_a170_587013d6fd4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionManagerInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, spatialinteractionmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct ISystemMediaTransportControlsInterop(::windows::core::IUnknown);
impl ISystemMediaTransportControlsInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<'a, P0, T>(&self, appwindow: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetForWindow)(::windows::core::Interface::as_raw(self), appwindow.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ISystemMediaTransportControlsInterop> for ::windows::core::IUnknown {
    fn from(value: ISystemMediaTransportControlsInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISystemMediaTransportControlsInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISystemMediaTransportControlsInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISystemMediaTransportControlsInterop> for ::windows::core::IUnknown {
    fn from(value: &ISystemMediaTransportControlsInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ISystemMediaTransportControlsInterop> for ::windows::core::IInspectable {
    fn from(value: ISystemMediaTransportControlsInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISystemMediaTransportControlsInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a ISystemMediaTransportControlsInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISystemMediaTransportControlsInterop> for ::windows::core::IInspectable {
    fn from(value: &ISystemMediaTransportControlsInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISystemMediaTransportControlsInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISystemMediaTransportControlsInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISystemMediaTransportControlsInterop {}
impl ::core::fmt::Debug for ISystemMediaTransportControlsInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISystemMediaTransportControlsInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISystemMediaTransportControlsInterop {
    type Vtable = ISystemMediaTransportControlsInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddb0472d_c911_4a1f_86d9_dc3d71a95f5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, mediatransportcontrol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IUIViewSettingsInterop(::windows::core::IUnknown);
impl IUIViewSettingsInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<'a, P0, T>(&self, hwnd: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetForWindow)(::windows::core::Interface::as_raw(self), hwnd.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IUIViewSettingsInterop> for ::windows::core::IUnknown {
    fn from(value: IUIViewSettingsInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IUIViewSettingsInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IUIViewSettingsInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIViewSettingsInterop> for ::windows::core::IUnknown {
    fn from(value: &IUIViewSettingsInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IUIViewSettingsInterop> for ::windows::core::IInspectable {
    fn from(value: IUIViewSettingsInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IUIViewSettingsInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IUIViewSettingsInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIViewSettingsInterop> for ::windows::core::IInspectable {
    fn from(value: &IUIViewSettingsInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IUIViewSettingsInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIViewSettingsInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIViewSettingsInterop {}
impl ::core::fmt::Debug for IUIViewSettingsInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIViewSettingsInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIViewSettingsInterop {
    type Vtable = IUIViewSettingsInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3694dbf9_8f68_44be_8ff5_195c98ede8a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIViewSettingsInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IUserActivityInterop(::windows::core::IUnknown);
impl IUserActivityInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSessionForWindow<'a, P0, T>(&self, window: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateSessionForWindow)(::windows::core::Interface::as_raw(self), window.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IUserActivityInterop> for ::windows::core::IUnknown {
    fn from(value: IUserActivityInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IUserActivityInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IUserActivityInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserActivityInterop> for ::windows::core::IUnknown {
    fn from(value: &IUserActivityInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IUserActivityInterop> for ::windows::core::IInspectable {
    fn from(value: IUserActivityInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IUserActivityInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IUserActivityInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserActivityInterop> for ::windows::core::IInspectable {
    fn from(value: &IUserActivityInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IUserActivityInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserActivityInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserActivityInterop {}
impl ::core::fmt::Debug for IUserActivityInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserActivityInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUserActivityInterop {
    type Vtable = IUserActivityInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ade314d_0e0a_40d9_824c_9a088a50059f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSessionForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, iid: *const ::windows::core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSessionForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IUserActivityRequestManagerInterop(::windows::core::IUnknown);
impl IUserActivityRequestManagerInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<'a, P0, T>(&self, window: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetForWindow)(::windows::core::Interface::as_raw(self), window.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IUserActivityRequestManagerInterop> for ::windows::core::IUnknown {
    fn from(value: IUserActivityRequestManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IUserActivityRequestManagerInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IUserActivityRequestManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserActivityRequestManagerInterop> for ::windows::core::IUnknown {
    fn from(value: &IUserActivityRequestManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IUserActivityRequestManagerInterop> for ::windows::core::IInspectable {
    fn from(value: IUserActivityRequestManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IUserActivityRequestManagerInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IUserActivityRequestManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserActivityRequestManagerInterop> for ::windows::core::IInspectable {
    fn from(value: &IUserActivityRequestManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IUserActivityRequestManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserActivityRequestManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserActivityRequestManagerInterop {}
impl ::core::fmt::Debug for IUserActivityRequestManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserActivityRequestManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUserActivityRequestManagerInterop {
    type Vtable = IUserActivityRequestManagerInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd69f876_9699_4715_9095_e37ea30dfa1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityRequestManagerInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, iid: *const ::windows::core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IUserActivitySourceHostInterop(::windows::core::IUnknown);
impl IUserActivitySourceHostInterop {
    pub unsafe fn SetActivitySourceHost<'a, P0>(&self, activitysourcehost: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
    {
        (::windows::core::Interface::vtable(self).SetActivitySourceHost)(::windows::core::Interface::as_raw(self), activitysourcehost.into().abi()).ok()
    }
}
impl ::core::convert::From<IUserActivitySourceHostInterop> for ::windows::core::IUnknown {
    fn from(value: IUserActivitySourceHostInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IUserActivitySourceHostInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IUserActivitySourceHostInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserActivitySourceHostInterop> for ::windows::core::IUnknown {
    fn from(value: &IUserActivitySourceHostInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IUserActivitySourceHostInterop> for ::windows::core::IInspectable {
    fn from(value: IUserActivitySourceHostInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IUserActivitySourceHostInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IUserActivitySourceHostInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserActivitySourceHostInterop> for ::windows::core::IInspectable {
    fn from(value: &IUserActivitySourceHostInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IUserActivitySourceHostInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserActivitySourceHostInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserActivitySourceHostInterop {}
impl ::core::fmt::Debug for IUserActivitySourceHostInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserActivitySourceHostInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUserActivitySourceHostInterop {
    type Vtable = IUserActivitySourceHostInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc15df8bc_8844_487a_b85b_7578e0f61419);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivitySourceHostInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SetActivitySourceHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activitysourcehost: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IUserConsentVerifierInterop(::windows::core::IUnknown);
impl IUserConsentVerifierInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestVerificationForWindowAsync<'a, P0, P1, T>(&self, appwindow: P0, message: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).RequestVerificationForWindowAsync)(::windows::core::Interface::as_raw(self), appwindow.into(), message.into().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IUserConsentVerifierInterop> for ::windows::core::IUnknown {
    fn from(value: IUserConsentVerifierInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IUserConsentVerifierInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IUserConsentVerifierInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserConsentVerifierInterop> for ::windows::core::IUnknown {
    fn from(value: &IUserConsentVerifierInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IUserConsentVerifierInterop> for ::windows::core::IInspectable {
    fn from(value: IUserConsentVerifierInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IUserConsentVerifierInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IUserConsentVerifierInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserConsentVerifierInterop> for ::windows::core::IInspectable {
    fn from(value: &IUserConsentVerifierInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IUserConsentVerifierInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserConsentVerifierInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserConsentVerifierInterop {}
impl ::core::fmt::Debug for IUserConsentVerifierInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserConsentVerifierInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUserConsentVerifierInterop {
    type Vtable = IUserConsentVerifierInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39e050c3_4e74_441a_8dc0_b81104df949c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserConsentVerifierInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestVerificationForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestVerificationForWindowAsync: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IWeakReference(::windows::core::IUnknown);
impl IWeakReference {
    pub unsafe fn Resolve<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).Resolve)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IWeakReference> for ::windows::core::IUnknown {
    fn from(value: IWeakReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWeakReference> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWeakReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWeakReference> for ::windows::core::IUnknown {
    fn from(value: &IWeakReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWeakReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWeakReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWeakReference {}
impl ::core::fmt::Debug for IWeakReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWeakReference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWeakReference {
    type Vtable = IWeakReference_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000037_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReference_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Resolve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, objectreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IWeakReferenceSource(::windows::core::IUnknown);
impl IWeakReferenceSource {
    pub unsafe fn GetWeakReference(&self) -> ::windows::core::Result<IWeakReference> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetWeakReference)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWeakReference>(result__)
    }
}
impl ::core::convert::From<IWeakReferenceSource> for ::windows::core::IUnknown {
    fn from(value: IWeakReferenceSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWeakReferenceSource> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWeakReferenceSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWeakReferenceSource> for ::windows::core::IUnknown {
    fn from(value: &IWeakReferenceSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWeakReferenceSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWeakReferenceSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWeakReferenceSource {}
impl ::core::fmt::Debug for IWeakReferenceSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWeakReferenceSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWeakReferenceSource {
    type Vtable = IWeakReferenceSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000038_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReferenceSource_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetWeakReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, weakreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerInterop(::windows::core::IUnknown);
impl IWebAuthenticationCoreManagerInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestTokenForWindowAsync<'a, P0, P1, T>(&self, appwindow: P0, request: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).RequestTokenForWindowAsync)(::windows::core::Interface::as_raw(self), appwindow.into(), request.into().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestTokenWithWebAccountForWindowAsync<'a, P0, P1, P2, T>(&self, appwindow: P0, request: P1, webaccount: P2) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).RequestTokenWithWebAccountForWindowAsync)(::windows::core::Interface::as_raw(self), appwindow.into(), request.into().abi(), webaccount.into().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IWebAuthenticationCoreManagerInterop> for ::windows::core::IUnknown {
    fn from(value: IWebAuthenticationCoreManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWebAuthenticationCoreManagerInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWebAuthenticationCoreManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAuthenticationCoreManagerInterop> for ::windows::core::IUnknown {
    fn from(value: &IWebAuthenticationCoreManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWebAuthenticationCoreManagerInterop> for ::windows::core::IInspectable {
    fn from(value: IWebAuthenticationCoreManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWebAuthenticationCoreManagerInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IWebAuthenticationCoreManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAuthenticationCoreManagerInterop> for ::windows::core::IInspectable {
    fn from(value: &IWebAuthenticationCoreManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWebAuthenticationCoreManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebAuthenticationCoreManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAuthenticationCoreManagerInterop {}
impl ::core::fmt::Debug for IWebAuthenticationCoreManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAuthenticationCoreManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWebAuthenticationCoreManagerInterop {
    type Vtable = IWebAuthenticationCoreManagerInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4b8e804_811e_4436_b69c_44cb67b72084);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestTokenForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, request: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestTokenForWindowAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestTokenWithWebAccountForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, request: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestTokenWithWebAccountForWindowAsync: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsErrorPropagationEnabled() -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsErrorPropagationEnabled() -> super::super::Foundation::BOOL;
    }
    IsErrorPropagationEnabled()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const MAX_ERROR_MESSAGE_CHARS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn MetaDataGetDispenser(rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MetaDataGetDispenser(rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    MetaDataGetDispenser(::core::mem::transmute(rclsid), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub type PINSPECT_HSTRING_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, readaddress: usize, length: u32, buffer: *mut u8) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub type PINSPECT_HSTRING_CALLBACK2 = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, readaddress: u64, length: u32, buffer: *mut u8) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub type PINSPECT_MEMORY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, readaddress: usize, length: u32, buffer: *mut u8) -> ::windows::core::HRESULT>;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ROPARAMIIDHANDLE(pub isize);
impl ROPARAMIIDHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for ROPARAMIIDHANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for ROPARAMIIDHANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for ROPARAMIIDHANDLE {}
impl ::core::fmt::Debug for ROPARAMIIDHANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ROPARAMIIDHANDLE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<ROPARAMIIDHANDLE>> for ROPARAMIIDHANDLE {
    fn from(optional: ::core::option::Option<ROPARAMIIDHANDLE>) -> ROPARAMIIDHANDLE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for ROPARAMIIDHANDLE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RO_ERROR_REPORTING_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_ERROR_REPORTING_NONE: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_ERROR_REPORTING_SUPPRESSEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_ERROR_REPORTING_FORCEEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_ERROR_REPORTING_USESETERRORINFO: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_ERROR_REPORTING_SUPPRESSSETERRORINFO: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(8u32);
impl ::core::marker::Copy for RO_ERROR_REPORTING_FLAGS {}
impl ::core::clone::Clone for RO_ERROR_REPORTING_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RO_ERROR_REPORTING_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RO_ERROR_REPORTING_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RO_ERROR_REPORTING_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RO_ERROR_REPORTING_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RO_ERROR_REPORTING_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RO_ERROR_REPORTING_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RO_ERROR_REPORTING_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RO_ERROR_REPORTING_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RO_ERROR_REPORTING_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RO_INIT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_INIT_SINGLETHREADED: RO_INIT_TYPE = RO_INIT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_INIT_MULTITHREADED: RO_INIT_TYPE = RO_INIT_TYPE(1i32);
impl ::core::marker::Copy for RO_INIT_TYPE {}
impl ::core::clone::Clone for RO_INIT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RO_INIT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RO_INIT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RO_INIT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RO_INIT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoActivateInstance<'a, P0>(activatableclassid: P0) -> ::windows::core::Result<::windows::core::IInspectable>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoActivateInstance(activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, instance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RoActivateInstance(activatableclassid.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IInspectable>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoCaptureErrorContext(hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoCaptureErrorContext(hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT;
    }
    RoCaptureErrorContext(hr).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoClearError() {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoClearError();
    }
    RoClearError()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoFailFastWithErrorContext(hrerror: ::windows::core::HRESULT) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoFailFastWithErrorContext(hrerror: ::windows::core::HRESULT);
    }
    RoFailFastWithErrorContext(hrerror)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoFreeParameterizedTypeExtra<'a, P0>(extra: P0)
where
    P0: ::std::convert::Into<ROPARAMIIDHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoFreeParameterizedTypeExtra(extra: ROPARAMIIDHANDLE);
    }
    RoFreeParameterizedTypeExtra(extra.into())
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoGetActivationFactory<'a, P0, T>(activatableclassid: P0) -> ::windows::core::Result<T>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
    T: ::windows::core::Interface,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoGetActivationFactory(activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, iid: *const ::windows::core::GUID, factory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::option::Option::None;
    RoGetActivationFactory(activatableclassid.into().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoGetAgileReference<'a, P0>(options: AgileReferenceOptions, riid: *const ::windows::core::GUID, punk: P0) -> ::windows::core::Result<IAgileReference>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoGetAgileReference(options: AgileReferenceOptions, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, ppagilereference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RoGetAgileReference(options, ::core::mem::transmute(riid), punk.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAgileReference>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoGetApartmentIdentifier() -> ::windows::core::Result<u64> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoGetApartmentIdentifier(apartmentidentifier: *mut u64) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RoGetApartmentIdentifier(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_System_Com_Marshal\"`*"]
#[cfg(feature = "Win32_System_Com_Marshal")]
#[inline]
pub unsafe fn RoGetBufferMarshaler() -> ::windows::core::Result<super::Com::Marshal::IMarshal> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoGetBufferMarshaler(buffermarshaler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RoGetBufferMarshaler(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::Marshal::IMarshal>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoGetErrorReportingFlags() -> ::windows::core::Result<u32> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoGetErrorReportingFlags(pflags: *mut u32) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RoGetErrorReportingFlags(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoGetMatchingRestrictedErrorInfo(hrin: ::windows::core::HRESULT) -> ::windows::core::Result<IRestrictedErrorInfo> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoGetMatchingRestrictedErrorInfo(hrin: ::windows::core::HRESULT, pprestrictederrorinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RoGetMatchingRestrictedErrorInfo(hrin, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRestrictedErrorInfo>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoGetParameterizedTypeInstanceIID<'a, P0>(nameelements: &[::windows::core::PWSTR], metadatalocator: P0, iid: *mut ::windows::core::GUID, pextra: *mut ROPARAMIIDHANDLE) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IRoMetaDataLocator>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoGetParameterizedTypeInstanceIID(nameelementcount: u32, nameelements: *const ::windows::core::PWSTR, metadatalocator: *mut ::core::ffi::c_void, iid: *mut ::windows::core::GUID, pextra: *mut ROPARAMIIDHANDLE) -> ::windows::core::HRESULT;
    }
    RoGetParameterizedTypeInstanceIID(nameelements.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(nameelements)), metadatalocator.into().abi(), ::core::mem::transmute(iid), ::core::mem::transmute(pextra)).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoGetServerActivatableClasses<'a, P0>(servername: P0, activatableclassids: *mut *mut ::windows::core::HSTRING, count: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoGetServerActivatableClasses(servername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, activatableclassids: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>, count: *mut u32) -> ::windows::core::HRESULT;
    }
    RoGetServerActivatableClasses(servername.into().abi(), ::core::mem::transmute(activatableclassids), ::core::mem::transmute(count)).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoInitialize(inittype: RO_INIT_TYPE) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoInitialize(inittype: RO_INIT_TYPE) -> ::windows::core::HRESULT;
    }
    RoInitialize(inittype).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: *const ::core::ffi::c_void, framecount: *mut u32, targetbacktraceaddress: *mut usize) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress: usize, machine: u16, readmemorycallback: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, framecount: *mut u32, targetbacktraceaddress: *mut usize) -> ::windows::core::HRESULT;
    }
    RoInspectCapturedStackBackTrace(targeterrorinfoaddress, machine, ::core::mem::transmute(readmemorycallback), ::core::mem::transmute(context), ::core::mem::transmute(framecount), ::core::mem::transmute(targetbacktraceaddress)).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoInspectThreadErrorInfo(targettebaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: *const ::core::ffi::c_void) -> ::windows::core::Result<usize> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoInspectThreadErrorInfo(targettebaddress: usize, machine: u16, readmemorycallback: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, targeterrorinfoaddress: *mut usize) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RoInspectThreadErrorInfo(targettebaddress, machine, ::core::mem::transmute(readmemorycallback), ::core::mem::transmute(context), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<usize>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RoOriginateError<'a, P0>(error: ::windows::core::HRESULT, message: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoOriginateError(error: ::windows::core::HRESULT, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> super::super::Foundation::BOOL;
    }
    RoOriginateError(error, message.into().abi())
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RoOriginateErrorW(error: ::windows::core::HRESULT, cchmax: u32, message: &[u16; 512]) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoOriginateErrorW(error: ::windows::core::HRESULT, cchmax: u32, message: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    RoOriginateErrorW(error, cchmax, ::core::mem::transmute(::windows::core::as_ptr_or_null(message)))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RoOriginateLanguageException<'a, P0, P1>(error: ::windows::core::HRESULT, message: P0, languageexception: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoOriginateLanguageException(error: ::windows::core::HRESULT, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, languageexception: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    RoOriginateLanguageException(error, message.into().abi(), languageexception.into().abi())
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoParameterizedTypeExtraGetTypeSignature<'a, P0>(extra: P0) -> ::windows::core::PSTR
where
    P0: ::std::convert::Into<ROPARAMIIDHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoParameterizedTypeExtraGetTypeSignature(extra: ROPARAMIIDHANDLE) -> ::windows::core::PSTR;
    }
    RoParameterizedTypeExtraGetTypeSignature(extra.into())
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoRegisterActivationFactories(activatableclassids: *const ::windows::core::HSTRING, activationfactorycallbacks: *const isize, count: u32) -> ::windows::core::Result<isize> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoRegisterActivationFactories(activatableclassids: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, activationfactorycallbacks: *const isize, count: u32, cookie: *mut isize) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RoRegisterActivationFactories(::core::mem::transmute(activatableclassids), ::core::mem::transmute(activationfactorycallbacks), ::core::mem::transmute(count), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<isize>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoRegisterForApartmentShutdown<'a, P0>(callbackobject: P0, apartmentidentifier: *mut u64, regcookie: *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IApartmentShutdown>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoRegisterForApartmentShutdown(callbackobject: *mut ::core::ffi::c_void, apartmentidentifier: *mut u64, regcookie: *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> ::windows::core::HRESULT;
    }
    RoRegisterForApartmentShutdown(callbackobject.into().abi(), ::core::mem::transmute(apartmentidentifier), ::core::mem::transmute(regcookie)).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoReportFailedDelegate<'a, P0, P1>(punkdelegate: P0, prestrictederrorinfo: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, IRestrictedErrorInfo>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoReportFailedDelegate(punkdelegate: *mut ::core::ffi::c_void, prestrictederrorinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    RoReportFailedDelegate(punkdelegate.into().abi(), prestrictederrorinfo.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoReportUnhandledError<'a, P0>(prestrictederrorinfo: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IRestrictedErrorInfo>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoReportUnhandledError(prestrictederrorinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    RoReportUnhandledError(prestrictederrorinfo.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoResolveRestrictedErrorInfoReference<'a, P0>(reference: P0) -> ::windows::core::Result<IRestrictedErrorInfo>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoResolveRestrictedErrorInfoReference(reference: ::windows::core::PCWSTR, pprestrictederrorinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RoResolveRestrictedErrorInfoReference(reference.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRestrictedErrorInfo>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoRevokeActivationFactories(cookie: isize) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoRevokeActivationFactories(cookie: isize);
    }
    RoRevokeActivationFactories(cookie)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoSetErrorReportingFlags(flags: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoSetErrorReportingFlags(flags: u32) -> ::windows::core::HRESULT;
    }
    RoSetErrorReportingFlags(flags).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RoTransformError<'a, P0>(olderror: ::windows::core::HRESULT, newerror: ::windows::core::HRESULT, message: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoTransformError(olderror: ::windows::core::HRESULT, newerror: ::windows::core::HRESULT, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> super::super::Foundation::BOOL;
    }
    RoTransformError(olderror, newerror, message.into().abi())
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RoTransformErrorW(olderror: ::windows::core::HRESULT, newerror: ::windows::core::HRESULT, cchmax: u32, message: &[u16; 512]) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoTransformErrorW(olderror: ::windows::core::HRESULT, newerror: ::windows::core::HRESULT, cchmax: u32, message: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    RoTransformErrorW(olderror, newerror, cchmax, ::core::mem::transmute(::windows::core::as_ptr_or_null(message)))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoUninitialize() {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoUninitialize();
    }
    RoUninitialize()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn RoUnregisterForApartmentShutdown<'a, P0>(regcookie: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<APARTMENT_SHUTDOWN_REGISTRATION_COOKIE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoUnregisterForApartmentShutdown(regcookie: APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> ::windows::core::HRESULT;
    }
    RoUnregisterForApartmentShutdown(regcookie.into()).ok()
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub struct ServerInformation {
    pub dwServerPid: u32,
    pub dwServerTid: u32,
    pub ui64ServerAddress: u64,
}
impl ::core::marker::Copy for ServerInformation {}
impl ::core::clone::Clone for ServerInformation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ServerInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ServerInformation").field("dwServerPid", &self.dwServerPid).field("dwServerTid", &self.dwServerTid).field("ui64ServerAddress", &self.ui64ServerAddress).finish()
    }
}
unsafe impl ::windows::core::Abi for ServerInformation {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ServerInformation {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ServerInformation>()) == 0 }
    }
}
impl ::core::cmp::Eq for ServerInformation {}
impl ::core::default::Default for ServerInformation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn SetRestrictedErrorInfo<'a, P0>(prestrictederrorinfo: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IRestrictedErrorInfo>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetRestrictedErrorInfo(prestrictederrorinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    SetRestrictedErrorInfo(prestrictederrorinfo.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TrustLevel(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const BaseTrust: TrustLevel = TrustLevel(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const PartialTrust: TrustLevel = TrustLevel(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const FullTrust: TrustLevel = TrustLevel(2i32);
impl ::core::marker::Copy for TrustLevel {}
impl ::core::clone::Clone for TrustLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TrustLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TrustLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for TrustLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TrustLevel").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsCompareStringOrdinal<'a, P0, P1>(string1: P0, string2: P1) -> ::windows::core::Result<i32>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowsCompareStringOrdinal(string1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, string2: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut i32) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsCompareStringOrdinal(string1.into().abi(), string2.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsConcatString<'a, P0, P1>(string1: P0, string2: P1) -> ::windows::core::Result<::windows::core::HSTRING>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowsConcatString(string1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, string2: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, newstring: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsConcatString(string1.into().abi(), string2.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::HSTRING>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsCreateString(sourcestring: &[u16]) -> ::windows::core::Result<::windows::core::HSTRING> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowsCreateString(sourcestring: ::windows::core::PCWSTR, length: u32, string: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsCreateString(::core::mem::transmute(::windows::core::as_ptr_or_null(sourcestring)), sourcestring.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::HSTRING>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsCreateStringReference<'a, P0>(sourcestring: P0, length: u32, hstringheader: *mut HSTRING_HEADER, string: *mut ::windows::core::HSTRING) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowsCreateStringReference(sourcestring: ::windows::core::PCWSTR, length: u32, hstringheader: *mut HSTRING_HEADER, string: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT;
    }
    WindowsCreateStringReference(sourcestring.into(), length, ::core::mem::transmute(hstringheader), ::core::mem::transmute(string)).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsDeleteString<'a, P0>(string: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowsDeleteString(string: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT;
    }
    WindowsDeleteString(string.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsDeleteStringBuffer<'a, P0>(bufferhandle: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HSTRING_BUFFER>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowsDeleteStringBuffer(bufferhandle: HSTRING_BUFFER) -> ::windows::core::HRESULT;
    }
    WindowsDeleteStringBuffer(bufferhandle.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsDuplicateString<'a, P0>(string: P0) -> ::windows::core::Result<::windows::core::HSTRING>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowsDuplicateString(string: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, newstring: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsDuplicateString(string.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::HSTRING>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsGetStringLen<'a, P0>(string: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowsGetStringLen(string: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> u32;
    }
    WindowsGetStringLen(string.into().abi())
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsGetStringRawBuffer<'a, P0>(string: P0, length: *mut u32) -> ::windows::core::PWSTR
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowsGetStringRawBuffer(string: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, length: *mut u32) -> ::windows::core::PWSTR;
    }
    WindowsGetStringRawBuffer(string.into().abi(), ::core::mem::transmute(length))
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsInspectString(targethstring: usize, machine: u16, callback: PINSPECT_HSTRING_CALLBACK, context: *const ::core::ffi::c_void, length: *mut u32, targetstringaddress: *mut usize) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowsInspectString(targethstring: usize, machine: u16, callback: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, length: *mut u32, targetstringaddress: *mut usize) -> ::windows::core::HRESULT;
    }
    WindowsInspectString(targethstring, machine, ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(length), ::core::mem::transmute(targetstringaddress)).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsInspectString2(targethstring: u64, machine: u16, callback: PINSPECT_HSTRING_CALLBACK2, context: *const ::core::ffi::c_void, length: *mut u32, targetstringaddress: *mut u64) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowsInspectString2(targethstring: u64, machine: u16, callback: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, length: *mut u32, targetstringaddress: *mut u64) -> ::windows::core::HRESULT;
    }
    WindowsInspectString2(targethstring, machine, ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(length), ::core::mem::transmute(targetstringaddress)).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WindowsIsStringEmpty<'a, P0>(string: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowsIsStringEmpty(string: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> super::super::Foundation::BOOL;
    }
    WindowsIsStringEmpty(string.into().abi())
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsPreallocateStringBuffer(length: u32, charbuffer: *mut *mut u16, bufferhandle: *mut HSTRING_BUFFER) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowsPreallocateStringBuffer(length: u32, charbuffer: *mut *mut u16, bufferhandle: *mut HSTRING_BUFFER) -> ::windows::core::HRESULT;
    }
    WindowsPreallocateStringBuffer(length, ::core::mem::transmute(charbuffer), ::core::mem::transmute(bufferhandle)).ok()
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsPromoteStringBuffer<'a, P0>(bufferhandle: P0) -> ::windows::core::Result<::windows::core::HSTRING>
where
    P0: ::std::convert::Into<HSTRING_BUFFER>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowsPromoteStringBuffer(bufferhandle: HSTRING_BUFFER, string: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsPromoteStringBuffer(bufferhandle.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::HSTRING>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsReplaceString<'a, P0, P1, P2>(string: P0, stringreplaced: P1, stringreplacewith: P2) -> ::windows::core::Result<::windows::core::HSTRING>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
    P2: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowsReplaceString(string: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, stringreplaced: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, stringreplacewith: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, newstring: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsReplaceString(string.into().abi(), stringreplaced.into().abi(), stringreplacewith.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::HSTRING>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WindowsStringHasEmbeddedNull<'a, P0>(string: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowsStringHasEmbeddedNull(string: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, hasembednull: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsStringHasEmbeddedNull(string.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsSubstring<'a, P0>(string: P0, startindex: u32) -> ::windows::core::Result<::windows::core::HSTRING>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowsSubstring(string: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, startindex: u32, newstring: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsSubstring(string.into().abi(), startindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::HSTRING>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsSubstringWithSpecifiedLength<'a, P0>(string: P0, startindex: u32, length: u32) -> ::windows::core::Result<::windows::core::HSTRING>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowsSubstringWithSpecifiedLength(string: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, startindex: u32, length: u32, newstring: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsSubstringWithSpecifiedLength(string.into().abi(), startindex, length, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::HSTRING>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsTrimStringEnd<'a, P0, P1>(string: P0, trimstring: P1) -> ::windows::core::Result<::windows::core::HSTRING>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowsTrimStringEnd(string: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, trimstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, newstring: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsTrimStringEnd(string.into().abi(), trimstring.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::HSTRING>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
#[inline]
pub unsafe fn WindowsTrimStringStart<'a, P0, P1>(string: P0, trimstring: P1) -> ::windows::core::Result<::windows::core::HSTRING>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowsTrimStringStart(string: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, trimstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, newstring: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WindowsTrimStringStart(string.into().abi(), trimstring.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::HSTRING>(result__)
}
#[repr(C)]
pub struct _RO_REGISTRATION_COOKIE(pub u8);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
