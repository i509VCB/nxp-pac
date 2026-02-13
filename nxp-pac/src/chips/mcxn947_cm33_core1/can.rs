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
    pub const fn cs0(self) -> crate::common::Reg<regs::Cs0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Message Buffer 0 CS Register"]
    #[inline(always)]
    pub const fn mb0_16b_cs(self) -> crate::common::Reg<regs::Mb016bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Message Buffer 0 CS Register"]
    #[inline(always)]
    pub const fn mb0_32b_cs(self) -> crate::common::Reg<regs::Mb032bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Message Buffer 0 CS Register"]
    #[inline(always)]
    pub const fn mb0_64b_cs(self) -> crate::common::Reg<regs::Mb064bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Message Buffer 0 CS Register"]
    #[inline(always)]
    pub const fn mb0_8b_cs(self) -> crate::common::Reg<regs::Mb08bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn id0(self) -> crate::common::Reg<regs::Id0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn mb0_16b_id(self) -> crate::common::Reg<regs::Mb016bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn mb0_32b_id(self) -> crate::common::Reg<regs::Mb032bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn mb0_64b_id(self) -> crate::common::Reg<regs::Mb064bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn mb0_8b_id(self) -> crate::common::Reg<regs::Mb08bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb0_16b_word0(self) -> crate::common::Reg<regs::Mb016bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word0(self) -> crate::common::Reg<regs::Mb032bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word0(self) -> crate::common::Reg<regs::Mb064bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb0_8b_word0(self) -> crate::common::Reg<regs::Mb08bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD0 Register"]
    #[inline(always)]
    pub const fn word00(self) -> crate::common::Reg<regs::Word00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb0_16b_word1(self) -> crate::common::Reg<regs::Mb016bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word1(self) -> crate::common::Reg<regs::Mb032bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word1(self) -> crate::common::Reg<regs::Mb064bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb0_8b_word1(self) -> crate::common::Reg<regs::Mb08bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Message Buffer 0 WORD1 Register"]
    #[inline(always)]
    pub const fn word10(self) -> crate::common::Reg<regs::Word10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Message Buffer 1 CS Register"]
    #[inline(always)]
    pub const fn cs1(self) -> crate::common::Reg<regs::Cs1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb0_16b_word2(self) -> crate::common::Reg<regs::Mb016bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word2(self) -> crate::common::Reg<regs::Mb032bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word2(self) -> crate::common::Reg<regs::Mb064bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Message Buffer 1 CS Register"]
    #[inline(always)]
    pub const fn mb1_8b_cs(self) -> crate::common::Reg<regs::Mb18bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Message Buffer 1 ID Register"]
    #[inline(always)]
    pub const fn id1(self) -> crate::common::Reg<regs::Id1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb0_16b_word3(self) -> crate::common::Reg<regs::Mb016bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word3(self) -> crate::common::Reg<regs::Mb032bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word3(self) -> crate::common::Reg<regs::Mb064bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Message Buffer 1 ID Register"]
    #[inline(always)]
    pub const fn mb1_8b_id(self) -> crate::common::Reg<regs::Mb18bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word4(self) -> crate::common::Reg<regs::Mb032bWord4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word4(self) -> crate::common::Reg<regs::Mb064bWord4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Message Buffer 1 CS Register"]
    #[inline(always)]
    pub const fn mb1_16b_cs(self) -> crate::common::Reg<regs::Mb116bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb1_8b_word0(self) -> crate::common::Reg<regs::Mb18bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD0 Register"]
    #[inline(always)]
    pub const fn word01(self) -> crate::common::Reg<regs::Word01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word5(self) -> crate::common::Reg<regs::Mb032bWord5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word5(self) -> crate::common::Reg<regs::Mb064bWord5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Message Buffer 1 ID Register"]
    #[inline(always)]
    pub const fn mb1_16b_id(self) -> crate::common::Reg<regs::Mb116bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb1_8b_word1(self) -> crate::common::Reg<regs::Mb18bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Message Buffer 1 WORD1 Register"]
    #[inline(always)]
    pub const fn word11(self) -> crate::common::Reg<regs::Word11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Message Buffer 2 CS Register"]
    #[inline(always)]
    pub const fn cs2(self) -> crate::common::Reg<regs::Cs2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word6(self) -> crate::common::Reg<regs::Mb032bWord6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word6(self) -> crate::common::Reg<regs::Mb064bWord6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb1_16b_word0(self) -> crate::common::Reg<regs::Mb116bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Message Buffer 2 CS Register"]
    #[inline(always)]
    pub const fn mb2_8b_cs(self) -> crate::common::Reg<regs::Mb28bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Message Buffer 2 ID Register"]
    #[inline(always)]
    pub const fn id2(self) -> crate::common::Reg<regs::Id2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word7(self) -> crate::common::Reg<regs::Mb032bWord7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word7(self) -> crate::common::Reg<regs::Mb064bWord7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb1_16b_word1(self) -> crate::common::Reg<regs::Mb116bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Message Buffer 2 ID Register"]
    #[inline(always)]
    pub const fn mb2_8b_id(self) -> crate::common::Reg<regs::Mb28bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word8(self) -> crate::common::Reg<regs::Mb064bWord8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb1_16b_word2(self) -> crate::common::Reg<regs::Mb116bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Message Buffer 1 CS Register"]
    #[inline(always)]
    pub const fn mb1_32b_cs(self) -> crate::common::Reg<regs::Mb132bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb2_8b_word0(self) -> crate::common::Reg<regs::Mb28bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD0 Register"]
    #[inline(always)]
    pub const fn word02(self) -> crate::common::Reg<regs::Word02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word9(self) -> crate::common::Reg<regs::Mb064bWord9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb1_16b_word3(self) -> crate::common::Reg<regs::Mb116bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Message Buffer 1 ID Register"]
    #[inline(always)]
    pub const fn mb1_32b_id(self) -> crate::common::Reg<regs::Mb132bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb2_8b_word1(self) -> crate::common::Reg<regs::Mb28bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Message Buffer 2 WORD1 Register"]
    #[inline(always)]
    pub const fn word12(self) -> crate::common::Reg<regs::Word12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Message Buffer 3 CS Register"]
    #[inline(always)]
    pub const fn cs3(self) -> crate::common::Reg<regs::Cs3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word10(self) -> crate::common::Reg<regs::Mb064bWord10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word0(self) -> crate::common::Reg<regs::Mb132bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Message Buffer 2 CS Register"]
    #[inline(always)]
    pub const fn mb2_16b_cs(self) -> crate::common::Reg<regs::Mb216bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Message Buffer 3 CS Register"]
    #[inline(always)]
    pub const fn mb3_8b_cs(self) -> crate::common::Reg<regs::Mb38bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Message Buffer 3 ID Register"]
    #[inline(always)]
    pub const fn id3(self) -> crate::common::Reg<regs::Id3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word11(self) -> crate::common::Reg<regs::Mb064bWord11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word1(self) -> crate::common::Reg<regs::Mb132bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Message Buffer 2 ID Register"]
    #[inline(always)]
    pub const fn mb2_16b_id(self) -> crate::common::Reg<regs::Mb216bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Message Buffer 3 ID Register"]
    #[inline(always)]
    pub const fn mb3_8b_id(self) -> crate::common::Reg<regs::Mb38bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word12(self) -> crate::common::Reg<regs::Mb064bWord12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word2(self) -> crate::common::Reg<regs::Mb132bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb2_16b_word0(self) -> crate::common::Reg<regs::Mb216bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb3_8b_word0(self) -> crate::common::Reg<regs::Mb38bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD0 Register"]
    #[inline(always)]
    pub const fn word03(self) -> crate::common::Reg<regs::Word03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word13(self) -> crate::common::Reg<regs::Mb064bWord13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word3(self) -> crate::common::Reg<regs::Mb132bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb2_16b_word1(self) -> crate::common::Reg<regs::Mb216bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb3_8b_word1(self) -> crate::common::Reg<regs::Mb38bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD1 Register"]
    #[inline(always)]
    pub const fn word13(self) -> crate::common::Reg<regs::Word13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "Message Buffer 4 CS Register"]
    #[inline(always)]
    pub const fn cs4(self) -> crate::common::Reg<regs::Cs4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word14(self) -> crate::common::Reg<regs::Mb064bWord14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word4(self) -> crate::common::Reg<regs::Mb132bWord4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb2_16b_word2(self) -> crate::common::Reg<regs::Mb216bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Message Buffer 4 CS Register"]
    #[inline(always)]
    pub const fn mb4_8b_cs(self) -> crate::common::Reg<regs::Mb48bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Message Buffer 4 ID Register"]
    #[inline(always)]
    pub const fn id4(self) -> crate::common::Reg<regs::Id4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word15(self) -> crate::common::Reg<regs::Mb064bWord15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word5(self) -> crate::common::Reg<regs::Mb132bWord5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb2_16b_word3(self) -> crate::common::Reg<regs::Mb216bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Message Buffer 4 ID Register"]
    #[inline(always)]
    pub const fn mb4_8b_id(self) -> crate::common::Reg<regs::Mb48bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word6(self) -> crate::common::Reg<regs::Mb132bWord6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Message Buffer 1 CS Register"]
    #[inline(always)]
    pub const fn mb1_64b_cs(self) -> crate::common::Reg<regs::Mb164bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Message Buffer 3 CS Register"]
    #[inline(always)]
    pub const fn mb3_16b_cs(self) -> crate::common::Reg<regs::Mb316bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb4_8b_word0(self) -> crate::common::Reg<regs::Mb48bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD0 Register"]
    #[inline(always)]
    pub const fn word04(self) -> crate::common::Reg<regs::Word04, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word7(self) -> crate::common::Reg<regs::Mb132bWord7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Message Buffer 1 ID Register"]
    #[inline(always)]
    pub const fn mb1_64b_id(self) -> crate::common::Reg<regs::Mb164bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Message Buffer 3 ID Register"]
    #[inline(always)]
    pub const fn mb3_16b_id(self) -> crate::common::Reg<regs::Mb316bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb4_8b_word1(self) -> crate::common::Reg<regs::Mb48bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD1 Register"]
    #[inline(always)]
    pub const fn word14(self) -> crate::common::Reg<regs::Word14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Message Buffer 5 CS Register"]
    #[inline(always)]
    pub const fn cs5(self) -> crate::common::Reg<regs::Cs5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word0(self) -> crate::common::Reg<regs::Mb164bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Message Buffer 2 CS Register"]
    #[inline(always)]
    pub const fn mb2_32b_cs(self) -> crate::common::Reg<regs::Mb232bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb3_16b_word0(self) -> crate::common::Reg<regs::Mb316bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Message Buffer 5 CS Register"]
    #[inline(always)]
    pub const fn mb5_8b_cs(self) -> crate::common::Reg<regs::Mb58bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Message Buffer 5 ID Register"]
    #[inline(always)]
    pub const fn id5(self) -> crate::common::Reg<regs::Id5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word1(self) -> crate::common::Reg<regs::Mb164bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Message Buffer 2 ID Register"]
    #[inline(always)]
    pub const fn mb2_32b_id(self) -> crate::common::Reg<regs::Mb232bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb3_16b_word1(self) -> crate::common::Reg<regs::Mb316bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Message Buffer 5 ID Register"]
    #[inline(always)]
    pub const fn mb5_8b_id(self) -> crate::common::Reg<regs::Mb58bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word2(self) -> crate::common::Reg<regs::Mb164bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word0(self) -> crate::common::Reg<regs::Mb232bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb3_16b_word2(self) -> crate::common::Reg<regs::Mb316bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb5_8b_word0(self) -> crate::common::Reg<regs::Mb58bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD0 Register"]
    #[inline(always)]
    pub const fn word05(self) -> crate::common::Reg<regs::Word05, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word3(self) -> crate::common::Reg<regs::Mb164bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word1(self) -> crate::common::Reg<regs::Mb232bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb3_16b_word3(self) -> crate::common::Reg<regs::Mb316bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb5_8b_word1(self) -> crate::common::Reg<regs::Mb58bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD1 Register"]
    #[inline(always)]
    pub const fn word15(self) -> crate::common::Reg<regs::Word15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Message Buffer 6 CS Register"]
    #[inline(always)]
    pub const fn cs6(self) -> crate::common::Reg<regs::Cs6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word4(self) -> crate::common::Reg<regs::Mb164bWord4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word2(self) -> crate::common::Reg<regs::Mb232bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Message Buffer 4 CS Register"]
    #[inline(always)]
    pub const fn mb4_16b_cs(self) -> crate::common::Reg<regs::Mb416bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Message Buffer 6 CS Register"]
    #[inline(always)]
    pub const fn mb6_8b_cs(self) -> crate::common::Reg<regs::Mb68bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Message Buffer 6 ID Register"]
    #[inline(always)]
    pub const fn id6(self) -> crate::common::Reg<regs::Id6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word5(self) -> crate::common::Reg<regs::Mb164bWord5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word3(self) -> crate::common::Reg<regs::Mb232bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Message Buffer 4 ID Register"]
    #[inline(always)]
    pub const fn mb4_16b_id(self) -> crate::common::Reg<regs::Mb416bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Message Buffer 6 ID Register"]
    #[inline(always)]
    pub const fn mb6_8b_id(self) -> crate::common::Reg<regs::Mb68bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word6(self) -> crate::common::Reg<regs::Mb164bWord6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word4(self) -> crate::common::Reg<regs::Mb232bWord4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb4_16b_word0(self) -> crate::common::Reg<regs::Mb416bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb6_8b_word0(self) -> crate::common::Reg<regs::Mb68bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD0 Register"]
    #[inline(always)]
    pub const fn word06(self) -> crate::common::Reg<regs::Word06, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word7(self) -> crate::common::Reg<regs::Mb164bWord7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word5(self) -> crate::common::Reg<regs::Mb232bWord5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb4_16b_word1(self) -> crate::common::Reg<regs::Mb416bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb6_8b_word1(self) -> crate::common::Reg<regs::Mb68bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD1 Register"]
    #[inline(always)]
    pub const fn word16(self) -> crate::common::Reg<regs::Word16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Message Buffer 7 CS Register"]
    #[inline(always)]
    pub const fn cs7(self) -> crate::common::Reg<regs::Cs7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word8(self) -> crate::common::Reg<regs::Mb164bWord8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word6(self) -> crate::common::Reg<regs::Mb232bWord6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb4_16b_word2(self) -> crate::common::Reg<regs::Mb416bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Message Buffer 7 CS Register"]
    #[inline(always)]
    pub const fn mb7_8b_cs(self) -> crate::common::Reg<regs::Mb78bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Message Buffer 7 ID Register"]
    #[inline(always)]
    pub const fn id7(self) -> crate::common::Reg<regs::Id7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word9(self) -> crate::common::Reg<regs::Mb164bWord9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word7(self) -> crate::common::Reg<regs::Mb232bWord7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb4_16b_word3(self) -> crate::common::Reg<regs::Mb416bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Message Buffer 7 ID Register"]
    #[inline(always)]
    pub const fn mb7_8b_id(self) -> crate::common::Reg<regs::Mb78bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word10(self) -> crate::common::Reg<regs::Mb164bWord10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Message Buffer 3 CS Register"]
    #[inline(always)]
    pub const fn mb3_32b_cs(self) -> crate::common::Reg<regs::Mb332bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Message Buffer 5 CS Register"]
    #[inline(always)]
    pub const fn mb5_16b_cs(self) -> crate::common::Reg<regs::Mb516bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb7_8b_word0(self) -> crate::common::Reg<regs::Mb78bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD0 Register"]
    #[inline(always)]
    pub const fn word07(self) -> crate::common::Reg<regs::Word07, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word11(self) -> crate::common::Reg<regs::Mb164bWord11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Message Buffer 3 ID Register"]
    #[inline(always)]
    pub const fn mb3_32b_id(self) -> crate::common::Reg<regs::Mb332bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Message Buffer 5 ID Register"]
    #[inline(always)]
    pub const fn mb5_16b_id(self) -> crate::common::Reg<regs::Mb516bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb7_8b_word1(self) -> crate::common::Reg<regs::Mb78bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Message Buffer 7 WORD1 Register"]
    #[inline(always)]
    pub const fn word17(self) -> crate::common::Reg<regs::Word17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Message Buffer 8 CS Register"]
    #[inline(always)]
    pub const fn cs8(self) -> crate::common::Reg<regs::Cs8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word12(self) -> crate::common::Reg<regs::Mb164bWord12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word0(self) -> crate::common::Reg<regs::Mb332bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb5_16b_word0(self) -> crate::common::Reg<regs::Mb516bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Message Buffer 8 CS Register"]
    #[inline(always)]
    pub const fn mb8_8b_cs(self) -> crate::common::Reg<regs::Mb88bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Message Buffer 8 ID Register"]
    #[inline(always)]
    pub const fn id8(self) -> crate::common::Reg<regs::Id8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word13(self) -> crate::common::Reg<regs::Mb164bWord13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word1(self) -> crate::common::Reg<regs::Mb332bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb5_16b_word1(self) -> crate::common::Reg<regs::Mb516bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Message Buffer 8 ID Register"]
    #[inline(always)]
    pub const fn mb8_8b_id(self) -> crate::common::Reg<regs::Mb88bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word14(self) -> crate::common::Reg<regs::Mb164bWord14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word2(self) -> crate::common::Reg<regs::Mb332bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb5_16b_word2(self) -> crate::common::Reg<regs::Mb516bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb8_8b_word0(self) -> crate::common::Reg<regs::Mb88bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD0 Register"]
    #[inline(always)]
    pub const fn word08(self) -> crate::common::Reg<regs::Word08, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word15(self) -> crate::common::Reg<regs::Mb164bWord15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word3(self) -> crate::common::Reg<regs::Mb332bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb5_16b_word3(self) -> crate::common::Reg<regs::Mb516bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb8_8b_word1(self) -> crate::common::Reg<regs::Mb88bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Message Buffer 8 WORD1 Register"]
    #[inline(always)]
    pub const fn word18(self) -> crate::common::Reg<regs::Word18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Message Buffer 9 CS Register"]
    #[inline(always)]
    pub const fn cs9(self) -> crate::common::Reg<regs::Cs9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Message Buffer 2 CS Register"]
    #[inline(always)]
    pub const fn mb2_64b_cs(self) -> crate::common::Reg<regs::Mb264bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word4(self) -> crate::common::Reg<regs::Mb332bWord4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Message Buffer 6 CS Register"]
    #[inline(always)]
    pub const fn mb6_16b_cs(self) -> crate::common::Reg<regs::Mb616bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Message Buffer 9 CS Register"]
    #[inline(always)]
    pub const fn mb9_8b_cs(self) -> crate::common::Reg<regs::Mb98bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Message Buffer 9 ID Register"]
    #[inline(always)]
    pub const fn id9(self) -> crate::common::Reg<regs::Id9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Message Buffer 2 ID Register"]
    #[inline(always)]
    pub const fn mb2_64b_id(self) -> crate::common::Reg<regs::Mb264bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word5(self) -> crate::common::Reg<regs::Mb332bWord5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Message Buffer 6 ID Register"]
    #[inline(always)]
    pub const fn mb6_16b_id(self) -> crate::common::Reg<regs::Mb616bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Message Buffer 9 ID Register"]
    #[inline(always)]
    pub const fn mb9_8b_id(self) -> crate::common::Reg<regs::Mb98bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word0(self) -> crate::common::Reg<regs::Mb264bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word6(self) -> crate::common::Reg<regs::Mb332bWord6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb6_16b_word0(self) -> crate::common::Reg<regs::Mb616bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb9_8b_word0(self) -> crate::common::Reg<regs::Mb98bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD0 Register"]
    #[inline(always)]
    pub const fn word09(self) -> crate::common::Reg<regs::Word09, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word1(self) -> crate::common::Reg<regs::Mb264bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word7(self) -> crate::common::Reg<regs::Mb332bWord7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb6_16b_word1(self) -> crate::common::Reg<regs::Mb616bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb9_8b_word1(self) -> crate::common::Reg<regs::Mb98bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Message Buffer 9 WORD1 Register"]
    #[inline(always)]
    pub const fn word19(self) -> crate::common::Reg<regs::Word19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Message Buffer 10 CS Register"]
    #[inline(always)]
    pub const fn cs10(self) -> crate::common::Reg<regs::Cs10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Message Buffer 10 CS Register"]
    #[inline(always)]
    pub const fn mb10_8b_cs(self) -> crate::common::Reg<regs::Mb108bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word2(self) -> crate::common::Reg<regs::Mb264bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Message Buffer 4 CS Register"]
    #[inline(always)]
    pub const fn mb4_32b_cs(self) -> crate::common::Reg<regs::Mb432bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb6_16b_word2(self) -> crate::common::Reg<regs::Mb616bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Message Buffer 10 ID Register"]
    #[inline(always)]
    pub const fn id10(self) -> crate::common::Reg<regs::Id10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Message Buffer 10 ID Register"]
    #[inline(always)]
    pub const fn mb10_8b_id(self) -> crate::common::Reg<regs::Mb108bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word3(self) -> crate::common::Reg<regs::Mb264bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Message Buffer 4 ID Register"]
    #[inline(always)]
    pub const fn mb4_32b_id(self) -> crate::common::Reg<regs::Mb432bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb6_16b_word3(self) -> crate::common::Reg<regs::Mb616bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb10_8b_word0(self) -> crate::common::Reg<regs::Mb108bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word4(self) -> crate::common::Reg<regs::Mb264bWord4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word0(self) -> crate::common::Reg<regs::Mb432bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Message Buffer 7 CS Register"]
    #[inline(always)]
    pub const fn mb7_16b_cs(self) -> crate::common::Reg<regs::Mb716bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD0 Register"]
    #[inline(always)]
    pub const fn word010(self) -> crate::common::Reg<regs::Word010, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb10_8b_word1(self) -> crate::common::Reg<regs::Mb108bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word5(self) -> crate::common::Reg<regs::Mb264bWord5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word1(self) -> crate::common::Reg<regs::Mb432bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Message Buffer 7 ID Register"]
    #[inline(always)]
    pub const fn mb7_16b_id(self) -> crate::common::Reg<regs::Mb716bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Message Buffer 10 WORD1 Register"]
    #[inline(always)]
    pub const fn word110(self) -> crate::common::Reg<regs::Word110, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Message Buffer 11 CS Register"]
    #[inline(always)]
    pub const fn cs11(self) -> crate::common::Reg<regs::Cs11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Message Buffer 11 CS Register"]
    #[inline(always)]
    pub const fn mb11_8b_cs(self) -> crate::common::Reg<regs::Mb118bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word6(self) -> crate::common::Reg<regs::Mb264bWord6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word2(self) -> crate::common::Reg<regs::Mb432bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb7_16b_word0(self) -> crate::common::Reg<regs::Mb716bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Message Buffer 11 ID Register"]
    #[inline(always)]
    pub const fn id11(self) -> crate::common::Reg<regs::Id11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Message Buffer 11 ID Register"]
    #[inline(always)]
    pub const fn mb11_8b_id(self) -> crate::common::Reg<regs::Mb118bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word7(self) -> crate::common::Reg<regs::Mb264bWord7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word3(self) -> crate::common::Reg<regs::Mb432bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb7_16b_word1(self) -> crate::common::Reg<regs::Mb716bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb11_8b_word0(self) -> crate::common::Reg<regs::Mb118bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word8(self) -> crate::common::Reg<regs::Mb264bWord8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word4(self) -> crate::common::Reg<regs::Mb432bWord4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb7_16b_word2(self) -> crate::common::Reg<regs::Mb716bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD0 Register"]
    #[inline(always)]
    pub const fn word011(self) -> crate::common::Reg<regs::Word011, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb11_8b_word1(self) -> crate::common::Reg<regs::Mb118bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word9(self) -> crate::common::Reg<regs::Mb264bWord9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word5(self) -> crate::common::Reg<regs::Mb432bWord5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb7_16b_word3(self) -> crate::common::Reg<regs::Mb716bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "Message Buffer 11 WORD1 Register"]
    #[inline(always)]
    pub const fn word111(self) -> crate::common::Reg<regs::Word111, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "Message Buffer 12 CS Register"]
    #[inline(always)]
    pub const fn cs12(self) -> crate::common::Reg<regs::Cs12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Message Buffer 12 CS Register"]
    #[inline(always)]
    pub const fn mb12_8b_cs(self) -> crate::common::Reg<regs::Mb128bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word10(self) -> crate::common::Reg<regs::Mb264bWord10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word6(self) -> crate::common::Reg<regs::Mb432bWord6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Message Buffer 8 CS Register"]
    #[inline(always)]
    pub const fn mb8_16b_cs(self) -> crate::common::Reg<regs::Mb816bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Message Buffer 12 ID Register"]
    #[inline(always)]
    pub const fn id12(self) -> crate::common::Reg<regs::Id12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Message Buffer 12 ID Register"]
    #[inline(always)]
    pub const fn mb12_8b_id(self) -> crate::common::Reg<regs::Mb128bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word11(self) -> crate::common::Reg<regs::Mb264bWord11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word7(self) -> crate::common::Reg<regs::Mb432bWord7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Message Buffer 8 ID Register"]
    #[inline(always)]
    pub const fn mb8_16b_id(self) -> crate::common::Reg<regs::Mb816bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Message Buffer 12 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb12_8b_word0(self) -> crate::common::Reg<regs::Mb128bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word12(self) -> crate::common::Reg<regs::Mb264bWord12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Message Buffer 5 CS Register"]
    #[inline(always)]
    pub const fn mb5_32b_cs(self) -> crate::common::Reg<regs::Mb532bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb8_16b_word0(self) -> crate::common::Reg<regs::Mb816bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Message Buffer 12 WORD0 Register"]
    #[inline(always)]
    pub const fn word012(self) -> crate::common::Reg<regs::Word012, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Message Buffer 12 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb12_8b_word1(self) -> crate::common::Reg<regs::Mb128bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word13(self) -> crate::common::Reg<regs::Mb264bWord13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Message Buffer 5 ID Register"]
    #[inline(always)]
    pub const fn mb5_32b_id(self) -> crate::common::Reg<regs::Mb532bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb8_16b_word1(self) -> crate::common::Reg<regs::Mb816bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Message Buffer 12 WORD1 Register"]
    #[inline(always)]
    pub const fn word112(self) -> crate::common::Reg<regs::Word112, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Message Buffer 13 CS Register"]
    #[inline(always)]
    pub const fn cs13(self) -> crate::common::Reg<regs::Cs13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Message Buffer 13 CS Register"]
    #[inline(always)]
    pub const fn mb13_8b_cs(self) -> crate::common::Reg<regs::Mb138bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word14(self) -> crate::common::Reg<regs::Mb264bWord14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word0(self) -> crate::common::Reg<regs::Mb532bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb8_16b_word2(self) -> crate::common::Reg<regs::Mb816bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Message Buffer 13 ID Register"]
    #[inline(always)]
    pub const fn id13(self) -> crate::common::Reg<regs::Id13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Message Buffer 13 ID Register"]
    #[inline(always)]
    pub const fn mb13_8b_id(self) -> crate::common::Reg<regs::Mb138bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word15(self) -> crate::common::Reg<regs::Mb264bWord15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word1(self) -> crate::common::Reg<regs::Mb532bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb8_16b_word3(self) -> crate::common::Reg<regs::Mb816bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Message Buffer 13 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb13_8b_word0(self) -> crate::common::Reg<regs::Mb138bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Message Buffer 3 CS Register"]
    #[inline(always)]
    pub const fn mb3_64b_cs(self) -> crate::common::Reg<regs::Mb364bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word2(self) -> crate::common::Reg<regs::Mb532bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Message Buffer 9 CS Register"]
    #[inline(always)]
    pub const fn mb9_16b_cs(self) -> crate::common::Reg<regs::Mb916bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Message Buffer 13 WORD0 Register"]
    #[inline(always)]
    pub const fn word013(self) -> crate::common::Reg<regs::Word013, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Message Buffer 13 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb13_8b_word1(self) -> crate::common::Reg<regs::Mb138bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Message Buffer 3 ID Register"]
    #[inline(always)]
    pub const fn mb3_64b_id(self) -> crate::common::Reg<regs::Mb364bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word3(self) -> crate::common::Reg<regs::Mb532bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Message Buffer 9 ID Register"]
    #[inline(always)]
    pub const fn mb9_16b_id(self) -> crate::common::Reg<regs::Mb916bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Message Buffer 13 WORD1 Register"]
    #[inline(always)]
    pub const fn word113(self) -> crate::common::Reg<regs::Word113, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Message Buffer 14 CS Register"]
    #[inline(always)]
    pub const fn cs14(self) -> crate::common::Reg<regs::Cs14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Message Buffer 14 CS Register"]
    #[inline(always)]
    pub const fn mb14_8b_cs(self) -> crate::common::Reg<regs::Mb148bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word0(self) -> crate::common::Reg<regs::Mb364bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word4(self) -> crate::common::Reg<regs::Mb532bWord4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb9_16b_word0(self) -> crate::common::Reg<regs::Mb916bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Message Buffer 14 ID Register"]
    #[inline(always)]
    pub const fn id14(self) -> crate::common::Reg<regs::Id14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Message Buffer 14 ID Register"]
    #[inline(always)]
    pub const fn mb14_8b_id(self) -> crate::common::Reg<regs::Mb148bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word1(self) -> crate::common::Reg<regs::Mb364bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word5(self) -> crate::common::Reg<regs::Mb532bWord5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb9_16b_word1(self) -> crate::common::Reg<regs::Mb916bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Message Buffer 14 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb14_8b_word0(self) -> crate::common::Reg<regs::Mb148bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word2(self) -> crate::common::Reg<regs::Mb364bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word6(self) -> crate::common::Reg<regs::Mb532bWord6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb9_16b_word2(self) -> crate::common::Reg<regs::Mb916bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Message Buffer 14 WORD0 Register"]
    #[inline(always)]
    pub const fn word014(self) -> crate::common::Reg<regs::Word014, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Message Buffer 14 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb14_8b_word1(self) -> crate::common::Reg<regs::Mb148bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word3(self) -> crate::common::Reg<regs::Mb364bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word7(self) -> crate::common::Reg<regs::Mb532bWord7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb9_16b_word3(self) -> crate::common::Reg<regs::Mb916bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "Message Buffer 14 WORD1 Register"]
    #[inline(always)]
    pub const fn word114(self) -> crate::common::Reg<regs::Word114, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "Message Buffer 15 CS Register"]
    #[inline(always)]
    pub const fn cs15(self) -> crate::common::Reg<regs::Cs15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Message Buffer 10 CS Register"]
    #[inline(always)]
    pub const fn mb10_16b_cs(self) -> crate::common::Reg<regs::Mb1016bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Message Buffer 15 CS Register"]
    #[inline(always)]
    pub const fn mb15_8b_cs(self) -> crate::common::Reg<regs::Mb158bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word4(self) -> crate::common::Reg<regs::Mb364bWord4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Message Buffer 6 CS Register"]
    #[inline(always)]
    pub const fn mb6_32b_cs(self) -> crate::common::Reg<regs::Mb632bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Message Buffer 15 ID Register"]
    #[inline(always)]
    pub const fn id15(self) -> crate::common::Reg<regs::Id15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Message Buffer 10 ID Register"]
    #[inline(always)]
    pub const fn mb10_16b_id(self) -> crate::common::Reg<regs::Mb1016bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Message Buffer 15 ID Register"]
    #[inline(always)]
    pub const fn mb15_8b_id(self) -> crate::common::Reg<regs::Mb158bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word5(self) -> crate::common::Reg<regs::Mb364bWord5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Message Buffer 6 ID Register"]
    #[inline(always)]
    pub const fn mb6_32b_id(self) -> crate::common::Reg<regs::Mb632bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb10_16b_word0(self) -> crate::common::Reg<regs::Mb1016bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Message Buffer 15 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb15_8b_word0(self) -> crate::common::Reg<regs::Mb158bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word6(self) -> crate::common::Reg<regs::Mb364bWord6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word0(self) -> crate::common::Reg<regs::Mb632bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Message Buffer 15 WORD0 Register"]
    #[inline(always)]
    pub const fn word015(self) -> crate::common::Reg<regs::Word015, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb10_16b_word1(self) -> crate::common::Reg<regs::Mb1016bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "Message Buffer 15 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb15_8b_word1(self) -> crate::common::Reg<regs::Mb158bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word7(self) -> crate::common::Reg<regs::Mb364bWord7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word1(self) -> crate::common::Reg<regs::Mb632bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "Message Buffer 15 WORD1 Register"]
    #[inline(always)]
    pub const fn word115(self) -> crate::common::Reg<regs::Word115, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "Message Buffer 16 CS Register"]
    #[inline(always)]
    pub const fn cs16(self) -> crate::common::Reg<regs::Cs16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb10_16b_word2(self) -> crate::common::Reg<regs::Mb1016bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Message Buffer 16 CS Register"]
    #[inline(always)]
    pub const fn mb16_8b_cs(self) -> crate::common::Reg<regs::Mb168bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word8(self) -> crate::common::Reg<regs::Mb364bWord8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word2(self) -> crate::common::Reg<regs::Mb632bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Message Buffer 16 ID Register"]
    #[inline(always)]
    pub const fn id16(self) -> crate::common::Reg<regs::Id16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb10_16b_word3(self) -> crate::common::Reg<regs::Mb1016bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Message Buffer 16 ID Register"]
    #[inline(always)]
    pub const fn mb16_8b_id(self) -> crate::common::Reg<regs::Mb168bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word9(self) -> crate::common::Reg<regs::Mb364bWord9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word3(self) -> crate::common::Reg<regs::Mb632bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Message Buffer 11 CS Register"]
    #[inline(always)]
    pub const fn mb11_16b_cs(self) -> crate::common::Reg<regs::Mb1116bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Message Buffer 16 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb16_8b_word0(self) -> crate::common::Reg<regs::Mb168bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word10(self) -> crate::common::Reg<regs::Mb364bWord10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word4(self) -> crate::common::Reg<regs::Mb632bWord4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Message Buffer 16 WORD0 Register"]
    #[inline(always)]
    pub const fn word016(self) -> crate::common::Reg<regs::Word016, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Message Buffer 11 ID Register"]
    #[inline(always)]
    pub const fn mb11_16b_id(self) -> crate::common::Reg<regs::Mb1116bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Message Buffer 16 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb16_8b_word1(self) -> crate::common::Reg<regs::Mb168bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word11(self) -> crate::common::Reg<regs::Mb364bWord11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word5(self) -> crate::common::Reg<regs::Mb632bWord5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Message Buffer 16 WORD1 Register"]
    #[inline(always)]
    pub const fn word116(self) -> crate::common::Reg<regs::Word116, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Message Buffer 17 CS Register"]
    #[inline(always)]
    pub const fn cs17(self) -> crate::common::Reg<regs::Cs17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb11_16b_word0(self) -> crate::common::Reg<regs::Mb1116bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Message Buffer 17 CS Register"]
    #[inline(always)]
    pub const fn mb17_8b_cs(self) -> crate::common::Reg<regs::Mb178bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word12(self) -> crate::common::Reg<regs::Mb364bWord12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word6(self) -> crate::common::Reg<regs::Mb632bWord6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Message Buffer 17 ID Register"]
    #[inline(always)]
    pub const fn id17(self) -> crate::common::Reg<regs::Id17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb11_16b_word1(self) -> crate::common::Reg<regs::Mb1116bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Message Buffer 17 ID Register"]
    #[inline(always)]
    pub const fn mb17_8b_id(self) -> crate::common::Reg<regs::Mb178bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word13(self) -> crate::common::Reg<regs::Mb364bWord13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word7(self) -> crate::common::Reg<regs::Mb632bWord7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb11_16b_word2(self) -> crate::common::Reg<regs::Mb1116bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "Message Buffer 17 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb17_8b_word0(self) -> crate::common::Reg<regs::Mb178bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word14(self) -> crate::common::Reg<regs::Mb364bWord14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "Message Buffer 7 CS Register"]
    #[inline(always)]
    pub const fn mb7_32b_cs(self) -> crate::common::Reg<regs::Mb732bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "Message Buffer 17 WORD0 Register"]
    #[inline(always)]
    pub const fn word017(self) -> crate::common::Reg<regs::Word017, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb11_16b_word3(self) -> crate::common::Reg<regs::Mb1116bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "Message Buffer 17 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb17_8b_word1(self) -> crate::common::Reg<regs::Mb178bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word15(self) -> crate::common::Reg<regs::Mb364bWord15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "Message Buffer 7 ID Register"]
    #[inline(always)]
    pub const fn mb7_32b_id(self) -> crate::common::Reg<regs::Mb732bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "Message Buffer 17 WORD1 Register"]
    #[inline(always)]
    pub const fn word117(self) -> crate::common::Reg<regs::Word117, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "Message Buffer 18 CS Register"]
    #[inline(always)]
    pub const fn cs18(self) -> crate::common::Reg<regs::Cs18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "Message Buffer 12 CS Register"]
    #[inline(always)]
    pub const fn mb12_16b_cs(self) -> crate::common::Reg<regs::Mb1216bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "Message Buffer 18 CS Register"]
    #[inline(always)]
    pub const fn mb18_8b_cs(self) -> crate::common::Reg<regs::Mb188bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "Message Buffer 4 CS Register"]
    #[inline(always)]
    pub const fn mb4_64b_cs(self) -> crate::common::Reg<regs::Mb464bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word0(self) -> crate::common::Reg<regs::Mb732bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "Message Buffer 18 ID Register"]
    #[inline(always)]
    pub const fn id18(self) -> crate::common::Reg<regs::Id18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "Message Buffer 12 ID Register"]
    #[inline(always)]
    pub const fn mb12_16b_id(self) -> crate::common::Reg<regs::Mb1216bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "Message Buffer 18 ID Register"]
    #[inline(always)]
    pub const fn mb18_8b_id(self) -> crate::common::Reg<regs::Mb188bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "Message Buffer 4 ID Register"]
    #[inline(always)]
    pub const fn mb4_64b_id(self) -> crate::common::Reg<regs::Mb464bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word1(self) -> crate::common::Reg<regs::Mb732bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "Message Buffer 12 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb12_16b_word0(self) -> crate::common::Reg<regs::Mb1216bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "Message Buffer 18 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb18_8b_word0(self) -> crate::common::Reg<regs::Mb188bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word0(self) -> crate::common::Reg<regs::Mb464bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word2(self) -> crate::common::Reg<regs::Mb732bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "Message Buffer 18 WORD0 Register"]
    #[inline(always)]
    pub const fn word018(self) -> crate::common::Reg<regs::Word018, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "Message Buffer 12 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb12_16b_word1(self) -> crate::common::Reg<regs::Mb1216bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "Message Buffer 18 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb18_8b_word1(self) -> crate::common::Reg<regs::Mb188bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word1(self) -> crate::common::Reg<regs::Mb464bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word3(self) -> crate::common::Reg<regs::Mb732bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "Message Buffer 18 WORD1 Register"]
    #[inline(always)]
    pub const fn word118(self) -> crate::common::Reg<regs::Word118, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "Message Buffer 19 CS Register"]
    #[inline(always)]
    pub const fn cs19(self) -> crate::common::Reg<regs::Cs19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Message Buffer 12 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb12_16b_word2(self) -> crate::common::Reg<regs::Mb1216bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Message Buffer 19 CS Register"]
    #[inline(always)]
    pub const fn mb19_8b_cs(self) -> crate::common::Reg<regs::Mb198bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word2(self) -> crate::common::Reg<regs::Mb464bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word4(self) -> crate::common::Reg<regs::Mb732bWord4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Message Buffer 19 ID Register"]
    #[inline(always)]
    pub const fn id19(self) -> crate::common::Reg<regs::Id19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "Message Buffer 12 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb12_16b_word3(self) -> crate::common::Reg<regs::Mb1216bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "Message Buffer 19 ID Register"]
    #[inline(always)]
    pub const fn mb19_8b_id(self) -> crate::common::Reg<regs::Mb198bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word3(self) -> crate::common::Reg<regs::Mb464bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word5(self) -> crate::common::Reg<regs::Mb732bWord5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "Message Buffer 13 CS Register"]
    #[inline(always)]
    pub const fn mb13_16b_cs(self) -> crate::common::Reg<regs::Mb1316bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "Message Buffer 19 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb19_8b_word0(self) -> crate::common::Reg<regs::Mb198bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word4(self) -> crate::common::Reg<regs::Mb464bWord4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word6(self) -> crate::common::Reg<regs::Mb732bWord6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "Message Buffer 19 WORD0 Register"]
    #[inline(always)]
    pub const fn word019(self) -> crate::common::Reg<regs::Word019, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "Message Buffer 13 ID Register"]
    #[inline(always)]
    pub const fn mb13_16b_id(self) -> crate::common::Reg<regs::Mb1316bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "Message Buffer 19 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb19_8b_word1(self) -> crate::common::Reg<regs::Mb198bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word5(self) -> crate::common::Reg<regs::Mb464bWord5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word7(self) -> crate::common::Reg<regs::Mb732bWord7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "Message Buffer 19 WORD1 Register"]
    #[inline(always)]
    pub const fn word119(self) -> crate::common::Reg<regs::Word119, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "Message Buffer 20 CS Register"]
    #[inline(always)]
    pub const fn cs20(self) -> crate::common::Reg<regs::Cs20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "Message Buffer 13 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb13_16b_word0(self) -> crate::common::Reg<regs::Mb1316bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "Message Buffer 20 CS Register"]
    #[inline(always)]
    pub const fn mb20_8b_cs(self) -> crate::common::Reg<regs::Mb208bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word6(self) -> crate::common::Reg<regs::Mb464bWord6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "Message Buffer 8 CS Register"]
    #[inline(always)]
    pub const fn mb8_32b_cs(self) -> crate::common::Reg<regs::Mb832bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "Message Buffer 20 ID Register"]
    #[inline(always)]
    pub const fn id20(self) -> crate::common::Reg<regs::Id20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "Message Buffer 13 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb13_16b_word1(self) -> crate::common::Reg<regs::Mb1316bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "Message Buffer 20 ID Register"]
    #[inline(always)]
    pub const fn mb20_8b_id(self) -> crate::common::Reg<regs::Mb208bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word7(self) -> crate::common::Reg<regs::Mb464bWord7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "Message Buffer 8 ID Register"]
    #[inline(always)]
    pub const fn mb8_32b_id(self) -> crate::common::Reg<regs::Mb832bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "Message Buffer 13 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb13_16b_word2(self) -> crate::common::Reg<regs::Mb1316bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "Message Buffer 20 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb20_8b_word0(self) -> crate::common::Reg<regs::Mb208bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word8(self) -> crate::common::Reg<regs::Mb464bWord8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word0(self) -> crate::common::Reg<regs::Mb832bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "Message Buffer 20 WORD0 Register"]
    #[inline(always)]
    pub const fn word020(self) -> crate::common::Reg<regs::Word020, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "Message Buffer 13 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb13_16b_word3(self) -> crate::common::Reg<regs::Mb1316bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "Message Buffer 20 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb20_8b_word1(self) -> crate::common::Reg<regs::Mb208bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word9(self) -> crate::common::Reg<regs::Mb464bWord9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word1(self) -> crate::common::Reg<regs::Mb832bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "Message Buffer 20 WORD1 Register"]
    #[inline(always)]
    pub const fn word120(self) -> crate::common::Reg<regs::Word120, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "Message Buffer 21 CS Register"]
    #[inline(always)]
    pub const fn cs21(self) -> crate::common::Reg<regs::Cs21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "Message Buffer 14 CS Register"]
    #[inline(always)]
    pub const fn mb14_16b_cs(self) -> crate::common::Reg<regs::Mb1416bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "Message Buffer 21 CS Register"]
    #[inline(always)]
    pub const fn mb21_8b_cs(self) -> crate::common::Reg<regs::Mb218bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word10(self) -> crate::common::Reg<regs::Mb464bWord10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word2(self) -> crate::common::Reg<regs::Mb832bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "Message Buffer 21 ID Register"]
    #[inline(always)]
    pub const fn id21(self) -> crate::common::Reg<regs::Id21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "Message Buffer 14 ID Register"]
    #[inline(always)]
    pub const fn mb14_16b_id(self) -> crate::common::Reg<regs::Mb1416bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "Message Buffer 21 ID Register"]
    #[inline(always)]
    pub const fn mb21_8b_id(self) -> crate::common::Reg<regs::Mb218bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word11(self) -> crate::common::Reg<regs::Mb464bWord11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word3(self) -> crate::common::Reg<regs::Mb832bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "Message Buffer 14 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb14_16b_word0(self) -> crate::common::Reg<regs::Mb1416bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "Message Buffer 21 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb21_8b_word0(self) -> crate::common::Reg<regs::Mb218bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word12(self) -> crate::common::Reg<regs::Mb464bWord12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word4(self) -> crate::common::Reg<regs::Mb832bWord4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "Message Buffer 21 WORD0 Register"]
    #[inline(always)]
    pub const fn word021(self) -> crate::common::Reg<regs::Word021, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "Message Buffer 14 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb14_16b_word1(self) -> crate::common::Reg<regs::Mb1416bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "Message Buffer 21 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb21_8b_word1(self) -> crate::common::Reg<regs::Mb218bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word13(self) -> crate::common::Reg<regs::Mb464bWord13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word5(self) -> crate::common::Reg<regs::Mb832bWord5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "Message Buffer 21 WORD1 Register"]
    #[inline(always)]
    pub const fn word121(self) -> crate::common::Reg<regs::Word121, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "Message Buffer 22 CS Register"]
    #[inline(always)]
    pub const fn cs22(self) -> crate::common::Reg<regs::Cs22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "Message Buffer 14 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb14_16b_word2(self) -> crate::common::Reg<regs::Mb1416bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "Message Buffer 22 CS Register"]
    #[inline(always)]
    pub const fn mb22_8b_cs(self) -> crate::common::Reg<regs::Mb228bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word14(self) -> crate::common::Reg<regs::Mb464bWord14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word6(self) -> crate::common::Reg<regs::Mb832bWord6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "Message Buffer 22 ID Register"]
    #[inline(always)]
    pub const fn id22(self) -> crate::common::Reg<regs::Id22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "Message Buffer 14 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb14_16b_word3(self) -> crate::common::Reg<regs::Mb1416bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "Message Buffer 22 ID Register"]
    #[inline(always)]
    pub const fn mb22_8b_id(self) -> crate::common::Reg<regs::Mb228bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word15(self) -> crate::common::Reg<regs::Mb464bWord15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word7(self) -> crate::common::Reg<regs::Mb832bWord7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "Message Buffer 15 CS Register"]
    #[inline(always)]
    pub const fn mb15_16b_cs(self) -> crate::common::Reg<regs::Mb1516bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "Message Buffer 22 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb22_8b_word0(self) -> crate::common::Reg<regs::Mb228bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "Message Buffer 5 CS Register"]
    #[inline(always)]
    pub const fn mb5_64b_cs(self) -> crate::common::Reg<regs::Mb564bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "Message Buffer 9 CS Register"]
    #[inline(always)]
    pub const fn mb9_32b_cs(self) -> crate::common::Reg<regs::Mb932bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "Message Buffer 22 WORD0 Register"]
    #[inline(always)]
    pub const fn word022(self) -> crate::common::Reg<regs::Word022, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "Message Buffer 15 ID Register"]
    #[inline(always)]
    pub const fn mb15_16b_id(self) -> crate::common::Reg<regs::Mb1516bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "Message Buffer 22 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb22_8b_word1(self) -> crate::common::Reg<regs::Mb228bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "Message Buffer 5 ID Register"]
    #[inline(always)]
    pub const fn mb5_64b_id(self) -> crate::common::Reg<regs::Mb564bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "Message Buffer 9 ID Register"]
    #[inline(always)]
    pub const fn mb9_32b_id(self) -> crate::common::Reg<regs::Mb932bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "Message Buffer 22 WORD1 Register"]
    #[inline(always)]
    pub const fn word122(self) -> crate::common::Reg<regs::Word122, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "Message Buffer 23 CS Register"]
    #[inline(always)]
    pub const fn cs23(self) -> crate::common::Reg<regs::Cs23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "Message Buffer 15 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb15_16b_word0(self) -> crate::common::Reg<regs::Mb1516bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "Message Buffer 23 CS Register"]
    #[inline(always)]
    pub const fn mb23_8b_cs(self) -> crate::common::Reg<regs::Mb238bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word0(self) -> crate::common::Reg<regs::Mb564bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word0(self) -> crate::common::Reg<regs::Mb932bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "Message Buffer 23 ID Register"]
    #[inline(always)]
    pub const fn id23(self) -> crate::common::Reg<regs::Id23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "Message Buffer 15 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb15_16b_word1(self) -> crate::common::Reg<regs::Mb1516bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "Message Buffer 23 ID Register"]
    #[inline(always)]
    pub const fn mb23_8b_id(self) -> crate::common::Reg<regs::Mb238bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word1(self) -> crate::common::Reg<regs::Mb564bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word1(self) -> crate::common::Reg<regs::Mb932bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "Message Buffer 15 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb15_16b_word2(self) -> crate::common::Reg<regs::Mb1516bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "Message Buffer 23 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb23_8b_word0(self) -> crate::common::Reg<regs::Mb238bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word2(self) -> crate::common::Reg<regs::Mb564bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word2(self) -> crate::common::Reg<regs::Mb932bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "Message Buffer 23 WORD0 Register"]
    #[inline(always)]
    pub const fn word023(self) -> crate::common::Reg<regs::Word023, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "Message Buffer 15 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb15_16b_word3(self) -> crate::common::Reg<regs::Mb1516bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "Message Buffer 23 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb23_8b_word1(self) -> crate::common::Reg<regs::Mb238bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word3(self) -> crate::common::Reg<regs::Mb564bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word3(self) -> crate::common::Reg<regs::Mb932bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "Message Buffer 23 WORD1 Register"]
    #[inline(always)]
    pub const fn word123(self) -> crate::common::Reg<regs::Word123, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "Message Buffer 24 CS Register"]
    #[inline(always)]
    pub const fn cs24(self) -> crate::common::Reg<regs::Cs24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Message Buffer 16 CS Register"]
    #[inline(always)]
    pub const fn mb16_16b_cs(self) -> crate::common::Reg<regs::Mb1616bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Message Buffer 24 CS Register"]
    #[inline(always)]
    pub const fn mb24_8b_cs(self) -> crate::common::Reg<regs::Mb248bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word4(self) -> crate::common::Reg<regs::Mb564bWord4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word4(self) -> crate::common::Reg<regs::Mb932bWord4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Message Buffer 24 ID Register"]
    #[inline(always)]
    pub const fn id24(self) -> crate::common::Reg<regs::Id24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Message Buffer 16 ID Register"]
    #[inline(always)]
    pub const fn mb16_16b_id(self) -> crate::common::Reg<regs::Mb1616bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Message Buffer 24 ID Register"]
    #[inline(always)]
    pub const fn mb24_8b_id(self) -> crate::common::Reg<regs::Mb248bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word5(self) -> crate::common::Reg<regs::Mb564bWord5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word5(self) -> crate::common::Reg<regs::Mb932bWord5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Message Buffer 16 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb16_16b_word0(self) -> crate::common::Reg<regs::Mb1616bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Message Buffer 24 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb24_8b_word0(self) -> crate::common::Reg<regs::Mb248bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word6(self) -> crate::common::Reg<regs::Mb564bWord6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word6(self) -> crate::common::Reg<regs::Mb932bWord6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Message Buffer 24 WORD0 Register"]
    #[inline(always)]
    pub const fn word024(self) -> crate::common::Reg<regs::Word024, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Message Buffer 16 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb16_16b_word1(self) -> crate::common::Reg<regs::Mb1616bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "Message Buffer 24 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb24_8b_word1(self) -> crate::common::Reg<regs::Mb248bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word7(self) -> crate::common::Reg<regs::Mb564bWord7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word7(self) -> crate::common::Reg<regs::Mb932bWord7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "Message Buffer 24 WORD1 Register"]
    #[inline(always)]
    pub const fn word124(self) -> crate::common::Reg<regs::Word124, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "Message Buffer 25 CS Register"]
    #[inline(always)]
    pub const fn cs25(self) -> crate::common::Reg<regs::Cs25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Message Buffer 10 CS Register"]
    #[inline(always)]
    pub const fn mb10_32b_cs(self) -> crate::common::Reg<regs::Mb1032bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Message Buffer 16 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb16_16b_word2(self) -> crate::common::Reg<regs::Mb1616bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Message Buffer 25 CS Register"]
    #[inline(always)]
    pub const fn mb25_8b_cs(self) -> crate::common::Reg<regs::Mb258bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word8(self) -> crate::common::Reg<regs::Mb564bWord8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Message Buffer 25 ID Register"]
    #[inline(always)]
    pub const fn id25(self) -> crate::common::Reg<regs::Id25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Message Buffer 10 ID Register"]
    #[inline(always)]
    pub const fn mb10_32b_id(self) -> crate::common::Reg<regs::Mb1032bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Message Buffer 16 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb16_16b_word3(self) -> crate::common::Reg<regs::Mb1616bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Message Buffer 25 ID Register"]
    #[inline(always)]
    pub const fn mb25_8b_id(self) -> crate::common::Reg<regs::Mb258bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word9(self) -> crate::common::Reg<regs::Mb564bWord9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word0(self) -> crate::common::Reg<regs::Mb1032bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Message Buffer 17 CS Register"]
    #[inline(always)]
    pub const fn mb17_16b_cs(self) -> crate::common::Reg<regs::Mb1716bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Message Buffer 25 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb25_8b_word0(self) -> crate::common::Reg<regs::Mb258bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word10(self) -> crate::common::Reg<regs::Mb564bWord10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Message Buffer 25 WORD0 Register"]
    #[inline(always)]
    pub const fn word025(self) -> crate::common::Reg<regs::Word025, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word1(self) -> crate::common::Reg<regs::Mb1032bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "Message Buffer 17 ID Register"]
    #[inline(always)]
    pub const fn mb17_16b_id(self) -> crate::common::Reg<regs::Mb1716bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "Message Buffer 25 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb25_8b_word1(self) -> crate::common::Reg<regs::Mb258bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word11(self) -> crate::common::Reg<regs::Mb564bWord11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "Message Buffer 25 WORD1 Register"]
    #[inline(always)]
    pub const fn word125(self) -> crate::common::Reg<regs::Word125, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "Message Buffer 26 CS Register"]
    #[inline(always)]
    pub const fn cs26(self) -> crate::common::Reg<regs::Cs26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word2(self) -> crate::common::Reg<regs::Mb1032bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "Message Buffer 17 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb17_16b_word0(self) -> crate::common::Reg<regs::Mb1716bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "Message Buffer 26 CS Register"]
    #[inline(always)]
    pub const fn mb26_8b_cs(self) -> crate::common::Reg<regs::Mb268bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word12(self) -> crate::common::Reg<regs::Mb564bWord12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "Message Buffer 26 ID Register"]
    #[inline(always)]
    pub const fn id26(self) -> crate::common::Reg<regs::Id26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word3(self) -> crate::common::Reg<regs::Mb1032bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "Message Buffer 17 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb17_16b_word1(self) -> crate::common::Reg<regs::Mb1716bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "Message Buffer 26 ID Register"]
    #[inline(always)]
    pub const fn mb26_8b_id(self) -> crate::common::Reg<regs::Mb268bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word13(self) -> crate::common::Reg<regs::Mb564bWord13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word4(self) -> crate::common::Reg<regs::Mb1032bWord4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "Message Buffer 17 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb17_16b_word2(self) -> crate::common::Reg<regs::Mb1716bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "Message Buffer 26 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb26_8b_word0(self) -> crate::common::Reg<regs::Mb268bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word14(self) -> crate::common::Reg<regs::Mb564bWord14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "Message Buffer 26 WORD0 Register"]
    #[inline(always)]
    pub const fn word026(self) -> crate::common::Reg<regs::Word026, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word5(self) -> crate::common::Reg<regs::Mb1032bWord5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "Message Buffer 17 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb17_16b_word3(self) -> crate::common::Reg<regs::Mb1716bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "Message Buffer 26 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb26_8b_word1(self) -> crate::common::Reg<regs::Mb268bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word15(self) -> crate::common::Reg<regs::Mb564bWord15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "Message Buffer 26 WORD1 Register"]
    #[inline(always)]
    pub const fn word126(self) -> crate::common::Reg<regs::Word126, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "Message Buffer 27 CS Register"]
    #[inline(always)]
    pub const fn cs27(self) -> crate::common::Reg<regs::Cs27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word6(self) -> crate::common::Reg<regs::Mb1032bWord6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "Message Buffer 18 CS Register"]
    #[inline(always)]
    pub const fn mb18_16b_cs(self) -> crate::common::Reg<regs::Mb1816bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "Message Buffer 27 CS Register"]
    #[inline(always)]
    pub const fn mb27_8b_cs(self) -> crate::common::Reg<regs::Mb278bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "Message Buffer 6 CS Register"]
    #[inline(always)]
    pub const fn mb6_64b_cs(self) -> crate::common::Reg<regs::Mb664bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "Message Buffer 27 ID Register"]
    #[inline(always)]
    pub const fn id27(self) -> crate::common::Reg<regs::Id27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word7(self) -> crate::common::Reg<regs::Mb1032bWord7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "Message Buffer 18 ID Register"]
    #[inline(always)]
    pub const fn mb18_16b_id(self) -> crate::common::Reg<regs::Mb1816bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "Message Buffer 27 ID Register"]
    #[inline(always)]
    pub const fn mb27_8b_id(self) -> crate::common::Reg<regs::Mb278bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "Message Buffer 6 ID Register"]
    #[inline(always)]
    pub const fn mb6_64b_id(self) -> crate::common::Reg<regs::Mb664bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "Message Buffer 11 CS Register"]
    #[inline(always)]
    pub const fn mb11_32b_cs(self) -> crate::common::Reg<regs::Mb1132bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "Message Buffer 18 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb18_16b_word0(self) -> crate::common::Reg<regs::Mb1816bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "Message Buffer 27 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb27_8b_word0(self) -> crate::common::Reg<regs::Mb278bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word0(self) -> crate::common::Reg<regs::Mb664bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "Message Buffer 27 WORD0 Register"]
    #[inline(always)]
    pub const fn word027(self) -> crate::common::Reg<regs::Word027, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "Message Buffer 11 ID Register"]
    #[inline(always)]
    pub const fn mb11_32b_id(self) -> crate::common::Reg<regs::Mb1132bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "Message Buffer 18 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb18_16b_word1(self) -> crate::common::Reg<regs::Mb1816bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "Message Buffer 27 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb27_8b_word1(self) -> crate::common::Reg<regs::Mb278bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word1(self) -> crate::common::Reg<regs::Mb664bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "Message Buffer 27 WORD1 Register"]
    #[inline(always)]
    pub const fn word127(self) -> crate::common::Reg<regs::Word127, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "Message Buffer 28 CS Register"]
    #[inline(always)]
    pub const fn cs28(self) -> crate::common::Reg<regs::Cs28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word0(self) -> crate::common::Reg<regs::Mb1132bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "Message Buffer 18 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb18_16b_word2(self) -> crate::common::Reg<regs::Mb1816bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "Message Buffer 28 CS Register"]
    #[inline(always)]
    pub const fn mb28_8b_cs(self) -> crate::common::Reg<regs::Mb288bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word2(self) -> crate::common::Reg<regs::Mb664bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "Message Buffer 28 ID Register"]
    #[inline(always)]
    pub const fn id28(self) -> crate::common::Reg<regs::Id28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word1(self) -> crate::common::Reg<regs::Mb1132bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "Message Buffer 18 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb18_16b_word3(self) -> crate::common::Reg<regs::Mb1816bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "Message Buffer 28 ID Register"]
    #[inline(always)]
    pub const fn mb28_8b_id(self) -> crate::common::Reg<regs::Mb288bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word3(self) -> crate::common::Reg<regs::Mb664bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word2(self) -> crate::common::Reg<regs::Mb1132bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "Message Buffer 19 CS Register"]
    #[inline(always)]
    pub const fn mb19_16b_cs(self) -> crate::common::Reg<regs::Mb1916bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "Message Buffer 28 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb28_8b_word0(self) -> crate::common::Reg<regs::Mb288bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word4(self) -> crate::common::Reg<regs::Mb664bWord4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "Message Buffer 28 WORD0 Register"]
    #[inline(always)]
    pub const fn word028(self) -> crate::common::Reg<regs::Word028, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word3(self) -> crate::common::Reg<regs::Mb1132bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "Message Buffer 19 ID Register"]
    #[inline(always)]
    pub const fn mb19_16b_id(self) -> crate::common::Reg<regs::Mb1916bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "Message Buffer 28 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb28_8b_word1(self) -> crate::common::Reg<regs::Mb288bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word5(self) -> crate::common::Reg<regs::Mb664bWord5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "Message Buffer 28 WORD1 Register"]
    #[inline(always)]
    pub const fn word128(self) -> crate::common::Reg<regs::Word128, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "Message Buffer 29 CS Register"]
    #[inline(always)]
    pub const fn cs29(self) -> crate::common::Reg<regs::Cs29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word4(self) -> crate::common::Reg<regs::Mb1132bWord4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "Message Buffer 19 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb19_16b_word0(self) -> crate::common::Reg<regs::Mb1916bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "Message Buffer 29 CS Register"]
    #[inline(always)]
    pub const fn mb29_8b_cs(self) -> crate::common::Reg<regs::Mb298bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word6(self) -> crate::common::Reg<regs::Mb664bWord6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "Message Buffer 29 ID Register"]
    #[inline(always)]
    pub const fn id29(self) -> crate::common::Reg<regs::Id29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word5(self) -> crate::common::Reg<regs::Mb1132bWord5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "Message Buffer 19 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb19_16b_word1(self) -> crate::common::Reg<regs::Mb1916bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "Message Buffer 29 ID Register"]
    #[inline(always)]
    pub const fn mb29_8b_id(self) -> crate::common::Reg<regs::Mb298bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word7(self) -> crate::common::Reg<regs::Mb664bWord7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word6(self) -> crate::common::Reg<regs::Mb1132bWord6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "Message Buffer 19 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb19_16b_word2(self) -> crate::common::Reg<regs::Mb1916bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "Message Buffer 29 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb29_8b_word0(self) -> crate::common::Reg<regs::Mb298bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word8(self) -> crate::common::Reg<regs::Mb664bWord8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "Message Buffer 29 WORD0 Register"]
    #[inline(always)]
    pub const fn word029(self) -> crate::common::Reg<regs::Word029, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word7(self) -> crate::common::Reg<regs::Mb1132bWord7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "Message Buffer 19 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb19_16b_word3(self) -> crate::common::Reg<regs::Mb1916bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "Message Buffer 29 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb29_8b_word1(self) -> crate::common::Reg<regs::Mb298bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word9(self) -> crate::common::Reg<regs::Mb664bWord9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "Message Buffer 29 WORD1 Register"]
    #[inline(always)]
    pub const fn word129(self) -> crate::common::Reg<regs::Word129, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "Message Buffer 30 CS Register"]
    #[inline(always)]
    pub const fn cs30(self) -> crate::common::Reg<regs::Cs30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "Message Buffer 20 CS Register"]
    #[inline(always)]
    pub const fn mb20_16b_cs(self) -> crate::common::Reg<regs::Mb2016bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "Message Buffer 30 CS Register"]
    #[inline(always)]
    pub const fn mb30_8b_cs(self) -> crate::common::Reg<regs::Mb308bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word10(self) -> crate::common::Reg<regs::Mb664bWord10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "Message Buffer 30 ID Register"]
    #[inline(always)]
    pub const fn id30(self) -> crate::common::Reg<regs::Id30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "Message Buffer 20 ID Register"]
    #[inline(always)]
    pub const fn mb20_16b_id(self) -> crate::common::Reg<regs::Mb2016bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "Message Buffer 30 ID Register"]
    #[inline(always)]
    pub const fn mb30_8b_id(self) -> crate::common::Reg<regs::Mb308bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word11(self) -> crate::common::Reg<regs::Mb664bWord11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "Message Buffer 20 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb20_16b_word0(self) -> crate::common::Reg<regs::Mb2016bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "Message Buffer 30 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb30_8b_word0(self) -> crate::common::Reg<regs::Mb308bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word12(self) -> crate::common::Reg<regs::Mb664bWord12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "Message Buffer 30 WORD0 Register"]
    #[inline(always)]
    pub const fn word030(self) -> crate::common::Reg<regs::Word030, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "Message Buffer 20 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb20_16b_word1(self) -> crate::common::Reg<regs::Mb2016bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "Message Buffer 30 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb30_8b_word1(self) -> crate::common::Reg<regs::Mb308bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word13(self) -> crate::common::Reg<regs::Mb664bWord13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "Message Buffer 30 WORD1 Register"]
    #[inline(always)]
    pub const fn word130(self) -> crate::common::Reg<regs::Word130, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "Message Buffer 31 CS Register"]
    #[inline(always)]
    pub const fn cs31(self) -> crate::common::Reg<regs::Cs31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "Message Buffer 20 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb20_16b_word2(self) -> crate::common::Reg<regs::Mb2016bWord2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "Message Buffer 31 CS Register"]
    #[inline(always)]
    pub const fn mb31_8b_cs(self) -> crate::common::Reg<regs::Mb318bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word14(self) -> crate::common::Reg<regs::Mb664bWord14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "Message Buffer 31 ID Register"]
    #[inline(always)]
    pub const fn id31(self) -> crate::common::Reg<regs::Id31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "Message Buffer 20 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb20_16b_word3(self) -> crate::common::Reg<regs::Mb2016bWord3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "Message Buffer 31 ID Register"]
    #[inline(always)]
    pub const fn mb31_8b_id(self) -> crate::common::Reg<regs::Mb318bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word15(self) -> crate::common::Reg<regs::Mb664bWord15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "Message Buffer 31 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb31_8b_word0(self) -> crate::common::Reg<regs::Mb318bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0278usize) as _) }
    }
    #[doc = "Message Buffer 31 WORD0 Register"]
    #[inline(always)]
    pub const fn word031(self) -> crate::common::Reg<regs::Word031, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0278usize) as _) }
    }
    #[doc = "Message Buffer 31 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb31_8b_word1(self) -> crate::common::Reg<regs::Mb318bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x027cusize) as _) }
    }
    #[doc = "Message Buffer 31 WORD1 Register"]
    #[inline(always)]
    pub const fn word131(self) -> crate::common::Reg<regs::Word131, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x027cusize) as _) }
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
    #[doc = "Array of registers: WMB_CS, WMB_ID, WMB_D03, WMB_D47"]
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
#[doc = "Array of registers: WMB_CS, WMB_ID, WMB_D03, WMB_D47"]
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
