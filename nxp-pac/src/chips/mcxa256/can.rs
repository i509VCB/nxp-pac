#[doc = "CAN"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can {
    ptr: *mut u8,
}
unsafe impl Send for Can {}
unsafe impl Sync for Can {}
impl Can {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Module Configuration"]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control 1"]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Free-Running Timer"]
    #[inline(always)]
    pub const fn timer(self) -> crate::common::Reg<regs::Timer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RX Message Buffers Global Mask"]
    #[inline(always)]
    pub const fn rxmgmask(self) -> crate::common::Reg<regs::Rxmgmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Receive 14 Mask"]
    #[inline(always)]
    pub const fn rx14mask(self) -> crate::common::Reg<regs::Rx14mask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Receive 15 Mask"]
    #[inline(always)]
    pub const fn rx15mask(self) -> crate::common::Reg<regs::Rx15mask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Error Counter"]
    #[inline(always)]
    pub const fn ecr(self) -> crate::common::Reg<regs::Ecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Error and Status 1"]
    #[inline(always)]
    pub const fn esr1(self) -> crate::common::Reg<regs::Esr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Interrupt Masks 1"]
    #[inline(always)]
    pub const fn imask1(self) -> crate::common::Reg<regs::Imask1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Interrupt Flags 1"]
    #[inline(always)]
    pub const fn iflag1(self) -> crate::common::Reg<regs::Iflag1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Control 2"]
    #[inline(always)]
    pub const fn ctrl2(self) -> crate::common::Reg<regs::Ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Error and Status 2"]
    #[inline(always)]
    pub const fn esr2(self) -> crate::common::Reg<regs::Esr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Cyclic Redundancy Check"]
    #[inline(always)]
    pub const fn crcr(self) -> crate::common::Reg<regs::Crcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Legacy RX FIFO Global Mask"]
    #[inline(always)]
    pub const fn rxfgmask(self) -> crate::common::Reg<regs::Rxfgmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Legacy RX FIFO Information"]
    #[inline(always)]
    pub const fn rxfir(self) -> crate::common::Reg<regs::Rxfir, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "CAN Bit Timing"]
    #[inline(always)]
    pub const fn cbt(self) -> crate::common::Reg<regs::Cbt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Message Buffer 0 CS Register"]
    #[inline(always)]
    pub const fn cs(self, n: usize) -> crate::common::Reg<regs::Cs, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 16usize) as _) }
    }
    #[doc = "Message Buffer 0 CS Register"]
    #[inline(always)]
    pub const fn mb_16b_cs(self, n: usize) -> crate::common::Reg<regs::MbbCs, crate::common::RW> {
        assert!(n < 21usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 24usize) as _) }
    }
    #[doc = "Message Buffer 0 CS Register"]
    #[inline(always)]
    pub const fn mb_32b_cs(self, n: usize) -> crate::common::Reg<regs::MbbCs, crate::common::RW> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 40usize) as _) }
    }
    #[doc = "Message Buffer 0 CS Register"]
    #[inline(always)]
    pub const fn mb_64b_cs(self, n: usize) -> crate::common::Reg<regs::MbbCs, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 72usize) as _) }
    }
    #[doc = "Message Buffer 0 CS Register"]
    #[inline(always)]
    pub const fn mb_8b_cs(self, n: usize) -> crate::common::Reg<regs::MbbCs, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 16usize) as _) }
    }
    #[doc = "Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn id(self, n: usize) -> crate::common::Reg<regs::Id, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize + n * 16usize) as _) }
    }
    #[doc = "Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn mb_16b_id(self, n: usize) -> crate::common::Reg<regs::MbbId, crate::common::RW> {
        assert!(n < 21usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize + n * 24usize) as _) }
    }
    #[doc = "Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn mb_32b_id(self, n: usize) -> crate::common::Reg<regs::MbbId, crate::common::RW> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize + n * 40usize) as _) }
    }
    #[doc = "Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn mb_64b_id(self, n: usize) -> crate::common::Reg<regs::MbbId, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize + n * 72usize) as _) }
    }
    #[doc = "Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn mb_8b_id(self, n: usize) -> crate::common::Reg<regs::MbbId, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize + n * 16usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD0 Register"]
    #[inline(always)]
    pub const fn word0(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize + n * 16usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD1 Register"]
    #[inline(always)]
    pub const fn word1(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize + n * 16usize) as _) }
    }
    #[doc = "Receive Individual Mask"]
    #[inline(always)]
    pub const fn rximr(self, n: usize) -> crate::common::Reg<regs::Rximr, crate::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0880usize + n * 4usize) as _)
        }
    }
    #[doc = "Pretended Networking Control 1"]
    #[inline(always)]
    pub const fn ctrl1_pn(self) -> crate::common::Reg<regs::Ctrl1Pn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b00usize) as _) }
    }
    #[doc = "Pretended Networking Control 2"]
    #[inline(always)]
    pub const fn ctrl2_pn(self) -> crate::common::Reg<regs::Ctrl2Pn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b04usize) as _) }
    }
    #[doc = "Pretended Networking Wake-Up Match"]
    #[inline(always)]
    pub const fn wu_mtc(self) -> crate::common::Reg<regs::WuMtc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b08usize) as _) }
    }
    #[doc = "Pretended Networking ID Filter 1"]
    #[inline(always)]
    pub const fn flt_id1(self) -> crate::common::Reg<regs::FltId1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b0cusize) as _) }
    }
    #[doc = "Pretended Networking Data Length Code (DLC) Filter"]
    #[inline(always)]
    pub const fn flt_dlc(self) -> crate::common::Reg<regs::FltDlc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b10usize) as _) }
    }
    #[doc = "Pretended Networking Payload Low Filter 1"]
    #[inline(always)]
    pub const fn pl1_lo(self) -> crate::common::Reg<regs::Pl1Lo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b14usize) as _) }
    }
    #[doc = "Pretended Networking Payload High Filter 1"]
    #[inline(always)]
    pub const fn pl1_hi(self) -> crate::common::Reg<regs::Pl1Hi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b18usize) as _) }
    }
    #[doc = "Pretended Networking ID Filter 2 or ID Mask"]
    #[inline(always)]
    pub const fn flt_id2_idmask(self) -> crate::common::Reg<regs::FltId2Idmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b1cusize) as _) }
    }
    #[doc = "Pretended Networking Payload Low Filter 2 and Payload Low Mask"]
    #[inline(always)]
    pub const fn pl2_plmask_lo(self) -> crate::common::Reg<regs::Pl2PlmaskLo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b20usize) as _) }
    }
    #[doc = "Pretended Networking Payload High Filter 2 and Payload High Mask"]
    #[inline(always)]
    pub const fn pl2_plmask_hi(self) -> crate::common::Reg<regs::Pl2PlmaskHi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b24usize) as _) }
    }
    #[doc = "Array of registers: WMB_CS, WMB_D03, WMB_D47, WMB_ID"]
    #[inline(always)]
    pub const fn wmb(self, n: usize) -> Wmb {
        assert!(n < 4usize);
        unsafe { Wmb::from_ptr(self.ptr.wrapping_add(0x0b40usize + n * 16usize) as _) }
    }
    #[doc = "Enhanced CAN Bit Timing Prescalers"]
    #[inline(always)]
    pub const fn eprs(self) -> crate::common::Reg<regs::Eprs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bf0usize) as _) }
    }
    #[doc = "Enhanced Nominal CAN Bit Timing"]
    #[inline(always)]
    pub const fn encbt(self) -> crate::common::Reg<regs::Encbt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bf4usize) as _) }
    }
    #[doc = "Enhanced Data Phase CAN Bit Timing"]
    #[inline(always)]
    pub const fn edcbt(self) -> crate::common::Reg<regs::Edcbt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bf8usize) as _) }
    }
    #[doc = "Enhanced Transceiver Delay Compensation"]
    #[inline(always)]
    pub const fn etdc(self) -> crate::common::Reg<regs::Etdc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bfcusize) as _) }
    }
    #[doc = "CAN FD Control"]
    #[inline(always)]
    pub const fn fdctrl(self) -> crate::common::Reg<regs::Fdctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize) as _) }
    }
    #[doc = "CAN FD Bit Timing"]
    #[inline(always)]
    pub const fn fdcbt(self) -> crate::common::Reg<regs::Fdcbt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c04usize) as _) }
    }
    #[doc = "CAN FD CRC"]
    #[inline(always)]
    pub const fn fdcrc(self) -> crate::common::Reg<regs::Fdcrc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c08usize) as _) }
    }
    #[doc = "Enhanced RX FIFO Control"]
    #[inline(always)]
    pub const fn erfcr(self) -> crate::common::Reg<regs::Erfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c0cusize) as _) }
    }
    #[doc = "Enhanced RX FIFO Interrupt Enable"]
    #[inline(always)]
    pub const fn erfier(self) -> crate::common::Reg<regs::Erfier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c10usize) as _) }
    }
    #[doc = "Enhanced RX FIFO Status"]
    #[inline(always)]
    pub const fn erfsr(self) -> crate::common::Reg<regs::Erfsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c14usize) as _) }
    }
    #[doc = "Enhanced RX FIFO Filter Element"]
    #[inline(always)]
    pub const fn erffel(self, n: usize) -> crate::common::Reg<regs::Erffel, crate::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3000usize + n * 4usize) as _)
        }
    }
}
#[doc = "Array of registers: WMB_CS, WMB_D03, WMB_D47, WMB_ID"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wmb {
    ptr: *mut u8,
}
unsafe impl Send for Wmb {}
unsafe impl Sync for Wmb {}
impl Wmb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Wake-Up Message Buffer"]
    #[inline(always)]
    pub const fn wmb_cs(self) -> crate::common::Reg<regs::WmbCs, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Wake-Up Message Buffer for ID"]
    #[inline(always)]
    pub const fn wmb_id(self) -> crate::common::Reg<regs::WmbId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Wake-Up Message Buffer for Data 0-3"]
    #[inline(always)]
    pub const fn wmb_d03(self) -> crate::common::Reg<regs::WmbD03, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Wake-Up Message Buffer Register Data 4-7"]
    #[inline(always)]
    pub const fn wmb_d47(self) -> crate::common::Reg<regs::WmbD47, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
