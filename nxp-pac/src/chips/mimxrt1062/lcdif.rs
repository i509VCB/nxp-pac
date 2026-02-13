#[doc = "LCDIF Register Reference Index"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcdif {
    ptr: *mut u8,
}
unsafe impl Send for Lcdif {}
unsafe impl Sync for Lcdif {}
impl Lcdif {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "LCDIF General Control Register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "LCDIF General Control Register"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::CtrlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "LCDIF General Control Register"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::CtrlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "LCDIF General Control Register"]
    #[inline(always)]
    pub const fn ctrl_tog(self) -> crate::common::Reg<regs::CtrlTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "LCDIF General Control1 Register"]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "LCDIF General Control1 Register"]
    #[inline(always)]
    pub const fn ctrl1_set(self) -> crate::common::Reg<regs::Ctrl1Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "LCDIF General Control1 Register"]
    #[inline(always)]
    pub const fn ctrl1_clr(self) -> crate::common::Reg<regs::Ctrl1Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "LCDIF General Control1 Register"]
    #[inline(always)]
    pub const fn ctrl1_tog(self) -> crate::common::Reg<regs::Ctrl1Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "LCDIF General Control2 Register"]
    #[inline(always)]
    pub const fn ctrl2(self) -> crate::common::Reg<regs::Ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "LCDIF General Control2 Register"]
    #[inline(always)]
    pub const fn ctrl2_set(self) -> crate::common::Reg<regs::Ctrl2Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "LCDIF General Control2 Register"]
    #[inline(always)]
    pub const fn ctrl2_clr(self) -> crate::common::Reg<regs::Ctrl2Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "LCDIF General Control2 Register"]
    #[inline(always)]
    pub const fn ctrl2_tog(self) -> crate::common::Reg<regs::Ctrl2Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "LCDIF Horizontal and Vertical Valid Data Count Register"]
    #[inline(always)]
    pub const fn transfer_count(
        self,
    ) -> crate::common::Reg<regs::TransferCount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "LCD Interface Current Buffer Address Register"]
    #[inline(always)]
    pub const fn cur_buf(self) -> crate::common::Reg<regs::CurBuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "LCD Interface Next Buffer Address Register"]
    #[inline(always)]
    pub const fn next_buf(self) -> crate::common::Reg<regs::NextBuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
    #[inline(always)]
    pub const fn vdctrl0(self) -> crate::common::Reg<regs::Vdctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
    #[inline(always)]
    pub const fn vdctrl0_set(self) -> crate::common::Reg<regs::Vdctrl0Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
    #[inline(always)]
    pub const fn vdctrl0_clr(self) -> crate::common::Reg<regs::Vdctrl0Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
    #[inline(always)]
    pub const fn vdctrl0_tog(self) -> crate::common::Reg<regs::Vdctrl0Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register1"]
    #[inline(always)]
    pub const fn vdctrl1(self) -> crate::common::Reg<regs::Vdctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register2"]
    #[inline(always)]
    pub const fn vdctrl2(self) -> crate::common::Reg<regs::Vdctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register3"]
    #[inline(always)]
    pub const fn vdctrl3(self) -> crate::common::Reg<regs::Vdctrl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register4"]
    #[inline(always)]
    pub const fn vdctrl4(self) -> crate::common::Reg<regs::Vdctrl4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Bus Master Error Status Register"]
    #[inline(always)]
    pub const fn bm_error_stat(self) -> crate::common::Reg<regs::BmErrorStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "CRC Status Register"]
    #[inline(always)]
    pub const fn crc_stat(self) -> crate::common::Reg<regs::CrcStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "LCD Interface Status Register"]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "LCDIF Pigeon Mode Control0 Register"]
    #[inline(always)]
    pub const fn pigeonctrl0(self) -> crate::common::Reg<regs::Pigeonctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "LCDIF Pigeon Mode Control0 Register"]
    #[inline(always)]
    pub const fn pigeonctrl0_set(
        self,
    ) -> crate::common::Reg<regs::Pigeonctrl0Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0384usize) as _) }
    }
    #[doc = "LCDIF Pigeon Mode Control0 Register"]
    #[inline(always)]
    pub const fn pigeonctrl0_clr(
        self,
    ) -> crate::common::Reg<regs::Pigeonctrl0Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[doc = "LCDIF Pigeon Mode Control0 Register"]
    #[inline(always)]
    pub const fn pigeonctrl0_tog(
        self,
    ) -> crate::common::Reg<regs::Pigeonctrl0Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[doc = "LCDIF Pigeon Mode Control1 Register"]
    #[inline(always)]
    pub const fn pigeonctrl1(self) -> crate::common::Reg<regs::Pigeonctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0390usize) as _) }
    }
    #[doc = "LCDIF Pigeon Mode Control1 Register"]
    #[inline(always)]
    pub const fn pigeonctrl1_set(
        self,
    ) -> crate::common::Reg<regs::Pigeonctrl1Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0394usize) as _) }
    }
    #[doc = "LCDIF Pigeon Mode Control1 Register"]
    #[inline(always)]
    pub const fn pigeonctrl1_clr(
        self,
    ) -> crate::common::Reg<regs::Pigeonctrl1Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0398usize) as _) }
    }
    #[doc = "LCDIF Pigeon Mode Control1 Register"]
    #[inline(always)]
    pub const fn pigeonctrl1_tog(
        self,
    ) -> crate::common::Reg<regs::Pigeonctrl1Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x039cusize) as _) }
    }
    #[doc = "LCDIF Pigeon Mode Control2 Register"]
    #[inline(always)]
    pub const fn pigeonctrl2(self) -> crate::common::Reg<regs::Pigeonctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a0usize) as _) }
    }
    #[doc = "LCDIF Pigeon Mode Control2 Register"]
    #[inline(always)]
    pub const fn pigeonctrl2_set(
        self,
    ) -> crate::common::Reg<regs::Pigeonctrl2Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a4usize) as _) }
    }
    #[doc = "LCDIF Pigeon Mode Control2 Register"]
    #[inline(always)]
    pub const fn pigeonctrl2_clr(
        self,
    ) -> crate::common::Reg<regs::Pigeonctrl2Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a8usize) as _) }
    }
    #[doc = "LCDIF Pigeon Mode Control2 Register"]
    #[inline(always)]
    pub const fn pigeonctrl2_tog(
        self,
    ) -> crate::common::Reg<regs::Pigeonctrl2Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03acusize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_0_0(self) -> crate::common::Reg<regs::Pigeon00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_0_1(self) -> crate::common::Reg<regs::Pigeon01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0810usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_0_2(self) -> crate::common::Reg<regs::Pigeon02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0820usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_1_0(self) -> crate::common::Reg<regs::Pigeon10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0840usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_1_1(self) -> crate::common::Reg<regs::Pigeon11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0850usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_1_2(self) -> crate::common::Reg<regs::Pigeon12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0860usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_2_0(self) -> crate::common::Reg<regs::Pigeon20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0880usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_2_1(self) -> crate::common::Reg<regs::Pigeon21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0890usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_2_2(self) -> crate::common::Reg<regs::Pigeon22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08a0usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_3_0(self) -> crate::common::Reg<regs::Pigeon30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08c0usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_3_1(self) -> crate::common::Reg<regs::Pigeon31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08d0usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_3_2(self) -> crate::common::Reg<regs::Pigeon32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08e0usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_4_0(self) -> crate::common::Reg<regs::Pigeon40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0900usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_4_1(self) -> crate::common::Reg<regs::Pigeon41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0910usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_4_2(self) -> crate::common::Reg<regs::Pigeon42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0920usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_5_0(self) -> crate::common::Reg<regs::Pigeon50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0940usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_5_1(self) -> crate::common::Reg<regs::Pigeon51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0950usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_5_2(self) -> crate::common::Reg<regs::Pigeon52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0960usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_6_0(self) -> crate::common::Reg<regs::Pigeon60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0980usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_6_1(self) -> crate::common::Reg<regs::Pigeon61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0990usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_6_2(self) -> crate::common::Reg<regs::Pigeon62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09a0usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_7_0(self) -> crate::common::Reg<regs::Pigeon70, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09c0usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_7_1(self) -> crate::common::Reg<regs::Pigeon71, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09d0usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_7_2(self) -> crate::common::Reg<regs::Pigeon72, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09e0usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_8_0(self) -> crate::common::Reg<regs::Pigeon80, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a00usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_8_1(self) -> crate::common::Reg<regs::Pigeon81, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a10usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_8_2(self) -> crate::common::Reg<regs::Pigeon82, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a20usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_9_0(self) -> crate::common::Reg<regs::Pigeon90, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a40usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_9_1(self) -> crate::common::Reg<regs::Pigeon91, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a50usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_9_2(self) -> crate::common::Reg<regs::Pigeon92, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a60usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_10_0(self) -> crate::common::Reg<regs::Pigeon100, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a80usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_10_1(self) -> crate::common::Reg<regs::Pigeon101, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a90usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_10_2(self) -> crate::common::Reg<regs::Pigeon102, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0aa0usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_11_0(self) -> crate::common::Reg<regs::Pigeon110, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ac0usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_11_1(self) -> crate::common::Reg<regs::Pigeon111, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ad0usize) as _) }
    }
    #[doc = "Panel Interface Signal Generator Register"]
    #[inline(always)]
    pub const fn pigeon_11_2(self) -> crate::common::Reg<regs::Pigeon112, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ae0usize) as _) }
    }
    #[doc = "Look Up Table Control Register"]
    #[inline(always)]
    pub const fn lut_ctrl(self) -> crate::common::Reg<regs::LutCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b00usize) as _) }
    }
    #[doc = "Lookup Table 0 Index Register"]
    #[inline(always)]
    pub const fn lut0_addr(self) -> crate::common::Reg<regs::Lut0Addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b10usize) as _) }
    }
    #[doc = "Lookup Table 0 Data Register"]
    #[inline(always)]
    pub const fn lut0_data(self) -> crate::common::Reg<regs::Lut0Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b20usize) as _) }
    }
    #[doc = "Lookup Table 1 Index Register"]
    #[inline(always)]
    pub const fn lut1_addr(self) -> crate::common::Reg<regs::Lut1Addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b30usize) as _) }
    }
    #[doc = "Lookup Table 1 Data Register"]
    #[inline(always)]
    pub const fn lut1_data(self) -> crate::common::Reg<regs::Lut1Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b40usize) as _) }
    }
}
pub mod regs;
pub mod vals;
