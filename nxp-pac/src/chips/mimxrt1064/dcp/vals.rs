#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch0statClrErrorCode(u8);
impl Ch0statClrErrorCode {
    #[doc = "Error signalled because the next pointer is 0x00000000"]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error signalled because the semaphore is non-zero and neither chain bit is set"]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error signalled because an error is reported reading/writing the context buffer"]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error signalled because an error is reported reading/writing the payload"]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error signalled because the control packet specifies an invalid mode select (for instance, blit + hash)"]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch0statClrErrorCode {
    pub const fn from_bits(val: u8) -> Ch0statClrErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ch0statClrErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("NEXT_CHAIN_IS_0"),
            0x02 => f.write_str("NO_CHAIN"),
            0x03 => f.write_str("CONTEXT_ERROR"),
            0x04 => f.write_str("PAYLOAD_ERROR"),
            0x05 => f.write_str("INVALID_MODE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch0statClrErrorCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "NEXT_CHAIN_IS_0"),
            0x02 => defmt::write!(f, "NO_CHAIN"),
            0x03 => defmt::write!(f, "CONTEXT_ERROR"),
            0x04 => defmt::write!(f, "PAYLOAD_ERROR"),
            0x05 => defmt::write!(f, "INVALID_MODE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ch0statClrErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch0statClrErrorCode {
        Ch0statClrErrorCode::from_bits(val)
    }
}
impl From<Ch0statClrErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch0statClrErrorCode) -> u8 {
        Ch0statClrErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch0statErrorCode(u8);
impl Ch0statErrorCode {
    #[doc = "Error signalled because the next pointer is 0x00000000"]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error signalled because the semaphore is non-zero and neither chain bit is set"]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error signalled because an error is reported reading/writing the context buffer"]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error signalled because an error is reported reading/writing the payload"]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error signalled because the control packet specifies an invalid mode select (for instance, blit + hash)"]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch0statErrorCode {
    pub const fn from_bits(val: u8) -> Ch0statErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ch0statErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("NEXT_CHAIN_IS_0"),
            0x02 => f.write_str("NO_CHAIN"),
            0x03 => f.write_str("CONTEXT_ERROR"),
            0x04 => f.write_str("PAYLOAD_ERROR"),
            0x05 => f.write_str("INVALID_MODE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch0statErrorCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "NEXT_CHAIN_IS_0"),
            0x02 => defmt::write!(f, "NO_CHAIN"),
            0x03 => defmt::write!(f, "CONTEXT_ERROR"),
            0x04 => defmt::write!(f, "PAYLOAD_ERROR"),
            0x05 => defmt::write!(f, "INVALID_MODE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ch0statErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch0statErrorCode {
        Ch0statErrorCode::from_bits(val)
    }
}
impl From<Ch0statErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch0statErrorCode) -> u8 {
        Ch0statErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch0statSetErrorCode(u8);
impl Ch0statSetErrorCode {
    #[doc = "Error signalled because the next pointer is 0x00000000"]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error signalled because the semaphore is non-zero and neither chain bit is set"]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error signalled because an error is reported reading/writing the context buffer"]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error signalled because an error is reported reading/writing the payload"]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error signalled because the control packet specifies an invalid mode select (for instance, blit + hash)"]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch0statSetErrorCode {
    pub const fn from_bits(val: u8) -> Ch0statSetErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ch0statSetErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("NEXT_CHAIN_IS_0"),
            0x02 => f.write_str("NO_CHAIN"),
            0x03 => f.write_str("CONTEXT_ERROR"),
            0x04 => f.write_str("PAYLOAD_ERROR"),
            0x05 => f.write_str("INVALID_MODE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch0statSetErrorCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "NEXT_CHAIN_IS_0"),
            0x02 => defmt::write!(f, "NO_CHAIN"),
            0x03 => defmt::write!(f, "CONTEXT_ERROR"),
            0x04 => defmt::write!(f, "PAYLOAD_ERROR"),
            0x05 => defmt::write!(f, "INVALID_MODE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ch0statSetErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch0statSetErrorCode {
        Ch0statSetErrorCode::from_bits(val)
    }
}
impl From<Ch0statSetErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch0statSetErrorCode) -> u8 {
        Ch0statSetErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch0statTogErrorCode(u8);
impl Ch0statTogErrorCode {
    #[doc = "Error signalled because the next pointer is 0x00000000"]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error signalled because the semaphore is non-zero and neither chain bit is set"]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error signalled because an error is reported reading/writing the context buffer"]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error signalled because an error is reported reading/writing the payload"]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error signalled because the control packet specifies an invalid mode select (for instance, blit + hash)"]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch0statTogErrorCode {
    pub const fn from_bits(val: u8) -> Ch0statTogErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ch0statTogErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("NEXT_CHAIN_IS_0"),
            0x02 => f.write_str("NO_CHAIN"),
            0x03 => f.write_str("CONTEXT_ERROR"),
            0x04 => f.write_str("PAYLOAD_ERROR"),
            0x05 => f.write_str("INVALID_MODE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch0statTogErrorCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "NEXT_CHAIN_IS_0"),
            0x02 => defmt::write!(f, "NO_CHAIN"),
            0x03 => defmt::write!(f, "CONTEXT_ERROR"),
            0x04 => defmt::write!(f, "PAYLOAD_ERROR"),
            0x05 => defmt::write!(f, "INVALID_MODE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ch0statTogErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch0statTogErrorCode {
        Ch0statTogErrorCode::from_bits(val)
    }
}
impl From<Ch0statTogErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch0statTogErrorCode) -> u8 {
        Ch0statTogErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch1statClrErrorCode(u8);
impl Ch1statClrErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported when reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported when reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch1statClrErrorCode {
    pub const fn from_bits(val: u8) -> Ch1statClrErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ch1statClrErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("NEXT_CHAIN_IS_0"),
            0x02 => f.write_str("NO_CHAIN"),
            0x03 => f.write_str("CONTEXT_ERROR"),
            0x04 => f.write_str("PAYLOAD_ERROR"),
            0x05 => f.write_str("INVALID_MODE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch1statClrErrorCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "NEXT_CHAIN_IS_0"),
            0x02 => defmt::write!(f, "NO_CHAIN"),
            0x03 => defmt::write!(f, "CONTEXT_ERROR"),
            0x04 => defmt::write!(f, "PAYLOAD_ERROR"),
            0x05 => defmt::write!(f, "INVALID_MODE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ch1statClrErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch1statClrErrorCode {
        Ch1statClrErrorCode::from_bits(val)
    }
}
impl From<Ch1statClrErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch1statClrErrorCode) -> u8 {
        Ch1statClrErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch1statErrorCode(u8);
impl Ch1statErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported when reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported when reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch1statErrorCode {
    pub const fn from_bits(val: u8) -> Ch1statErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ch1statErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("NEXT_CHAIN_IS_0"),
            0x02 => f.write_str("NO_CHAIN"),
            0x03 => f.write_str("CONTEXT_ERROR"),
            0x04 => f.write_str("PAYLOAD_ERROR"),
            0x05 => f.write_str("INVALID_MODE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch1statErrorCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "NEXT_CHAIN_IS_0"),
            0x02 => defmt::write!(f, "NO_CHAIN"),
            0x03 => defmt::write!(f, "CONTEXT_ERROR"),
            0x04 => defmt::write!(f, "PAYLOAD_ERROR"),
            0x05 => defmt::write!(f, "INVALID_MODE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ch1statErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch1statErrorCode {
        Ch1statErrorCode::from_bits(val)
    }
}
impl From<Ch1statErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch1statErrorCode) -> u8 {
        Ch1statErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch1statSetErrorCode(u8);
impl Ch1statSetErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported when reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported when reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch1statSetErrorCode {
    pub const fn from_bits(val: u8) -> Ch1statSetErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ch1statSetErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("NEXT_CHAIN_IS_0"),
            0x02 => f.write_str("NO_CHAIN"),
            0x03 => f.write_str("CONTEXT_ERROR"),
            0x04 => f.write_str("PAYLOAD_ERROR"),
            0x05 => f.write_str("INVALID_MODE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch1statSetErrorCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "NEXT_CHAIN_IS_0"),
            0x02 => defmt::write!(f, "NO_CHAIN"),
            0x03 => defmt::write!(f, "CONTEXT_ERROR"),
            0x04 => defmt::write!(f, "PAYLOAD_ERROR"),
            0x05 => defmt::write!(f, "INVALID_MODE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ch1statSetErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch1statSetErrorCode {
        Ch1statSetErrorCode::from_bits(val)
    }
}
impl From<Ch1statSetErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch1statSetErrorCode) -> u8 {
        Ch1statSetErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch1statTogErrorCode(u8);
impl Ch1statTogErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported when reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported when reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch1statTogErrorCode {
    pub const fn from_bits(val: u8) -> Ch1statTogErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ch1statTogErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("NEXT_CHAIN_IS_0"),
            0x02 => f.write_str("NO_CHAIN"),
            0x03 => f.write_str("CONTEXT_ERROR"),
            0x04 => f.write_str("PAYLOAD_ERROR"),
            0x05 => f.write_str("INVALID_MODE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch1statTogErrorCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "NEXT_CHAIN_IS_0"),
            0x02 => defmt::write!(f, "NO_CHAIN"),
            0x03 => defmt::write!(f, "CONTEXT_ERROR"),
            0x04 => defmt::write!(f, "PAYLOAD_ERROR"),
            0x05 => defmt::write!(f, "INVALID_MODE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ch1statTogErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch1statTogErrorCode {
        Ch1statTogErrorCode::from_bits(val)
    }
}
impl From<Ch1statTogErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch1statTogErrorCode) -> u8 {
        Ch1statTogErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch2statClrErrorCode(u8);
impl Ch2statClrErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for instance, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch2statClrErrorCode {
    pub const fn from_bits(val: u8) -> Ch2statClrErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ch2statClrErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("NEXT_CHAIN_IS_0"),
            0x02 => f.write_str("NO_CHAIN"),
            0x03 => f.write_str("CONTEXT_ERROR"),
            0x04 => f.write_str("PAYLOAD_ERROR"),
            0x05 => f.write_str("INVALID_MODE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch2statClrErrorCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "NEXT_CHAIN_IS_0"),
            0x02 => defmt::write!(f, "NO_CHAIN"),
            0x03 => defmt::write!(f, "CONTEXT_ERROR"),
            0x04 => defmt::write!(f, "PAYLOAD_ERROR"),
            0x05 => defmt::write!(f, "INVALID_MODE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ch2statClrErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch2statClrErrorCode {
        Ch2statClrErrorCode::from_bits(val)
    }
}
impl From<Ch2statClrErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch2statClrErrorCode) -> u8 {
        Ch2statClrErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch2statErrorCode(u8);
impl Ch2statErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for instance, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch2statErrorCode {
    pub const fn from_bits(val: u8) -> Ch2statErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ch2statErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("NEXT_CHAIN_IS_0"),
            0x02 => f.write_str("NO_CHAIN"),
            0x03 => f.write_str("CONTEXT_ERROR"),
            0x04 => f.write_str("PAYLOAD_ERROR"),
            0x05 => f.write_str("INVALID_MODE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch2statErrorCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "NEXT_CHAIN_IS_0"),
            0x02 => defmt::write!(f, "NO_CHAIN"),
            0x03 => defmt::write!(f, "CONTEXT_ERROR"),
            0x04 => defmt::write!(f, "PAYLOAD_ERROR"),
            0x05 => defmt::write!(f, "INVALID_MODE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ch2statErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch2statErrorCode {
        Ch2statErrorCode::from_bits(val)
    }
}
impl From<Ch2statErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch2statErrorCode) -> u8 {
        Ch2statErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch2statSetErrorCode(u8);
impl Ch2statSetErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for instance, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch2statSetErrorCode {
    pub const fn from_bits(val: u8) -> Ch2statSetErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ch2statSetErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("NEXT_CHAIN_IS_0"),
            0x02 => f.write_str("NO_CHAIN"),
            0x03 => f.write_str("CONTEXT_ERROR"),
            0x04 => f.write_str("PAYLOAD_ERROR"),
            0x05 => f.write_str("INVALID_MODE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch2statSetErrorCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "NEXT_CHAIN_IS_0"),
            0x02 => defmt::write!(f, "NO_CHAIN"),
            0x03 => defmt::write!(f, "CONTEXT_ERROR"),
            0x04 => defmt::write!(f, "PAYLOAD_ERROR"),
            0x05 => defmt::write!(f, "INVALID_MODE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ch2statSetErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch2statSetErrorCode {
        Ch2statSetErrorCode::from_bits(val)
    }
}
impl From<Ch2statSetErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch2statSetErrorCode) -> u8 {
        Ch2statSetErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch2statTogErrorCode(u8);
impl Ch2statTogErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for instance, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch2statTogErrorCode {
    pub const fn from_bits(val: u8) -> Ch2statTogErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ch2statTogErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("NEXT_CHAIN_IS_0"),
            0x02 => f.write_str("NO_CHAIN"),
            0x03 => f.write_str("CONTEXT_ERROR"),
            0x04 => f.write_str("PAYLOAD_ERROR"),
            0x05 => f.write_str("INVALID_MODE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch2statTogErrorCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "NEXT_CHAIN_IS_0"),
            0x02 => defmt::write!(f, "NO_CHAIN"),
            0x03 => defmt::write!(f, "CONTEXT_ERROR"),
            0x04 => defmt::write!(f, "PAYLOAD_ERROR"),
            0x05 => defmt::write!(f, "INVALID_MODE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ch2statTogErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch2statTogErrorCode {
        Ch2statTogErrorCode::from_bits(val)
    }
}
impl From<Ch2statTogErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch2statTogErrorCode) -> u8 {
        Ch2statTogErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch3statClrErrorCode(u8);
impl Ch3statClrErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch3statClrErrorCode {
    pub const fn from_bits(val: u8) -> Ch3statClrErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ch3statClrErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("NEXT_CHAIN_IS_0"),
            0x02 => f.write_str("NO_CHAIN"),
            0x03 => f.write_str("CONTEXT_ERROR"),
            0x04 => f.write_str("PAYLOAD_ERROR"),
            0x05 => f.write_str("INVALID_MODE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch3statClrErrorCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "NEXT_CHAIN_IS_0"),
            0x02 => defmt::write!(f, "NO_CHAIN"),
            0x03 => defmt::write!(f, "CONTEXT_ERROR"),
            0x04 => defmt::write!(f, "PAYLOAD_ERROR"),
            0x05 => defmt::write!(f, "INVALID_MODE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ch3statClrErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch3statClrErrorCode {
        Ch3statClrErrorCode::from_bits(val)
    }
}
impl From<Ch3statClrErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch3statClrErrorCode) -> u8 {
        Ch3statClrErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch3statErrorCode(u8);
impl Ch3statErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch3statErrorCode {
    pub const fn from_bits(val: u8) -> Ch3statErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ch3statErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("NEXT_CHAIN_IS_0"),
            0x02 => f.write_str("NO_CHAIN"),
            0x03 => f.write_str("CONTEXT_ERROR"),
            0x04 => f.write_str("PAYLOAD_ERROR"),
            0x05 => f.write_str("INVALID_MODE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch3statErrorCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "NEXT_CHAIN_IS_0"),
            0x02 => defmt::write!(f, "NO_CHAIN"),
            0x03 => defmt::write!(f, "CONTEXT_ERROR"),
            0x04 => defmt::write!(f, "PAYLOAD_ERROR"),
            0x05 => defmt::write!(f, "INVALID_MODE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ch3statErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch3statErrorCode {
        Ch3statErrorCode::from_bits(val)
    }
}
impl From<Ch3statErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch3statErrorCode) -> u8 {
        Ch3statErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch3statSetErrorCode(u8);
impl Ch3statSetErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch3statSetErrorCode {
    pub const fn from_bits(val: u8) -> Ch3statSetErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ch3statSetErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("NEXT_CHAIN_IS_0"),
            0x02 => f.write_str("NO_CHAIN"),
            0x03 => f.write_str("CONTEXT_ERROR"),
            0x04 => f.write_str("PAYLOAD_ERROR"),
            0x05 => f.write_str("INVALID_MODE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch3statSetErrorCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "NEXT_CHAIN_IS_0"),
            0x02 => defmt::write!(f, "NO_CHAIN"),
            0x03 => defmt::write!(f, "CONTEXT_ERROR"),
            0x04 => defmt::write!(f, "PAYLOAD_ERROR"),
            0x05 => defmt::write!(f, "INVALID_MODE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ch3statSetErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch3statSetErrorCode {
        Ch3statSetErrorCode::from_bits(val)
    }
}
impl From<Ch3statSetErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch3statSetErrorCode) -> u8 {
        Ch3statSetErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch3statTogErrorCode(u8);
impl Ch3statTogErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch3statTogErrorCode {
    pub const fn from_bits(val: u8) -> Ch3statTogErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ch3statTogErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("NEXT_CHAIN_IS_0"),
            0x02 => f.write_str("NO_CHAIN"),
            0x03 => f.write_str("CONTEXT_ERROR"),
            0x04 => f.write_str("PAYLOAD_ERROR"),
            0x05 => f.write_str("INVALID_MODE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch3statTogErrorCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "NEXT_CHAIN_IS_0"),
            0x02 => defmt::write!(f, "NO_CHAIN"),
            0x03 => defmt::write!(f, "CONTEXT_ERROR"),
            0x04 => defmt::write!(f, "PAYLOAD_ERROR"),
            0x05 => defmt::write!(f, "INVALID_MODE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ch3statTogErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch3statTogErrorCode {
        Ch3statTogErrorCode::from_bits(val)
    }
}
impl From<Ch3statTogErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch3statTogErrorCode) -> u8 {
        Ch3statTogErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChannelctrlClrEnableChannel(u8);
impl ChannelctrlClrEnableChannel {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl ChannelctrlClrEnableChannel {
    pub const fn from_bits(val: u8) -> ChannelctrlClrEnableChannel {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for ChannelctrlClrEnableChannel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("CH0"),
            0x02 => f.write_str("CH1"),
            0x04 => f.write_str("CH2"),
            0x08 => f.write_str("CH3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChannelctrlClrEnableChannel {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "CH0"),
            0x02 => defmt::write!(f, "CH1"),
            0x04 => defmt::write!(f, "CH2"),
            0x08 => defmt::write!(f, "CH3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for ChannelctrlClrEnableChannel {
    #[inline(always)]
    fn from(val: u8) -> ChannelctrlClrEnableChannel {
        ChannelctrlClrEnableChannel::from_bits(val)
    }
}
impl From<ChannelctrlClrEnableChannel> for u8 {
    #[inline(always)]
    fn from(val: ChannelctrlClrEnableChannel) -> u8 {
        ChannelctrlClrEnableChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChannelctrlClrHighPriorityChannel(u8);
impl ChannelctrlClrHighPriorityChannel {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl ChannelctrlClrHighPriorityChannel {
    pub const fn from_bits(val: u8) -> ChannelctrlClrHighPriorityChannel {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for ChannelctrlClrHighPriorityChannel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("CH0"),
            0x02 => f.write_str("CH1"),
            0x04 => f.write_str("CH2"),
            0x08 => f.write_str("CH3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChannelctrlClrHighPriorityChannel {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "CH0"),
            0x02 => defmt::write!(f, "CH1"),
            0x04 => defmt::write!(f, "CH2"),
            0x08 => defmt::write!(f, "CH3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for ChannelctrlClrHighPriorityChannel {
    #[inline(always)]
    fn from(val: u8) -> ChannelctrlClrHighPriorityChannel {
        ChannelctrlClrHighPriorityChannel::from_bits(val)
    }
}
impl From<ChannelctrlClrHighPriorityChannel> for u8 {
    #[inline(always)]
    fn from(val: ChannelctrlClrHighPriorityChannel) -> u8 {
        ChannelctrlClrHighPriorityChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChannelctrlEnableChannel(u8);
impl ChannelctrlEnableChannel {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl ChannelctrlEnableChannel {
    pub const fn from_bits(val: u8) -> ChannelctrlEnableChannel {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for ChannelctrlEnableChannel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("CH0"),
            0x02 => f.write_str("CH1"),
            0x04 => f.write_str("CH2"),
            0x08 => f.write_str("CH3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChannelctrlEnableChannel {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "CH0"),
            0x02 => defmt::write!(f, "CH1"),
            0x04 => defmt::write!(f, "CH2"),
            0x08 => defmt::write!(f, "CH3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for ChannelctrlEnableChannel {
    #[inline(always)]
    fn from(val: u8) -> ChannelctrlEnableChannel {
        ChannelctrlEnableChannel::from_bits(val)
    }
}
impl From<ChannelctrlEnableChannel> for u8 {
    #[inline(always)]
    fn from(val: ChannelctrlEnableChannel) -> u8 {
        ChannelctrlEnableChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChannelctrlHighPriorityChannel(u8);
impl ChannelctrlHighPriorityChannel {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl ChannelctrlHighPriorityChannel {
    pub const fn from_bits(val: u8) -> ChannelctrlHighPriorityChannel {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for ChannelctrlHighPriorityChannel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("CH0"),
            0x02 => f.write_str("CH1"),
            0x04 => f.write_str("CH2"),
            0x08 => f.write_str("CH3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChannelctrlHighPriorityChannel {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "CH0"),
            0x02 => defmt::write!(f, "CH1"),
            0x04 => defmt::write!(f, "CH2"),
            0x08 => defmt::write!(f, "CH3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for ChannelctrlHighPriorityChannel {
    #[inline(always)]
    fn from(val: u8) -> ChannelctrlHighPriorityChannel {
        ChannelctrlHighPriorityChannel::from_bits(val)
    }
}
impl From<ChannelctrlHighPriorityChannel> for u8 {
    #[inline(always)]
    fn from(val: ChannelctrlHighPriorityChannel) -> u8 {
        ChannelctrlHighPriorityChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChannelctrlSetEnableChannel(u8);
impl ChannelctrlSetEnableChannel {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl ChannelctrlSetEnableChannel {
    pub const fn from_bits(val: u8) -> ChannelctrlSetEnableChannel {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for ChannelctrlSetEnableChannel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("CH0"),
            0x02 => f.write_str("CH1"),
            0x04 => f.write_str("CH2"),
            0x08 => f.write_str("CH3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChannelctrlSetEnableChannel {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "CH0"),
            0x02 => defmt::write!(f, "CH1"),
            0x04 => defmt::write!(f, "CH2"),
            0x08 => defmt::write!(f, "CH3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for ChannelctrlSetEnableChannel {
    #[inline(always)]
    fn from(val: u8) -> ChannelctrlSetEnableChannel {
        ChannelctrlSetEnableChannel::from_bits(val)
    }
}
impl From<ChannelctrlSetEnableChannel> for u8 {
    #[inline(always)]
    fn from(val: ChannelctrlSetEnableChannel) -> u8 {
        ChannelctrlSetEnableChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChannelctrlSetHighPriorityChannel(u8);
impl ChannelctrlSetHighPriorityChannel {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl ChannelctrlSetHighPriorityChannel {
    pub const fn from_bits(val: u8) -> ChannelctrlSetHighPriorityChannel {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for ChannelctrlSetHighPriorityChannel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("CH0"),
            0x02 => f.write_str("CH1"),
            0x04 => f.write_str("CH2"),
            0x08 => f.write_str("CH3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChannelctrlSetHighPriorityChannel {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "CH0"),
            0x02 => defmt::write!(f, "CH1"),
            0x04 => defmt::write!(f, "CH2"),
            0x08 => defmt::write!(f, "CH3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for ChannelctrlSetHighPriorityChannel {
    #[inline(always)]
    fn from(val: u8) -> ChannelctrlSetHighPriorityChannel {
        ChannelctrlSetHighPriorityChannel::from_bits(val)
    }
}
impl From<ChannelctrlSetHighPriorityChannel> for u8 {
    #[inline(always)]
    fn from(val: ChannelctrlSetHighPriorityChannel) -> u8 {
        ChannelctrlSetHighPriorityChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChannelctrlTogEnableChannel(u8);
impl ChannelctrlTogEnableChannel {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl ChannelctrlTogEnableChannel {
    pub const fn from_bits(val: u8) -> ChannelctrlTogEnableChannel {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for ChannelctrlTogEnableChannel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("CH0"),
            0x02 => f.write_str("CH1"),
            0x04 => f.write_str("CH2"),
            0x08 => f.write_str("CH3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChannelctrlTogEnableChannel {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "CH0"),
            0x02 => defmt::write!(f, "CH1"),
            0x04 => defmt::write!(f, "CH2"),
            0x08 => defmt::write!(f, "CH3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for ChannelctrlTogEnableChannel {
    #[inline(always)]
    fn from(val: u8) -> ChannelctrlTogEnableChannel {
        ChannelctrlTogEnableChannel::from_bits(val)
    }
}
impl From<ChannelctrlTogEnableChannel> for u8 {
    #[inline(always)]
    fn from(val: ChannelctrlTogEnableChannel) -> u8 {
        ChannelctrlTogEnableChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChannelctrlTogHighPriorityChannel(u8);
impl ChannelctrlTogHighPriorityChannel {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl ChannelctrlTogHighPriorityChannel {
    pub const fn from_bits(val: u8) -> ChannelctrlTogHighPriorityChannel {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for ChannelctrlTogHighPriorityChannel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("CH0"),
            0x02 => f.write_str("CH1"),
            0x04 => f.write_str("CH2"),
            0x08 => f.write_str("CH3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChannelctrlTogHighPriorityChannel {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "CH0"),
            0x02 => defmt::write!(f, "CH1"),
            0x04 => defmt::write!(f, "CH2"),
            0x08 => defmt::write!(f, "CH3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for ChannelctrlTogHighPriorityChannel {
    #[inline(always)]
    fn from(val: u8) -> ChannelctrlTogHighPriorityChannel {
        ChannelctrlTogHighPriorityChannel::from_bits(val)
    }
}
impl From<ChannelctrlTogHighPriorityChannel> for u8 {
    #[inline(always)]
    fn from(val: ChannelctrlTogHighPriorityChannel) -> u8 {
        ChannelctrlTogHighPriorityChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CipherAlgorithms(u16);
impl CipherAlgorithms {
    #[doc = "AES128"]
    pub const AES128: Self = Self(0x01);
}
impl CipherAlgorithms {
    pub const fn from_bits(val: u16) -> CipherAlgorithms {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for CipherAlgorithms {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("AES128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CipherAlgorithms {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "AES128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for CipherAlgorithms {
    #[inline(always)]
    fn from(val: u16) -> CipherAlgorithms {
        CipherAlgorithms::from_bits(val)
    }
}
impl From<CipherAlgorithms> for u16 {
    #[inline(always)]
    fn from(val: CipherAlgorithms) -> u16 {
        CipherAlgorithms::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CipherEncrypt {
    #[doc = "DECRYPT"]
    DECRYPT = 0x0,
    #[doc = "ENCRYPT"]
    ENCRYPT = 0x01,
}
impl CipherEncrypt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CipherEncrypt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CipherEncrypt {
    #[inline(always)]
    fn from(val: u8) -> CipherEncrypt {
        CipherEncrypt::from_bits(val)
    }
}
impl From<CipherEncrypt> for u8 {
    #[inline(always)]
    fn from(val: CipherEncrypt) -> u8 {
        CipherEncrypt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CipherMode {
    #[doc = "ECB"]
    ECB = 0x0,
    #[doc = "CBC"]
    CBC = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl CipherMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CipherMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CipherMode {
    #[inline(always)]
    fn from(val: u8) -> CipherMode {
        CipherMode::from_bits(val)
    }
}
impl From<CipherMode> for u8 {
    #[inline(always)]
    fn from(val: CipherMode) -> u8 {
        CipherMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CipherSelect {
    #[doc = "AES128"]
    AES128 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl CipherSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CipherSelect {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CipherSelect {
    #[inline(always)]
    fn from(val: u8) -> CipherSelect {
        CipherSelect::from_bits(val)
    }
}
impl From<CipherSelect> for u8 {
    #[inline(always)]
    fn from(val: CipherSelect) -> u8 {
        CipherSelect::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CtrlChannelInterruptEnable(u8);
impl CtrlChannelInterruptEnable {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl CtrlChannelInterruptEnable {
    pub const fn from_bits(val: u8) -> CtrlChannelInterruptEnable {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CtrlChannelInterruptEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("CH0"),
            0x02 => f.write_str("CH1"),
            0x04 => f.write_str("CH2"),
            0x08 => f.write_str("CH3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlChannelInterruptEnable {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "CH0"),
            0x02 => defmt::write!(f, "CH1"),
            0x04 => defmt::write!(f, "CH2"),
            0x08 => defmt::write!(f, "CH3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CtrlChannelInterruptEnable {
    #[inline(always)]
    fn from(val: u8) -> CtrlChannelInterruptEnable {
        CtrlChannelInterruptEnable::from_bits(val)
    }
}
impl From<CtrlChannelInterruptEnable> for u8 {
    #[inline(always)]
    fn from(val: CtrlChannelInterruptEnable) -> u8 {
        CtrlChannelInterruptEnable::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CtrlClrChannelInterruptEnable(u8);
impl CtrlClrChannelInterruptEnable {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl CtrlClrChannelInterruptEnable {
    pub const fn from_bits(val: u8) -> CtrlClrChannelInterruptEnable {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CtrlClrChannelInterruptEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("CH0"),
            0x02 => f.write_str("CH1"),
            0x04 => f.write_str("CH2"),
            0x08 => f.write_str("CH3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlClrChannelInterruptEnable {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "CH0"),
            0x02 => defmt::write!(f, "CH1"),
            0x04 => defmt::write!(f, "CH2"),
            0x08 => defmt::write!(f, "CH3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CtrlClrChannelInterruptEnable {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrChannelInterruptEnable {
        CtrlClrChannelInterruptEnable::from_bits(val)
    }
}
impl From<CtrlClrChannelInterruptEnable> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrChannelInterruptEnable) -> u8 {
        CtrlClrChannelInterruptEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClrPresentCrypto {
    #[doc = "Absent"]
    ABSENT = 0x0,
    #[doc = "Present"]
    PRESENT = 0x01,
}
impl CtrlClrPresentCrypto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlClrPresentCrypto {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlClrPresentCrypto {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrPresentCrypto {
        CtrlClrPresentCrypto::from_bits(val)
    }
}
impl From<CtrlClrPresentCrypto> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrPresentCrypto) -> u8 {
        CtrlClrPresentCrypto::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClrPresentSha {
    #[doc = "Absent"]
    ABSENT = 0x0,
    #[doc = "Present"]
    PRESENT = 0x01,
}
impl CtrlClrPresentSha {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlClrPresentSha {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlClrPresentSha {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrPresentSha {
        CtrlClrPresentSha::from_bits(val)
    }
}
impl From<CtrlClrPresentSha> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrPresentSha) -> u8 {
        CtrlClrPresentSha::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlPresentCrypto {
    #[doc = "Absent"]
    ABSENT = 0x0,
    #[doc = "Present"]
    PRESENT = 0x01,
}
impl CtrlPresentCrypto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlPresentCrypto {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlPresentCrypto {
    #[inline(always)]
    fn from(val: u8) -> CtrlPresentCrypto {
        CtrlPresentCrypto::from_bits(val)
    }
}
impl From<CtrlPresentCrypto> for u8 {
    #[inline(always)]
    fn from(val: CtrlPresentCrypto) -> u8 {
        CtrlPresentCrypto::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlPresentSha {
    #[doc = "Absent"]
    ABSENT = 0x0,
    #[doc = "Present"]
    PRESENT = 0x01,
}
impl CtrlPresentSha {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlPresentSha {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlPresentSha {
    #[inline(always)]
    fn from(val: u8) -> CtrlPresentSha {
        CtrlPresentSha::from_bits(val)
    }
}
impl From<CtrlPresentSha> for u8 {
    #[inline(always)]
    fn from(val: CtrlPresentSha) -> u8 {
        CtrlPresentSha::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CtrlSetChannelInterruptEnable(u8);
impl CtrlSetChannelInterruptEnable {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl CtrlSetChannelInterruptEnable {
    pub const fn from_bits(val: u8) -> CtrlSetChannelInterruptEnable {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CtrlSetChannelInterruptEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("CH0"),
            0x02 => f.write_str("CH1"),
            0x04 => f.write_str("CH2"),
            0x08 => f.write_str("CH3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlSetChannelInterruptEnable {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "CH0"),
            0x02 => defmt::write!(f, "CH1"),
            0x04 => defmt::write!(f, "CH2"),
            0x08 => defmt::write!(f, "CH3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CtrlSetChannelInterruptEnable {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetChannelInterruptEnable {
        CtrlSetChannelInterruptEnable::from_bits(val)
    }
}
impl From<CtrlSetChannelInterruptEnable> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetChannelInterruptEnable) -> u8 {
        CtrlSetChannelInterruptEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlSetPresentCrypto {
    #[doc = "Absent"]
    ABSENT = 0x0,
    #[doc = "Present"]
    PRESENT = 0x01,
}
impl CtrlSetPresentCrypto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlSetPresentCrypto {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlSetPresentCrypto {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetPresentCrypto {
        CtrlSetPresentCrypto::from_bits(val)
    }
}
impl From<CtrlSetPresentCrypto> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetPresentCrypto) -> u8 {
        CtrlSetPresentCrypto::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlSetPresentSha {
    #[doc = "Absent"]
    ABSENT = 0x0,
    #[doc = "Present"]
    PRESENT = 0x01,
}
impl CtrlSetPresentSha {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlSetPresentSha {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlSetPresentSha {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetPresentSha {
        CtrlSetPresentSha::from_bits(val)
    }
}
impl From<CtrlSetPresentSha> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetPresentSha) -> u8 {
        CtrlSetPresentSha::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CtrlTogChannelInterruptEnable(u8);
impl CtrlTogChannelInterruptEnable {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl CtrlTogChannelInterruptEnable {
    pub const fn from_bits(val: u8) -> CtrlTogChannelInterruptEnable {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for CtrlTogChannelInterruptEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("CH0"),
            0x02 => f.write_str("CH1"),
            0x04 => f.write_str("CH2"),
            0x08 => f.write_str("CH3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlTogChannelInterruptEnable {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "CH0"),
            0x02 => defmt::write!(f, "CH1"),
            0x04 => defmt::write!(f, "CH2"),
            0x08 => defmt::write!(f, "CH3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for CtrlTogChannelInterruptEnable {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogChannelInterruptEnable {
        CtrlTogChannelInterruptEnable::from_bits(val)
    }
}
impl From<CtrlTogChannelInterruptEnable> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogChannelInterruptEnable) -> u8 {
        CtrlTogChannelInterruptEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlTogPresentCrypto {
    #[doc = "Absent"]
    ABSENT = 0x0,
    #[doc = "Present"]
    PRESENT = 0x01,
}
impl CtrlTogPresentCrypto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlTogPresentCrypto {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlTogPresentCrypto {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogPresentCrypto {
        CtrlTogPresentCrypto::from_bits(val)
    }
}
impl From<CtrlTogPresentCrypto> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogPresentCrypto) -> u8 {
        CtrlTogPresentCrypto::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlTogPresentSha {
    #[doc = "Absent"]
    ABSENT = 0x0,
    #[doc = "Present"]
    PRESENT = 0x01,
}
impl CtrlTogPresentSha {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlTogPresentSha {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlTogPresentSha {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogPresentSha {
        CtrlTogPresentSha::from_bits(val)
    }
}
impl From<CtrlTogPresentSha> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogPresentSha) -> u8 {
        CtrlTogPresentSha::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct HashAlgorithms(u16);
impl HashAlgorithms {
    #[doc = "SHA1"]
    pub const SHA1: Self = Self(0x01);
    #[doc = "CRC32"]
    pub const CRC32: Self = Self(0x02);
    #[doc = "SHA256"]
    pub const SHA256: Self = Self(0x04);
}
impl HashAlgorithms {
    pub const fn from_bits(val: u16) -> HashAlgorithms {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for HashAlgorithms {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("SHA1"),
            0x02 => f.write_str("CRC32"),
            0x04 => f.write_str("SHA256"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashAlgorithms {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "SHA1"),
            0x02 => defmt::write!(f, "CRC32"),
            0x04 => defmt::write!(f, "SHA256"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for HashAlgorithms {
    #[inline(always)]
    fn from(val: u16) -> HashAlgorithms {
        HashAlgorithms::from_bits(val)
    }
}
impl From<HashAlgorithms> for u16 {
    #[inline(always)]
    fn from(val: HashAlgorithms) -> u16 {
        HashAlgorithms::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HashOutput {
    #[doc = "INPUT"]
    INPUT = 0x0,
    #[doc = "OUTPUT"]
    OUTPUT = 0x01,
}
impl HashOutput {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HashOutput {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HashOutput {
    #[inline(always)]
    fn from(val: u8) -> HashOutput {
        HashOutput::from_bits(val)
    }
}
impl From<HashOutput> for u8 {
    #[inline(always)]
    fn from(val: HashOutput) -> u8 {
        HashOutput::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HashSelect {
    #[doc = "SHA1"]
    SHA1 = 0x0,
    #[doc = "CRC32"]
    CRC32 = 0x01,
    #[doc = "SHA256"]
    SHA256 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl HashSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HashSelect {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HashSelect {
    #[inline(always)]
    fn from(val: u8) -> HashSelect {
        HashSelect::from_bits(val)
    }
}
impl From<HashSelect> for u8 {
    #[inline(always)]
    fn from(val: HashSelect) -> u8 {
        HashSelect::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Index(u8);
impl Index {
    #[doc = "CONTROL"]
    pub const CONTROL: Self = Self(0x01);
    #[doc = "OTPKEY0"]
    pub const OTPKEY0: Self = Self(0x10);
    #[doc = "OTPKEY1"]
    pub const OTPKEY1: Self = Self(0x11);
    #[doc = "OTPKEY2"]
    pub const OTPKEY2: Self = Self(0x12);
    #[doc = "OTPKEY3"]
    pub const OTPKEY3: Self = Self(0x13);
}
impl Index {
    pub const fn from_bits(val: u8) -> Index {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Index {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("CONTROL"),
            0x10 => f.write_str("OTPKEY0"),
            0x11 => f.write_str("OTPKEY1"),
            0x12 => f.write_str("OTPKEY2"),
            0x13 => f.write_str("OTPKEY3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Index {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "CONTROL"),
            0x10 => defmt::write!(f, "OTPKEY0"),
            0x11 => defmt::write!(f, "OTPKEY1"),
            0x12 => defmt::write!(f, "OTPKEY2"),
            0x13 => defmt::write!(f, "OTPKEY3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Index {
    #[inline(always)]
    fn from(val: u8) -> Index {
        Index::from_bits(val)
    }
}
impl From<Index> for u8 {
    #[inline(always)]
    fn from(val: Index) -> u8 {
        Index::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct KeySelect(u8);
impl KeySelect {
    #[doc = "KEY0"]
    pub const KEY0: Self = Self(0x0);
    #[doc = "KEY1"]
    pub const KEY1: Self = Self(0x01);
    #[doc = "KEY2"]
    pub const KEY2: Self = Self(0x02);
    #[doc = "KEY3"]
    pub const KEY3: Self = Self(0x03);
    #[doc = "UNIQUE_KEY"]
    pub const UNIQUE_KEY: Self = Self(0xfe);
    #[doc = "OTP_KEY"]
    pub const OTP_KEY: Self = Self(0xff);
}
impl KeySelect {
    pub const fn from_bits(val: u8) -> KeySelect {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for KeySelect {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("KEY0"),
            0x01 => f.write_str("KEY1"),
            0x02 => f.write_str("KEY2"),
            0x03 => f.write_str("KEY3"),
            0xfe => f.write_str("UNIQUE_KEY"),
            0xff => f.write_str("OTP_KEY"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KeySelect {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "KEY0"),
            0x01 => defmt::write!(f, "KEY1"),
            0x02 => defmt::write!(f, "KEY2"),
            0x03 => defmt::write!(f, "KEY3"),
            0xfe => defmt::write!(f, "UNIQUE_KEY"),
            0xff => defmt::write!(f, "OTP_KEY"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for KeySelect {
    #[inline(always)]
    fn from(val: u8) -> KeySelect {
        KeySelect::from_bits(val)
    }
}
impl From<KeySelect> for u8 {
    #[inline(always)]
    fn from(val: KeySelect) -> u8 {
        KeySelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatClrCurChannel {
    #[doc = "None"]
    NONE = 0x0,
    #[doc = "CH0"]
    CH0 = 0x01,
    #[doc = "CH1"]
    CH1 = 0x02,
    #[doc = "CH2"]
    CH2 = 0x03,
    #[doc = "CH3"]
    CH3 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl StatClrCurChannel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatClrCurChannel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatClrCurChannel {
    #[inline(always)]
    fn from(val: u8) -> StatClrCurChannel {
        StatClrCurChannel::from_bits(val)
    }
}
impl From<StatClrCurChannel> for u8 {
    #[inline(always)]
    fn from(val: StatClrCurChannel) -> u8 {
        StatClrCurChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct StatClrReadyChannels(u8);
impl StatClrReadyChannels {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl StatClrReadyChannels {
    pub const fn from_bits(val: u8) -> StatClrReadyChannels {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for StatClrReadyChannels {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("CH0"),
            0x02 => f.write_str("CH1"),
            0x04 => f.write_str("CH2"),
            0x08 => f.write_str("CH3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for StatClrReadyChannels {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "CH0"),
            0x02 => defmt::write!(f, "CH1"),
            0x04 => defmt::write!(f, "CH2"),
            0x08 => defmt::write!(f, "CH3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for StatClrReadyChannels {
    #[inline(always)]
    fn from(val: u8) -> StatClrReadyChannels {
        StatClrReadyChannels::from_bits(val)
    }
}
impl From<StatClrReadyChannels> for u8 {
    #[inline(always)]
    fn from(val: StatClrReadyChannels) -> u8 {
        StatClrReadyChannels::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatCurChannel {
    #[doc = "None"]
    NONE = 0x0,
    #[doc = "CH0"]
    CH0 = 0x01,
    #[doc = "CH1"]
    CH1 = 0x02,
    #[doc = "CH2"]
    CH2 = 0x03,
    #[doc = "CH3"]
    CH3 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl StatCurChannel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatCurChannel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatCurChannel {
    #[inline(always)]
    fn from(val: u8) -> StatCurChannel {
        StatCurChannel::from_bits(val)
    }
}
impl From<StatCurChannel> for u8 {
    #[inline(always)]
    fn from(val: StatCurChannel) -> u8 {
        StatCurChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct StatReadyChannels(u8);
impl StatReadyChannels {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl StatReadyChannels {
    pub const fn from_bits(val: u8) -> StatReadyChannels {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for StatReadyChannels {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("CH0"),
            0x02 => f.write_str("CH1"),
            0x04 => f.write_str("CH2"),
            0x08 => f.write_str("CH3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for StatReadyChannels {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "CH0"),
            0x02 => defmt::write!(f, "CH1"),
            0x04 => defmt::write!(f, "CH2"),
            0x08 => defmt::write!(f, "CH3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for StatReadyChannels {
    #[inline(always)]
    fn from(val: u8) -> StatReadyChannels {
        StatReadyChannels::from_bits(val)
    }
}
impl From<StatReadyChannels> for u8 {
    #[inline(always)]
    fn from(val: StatReadyChannels) -> u8 {
        StatReadyChannels::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatSetCurChannel {
    #[doc = "None"]
    NONE = 0x0,
    #[doc = "CH0"]
    CH0 = 0x01,
    #[doc = "CH1"]
    CH1 = 0x02,
    #[doc = "CH2"]
    CH2 = 0x03,
    #[doc = "CH3"]
    CH3 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl StatSetCurChannel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatSetCurChannel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatSetCurChannel {
    #[inline(always)]
    fn from(val: u8) -> StatSetCurChannel {
        StatSetCurChannel::from_bits(val)
    }
}
impl From<StatSetCurChannel> for u8 {
    #[inline(always)]
    fn from(val: StatSetCurChannel) -> u8 {
        StatSetCurChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct StatSetReadyChannels(u8);
impl StatSetReadyChannels {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl StatSetReadyChannels {
    pub const fn from_bits(val: u8) -> StatSetReadyChannels {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for StatSetReadyChannels {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("CH0"),
            0x02 => f.write_str("CH1"),
            0x04 => f.write_str("CH2"),
            0x08 => f.write_str("CH3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for StatSetReadyChannels {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "CH0"),
            0x02 => defmt::write!(f, "CH1"),
            0x04 => defmt::write!(f, "CH2"),
            0x08 => defmt::write!(f, "CH3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for StatSetReadyChannels {
    #[inline(always)]
    fn from(val: u8) -> StatSetReadyChannels {
        StatSetReadyChannels::from_bits(val)
    }
}
impl From<StatSetReadyChannels> for u8 {
    #[inline(always)]
    fn from(val: StatSetReadyChannels) -> u8 {
        StatSetReadyChannels::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatTogCurChannel {
    #[doc = "None"]
    NONE = 0x0,
    #[doc = "CH0"]
    CH0 = 0x01,
    #[doc = "CH1"]
    CH1 = 0x02,
    #[doc = "CH2"]
    CH2 = 0x03,
    #[doc = "CH3"]
    CH3 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl StatTogCurChannel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatTogCurChannel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatTogCurChannel {
    #[inline(always)]
    fn from(val: u8) -> StatTogCurChannel {
        StatTogCurChannel::from_bits(val)
    }
}
impl From<StatTogCurChannel> for u8 {
    #[inline(always)]
    fn from(val: StatTogCurChannel) -> u8 {
        StatTogCurChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct StatTogReadyChannels(u8);
impl StatTogReadyChannels {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl StatTogReadyChannels {
    pub const fn from_bits(val: u8) -> StatTogReadyChannels {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for StatTogReadyChannels {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("CH0"),
            0x02 => f.write_str("CH1"),
            0x04 => f.write_str("CH2"),
            0x08 => f.write_str("CH3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for StatTogReadyChannels {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "CH0"),
            0x02 => defmt::write!(f, "CH1"),
            0x04 => defmt::write!(f, "CH2"),
            0x08 => defmt::write!(f, "CH3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for StatTogReadyChannels {
    #[inline(always)]
    fn from(val: u8) -> StatTogReadyChannels {
        StatTogReadyChannels::from_bits(val)
    }
}
impl From<StatTogReadyChannels> for u8 {
    #[inline(always)]
    fn from(val: StatTogReadyChannels) -> u8 {
        StatTogReadyChannels::to_bits(val)
    }
}
