#[doc = "FLEXIO"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexio0 {
    ptr: *mut u8,
}
unsafe impl Send for Flexio0 {}
unsafe impl Sync for Flexio0 {}
impl Flexio0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID"]
    #[inline(always)]
    pub const fn verid(self) -> crate::common::Reg<regs::Verid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Parameter"]
    #[inline(always)]
    pub const fn param(self) -> crate::common::Reg<regs::Param, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "FLEXIO Control"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Pin State"]
    #[inline(always)]
    pub const fn pin(self) -> crate::common::Reg<regs::Pin, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Shifter Status"]
    #[inline(always)]
    pub const fn shiftstat(self) -> crate::common::Reg<regs::Shiftstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Shifter Error"]
    #[inline(always)]
    pub const fn shifterr(self) -> crate::common::Reg<regs::Shifterr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Timer Status Flag"]
    #[inline(always)]
    pub const fn timstat(self) -> crate::common::Reg<regs::Timstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Shifter Status Interrupt Enable"]
    #[inline(always)]
    pub const fn shiftsien(self) -> crate::common::Reg<regs::Shiftsien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Shifter Error Interrupt Enable"]
    #[inline(always)]
    pub const fn shifteien(self) -> crate::common::Reg<regs::Shifteien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Timer Interrupt Enable"]
    #[inline(always)]
    pub const fn timien(self) -> crate::common::Reg<regs::Timien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Shifter Status DMA Enable"]
    #[inline(always)]
    pub const fn shiftsden(self) -> crate::common::Reg<regs::Shiftsden, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Timer Status DMA Enable"]
    #[inline(always)]
    pub const fn timersden(self) -> crate::common::Reg<regs::Timersden, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Shifter State"]
    #[inline(always)]
    pub const fn shiftstate(self) -> crate::common::Reg<regs::Shiftstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Trigger Status"]
    #[inline(always)]
    pub const fn trgstat(self) -> crate::common::Reg<regs::Trgstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "External Trigger Interrupt Enable"]
    #[inline(always)]
    pub const fn trigien(self) -> crate::common::Reg<regs::Trigien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Pin Status"]
    #[inline(always)]
    pub const fn pinstat(self) -> crate::common::Reg<regs::Pinstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Pin Interrupt Enable"]
    #[inline(always)]
    pub const fn pinien(self) -> crate::common::Reg<regs::Pinien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Pin Rising Edge Enable"]
    #[inline(always)]
    pub const fn pinren(self) -> crate::common::Reg<regs::Pinren, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Pin Falling Edge Enable"]
    #[inline(always)]
    pub const fn pinfen(self) -> crate::common::Reg<regs::Pinfen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Pin Output Data"]
    #[inline(always)]
    pub const fn pinoutd(self) -> crate::common::Reg<regs::Pinoutd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Pin Output Enable"]
    #[inline(always)]
    pub const fn pinoute(self) -> crate::common::Reg<regs::Pinoute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Pin Output Disable"]
    #[inline(always)]
    pub const fn pinoutdis(self) -> crate::common::Reg<regs::Pinoutdis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Pin Output Clear"]
    #[inline(always)]
    pub const fn pinoutclr(self) -> crate::common::Reg<regs::Pinoutclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Pin Output Set"]
    #[inline(always)]
    pub const fn pinoutset(self) -> crate::common::Reg<regs::Pinoutset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Pin Output Toggle"]
    #[inline(always)]
    pub const fn pinouttog(self) -> crate::common::Reg<regs::Pinouttog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Shifter Control"]
    #[inline(always)]
    pub const fn shiftctl(self, n: usize) -> crate::common::Reg<regs::Shiftctl, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "Shifter Configuration"]
    #[inline(always)]
    pub const fn shiftcfg(self, n: usize) -> crate::common::Reg<regs::Shiftcfg, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Buffer"]
    #[inline(always)]
    pub const fn shiftbuf(self, n: usize) -> crate::common::Reg<regs::Shiftbuf, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Buffer Bit Swapped"]
    #[inline(always)]
    pub const fn shiftbufbis(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Shiftbufbis, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Buffer Byte Swapped"]
    #[inline(always)]
    pub const fn shiftbufbys(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Shiftbufbys, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Buffer Bit Byte Swapped"]
    #[inline(always)]
    pub const fn shiftbufbbs(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Shiftbufbbs, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize + n * 4usize) as _)
        }
    }
    #[doc = "Timer Control"]
    #[inline(always)]
    pub const fn timctl(self, n: usize) -> crate::common::Reg<regs::Timctl, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
    #[doc = "Timer Configuration"]
    #[inline(always)]
    pub const fn timcfg(self, n: usize) -> crate::common::Reg<regs::Timcfg, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0480usize + n * 4usize) as _)
        }
    }
    #[doc = "Timer Compare"]
    #[inline(always)]
    pub const fn timcmp(self, n: usize) -> crate::common::Reg<regs::Timcmp, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Buffer Nibble Byte Swapped"]
    #[inline(always)]
    pub const fn shiftbufnbs(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Shiftbufnbs, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0680usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Buffer Halfword Swapped"]
    #[inline(always)]
    pub const fn shiftbufhws(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Shiftbufhws, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Buffer Nibble Swapped"]
    #[inline(always)]
    pub const fn shiftbufnis(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Shiftbufnis, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0780usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Buffer Odd Even Swapped"]
    #[inline(always)]
    pub const fn shiftbufoes(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Shiftbufoes, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Buffer Even Odd Swapped"]
    #[inline(always)]
    pub const fn shiftbufeos(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Shiftbufeos, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0880usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Buffer Halfword Byte Swapped"]
    #[inline(always)]
    pub const fn shiftbufhbs(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Shiftbufhbs, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0900usize + n * 4usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
