#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct MDL {
    pub MdlNext: *mut MDL,
    pub MdlSize: i16,
    pub MdlFlags: i16,
    pub Process: *mut MDL_0,
    pub lpMappedSystemVa: *mut u32,
    pub lpStartVa: *mut u32,
    pub ByteCount: u32,
    pub ByteOffset: u32,
}
impl ::core::marker::Copy for MDL {}
impl ::core::clone::Clone for MDL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MDL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MDL").field("MdlNext", &self.MdlNext).field("MdlSize", &self.MdlSize).field("MdlFlags", &self.MdlFlags).field("Process", &self.Process).field("lpMappedSystemVa", &self.lpMappedSystemVa).field("lpStartVa", &self.lpStartVa).field("ByteCount", &self.ByteCount).field("ByteOffset", &self.ByteOffset).finish()
    }
}
impl ::core::cmp::PartialEq for MDL {
    fn eq(&self, other: &Self) -> bool {
        self.MdlNext == other.MdlNext && self.MdlSize == other.MdlSize && self.MdlFlags == other.MdlFlags && self.Process == other.Process && self.lpMappedSystemVa == other.lpMappedSystemVa && self.lpStartVa == other.lpStartVa && self.ByteCount == other.ByteCount && self.ByteOffset == other.ByteOffset
    }
}
impl ::core::cmp::Eq for MDL {}
impl ::core::default::Default for MDL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MDL_0(pub u8);
impl ::core::marker::Copy for MDL_0 {}
impl ::core::clone::Clone for MDL_0 {
    fn clone(&self) -> Self {
        *self
    }
}
