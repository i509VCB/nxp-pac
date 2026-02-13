#[doc = "CAN"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can3 {
    ptr: *mut u8,
}
unsafe impl Send for Can3 {}
unsafe impl Sync for Can3 {}
impl Can3 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Module Configuration Register"]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control 1 Register"]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Free Running Timer"]
    #[inline(always)]
    pub const fn timer(self) -> crate::common::Reg<regs::Timer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Rx Mailboxes Global Mask Register"]
    #[inline(always)]
    pub const fn rxmgmask(self) -> crate::common::Reg<regs::Rxmgmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Rx 14 Mask Register"]
    #[inline(always)]
    pub const fn rx14mask(self) -> crate::common::Reg<regs::Rx14mask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Rx 15 Mask Register"]
    #[inline(always)]
    pub const fn rx15mask(self) -> crate::common::Reg<regs::Rx15mask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Error Counter"]
    #[inline(always)]
    pub const fn ecr(self) -> crate::common::Reg<regs::Ecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Error and Status 1 Register"]
    #[inline(always)]
    pub const fn esr1(self) -> crate::common::Reg<regs::Esr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Interrupt Masks 2 Register"]
    #[inline(always)]
    pub const fn imask2(self) -> crate::common::Reg<regs::Imask2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Interrupt Masks 1 Register"]
    #[inline(always)]
    pub const fn imask1(self) -> crate::common::Reg<regs::Imask1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Interrupt Flags 2 Register"]
    #[inline(always)]
    pub const fn iflag2(self) -> crate::common::Reg<regs::Iflag2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Interrupt Flags 1 Register"]
    #[inline(always)]
    pub const fn iflag1(self) -> crate::common::Reg<regs::Iflag1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Control 2 Register"]
    #[inline(always)]
    pub const fn ctrl2(self) -> crate::common::Reg<regs::Ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Error and Status 2 Register"]
    #[inline(always)]
    pub const fn esr2(self) -> crate::common::Reg<regs::Esr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "CRC Register"]
    #[inline(always)]
    pub const fn crcr(self) -> crate::common::Reg<regs::Crcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Rx FIFO Global Mask Register"]
    #[inline(always)]
    pub const fn rxfgmask(self) -> crate::common::Reg<regs::Rxfgmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Rx FIFO Information Register"]
    #[inline(always)]
    pub const fn rxfir(self) -> crate::common::Reg<regs::Rxfir, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "CAN Bit Timing Register"]
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
    pub const fn mb0_16b_cs_l(self) -> crate::common::Reg<regs::Mb016bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Message Buffer 0 CS Register"]
    #[inline(always)]
    pub const fn mb0_32b_cs_l(self) -> crate::common::Reg<regs::Mb032bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Message Buffer 0 CS Register"]
    #[inline(always)]
    pub const fn mb0_64b_cs_l(self) -> crate::common::Reg<regs::Mb064bCsL, crate::common::RW> {
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
    pub const fn mb0_16b_id_l(self) -> crate::common::Reg<regs::Mb016bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn mb0_32b_id_l(self) -> crate::common::Reg<regs::Mb032bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn mb0_64b_id_l(self) -> crate::common::Reg<regs::Mb064bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn mb0_8b_id(self) -> crate::common::Reg<regs::Mb08bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb0_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb016bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb032bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord0L, crate::common::RW> {
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
    pub const fn mb0_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb016bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb032bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord1L, crate::common::RW> {
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
    pub const fn mb0_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb016bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb032bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord2L, crate::common::RW> {
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
    pub const fn mb0_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb016bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb032bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Message Buffer 1 ID Register"]
    #[inline(always)]
    pub const fn mb1_8b_id(self) -> crate::common::Reg<regs::Mb18bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word4_l(
        self,
    ) -> crate::common::Reg<regs::Mb032bWord4L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word4_l(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord4L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Message Buffer 1 CS Register"]
    #[inline(always)]
    pub const fn mb1_16b_cs_l(self) -> crate::common::Reg<regs::Mb116bCsL, crate::common::RW> {
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
    pub const fn mb0_32b_word5_l(
        self,
    ) -> crate::common::Reg<regs::Mb032bWord5L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word5_l(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord5L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Message Buffer 1 ID Register"]
    #[inline(always)]
    pub const fn mb1_16b_id_l(self) -> crate::common::Reg<regs::Mb116bIdL, crate::common::RW> {
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
    pub const fn mb0_32b_word6_l(
        self,
    ) -> crate::common::Reg<regs::Mb032bWord6L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word6_l(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord6L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb1_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb116bWord0L, crate::common::RW> {
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
    pub const fn mb0_32b_word7_l(
        self,
    ) -> crate::common::Reg<regs::Mb032bWord7L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word7_l(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord7L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb1_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb116bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Message Buffer 2 ID Register"]
    #[inline(always)]
    pub const fn mb2_8b_id(self) -> crate::common::Reg<regs::Mb28bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word8_l(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord8L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb1_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb116bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Message Buffer 1 CS Register"]
    #[inline(always)]
    pub const fn mb1_32b_cs_l(self) -> crate::common::Reg<regs::Mb132bCsL, crate::common::RW> {
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
    pub const fn mb0_64b_word9_l(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord9L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb1_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb116bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Message Buffer 1 ID Register"]
    #[inline(always)]
    pub const fn mb1_32b_id_l(self) -> crate::common::Reg<regs::Mb132bIdL, crate::common::RW> {
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
    pub const fn mb0_64b_word10_l(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord10L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb132bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Message Buffer 2 CS Register"]
    #[inline(always)]
    pub const fn mb2_16b_cs_l(self) -> crate::common::Reg<regs::Mb216bCsL, crate::common::RW> {
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
    pub const fn mb0_64b_word11_l(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord11L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb132bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Message Buffer 2 ID Register"]
    #[inline(always)]
    pub const fn mb2_16b_id_l(self) -> crate::common::Reg<regs::Mb216bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Message Buffer 3 ID Register"]
    #[inline(always)]
    pub const fn mb3_8b_id(self) -> crate::common::Reg<regs::Mb38bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word12_l(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord12L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb132bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb2_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb216bWord0L, crate::common::RW> {
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
    pub const fn mb0_64b_word13_l(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord13L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb132bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb2_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb216bWord1L, crate::common::RW> {
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
    pub const fn mb0_64b_word14_l(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord14L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word4_l(
        self,
    ) -> crate::common::Reg<regs::Mb132bWord4L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb2_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb216bWord2L, crate::common::RW> {
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
    pub const fn mb0_64b_word15_l(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord15L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word5_l(
        self,
    ) -> crate::common::Reg<regs::Mb132bWord5L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb2_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb216bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Message Buffer 4 ID Register"]
    #[inline(always)]
    pub const fn mb4_8b_id(self) -> crate::common::Reg<regs::Mb48bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word6_l(
        self,
    ) -> crate::common::Reg<regs::Mb132bWord6L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Message Buffer 1 CS Register"]
    #[inline(always)]
    pub const fn mb1_64b_cs_l(self) -> crate::common::Reg<regs::Mb164bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Message Buffer 3 CS Register"]
    #[inline(always)]
    pub const fn mb3_16b_cs_l(self) -> crate::common::Reg<regs::Mb316bCsL, crate::common::RW> {
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
    pub const fn mb1_32b_word7_l(
        self,
    ) -> crate::common::Reg<regs::Mb132bWord7L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Message Buffer 1 ID Register"]
    #[inline(always)]
    pub const fn mb1_64b_id_l(self) -> crate::common::Reg<regs::Mb164bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Message Buffer 3 ID Register"]
    #[inline(always)]
    pub const fn mb3_16b_id_l(self) -> crate::common::Reg<regs::Mb316bIdL, crate::common::RW> {
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
    pub const fn mb1_64b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Message Buffer 2 CS Register"]
    #[inline(always)]
    pub const fn mb2_32b_cs_l(self) -> crate::common::Reg<regs::Mb232bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb3_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb316bWord0L, crate::common::RW> {
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
    pub const fn mb1_64b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Message Buffer 2 ID Register"]
    #[inline(always)]
    pub const fn mb2_32b_id_l(self) -> crate::common::Reg<regs::Mb232bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb3_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb316bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Message Buffer 5 ID Register"]
    #[inline(always)]
    pub const fn mb5_8b_id(self) -> crate::common::Reg<regs::Mb58bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb232bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb3_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb316bWord2L, crate::common::RW> {
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
    pub const fn mb1_64b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb232bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb3_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb316bWord3L, crate::common::RW> {
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
    pub const fn mb1_64b_word4_l(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord4L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb232bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Message Buffer 4 CS Register"]
    #[inline(always)]
    pub const fn mb4_16b_cs_l(self) -> crate::common::Reg<regs::Mb416bCsL, crate::common::RW> {
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
    pub const fn mb1_64b_word5_l(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord5L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb232bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Message Buffer 4 ID Register"]
    #[inline(always)]
    pub const fn mb4_16b_id_l(self) -> crate::common::Reg<regs::Mb416bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Message Buffer 6 ID Register"]
    #[inline(always)]
    pub const fn mb6_8b_id(self) -> crate::common::Reg<regs::Mb68bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word6_l(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord6L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word4_l(
        self,
    ) -> crate::common::Reg<regs::Mb232bWord4L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb4_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb416bWord0L, crate::common::RW> {
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
    pub const fn mb1_64b_word7_l(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord7L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word5_l(
        self,
    ) -> crate::common::Reg<regs::Mb232bWord5L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb4_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb416bWord1L, crate::common::RW> {
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
    pub const fn mb1_64b_word8_l(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord8L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word6_l(
        self,
    ) -> crate::common::Reg<regs::Mb232bWord6L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb4_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb416bWord2L, crate::common::RW> {
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
    pub const fn mb1_64b_word9_l(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord9L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word7_l(
        self,
    ) -> crate::common::Reg<regs::Mb232bWord7L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb4_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb416bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Message Buffer 7 ID Register"]
    #[inline(always)]
    pub const fn mb7_8b_id(self) -> crate::common::Reg<regs::Mb78bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word10_l(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord10L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Message Buffer 3 CS Register"]
    #[inline(always)]
    pub const fn mb3_32b_cs_l(self) -> crate::common::Reg<regs::Mb332bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Message Buffer 5 CS Register"]
    #[inline(always)]
    pub const fn mb5_16b_cs_l(self) -> crate::common::Reg<regs::Mb516bCsL, crate::common::RW> {
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
    pub const fn mb1_64b_word11_l(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord11L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Message Buffer 3 ID Register"]
    #[inline(always)]
    pub const fn mb3_32b_id_l(self) -> crate::common::Reg<regs::Mb332bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Message Buffer 5 ID Register"]
    #[inline(always)]
    pub const fn mb5_16b_id_l(self) -> crate::common::Reg<regs::Mb516bIdL, crate::common::RW> {
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
    pub const fn mb1_64b_word12_l(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord12L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb332bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb5_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb516bWord0L, crate::common::RW> {
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
    pub const fn mb1_64b_word13_l(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord13L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb332bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb5_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb516bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Message Buffer 8 ID Register"]
    #[inline(always)]
    pub const fn mb8_8b_id(self) -> crate::common::Reg<regs::Mb88bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word14_l(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord14L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb332bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb5_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb516bWord2L, crate::common::RW> {
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
    pub const fn mb1_64b_word15_l(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord15L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb332bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb5_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb516bWord3L, crate::common::RW> {
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
    pub const fn mb2_64b_cs_l(self) -> crate::common::Reg<regs::Mb264bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word4_l(
        self,
    ) -> crate::common::Reg<regs::Mb332bWord4L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Message Buffer 6 CS Register"]
    #[inline(always)]
    pub const fn mb6_16b_cs_l(self) -> crate::common::Reg<regs::Mb616bCsL, crate::common::RW> {
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
    pub const fn mb2_64b_id_l(self) -> crate::common::Reg<regs::Mb264bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word5_l(
        self,
    ) -> crate::common::Reg<regs::Mb332bWord5L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Message Buffer 6 ID Register"]
    #[inline(always)]
    pub const fn mb6_16b_id_l(self) -> crate::common::Reg<regs::Mb616bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Message Buffer 9 ID Register"]
    #[inline(always)]
    pub const fn mb9_8b_id(self) -> crate::common::Reg<regs::Mb98bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word6_l(
        self,
    ) -> crate::common::Reg<regs::Mb332bWord6L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb6_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb616bWord0L, crate::common::RW> {
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
    pub const fn mb2_64b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word7_l(
        self,
    ) -> crate::common::Reg<regs::Mb332bWord7L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb6_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb616bWord1L, crate::common::RW> {
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
    pub const fn mb2_64b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Message Buffer 4 CS Register"]
    #[inline(always)]
    pub const fn mb4_32b_cs_l(self) -> crate::common::Reg<regs::Mb432bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb6_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb616bWord2L, crate::common::RW> {
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
    pub const fn mb2_64b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Message Buffer 4 ID Register"]
    #[inline(always)]
    pub const fn mb4_32b_id_l(self) -> crate::common::Reg<regs::Mb432bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb6_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb616bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb10_8b_word0(self) -> crate::common::Reg<regs::Mb108bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word4_l(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord4L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb432bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Message Buffer 7 CS Register"]
    #[inline(always)]
    pub const fn mb7_16b_cs_l(self) -> crate::common::Reg<regs::Mb716bCsL, crate::common::RW> {
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
    pub const fn mb2_64b_word5_l(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord5L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb432bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Message Buffer 7 ID Register"]
    #[inline(always)]
    pub const fn mb7_16b_id_l(self) -> crate::common::Reg<regs::Mb716bIdL, crate::common::RW> {
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
    pub const fn mb2_64b_word6_l(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord6L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb432bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb7_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb716bWord0L, crate::common::RW> {
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
    pub const fn mb2_64b_word7_l(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord7L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb432bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb7_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb716bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb11_8b_word0(self) -> crate::common::Reg<regs::Mb118bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word8_l(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord8L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word4_l(
        self,
    ) -> crate::common::Reg<regs::Mb432bWord4L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb7_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb716bWord2L, crate::common::RW> {
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
    pub const fn mb2_64b_word9_l(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord9L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word5_l(
        self,
    ) -> crate::common::Reg<regs::Mb432bWord5L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb7_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb716bWord3L, crate::common::RW> {
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
    pub const fn mb2_64b_word10_l(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord10L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word6_l(
        self,
    ) -> crate::common::Reg<regs::Mb432bWord6L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Message Buffer 8 CS Register"]
    #[inline(always)]
    pub const fn mb8_16b_cs_l(self) -> crate::common::Reg<regs::Mb816bCsL, crate::common::RW> {
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
    pub const fn mb2_64b_word11_l(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord11L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word7_l(
        self,
    ) -> crate::common::Reg<regs::Mb432bWord7L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Message Buffer 8 ID Register"]
    #[inline(always)]
    pub const fn mb8_16b_id_l(self) -> crate::common::Reg<regs::Mb816bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Message Buffer 12 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb12_8b_word0(self) -> crate::common::Reg<regs::Mb128bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word12_l(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord12L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Message Buffer 5 CS Register"]
    #[inline(always)]
    pub const fn mb5_32b_cs_l(self) -> crate::common::Reg<regs::Mb532bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb8_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb816bWord0L, crate::common::RW> {
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
    pub const fn mb2_64b_word13_l(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord13L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Message Buffer 5 ID Register"]
    #[inline(always)]
    pub const fn mb5_32b_id_l(self) -> crate::common::Reg<regs::Mb532bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb8_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb816bWord1L, crate::common::RW> {
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
    pub const fn mb2_64b_word14_l(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord14L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb532bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb8_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb816bWord2L, crate::common::RW> {
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
    pub const fn mb2_64b_word15_l(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord15L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb532bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb8_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb816bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Message Buffer 13 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb13_8b_word0(self) -> crate::common::Reg<regs::Mb138bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Message Buffer 3 CS Register"]
    #[inline(always)]
    pub const fn mb3_64b_cs_l(self) -> crate::common::Reg<regs::Mb364bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb532bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Message Buffer 9 CS Register"]
    #[inline(always)]
    pub const fn mb9_16b_cs_l(self) -> crate::common::Reg<regs::Mb916bCsL, crate::common::RW> {
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
    pub const fn mb3_64b_id_l(self) -> crate::common::Reg<regs::Mb364bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb532bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Message Buffer 9 ID Register"]
    #[inline(always)]
    pub const fn mb9_16b_id_l(self) -> crate::common::Reg<regs::Mb916bIdL, crate::common::RW> {
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
    pub const fn mb3_64b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word4_l(
        self,
    ) -> crate::common::Reg<regs::Mb532bWord4L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb9_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb916bWord0L, crate::common::RW> {
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
    pub const fn mb3_64b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word5_l(
        self,
    ) -> crate::common::Reg<regs::Mb532bWord5L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb9_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb916bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Message Buffer 14 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb14_8b_word0(self) -> crate::common::Reg<regs::Mb148bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word6_l(
        self,
    ) -> crate::common::Reg<regs::Mb532bWord6L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb9_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb916bWord2L, crate::common::RW> {
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
    pub const fn mb3_64b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word7_l(
        self,
    ) -> crate::common::Reg<regs::Mb532bWord7L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb9_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb916bWord3L, crate::common::RW> {
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
    pub const fn mb10_16b_cs_l(self) -> crate::common::Reg<regs::Mb1016bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Message Buffer 15 CS Register"]
    #[inline(always)]
    pub const fn mb15_8b_cs(self) -> crate::common::Reg<regs::Mb158bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word4_l(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord4L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Message Buffer 6 CS Register"]
    #[inline(always)]
    pub const fn mb6_32b_cs_l(self) -> crate::common::Reg<regs::Mb632bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Message Buffer 15 ID Register"]
    #[inline(always)]
    pub const fn id15(self) -> crate::common::Reg<regs::Id15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Message Buffer 10 ID Register"]
    #[inline(always)]
    pub const fn mb10_16b_id_l(self) -> crate::common::Reg<regs::Mb1016bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Message Buffer 15 ID Register"]
    #[inline(always)]
    pub const fn mb15_8b_id(self) -> crate::common::Reg<regs::Mb158bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word5_l(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord5L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Message Buffer 6 ID Register"]
    #[inline(always)]
    pub const fn mb6_32b_id_l(self) -> crate::common::Reg<regs::Mb632bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb10_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb1016bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Message Buffer 15 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb15_8b_word0(self) -> crate::common::Reg<regs::Mb158bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word6_l(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord6L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb632bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Message Buffer 15 WORD0 Register"]
    #[inline(always)]
    pub const fn word015(self) -> crate::common::Reg<regs::Word015, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb10_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb1016bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "Message Buffer 15 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb15_8b_word1(self) -> crate::common::Reg<regs::Mb158bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word7_l(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord7L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb632bWord1L, crate::common::RW> {
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
    pub const fn mb10_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb1016bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Message Buffer 16 CS Register"]
    #[inline(always)]
    pub const fn mb16_8b_cs(self) -> crate::common::Reg<regs::Mb168bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word8_l(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord8L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb632bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Message Buffer 16 ID Register"]
    #[inline(always)]
    pub const fn id16(self) -> crate::common::Reg<regs::Id16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb10_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb1016bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Message Buffer 16 ID Register"]
    #[inline(always)]
    pub const fn mb16_8b_id(self) -> crate::common::Reg<regs::Mb168bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word9_l(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord9L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb632bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Message Buffer 11 CS Register"]
    #[inline(always)]
    pub const fn mb11_16b_cs_l(self) -> crate::common::Reg<regs::Mb1116bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Message Buffer 16 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb16_8b_word0(self) -> crate::common::Reg<regs::Mb168bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word10_l(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord10L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word4_l(
        self,
    ) -> crate::common::Reg<regs::Mb632bWord4L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Message Buffer 16 WORD0 Register"]
    #[inline(always)]
    pub const fn word016(self) -> crate::common::Reg<regs::Word016, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Message Buffer 11 ID Register"]
    #[inline(always)]
    pub const fn mb11_16b_id_l(self) -> crate::common::Reg<regs::Mb1116bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Message Buffer 16 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb16_8b_word1(self) -> crate::common::Reg<regs::Mb168bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word11_l(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord11L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word5_l(
        self,
    ) -> crate::common::Reg<regs::Mb632bWord5L, crate::common::RW> {
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
    pub const fn mb11_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb1116bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Message Buffer 17 CS Register"]
    #[inline(always)]
    pub const fn mb17_8b_cs(self) -> crate::common::Reg<regs::Mb178bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word12_l(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord12L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word6_l(
        self,
    ) -> crate::common::Reg<regs::Mb632bWord6L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Message Buffer 17 ID Register"]
    #[inline(always)]
    pub const fn id17(self) -> crate::common::Reg<regs::Id17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb11_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb1116bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Message Buffer 17 ID Register"]
    #[inline(always)]
    pub const fn mb17_8b_id(self) -> crate::common::Reg<regs::Mb178bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word13_l(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord13L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word7_l(
        self,
    ) -> crate::common::Reg<regs::Mb632bWord7L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb11_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb1116bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "Message Buffer 17 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb17_8b_word0(self) -> crate::common::Reg<regs::Mb178bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word14_l(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord14L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "Message Buffer 7 CS Register"]
    #[inline(always)]
    pub const fn mb7_32b_cs_l(self) -> crate::common::Reg<regs::Mb732bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "Message Buffer 17 WORD0 Register"]
    #[inline(always)]
    pub const fn word017(self) -> crate::common::Reg<regs::Word017, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb11_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb1116bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "Message Buffer 17 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb17_8b_word1(self) -> crate::common::Reg<regs::Mb178bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word15_l(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord15L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "Message Buffer 7 ID Register"]
    #[inline(always)]
    pub const fn mb7_32b_id_l(self) -> crate::common::Reg<regs::Mb732bIdL, crate::common::RW> {
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
    pub const fn mb12_16b_cs_l(self) -> crate::common::Reg<regs::Mb1216bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "Message Buffer 18 CS Register"]
    #[inline(always)]
    pub const fn mb18_8b_cs(self) -> crate::common::Reg<regs::Mb188bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "Message Buffer 4 CS Register"]
    #[inline(always)]
    pub const fn mb4_64b_cs_l(self) -> crate::common::Reg<regs::Mb464bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb732bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "Message Buffer 18 ID Register"]
    #[inline(always)]
    pub const fn id18(self) -> crate::common::Reg<regs::Id18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "Message Buffer 12 ID Register"]
    #[inline(always)]
    pub const fn mb12_16b_id_l(self) -> crate::common::Reg<regs::Mb1216bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "Message Buffer 18 ID Register"]
    #[inline(always)]
    pub const fn mb18_8b_id(self) -> crate::common::Reg<regs::Mb188bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "Message Buffer 4 ID Register"]
    #[inline(always)]
    pub const fn mb4_64b_id_l(self) -> crate::common::Reg<regs::Mb464bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb732bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "Message Buffer 12 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb12_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb1216bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "Message Buffer 18 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb18_8b_word0(self) -> crate::common::Reg<regs::Mb188bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb732bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "Message Buffer 18 WORD0 Register"]
    #[inline(always)]
    pub const fn word018(self) -> crate::common::Reg<regs::Word018, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "Message Buffer 12 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb12_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb1216bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "Message Buffer 18 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb18_8b_word1(self) -> crate::common::Reg<regs::Mb188bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb732bWord3L, crate::common::RW> {
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
    pub const fn mb12_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb1216bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Message Buffer 19 CS Register"]
    #[inline(always)]
    pub const fn mb19_8b_cs(self) -> crate::common::Reg<regs::Mb198bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word4_l(
        self,
    ) -> crate::common::Reg<regs::Mb732bWord4L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Message Buffer 19 ID Register"]
    #[inline(always)]
    pub const fn id19(self) -> crate::common::Reg<regs::Id19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "Message Buffer 12 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb12_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb1216bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "Message Buffer 19 ID Register"]
    #[inline(always)]
    pub const fn mb19_8b_id(self) -> crate::common::Reg<regs::Mb198bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word5_l(
        self,
    ) -> crate::common::Reg<regs::Mb732bWord5L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "Message Buffer 13 CS Register"]
    #[inline(always)]
    pub const fn mb13_16b_cs_l(self) -> crate::common::Reg<regs::Mb1316bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "Message Buffer 19 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb19_8b_word0(self) -> crate::common::Reg<regs::Mb198bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word4_l(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord4L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word6_l(
        self,
    ) -> crate::common::Reg<regs::Mb732bWord6L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "Message Buffer 19 WORD0 Register"]
    #[inline(always)]
    pub const fn word019(self) -> crate::common::Reg<regs::Word019, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "Message Buffer 13 ID Register"]
    #[inline(always)]
    pub const fn mb13_16b_id_l(self) -> crate::common::Reg<regs::Mb1316bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "Message Buffer 19 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb19_8b_word1(self) -> crate::common::Reg<regs::Mb198bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word5_l(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord5L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word7_l(
        self,
    ) -> crate::common::Reg<regs::Mb732bWord7L, crate::common::RW> {
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
    pub const fn mb13_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb1316bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "Message Buffer 20 CS Register"]
    #[inline(always)]
    pub const fn mb20_8b_cs(self) -> crate::common::Reg<regs::Mb208bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word6_l(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord6L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "Message Buffer 8 CS Register"]
    #[inline(always)]
    pub const fn mb8_32b_cs_l(self) -> crate::common::Reg<regs::Mb832bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "Message Buffer 20 ID Register"]
    #[inline(always)]
    pub const fn id20(self) -> crate::common::Reg<regs::Id20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "Message Buffer 13 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb13_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb1316bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "Message Buffer 20 ID Register"]
    #[inline(always)]
    pub const fn mb20_8b_id(self) -> crate::common::Reg<regs::Mb208bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word7_l(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord7L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "Message Buffer 8 ID Register"]
    #[inline(always)]
    pub const fn mb8_32b_id_l(self) -> crate::common::Reg<regs::Mb832bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "Message Buffer 13 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb13_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb1316bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "Message Buffer 20 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb20_8b_word0(self) -> crate::common::Reg<regs::Mb208bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word8_l(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord8L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb832bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "Message Buffer 20 WORD0 Register"]
    #[inline(always)]
    pub const fn word020(self) -> crate::common::Reg<regs::Word020, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "Message Buffer 13 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb13_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb1316bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "Message Buffer 20 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb20_8b_word1(self) -> crate::common::Reg<regs::Mb208bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word9_l(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord9L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb832bWord1L, crate::common::RW> {
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
    pub const fn mb14_16b_cs_l(self) -> crate::common::Reg<regs::Mb1416bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "Message Buffer 21 CS Register"]
    #[inline(always)]
    pub const fn mb21_8b_cs(self) -> crate::common::Reg<regs::Mb218bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word10_l(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord10L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb832bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "Message Buffer 21 ID Register"]
    #[inline(always)]
    pub const fn id21(self) -> crate::common::Reg<regs::Id21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "Message Buffer 14 ID Register"]
    #[inline(always)]
    pub const fn mb14_16b_id_l(self) -> crate::common::Reg<regs::Mb1416bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "Message Buffer 21 ID Register"]
    #[inline(always)]
    pub const fn mb21_8b_id(self) -> crate::common::Reg<regs::Mb218bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word11_l(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord11L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb832bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "Message Buffer 14 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb14_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb1416bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "Message Buffer 21 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb21_8b_word0(self) -> crate::common::Reg<regs::Mb218bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word12_l(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord12L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word4_l(
        self,
    ) -> crate::common::Reg<regs::Mb832bWord4L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "Message Buffer 21 WORD0 Register"]
    #[inline(always)]
    pub const fn word021(self) -> crate::common::Reg<regs::Word021, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "Message Buffer 14 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb14_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb1416bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "Message Buffer 21 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb21_8b_word1(self) -> crate::common::Reg<regs::Mb218bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word13_l(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord13L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word5_l(
        self,
    ) -> crate::common::Reg<regs::Mb832bWord5L, crate::common::RW> {
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
    pub const fn mb14_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb1416bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "Message Buffer 22 CS Register"]
    #[inline(always)]
    pub const fn mb22_8b_cs(self) -> crate::common::Reg<regs::Mb228bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word14_l(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord14L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word6_l(
        self,
    ) -> crate::common::Reg<regs::Mb832bWord6L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "Message Buffer 22 ID Register"]
    #[inline(always)]
    pub const fn id22(self) -> crate::common::Reg<regs::Id22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "Message Buffer 14 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb14_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb1416bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "Message Buffer 22 ID Register"]
    #[inline(always)]
    pub const fn mb22_8b_id(self) -> crate::common::Reg<regs::Mb228bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word15_l(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord15L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word7_l(
        self,
    ) -> crate::common::Reg<regs::Mb832bWord7L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "Message Buffer 15 CS Register"]
    #[inline(always)]
    pub const fn mb15_16b_cs_l(self) -> crate::common::Reg<regs::Mb1516bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "Message Buffer 22 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb22_8b_word0(self) -> crate::common::Reg<regs::Mb228bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "Message Buffer 5 CS Register"]
    #[inline(always)]
    pub const fn mb5_64b_cs_l(self) -> crate::common::Reg<regs::Mb564bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "Message Buffer 9 CS Register"]
    #[inline(always)]
    pub const fn mb9_32b_cs_l(self) -> crate::common::Reg<regs::Mb932bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "Message Buffer 22 WORD0 Register"]
    #[inline(always)]
    pub const fn word022(self) -> crate::common::Reg<regs::Word022, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "Message Buffer 15 ID Register"]
    #[inline(always)]
    pub const fn mb15_16b_id_l(self) -> crate::common::Reg<regs::Mb1516bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "Message Buffer 22 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb22_8b_word1(self) -> crate::common::Reg<regs::Mb228bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "Message Buffer 5 ID Register"]
    #[inline(always)]
    pub const fn mb5_64b_id_l(self) -> crate::common::Reg<regs::Mb564bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "Message Buffer 9 ID Register"]
    #[inline(always)]
    pub const fn mb9_32b_id_l(self) -> crate::common::Reg<regs::Mb932bIdL, crate::common::RW> {
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
    pub const fn mb15_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb1516bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "Message Buffer 23 CS Register"]
    #[inline(always)]
    pub const fn mb23_8b_cs(self) -> crate::common::Reg<regs::Mb238bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb932bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "Message Buffer 23 ID Register"]
    #[inline(always)]
    pub const fn id23(self) -> crate::common::Reg<regs::Id23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "Message Buffer 15 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb15_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb1516bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "Message Buffer 23 ID Register"]
    #[inline(always)]
    pub const fn mb23_8b_id(self) -> crate::common::Reg<regs::Mb238bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb932bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "Message Buffer 15 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb15_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb1516bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "Message Buffer 23 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb23_8b_word0(self) -> crate::common::Reg<regs::Mb238bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb932bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "Message Buffer 23 WORD0 Register"]
    #[inline(always)]
    pub const fn word023(self) -> crate::common::Reg<regs::Word023, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "Message Buffer 15 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb15_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb1516bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "Message Buffer 23 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb23_8b_word1(self) -> crate::common::Reg<regs::Mb238bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb932bWord3L, crate::common::RW> {
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
    pub const fn mb16_16b_cs_l(self) -> crate::common::Reg<regs::Mb1616bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Message Buffer 24 CS Register"]
    #[inline(always)]
    pub const fn mb24_8b_cs(self) -> crate::common::Reg<regs::Mb248bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word4_l(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord4L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word4_l(
        self,
    ) -> crate::common::Reg<regs::Mb932bWord4L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Message Buffer 24 ID Register"]
    #[inline(always)]
    pub const fn id24(self) -> crate::common::Reg<regs::Id24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Message Buffer 16 ID Register"]
    #[inline(always)]
    pub const fn mb16_16b_id_l(self) -> crate::common::Reg<regs::Mb1616bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Message Buffer 24 ID Register"]
    #[inline(always)]
    pub const fn mb24_8b_id(self) -> crate::common::Reg<regs::Mb248bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word5_l(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord5L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word5_l(
        self,
    ) -> crate::common::Reg<regs::Mb932bWord5L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Message Buffer 16 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb16_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb1616bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Message Buffer 24 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb24_8b_word0(self) -> crate::common::Reg<regs::Mb248bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word6_l(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord6L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word6_l(
        self,
    ) -> crate::common::Reg<regs::Mb932bWord6L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Message Buffer 24 WORD0 Register"]
    #[inline(always)]
    pub const fn word024(self) -> crate::common::Reg<regs::Word024, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Message Buffer 16 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb16_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb1616bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "Message Buffer 24 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb24_8b_word1(self) -> crate::common::Reg<regs::Mb248bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word7_l(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord7L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word7_l(
        self,
    ) -> crate::common::Reg<regs::Mb932bWord7L, crate::common::RW> {
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
    pub const fn mb10_32b_cs_l(self) -> crate::common::Reg<regs::Mb1032bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Message Buffer 16 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb16_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb1616bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Message Buffer 25 CS Register"]
    #[inline(always)]
    pub const fn mb25_8b_cs(self) -> crate::common::Reg<regs::Mb258bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word8_l(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord8L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Message Buffer 25 ID Register"]
    #[inline(always)]
    pub const fn id25(self) -> crate::common::Reg<regs::Id25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Message Buffer 10 ID Register"]
    #[inline(always)]
    pub const fn mb10_32b_id_l(self) -> crate::common::Reg<regs::Mb1032bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Message Buffer 16 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb16_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb1616bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Message Buffer 25 ID Register"]
    #[inline(always)]
    pub const fn mb25_8b_id(self) -> crate::common::Reg<regs::Mb258bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word9_l(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord9L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb1032bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Message Buffer 17 CS Register"]
    #[inline(always)]
    pub const fn mb17_16b_cs_l(self) -> crate::common::Reg<regs::Mb1716bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Message Buffer 25 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb25_8b_word0(self) -> crate::common::Reg<regs::Mb258bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word10_l(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord10L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Message Buffer 25 WORD0 Register"]
    #[inline(always)]
    pub const fn word025(self) -> crate::common::Reg<regs::Word025, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb1032bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "Message Buffer 17 ID Register"]
    #[inline(always)]
    pub const fn mb17_16b_id_l(self) -> crate::common::Reg<regs::Mb1716bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "Message Buffer 25 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb25_8b_word1(self) -> crate::common::Reg<regs::Mb258bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word11_l(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord11L, crate::common::RW> {
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
    pub const fn mb10_32b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb1032bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "Message Buffer 17 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb17_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb1716bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "Message Buffer 26 CS Register"]
    #[inline(always)]
    pub const fn mb26_8b_cs(self) -> crate::common::Reg<regs::Mb268bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word12_l(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord12L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "Message Buffer 26 ID Register"]
    #[inline(always)]
    pub const fn id26(self) -> crate::common::Reg<regs::Id26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb1032bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "Message Buffer 17 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb17_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb1716bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "Message Buffer 26 ID Register"]
    #[inline(always)]
    pub const fn mb26_8b_id(self) -> crate::common::Reg<regs::Mb268bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word13_l(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord13L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word4_l(
        self,
    ) -> crate::common::Reg<regs::Mb1032bWord4L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "Message Buffer 17 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb17_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb1716bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "Message Buffer 26 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb26_8b_word0(self) -> crate::common::Reg<regs::Mb268bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word14_l(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord14L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "Message Buffer 26 WORD0 Register"]
    #[inline(always)]
    pub const fn word026(self) -> crate::common::Reg<regs::Word026, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word5_l(
        self,
    ) -> crate::common::Reg<regs::Mb1032bWord5L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "Message Buffer 17 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb17_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb1716bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "Message Buffer 26 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb26_8b_word1(self) -> crate::common::Reg<regs::Mb268bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word15_l(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord15L, crate::common::RW> {
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
    pub const fn mb10_32b_word6_l(
        self,
    ) -> crate::common::Reg<regs::Mb1032bWord6L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "Message Buffer 18 CS Register"]
    #[inline(always)]
    pub const fn mb18_16b_cs_l(self) -> crate::common::Reg<regs::Mb1816bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "Message Buffer 27 CS Register"]
    #[inline(always)]
    pub const fn mb27_8b_cs(self) -> crate::common::Reg<regs::Mb278bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "Message Buffer 6 CS Register"]
    #[inline(always)]
    pub const fn mb6_64b_cs_l(self) -> crate::common::Reg<regs::Mb664bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "Message Buffer 27 ID Register"]
    #[inline(always)]
    pub const fn id27(self) -> crate::common::Reg<regs::Id27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word7_l(
        self,
    ) -> crate::common::Reg<regs::Mb1032bWord7L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "Message Buffer 18 ID Register"]
    #[inline(always)]
    pub const fn mb18_16b_id_l(self) -> crate::common::Reg<regs::Mb1816bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "Message Buffer 27 ID Register"]
    #[inline(always)]
    pub const fn mb27_8b_id(self) -> crate::common::Reg<regs::Mb278bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "Message Buffer 6 ID Register"]
    #[inline(always)]
    pub const fn mb6_64b_id_l(self) -> crate::common::Reg<regs::Mb664bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "Message Buffer 11 CS Register"]
    #[inline(always)]
    pub const fn mb11_32b_cs_l(self) -> crate::common::Reg<regs::Mb1132bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "Message Buffer 18 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb18_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb1816bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "Message Buffer 27 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb27_8b_word0(self) -> crate::common::Reg<regs::Mb278bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "Message Buffer 27 WORD0 Register"]
    #[inline(always)]
    pub const fn word027(self) -> crate::common::Reg<regs::Word027, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "Message Buffer 11 ID Register"]
    #[inline(always)]
    pub const fn mb11_32b_id_l(self) -> crate::common::Reg<regs::Mb1132bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "Message Buffer 18 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb18_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb1816bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "Message Buffer 27 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb27_8b_word1(self) -> crate::common::Reg<regs::Mb278bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord1L, crate::common::RW> {
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
    pub const fn mb11_32b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb1132bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "Message Buffer 18 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb18_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb1816bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "Message Buffer 28 CS Register"]
    #[inline(always)]
    pub const fn mb28_8b_cs(self) -> crate::common::Reg<regs::Mb288bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "Message Buffer 28 ID Register"]
    #[inline(always)]
    pub const fn id28(self) -> crate::common::Reg<regs::Id28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb1132bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "Message Buffer 18 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb18_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb1816bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "Message Buffer 28 ID Register"]
    #[inline(always)]
    pub const fn mb28_8b_id(self) -> crate::common::Reg<regs::Mb288bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb1132bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "Message Buffer 19 CS Register"]
    #[inline(always)]
    pub const fn mb19_16b_cs_l(self) -> crate::common::Reg<regs::Mb1916bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "Message Buffer 28 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb28_8b_word0(self) -> crate::common::Reg<regs::Mb288bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word4_l(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord4L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "Message Buffer 28 WORD0 Register"]
    #[inline(always)]
    pub const fn word028(self) -> crate::common::Reg<regs::Word028, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb1132bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "Message Buffer 19 ID Register"]
    #[inline(always)]
    pub const fn mb19_16b_id_l(self) -> crate::common::Reg<regs::Mb1916bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "Message Buffer 28 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb28_8b_word1(self) -> crate::common::Reg<regs::Mb288bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word5_l(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord5L, crate::common::RW> {
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
    pub const fn mb11_32b_word4_l(
        self,
    ) -> crate::common::Reg<regs::Mb1132bWord4L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "Message Buffer 19 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb19_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb1916bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "Message Buffer 29 CS Register"]
    #[inline(always)]
    pub const fn mb29_8b_cs(self) -> crate::common::Reg<regs::Mb298bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word6_l(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord6L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "Message Buffer 29 ID Register"]
    #[inline(always)]
    pub const fn id29(self) -> crate::common::Reg<regs::Id29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word5_l(
        self,
    ) -> crate::common::Reg<regs::Mb1132bWord5L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "Message Buffer 19 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb19_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb1916bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "Message Buffer 29 ID Register"]
    #[inline(always)]
    pub const fn mb29_8b_id(self) -> crate::common::Reg<regs::Mb298bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word7_l(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord7L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word6_l(
        self,
    ) -> crate::common::Reg<regs::Mb1132bWord6L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "Message Buffer 19 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb19_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb1916bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "Message Buffer 29 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb29_8b_word0(self) -> crate::common::Reg<regs::Mb298bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word8_l(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord8L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "Message Buffer 29 WORD0 Register"]
    #[inline(always)]
    pub const fn word029(self) -> crate::common::Reg<regs::Word029, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word7_l(
        self,
    ) -> crate::common::Reg<regs::Mb1132bWord7L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "Message Buffer 19 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb19_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb1916bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "Message Buffer 29 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb29_8b_word1(self) -> crate::common::Reg<regs::Mb298bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word9_l(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord9L, crate::common::RW> {
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
    pub const fn mb20_16b_cs_l(self) -> crate::common::Reg<regs::Mb2016bCsL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "Message Buffer 30 CS Register"]
    #[inline(always)]
    pub const fn mb30_8b_cs(self) -> crate::common::Reg<regs::Mb308bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word10_l(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord10L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "Message Buffer 30 ID Register"]
    #[inline(always)]
    pub const fn id30(self) -> crate::common::Reg<regs::Id30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "Message Buffer 20 ID Register"]
    #[inline(always)]
    pub const fn mb20_16b_id_l(self) -> crate::common::Reg<regs::Mb2016bIdL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "Message Buffer 30 ID Register"]
    #[inline(always)]
    pub const fn mb30_8b_id(self) -> crate::common::Reg<regs::Mb308bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word11_l(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord11L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "Message Buffer 20 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb20_16b_word0_l(
        self,
    ) -> crate::common::Reg<regs::Mb2016bWord0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "Message Buffer 30 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb30_8b_word0(self) -> crate::common::Reg<regs::Mb308bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word12_l(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord12L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "Message Buffer 30 WORD0 Register"]
    #[inline(always)]
    pub const fn word030(self) -> crate::common::Reg<regs::Word030, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "Message Buffer 20 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb20_16b_word1_l(
        self,
    ) -> crate::common::Reg<regs::Mb2016bWord1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "Message Buffer 30 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb30_8b_word1(self) -> crate::common::Reg<regs::Mb308bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word13_l(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord13L, crate::common::RW> {
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
    pub const fn mb20_16b_word2_l(
        self,
    ) -> crate::common::Reg<regs::Mb2016bWord2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "Message Buffer 31 CS Register"]
    #[inline(always)]
    pub const fn mb31_8b_cs(self) -> crate::common::Reg<regs::Mb318bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word14_l(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord14L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "Message Buffer 31 ID Register"]
    #[inline(always)]
    pub const fn id31(self) -> crate::common::Reg<regs::Id31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "Message Buffer 20 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb20_16b_word3_l(
        self,
    ) -> crate::common::Reg<regs::Mb2016bWord3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "Message Buffer 31 ID Register"]
    #[inline(always)]
    pub const fn mb31_8b_id(self) -> crate::common::Reg<regs::Mb318bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word15_l(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord15L, crate::common::RW> {
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
    #[doc = "Message Buffer 32 CS Register"]
    #[inline(always)]
    pub const fn cs32(self) -> crate::common::Reg<regs::Cs32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "Message Buffer 0 CS Register"]
    #[inline(always)]
    pub const fn mb0_16b_cs_h(self) -> crate::common::Reg<regs::Mb016bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "Message Buffer 0 CS Register"]
    #[inline(always)]
    pub const fn mb0_32b_cs_h(self) -> crate::common::Reg<regs::Mb032bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "Message Buffer 0 CS Register"]
    #[inline(always)]
    pub const fn mb0_64b_cs_h(self) -> crate::common::Reg<regs::Mb064bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "Message Buffer 32 CS Register"]
    #[inline(always)]
    pub const fn mb32_8b_cs(self) -> crate::common::Reg<regs::Mb328bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "Message Buffer 32 ID Register"]
    #[inline(always)]
    pub const fn id32(self) -> crate::common::Reg<regs::Id32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn mb0_16b_id_h(self) -> crate::common::Reg<regs::Mb016bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn mb0_32b_id_h(self) -> crate::common::Reg<regs::Mb032bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn mb0_64b_id_h(self) -> crate::common::Reg<regs::Mb064bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "Message Buffer 32 ID Register"]
    #[inline(always)]
    pub const fn mb32_8b_id(self) -> crate::common::Reg<regs::Mb328bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb0_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb016bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb032bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "Message Buffer 32 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb32_8b_word0(self) -> crate::common::Reg<regs::Mb328bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "Message Buffer 32 WORD0 Register"]
    #[inline(always)]
    pub const fn word032(self) -> crate::common::Reg<regs::Word032, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb0_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb016bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x028cusize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb032bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x028cusize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x028cusize) as _) }
    }
    #[doc = "Message Buffer 32 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb32_8b_word1(self) -> crate::common::Reg<regs::Mb328bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x028cusize) as _) }
    }
    #[doc = "Message Buffer 32 WORD1 Register"]
    #[inline(always)]
    pub const fn word132(self) -> crate::common::Reg<regs::Word132, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x028cusize) as _) }
    }
    #[doc = "Message Buffer 33 CS Register"]
    #[inline(always)]
    pub const fn cs33(self) -> crate::common::Reg<regs::Cs33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb0_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb016bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb032bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "Message Buffer 33 CS Register"]
    #[inline(always)]
    pub const fn mb33_8b_cs(self) -> crate::common::Reg<regs::Mb338bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "Message Buffer 33 ID Register"]
    #[inline(always)]
    pub const fn id33(self) -> crate::common::Reg<regs::Id33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb0_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb016bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb032bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "Message Buffer 33 ID Register"]
    #[inline(always)]
    pub const fn mb33_8b_id(self) -> crate::common::Reg<regs::Mb338bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word4_h(
        self,
    ) -> crate::common::Reg<regs::Mb032bWord4H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0298usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word4_h(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord4H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0298usize) as _) }
    }
    #[doc = "Message Buffer 1 CS Register"]
    #[inline(always)]
    pub const fn mb1_16b_cs_h(self) -> crate::common::Reg<regs::Mb116bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0298usize) as _) }
    }
    #[doc = "Message Buffer 33 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb33_8b_word0(self) -> crate::common::Reg<regs::Mb338bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0298usize) as _) }
    }
    #[doc = "Message Buffer 33 WORD0 Register"]
    #[inline(always)]
    pub const fn word033(self) -> crate::common::Reg<regs::Word033, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0298usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word5_h(
        self,
    ) -> crate::common::Reg<regs::Mb032bWord5H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x029cusize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word5_h(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord5H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x029cusize) as _) }
    }
    #[doc = "Message Buffer 1 ID Register"]
    #[inline(always)]
    pub const fn mb1_16b_id_h(self) -> crate::common::Reg<regs::Mb116bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x029cusize) as _) }
    }
    #[doc = "Message Buffer 33 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb33_8b_word1(self) -> crate::common::Reg<regs::Mb338bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x029cusize) as _) }
    }
    #[doc = "Message Buffer 33 WORD1 Register"]
    #[inline(always)]
    pub const fn word133(self) -> crate::common::Reg<regs::Word133, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x029cusize) as _) }
    }
    #[doc = "Message Buffer 34 CS Register"]
    #[inline(always)]
    pub const fn cs34(self) -> crate::common::Reg<regs::Cs34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word6_h(
        self,
    ) -> crate::common::Reg<regs::Mb032bWord6H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word6_h(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord6H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb1_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb116bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "Message Buffer 34 CS Register"]
    #[inline(always)]
    pub const fn mb34_8b_cs(self) -> crate::common::Reg<regs::Mb348bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "Message Buffer 34 ID Register"]
    #[inline(always)]
    pub const fn id34(self) -> crate::common::Reg<regs::Id34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word7_h(
        self,
    ) -> crate::common::Reg<regs::Mb032bWord7H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word7_h(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord7H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb1_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb116bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "Message Buffer 34 ID Register"]
    #[inline(always)]
    pub const fn mb34_8b_id(self) -> crate::common::Reg<regs::Mb348bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word8_h(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord8H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb1_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb116bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "Message Buffer 1 CS Register"]
    #[inline(always)]
    pub const fn mb1_32b_cs_h(self) -> crate::common::Reg<regs::Mb132bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "Message Buffer 34 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb34_8b_word0(self) -> crate::common::Reg<regs::Mb348bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "Message Buffer 34 WORD0 Register"]
    #[inline(always)]
    pub const fn word034(self) -> crate::common::Reg<regs::Word034, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word9_h(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord9H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02acusize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb1_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb116bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02acusize) as _) }
    }
    #[doc = "Message Buffer 1 ID Register"]
    #[inline(always)]
    pub const fn mb1_32b_id_h(self) -> crate::common::Reg<regs::Mb132bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02acusize) as _) }
    }
    #[doc = "Message Buffer 34 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb34_8b_word1(self) -> crate::common::Reg<regs::Mb348bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02acusize) as _) }
    }
    #[doc = "Message Buffer 34 WORD1 Register"]
    #[inline(always)]
    pub const fn word134(self) -> crate::common::Reg<regs::Word134, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02acusize) as _) }
    }
    #[doc = "Message Buffer 35 CS Register"]
    #[inline(always)]
    pub const fn cs35(self) -> crate::common::Reg<regs::Cs35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word10_h(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord10H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb132bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "Message Buffer 2 CS Register"]
    #[inline(always)]
    pub const fn mb2_16b_cs_h(self) -> crate::common::Reg<regs::Mb216bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "Message Buffer 35 CS Register"]
    #[inline(always)]
    pub const fn mb35_8b_cs(self) -> crate::common::Reg<regs::Mb358bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "Message Buffer 35 ID Register"]
    #[inline(always)]
    pub const fn id35(self) -> crate::common::Reg<regs::Id35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word11_h(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord11H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb132bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "Message Buffer 2 ID Register"]
    #[inline(always)]
    pub const fn mb2_16b_id_h(self) -> crate::common::Reg<regs::Mb216bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "Message Buffer 35 ID Register"]
    #[inline(always)]
    pub const fn mb35_8b_id(self) -> crate::common::Reg<regs::Mb358bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word12_h(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord12H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb132bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb2_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb216bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "Message Buffer 35 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb35_8b_word0(self) -> crate::common::Reg<regs::Mb358bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "Message Buffer 35 WORD0 Register"]
    #[inline(always)]
    pub const fn word035(self) -> crate::common::Reg<regs::Word035, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word13_h(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord13H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02bcusize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb132bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02bcusize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb2_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb216bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02bcusize) as _) }
    }
    #[doc = "Message Buffer 35 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb35_8b_word1(self) -> crate::common::Reg<regs::Mb358bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02bcusize) as _) }
    }
    #[doc = "Message Buffer 35 WORD1 Register"]
    #[inline(always)]
    pub const fn word135(self) -> crate::common::Reg<regs::Word135, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02bcusize) as _) }
    }
    #[doc = "Message Buffer 36 CS Register"]
    #[inline(always)]
    pub const fn cs36(self) -> crate::common::Reg<regs::Cs36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word14_h(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord14H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word4_h(
        self,
    ) -> crate::common::Reg<regs::Mb132bWord4H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb2_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb216bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "Message Buffer 36 CS Register"]
    #[inline(always)]
    pub const fn mb36_8b_cs(self) -> crate::common::Reg<regs::Mb368bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "Message Buffer 36 ID Register"]
    #[inline(always)]
    pub const fn id36(self) -> crate::common::Reg<regs::Id36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c4usize) as _) }
    }
    #[doc = "Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word15_h(
        self,
    ) -> crate::common::Reg<regs::Mb064bWord15H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word5_h(
        self,
    ) -> crate::common::Reg<regs::Mb132bWord5H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c4usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb2_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb216bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c4usize) as _) }
    }
    #[doc = "Message Buffer 36 ID Register"]
    #[inline(always)]
    pub const fn mb36_8b_id(self) -> crate::common::Reg<regs::Mb368bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word6_h(
        self,
    ) -> crate::common::Reg<regs::Mb132bWord6H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c8usize) as _) }
    }
    #[doc = "Message Buffer 1 CS Register"]
    #[inline(always)]
    pub const fn mb1_64b_cs_h(self) -> crate::common::Reg<regs::Mb164bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c8usize) as _) }
    }
    #[doc = "Message Buffer 36 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb36_8b_word0(self) -> crate::common::Reg<regs::Mb368bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c8usize) as _) }
    }
    #[doc = "Message Buffer 3 CS Register"]
    #[inline(always)]
    pub const fn mb3_16b_cs_h(self) -> crate::common::Reg<regs::Mb316bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c8usize) as _) }
    }
    #[doc = "Message Buffer 36 WORD0 Register"]
    #[inline(always)]
    pub const fn word036(self) -> crate::common::Reg<regs::Word036, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c8usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word7_h(
        self,
    ) -> crate::common::Reg<regs::Mb132bWord7H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ccusize) as _) }
    }
    #[doc = "Message Buffer 1 ID Register"]
    #[inline(always)]
    pub const fn mb1_64b_id_h(self) -> crate::common::Reg<regs::Mb164bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ccusize) as _) }
    }
    #[doc = "Message Buffer 36 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb36_8b_word1(self) -> crate::common::Reg<regs::Mb368bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ccusize) as _) }
    }
    #[doc = "Message Buffer 3 ID Register"]
    #[inline(always)]
    pub const fn mb3_16b_id_h(self) -> crate::common::Reg<regs::Mb316bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ccusize) as _) }
    }
    #[doc = "Message Buffer 36 WORD1 Register"]
    #[inline(always)]
    pub const fn word136(self) -> crate::common::Reg<regs::Word136, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ccusize) as _) }
    }
    #[doc = "Message Buffer 37 CS Register"]
    #[inline(always)]
    pub const fn cs37(self) -> crate::common::Reg<regs::Cs37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d0usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d0usize) as _) }
    }
    #[doc = "Message Buffer 2 CS Register"]
    #[inline(always)]
    pub const fn mb2_32b_cs_h(self) -> crate::common::Reg<regs::Mb232bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d0usize) as _) }
    }
    #[doc = "Message Buffer 37 CS Register"]
    #[inline(always)]
    pub const fn mb37_8b_cs(self) -> crate::common::Reg<regs::Mb378bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d0usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb3_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb316bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d0usize) as _) }
    }
    #[doc = "Message Buffer 37 ID Register"]
    #[inline(always)]
    pub const fn id37(self) -> crate::common::Reg<regs::Id37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d4usize) as _) }
    }
    #[doc = "Message Buffer 2 ID Register"]
    #[inline(always)]
    pub const fn mb2_32b_id_h(self) -> crate::common::Reg<regs::Mb232bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d4usize) as _) }
    }
    #[doc = "Message Buffer 37 ID Register"]
    #[inline(always)]
    pub const fn mb37_8b_id(self) -> crate::common::Reg<regs::Mb378bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d4usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb3_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb316bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d8usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb232bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d8usize) as _) }
    }
    #[doc = "Message Buffer 37 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb37_8b_word0(self) -> crate::common::Reg<regs::Mb378bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d8usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb3_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb316bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d8usize) as _) }
    }
    #[doc = "Message Buffer 37 WORD0 Register"]
    #[inline(always)]
    pub const fn word037(self) -> crate::common::Reg<regs::Word037, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d8usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02dcusize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb232bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02dcusize) as _) }
    }
    #[doc = "Message Buffer 37 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb37_8b_word1(self) -> crate::common::Reg<regs::Mb378bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02dcusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb3_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb316bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02dcusize) as _) }
    }
    #[doc = "Message Buffer 37 WORD1 Register"]
    #[inline(always)]
    pub const fn word137(self) -> crate::common::Reg<regs::Word137, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02dcusize) as _) }
    }
    #[doc = "Message Buffer 38 CS Register"]
    #[inline(always)]
    pub const fn cs38(self) -> crate::common::Reg<regs::Cs38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e0usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word4_h(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord4H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e0usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb232bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e0usize) as _) }
    }
    #[doc = "Message Buffer 38 CS Register"]
    #[inline(always)]
    pub const fn mb38_8b_cs(self) -> crate::common::Reg<regs::Mb388bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e0usize) as _) }
    }
    #[doc = "Message Buffer 4 CS Register"]
    #[inline(always)]
    pub const fn mb4_16b_cs_h(self) -> crate::common::Reg<regs::Mb416bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e0usize) as _) }
    }
    #[doc = "Message Buffer 38 ID Register"]
    #[inline(always)]
    pub const fn id38(self) -> crate::common::Reg<regs::Id38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word5_h(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord5H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e4usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb232bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e4usize) as _) }
    }
    #[doc = "Message Buffer 38 ID Register"]
    #[inline(always)]
    pub const fn mb38_8b_id(self) -> crate::common::Reg<regs::Mb388bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e4usize) as _) }
    }
    #[doc = "Message Buffer 4 ID Register"]
    #[inline(always)]
    pub const fn mb4_16b_id_h(self) -> crate::common::Reg<regs::Mb416bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word6_h(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord6H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e8usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word4_h(
        self,
    ) -> crate::common::Reg<regs::Mb232bWord4H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e8usize) as _) }
    }
    #[doc = "Message Buffer 38 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb38_8b_word0(self) -> crate::common::Reg<regs::Mb388bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e8usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb4_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb416bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e8usize) as _) }
    }
    #[doc = "Message Buffer 38 WORD0 Register"]
    #[inline(always)]
    pub const fn word038(self) -> crate::common::Reg<regs::Word038, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e8usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word7_h(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord7H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ecusize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word5_h(
        self,
    ) -> crate::common::Reg<regs::Mb232bWord5H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ecusize) as _) }
    }
    #[doc = "Message Buffer 38 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb38_8b_word1(self) -> crate::common::Reg<regs::Mb388bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ecusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb4_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb416bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ecusize) as _) }
    }
    #[doc = "Message Buffer 38 WORD1 Register"]
    #[inline(always)]
    pub const fn word138(self) -> crate::common::Reg<regs::Word138, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ecusize) as _) }
    }
    #[doc = "Message Buffer 39 CS Register"]
    #[inline(always)]
    pub const fn cs39(self) -> crate::common::Reg<regs::Cs39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f0usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word8_h(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord8H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f0usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word6_h(
        self,
    ) -> crate::common::Reg<regs::Mb232bWord6H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f0usize) as _) }
    }
    #[doc = "Message Buffer 39 CS Register"]
    #[inline(always)]
    pub const fn mb39_8b_cs(self) -> crate::common::Reg<regs::Mb398bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f0usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb4_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb416bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f0usize) as _) }
    }
    #[doc = "Message Buffer 39 ID Register"]
    #[inline(always)]
    pub const fn id39(self) -> crate::common::Reg<regs::Id39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word9_h(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord9H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f4usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word7_h(
        self,
    ) -> crate::common::Reg<regs::Mb232bWord7H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f4usize) as _) }
    }
    #[doc = "Message Buffer 39 ID Register"]
    #[inline(always)]
    pub const fn mb39_8b_id(self) -> crate::common::Reg<regs::Mb398bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f4usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb4_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb416bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f4usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word10_h(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord10H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f8usize) as _) }
    }
    #[doc = "Message Buffer 39 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb39_8b_word0(self) -> crate::common::Reg<regs::Mb398bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f8usize) as _) }
    }
    #[doc = "Message Buffer 3 CS Register"]
    #[inline(always)]
    pub const fn mb3_32b_cs_h(self) -> crate::common::Reg<regs::Mb332bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f8usize) as _) }
    }
    #[doc = "Message Buffer 5 CS Register"]
    #[inline(always)]
    pub const fn mb5_16b_cs_h(self) -> crate::common::Reg<regs::Mb516bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f8usize) as _) }
    }
    #[doc = "Message Buffer 39 WORD0 Register"]
    #[inline(always)]
    pub const fn word039(self) -> crate::common::Reg<regs::Word039, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f8usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word11_h(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord11H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02fcusize) as _) }
    }
    #[doc = "Message Buffer 39 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb39_8b_word1(self) -> crate::common::Reg<regs::Mb398bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02fcusize) as _) }
    }
    #[doc = "Message Buffer 3 ID Register"]
    #[inline(always)]
    pub const fn mb3_32b_id_h(self) -> crate::common::Reg<regs::Mb332bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02fcusize) as _) }
    }
    #[doc = "Message Buffer 5 ID Register"]
    #[inline(always)]
    pub const fn mb5_16b_id_h(self) -> crate::common::Reg<regs::Mb516bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02fcusize) as _) }
    }
    #[doc = "Message Buffer 39 WORD1 Register"]
    #[inline(always)]
    pub const fn word139(self) -> crate::common::Reg<regs::Word139, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02fcusize) as _) }
    }
    #[doc = "Message Buffer 40 CS Register"]
    #[inline(always)]
    pub const fn cs40(self) -> crate::common::Reg<regs::Cs40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word12_h(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord12H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb332bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "Message Buffer 40 CS Register"]
    #[inline(always)]
    pub const fn mb40_8b_cs(self) -> crate::common::Reg<regs::Mb408bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb5_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb516bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "Message Buffer 40 ID Register"]
    #[inline(always)]
    pub const fn id40(self) -> crate::common::Reg<regs::Id40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word13_h(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord13H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb332bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "Message Buffer 40 ID Register"]
    #[inline(always)]
    pub const fn mb40_8b_id(self) -> crate::common::Reg<regs::Mb408bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb5_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb516bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word14_h(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord14H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb332bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "Message Buffer 40 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb40_8b_word0(self) -> crate::common::Reg<regs::Mb408bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb5_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb516bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "Message Buffer 40 WORD0 Register"]
    #[inline(always)]
    pub const fn word040(self) -> crate::common::Reg<regs::Word040, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word15_h(
        self,
    ) -> crate::common::Reg<regs::Mb164bWord15H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb332bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "Message Buffer 40 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb40_8b_word1(self) -> crate::common::Reg<regs::Mb408bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb5_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb516bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "Message Buffer 40 WORD1 Register"]
    #[inline(always)]
    pub const fn word140(self) -> crate::common::Reg<regs::Word140, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "Message Buffer 41 CS Register"]
    #[inline(always)]
    pub const fn cs41(self) -> crate::common::Reg<regs::Cs41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "Message Buffer 2 CS Register"]
    #[inline(always)]
    pub const fn mb2_64b_cs_h(self) -> crate::common::Reg<regs::Mb264bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word4_h(
        self,
    ) -> crate::common::Reg<regs::Mb332bWord4H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "Message Buffer 41 CS Register"]
    #[inline(always)]
    pub const fn mb41_8b_cs(self) -> crate::common::Reg<regs::Mb418bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "Message Buffer 6 CS Register"]
    #[inline(always)]
    pub const fn mb6_16b_cs_h(self) -> crate::common::Reg<regs::Mb616bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "Message Buffer 41 ID Register"]
    #[inline(always)]
    pub const fn id41(self) -> crate::common::Reg<regs::Id41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0314usize) as _) }
    }
    #[doc = "Message Buffer 2 ID Register"]
    #[inline(always)]
    pub const fn mb2_64b_id_h(self) -> crate::common::Reg<regs::Mb264bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0314usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word5_h(
        self,
    ) -> crate::common::Reg<regs::Mb332bWord5H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0314usize) as _) }
    }
    #[doc = "Message Buffer 41 ID Register"]
    #[inline(always)]
    pub const fn mb41_8b_id(self) -> crate::common::Reg<regs::Mb418bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0314usize) as _) }
    }
    #[doc = "Message Buffer 6 ID Register"]
    #[inline(always)]
    pub const fn mb6_16b_id_h(self) -> crate::common::Reg<regs::Mb616bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0314usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word6_h(
        self,
    ) -> crate::common::Reg<regs::Mb332bWord6H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
    #[doc = "Message Buffer 41 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb41_8b_word0(self) -> crate::common::Reg<regs::Mb418bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb6_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb616bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
    #[doc = "Message Buffer 41 WORD0 Register"]
    #[inline(always)]
    pub const fn word041(self) -> crate::common::Reg<regs::Word041, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x031cusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word7_h(
        self,
    ) -> crate::common::Reg<regs::Mb332bWord7H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x031cusize) as _) }
    }
    #[doc = "Message Buffer 41 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb41_8b_word1(self) -> crate::common::Reg<regs::Mb418bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x031cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb6_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb616bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x031cusize) as _) }
    }
    #[doc = "Message Buffer 41 WORD1 Register"]
    #[inline(always)]
    pub const fn word141(self) -> crate::common::Reg<regs::Word141, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x031cusize) as _) }
    }
    #[doc = "Message Buffer 42 CS Register"]
    #[inline(always)]
    pub const fn cs42(self) -> crate::common::Reg<regs::Cs42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "Message Buffer 42 CS Register"]
    #[inline(always)]
    pub const fn mb42_8b_cs(self) -> crate::common::Reg<regs::Mb428bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "Message Buffer 4 CS Register"]
    #[inline(always)]
    pub const fn mb4_32b_cs_h(self) -> crate::common::Reg<regs::Mb432bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb6_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb616bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "Message Buffer 42 ID Register"]
    #[inline(always)]
    pub const fn id42(self) -> crate::common::Reg<regs::Id42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0324usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0324usize) as _) }
    }
    #[doc = "Message Buffer 42 ID Register"]
    #[inline(always)]
    pub const fn mb42_8b_id(self) -> crate::common::Reg<regs::Mb428bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0324usize) as _) }
    }
    #[doc = "Message Buffer 4 ID Register"]
    #[inline(always)]
    pub const fn mb4_32b_id_h(self) -> crate::common::Reg<regs::Mb432bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0324usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb6_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb616bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0324usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word4_h(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord4H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0328usize) as _) }
    }
    #[doc = "Message Buffer 42 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb42_8b_word0(self) -> crate::common::Reg<regs::Mb428bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0328usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb432bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0328usize) as _) }
    }
    #[doc = "Message Buffer 7 CS Register"]
    #[inline(always)]
    pub const fn mb7_16b_cs_h(self) -> crate::common::Reg<regs::Mb716bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0328usize) as _) }
    }
    #[doc = "Message Buffer 42 WORD0 Register"]
    #[inline(always)]
    pub const fn word042(self) -> crate::common::Reg<regs::Word042, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0328usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word5_h(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord5H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x032cusize) as _) }
    }
    #[doc = "Message Buffer 42 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb42_8b_word1(self) -> crate::common::Reg<regs::Mb428bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x032cusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb432bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x032cusize) as _) }
    }
    #[doc = "Message Buffer 7 ID Register"]
    #[inline(always)]
    pub const fn mb7_16b_id_h(self) -> crate::common::Reg<regs::Mb716bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x032cusize) as _) }
    }
    #[doc = "Message Buffer 42 WORD1 Register"]
    #[inline(always)]
    pub const fn word142(self) -> crate::common::Reg<regs::Word142, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x032cusize) as _) }
    }
    #[doc = "Message Buffer 43 CS Register"]
    #[inline(always)]
    pub const fn cs43(self) -> crate::common::Reg<regs::Cs43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word6_h(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord6H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
    #[doc = "Message Buffer 43 CS Register"]
    #[inline(always)]
    pub const fn mb43_8b_cs(self) -> crate::common::Reg<regs::Mb438bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb432bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb7_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb716bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
    #[doc = "Message Buffer 43 ID Register"]
    #[inline(always)]
    pub const fn id43(self) -> crate::common::Reg<regs::Id43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0334usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word7_h(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord7H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0334usize) as _) }
    }
    #[doc = "Message Buffer 43 ID Register"]
    #[inline(always)]
    pub const fn mb43_8b_id(self) -> crate::common::Reg<regs::Mb438bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0334usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb432bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0334usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb7_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb716bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0334usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word8_h(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord8H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0338usize) as _) }
    }
    #[doc = "Message Buffer 43 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb43_8b_word0(self) -> crate::common::Reg<regs::Mb438bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0338usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word4_h(
        self,
    ) -> crate::common::Reg<regs::Mb432bWord4H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0338usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb7_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb716bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0338usize) as _) }
    }
    #[doc = "Message Buffer 43 WORD0 Register"]
    #[inline(always)]
    pub const fn word043(self) -> crate::common::Reg<regs::Word043, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0338usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word9_h(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord9H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x033cusize) as _) }
    }
    #[doc = "Message Buffer 43 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb43_8b_word1(self) -> crate::common::Reg<regs::Mb438bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x033cusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word5_h(
        self,
    ) -> crate::common::Reg<regs::Mb432bWord5H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x033cusize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb7_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb716bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x033cusize) as _) }
    }
    #[doc = "Message Buffer 43 WORD1 Register"]
    #[inline(always)]
    pub const fn word143(self) -> crate::common::Reg<regs::Word143, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x033cusize) as _) }
    }
    #[doc = "Message Buffer 44 CS Register"]
    #[inline(always)]
    pub const fn cs44(self) -> crate::common::Reg<regs::Cs44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0340usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word10_h(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord10H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0340usize) as _) }
    }
    #[doc = "Message Buffer 44 CS Register"]
    #[inline(always)]
    pub const fn mb44_8b_cs(self) -> crate::common::Reg<regs::Mb448bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0340usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word6_h(
        self,
    ) -> crate::common::Reg<regs::Mb432bWord6H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0340usize) as _) }
    }
    #[doc = "Message Buffer 8 CS Register"]
    #[inline(always)]
    pub const fn mb8_16b_cs_h(self) -> crate::common::Reg<regs::Mb816bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0340usize) as _) }
    }
    #[doc = "Message Buffer 44 ID Register"]
    #[inline(always)]
    pub const fn id44(self) -> crate::common::Reg<regs::Id44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0344usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word11_h(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord11H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0344usize) as _) }
    }
    #[doc = "Message Buffer 44 ID Register"]
    #[inline(always)]
    pub const fn mb44_8b_id(self) -> crate::common::Reg<regs::Mb448bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0344usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word7_h(
        self,
    ) -> crate::common::Reg<regs::Mb432bWord7H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0344usize) as _) }
    }
    #[doc = "Message Buffer 8 ID Register"]
    #[inline(always)]
    pub const fn mb8_16b_id_h(self) -> crate::common::Reg<regs::Mb816bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0344usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word12_h(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord12H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0348usize) as _) }
    }
    #[doc = "Message Buffer 44 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb44_8b_word0(self) -> crate::common::Reg<regs::Mb448bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0348usize) as _) }
    }
    #[doc = "Message Buffer 5 CS Register"]
    #[inline(always)]
    pub const fn mb5_32b_cs_h(self) -> crate::common::Reg<regs::Mb532bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0348usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb8_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb816bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0348usize) as _) }
    }
    #[doc = "Message Buffer 44 WORD0 Register"]
    #[inline(always)]
    pub const fn word044(self) -> crate::common::Reg<regs::Word044, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0348usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word13_h(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord13H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x034cusize) as _) }
    }
    #[doc = "Message Buffer 44 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb44_8b_word1(self) -> crate::common::Reg<regs::Mb448bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x034cusize) as _) }
    }
    #[doc = "Message Buffer 5 ID Register"]
    #[inline(always)]
    pub const fn mb5_32b_id_h(self) -> crate::common::Reg<regs::Mb532bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x034cusize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb8_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb816bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x034cusize) as _) }
    }
    #[doc = "Message Buffer 44 WORD1 Register"]
    #[inline(always)]
    pub const fn word144(self) -> crate::common::Reg<regs::Word144, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x034cusize) as _) }
    }
    #[doc = "Message Buffer 45 CS Register"]
    #[inline(always)]
    pub const fn cs45(self) -> crate::common::Reg<regs::Cs45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0350usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word14_h(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord14H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0350usize) as _) }
    }
    #[doc = "Message Buffer 45 CS Register"]
    #[inline(always)]
    pub const fn mb45_8b_cs(self) -> crate::common::Reg<regs::Mb458bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0350usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb532bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0350usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb8_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb816bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0350usize) as _) }
    }
    #[doc = "Message Buffer 45 ID Register"]
    #[inline(always)]
    pub const fn id45(self) -> crate::common::Reg<regs::Id45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0354usize) as _) }
    }
    #[doc = "Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word15_h(
        self,
    ) -> crate::common::Reg<regs::Mb264bWord15H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0354usize) as _) }
    }
    #[doc = "Message Buffer 45 ID Register"]
    #[inline(always)]
    pub const fn mb45_8b_id(self) -> crate::common::Reg<regs::Mb458bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0354usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb532bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0354usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb8_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb816bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0354usize) as _) }
    }
    #[doc = "Message Buffer 3 CS Register"]
    #[inline(always)]
    pub const fn mb3_64b_cs_h(self) -> crate::common::Reg<regs::Mb364bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0358usize) as _) }
    }
    #[doc = "Message Buffer 45 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb45_8b_word0(self) -> crate::common::Reg<regs::Mb458bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0358usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb532bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0358usize) as _) }
    }
    #[doc = "Message Buffer 9 CS Register"]
    #[inline(always)]
    pub const fn mb9_16b_cs_h(self) -> crate::common::Reg<regs::Mb916bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0358usize) as _) }
    }
    #[doc = "Message Buffer 45 WORD0 Register"]
    #[inline(always)]
    pub const fn word045(self) -> crate::common::Reg<regs::Word045, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0358usize) as _) }
    }
    #[doc = "Message Buffer 3 ID Register"]
    #[inline(always)]
    pub const fn mb3_64b_id_h(self) -> crate::common::Reg<regs::Mb364bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x035cusize) as _) }
    }
    #[doc = "Message Buffer 45 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb45_8b_word1(self) -> crate::common::Reg<regs::Mb458bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x035cusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb532bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x035cusize) as _) }
    }
    #[doc = "Message Buffer 9 ID Register"]
    #[inline(always)]
    pub const fn mb9_16b_id_h(self) -> crate::common::Reg<regs::Mb916bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x035cusize) as _) }
    }
    #[doc = "Message Buffer 45 WORD1 Register"]
    #[inline(always)]
    pub const fn word145(self) -> crate::common::Reg<regs::Word145, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x035cusize) as _) }
    }
    #[doc = "Message Buffer 46 CS Register"]
    #[inline(always)]
    pub const fn cs46(self) -> crate::common::Reg<regs::Cs46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize) as _) }
    }
    #[doc = "Message Buffer 46 CS Register"]
    #[inline(always)]
    pub const fn mb46_8b_cs(self) -> crate::common::Reg<regs::Mb468bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word4_h(
        self,
    ) -> crate::common::Reg<regs::Mb532bWord4H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb9_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb916bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize) as _) }
    }
    #[doc = "Message Buffer 46 ID Register"]
    #[inline(always)]
    pub const fn id46(self) -> crate::common::Reg<regs::Id46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0364usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0364usize) as _) }
    }
    #[doc = "Message Buffer 46 ID Register"]
    #[inline(always)]
    pub const fn mb46_8b_id(self) -> crate::common::Reg<regs::Mb468bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0364usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word5_h(
        self,
    ) -> crate::common::Reg<regs::Mb532bWord5H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0364usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb9_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb916bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0364usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0368usize) as _) }
    }
    #[doc = "Message Buffer 46 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb46_8b_word0(self) -> crate::common::Reg<regs::Mb468bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0368usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word6_h(
        self,
    ) -> crate::common::Reg<regs::Mb532bWord6H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0368usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb9_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb916bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0368usize) as _) }
    }
    #[doc = "Message Buffer 46 WORD0 Register"]
    #[inline(always)]
    pub const fn word046(self) -> crate::common::Reg<regs::Word046, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0368usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x036cusize) as _) }
    }
    #[doc = "Message Buffer 46 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb46_8b_word1(self) -> crate::common::Reg<regs::Mb468bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x036cusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word7_h(
        self,
    ) -> crate::common::Reg<regs::Mb532bWord7H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x036cusize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb9_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb916bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x036cusize) as _) }
    }
    #[doc = "Message Buffer 46 WORD1 Register"]
    #[inline(always)]
    pub const fn word146(self) -> crate::common::Reg<regs::Word146, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x036cusize) as _) }
    }
    #[doc = "Message Buffer 47 CS Register"]
    #[inline(always)]
    pub const fn cs47(self) -> crate::common::Reg<regs::Cs47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0370usize) as _) }
    }
    #[doc = "Message Buffer 10 CS Register"]
    #[inline(always)]
    pub const fn mb10_16b_cs_h(self) -> crate::common::Reg<regs::Mb1016bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0370usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word4_h(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord4H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0370usize) as _) }
    }
    #[doc = "Message Buffer 47 CS Register"]
    #[inline(always)]
    pub const fn mb47_8b_cs(self) -> crate::common::Reg<regs::Mb478bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0370usize) as _) }
    }
    #[doc = "Message Buffer 6 CS Register"]
    #[inline(always)]
    pub const fn mb6_32b_cs_h(self) -> crate::common::Reg<regs::Mb632bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0370usize) as _) }
    }
    #[doc = "Message Buffer 47 ID Register"]
    #[inline(always)]
    pub const fn id47(self) -> crate::common::Reg<regs::Id47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0374usize) as _) }
    }
    #[doc = "Message Buffer 10 ID Register"]
    #[inline(always)]
    pub const fn mb10_16b_id_h(self) -> crate::common::Reg<regs::Mb1016bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0374usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word5_h(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord5H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0374usize) as _) }
    }
    #[doc = "Message Buffer 47 ID Register"]
    #[inline(always)]
    pub const fn mb47_8b_id(self) -> crate::common::Reg<regs::Mb478bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0374usize) as _) }
    }
    #[doc = "Message Buffer 6 ID Register"]
    #[inline(always)]
    pub const fn mb6_32b_id_h(self) -> crate::common::Reg<regs::Mb632bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0374usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb10_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb1016bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0378usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word6_h(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord6H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0378usize) as _) }
    }
    #[doc = "Message Buffer 47 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb47_8b_word0(self) -> crate::common::Reg<regs::Mb478bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0378usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb632bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0378usize) as _) }
    }
    #[doc = "Message Buffer 47 WORD0 Register"]
    #[inline(always)]
    pub const fn word047(self) -> crate::common::Reg<regs::Word047, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0378usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb10_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb1016bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x037cusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word7_h(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord7H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x037cusize) as _) }
    }
    #[doc = "Message Buffer 47 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb47_8b_word1(self) -> crate::common::Reg<regs::Mb478bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x037cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb632bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x037cusize) as _) }
    }
    #[doc = "Message Buffer 47 WORD1 Register"]
    #[inline(always)]
    pub const fn word147(self) -> crate::common::Reg<regs::Word147, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x037cusize) as _) }
    }
    #[doc = "Message Buffer 48 CS Register"]
    #[inline(always)]
    pub const fn cs48(self) -> crate::common::Reg<regs::Cs48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb10_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb1016bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word8_h(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord8H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "Message Buffer 48 CS Register"]
    #[inline(always)]
    pub const fn mb48_8b_cs(self) -> crate::common::Reg<regs::Mb488bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb632bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "Message Buffer 48 ID Register"]
    #[inline(always)]
    pub const fn id48(self) -> crate::common::Reg<regs::Id48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0384usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb10_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb1016bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0384usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word9_h(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord9H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0384usize) as _) }
    }
    #[doc = "Message Buffer 48 ID Register"]
    #[inline(always)]
    pub const fn mb48_8b_id(self) -> crate::common::Reg<regs::Mb488bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0384usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb632bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0384usize) as _) }
    }
    #[doc = "Message Buffer 11 CS Register"]
    #[inline(always)]
    pub const fn mb11_16b_cs_h(self) -> crate::common::Reg<regs::Mb1116bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word10_h(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord10H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[doc = "Message Buffer 48 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb48_8b_word0(self) -> crate::common::Reg<regs::Mb488bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word4_h(
        self,
    ) -> crate::common::Reg<regs::Mb632bWord4H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[doc = "Message Buffer 48 WORD0 Register"]
    #[inline(always)]
    pub const fn word048(self) -> crate::common::Reg<regs::Word048, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[doc = "Message Buffer 11 ID Register"]
    #[inline(always)]
    pub const fn mb11_16b_id_h(self) -> crate::common::Reg<regs::Mb1116bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word11_h(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord11H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[doc = "Message Buffer 48 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb48_8b_word1(self) -> crate::common::Reg<regs::Mb488bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word5_h(
        self,
    ) -> crate::common::Reg<regs::Mb632bWord5H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[doc = "Message Buffer 48 WORD1 Register"]
    #[inline(always)]
    pub const fn word148(self) -> crate::common::Reg<regs::Word148, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[doc = "Message Buffer 49 CS Register"]
    #[inline(always)]
    pub const fn cs49(self) -> crate::common::Reg<regs::Cs49, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0390usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb11_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb1116bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0390usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word12_h(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord12H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0390usize) as _) }
    }
    #[doc = "Message Buffer 49 CS Register"]
    #[inline(always)]
    pub const fn mb49_8b_cs(self) -> crate::common::Reg<regs::Mb498bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0390usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word6_h(
        self,
    ) -> crate::common::Reg<regs::Mb632bWord6H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0390usize) as _) }
    }
    #[doc = "Message Buffer 49 ID Register"]
    #[inline(always)]
    pub const fn id49(self) -> crate::common::Reg<regs::Id49, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0394usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb11_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb1116bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0394usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word13_h(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord13H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0394usize) as _) }
    }
    #[doc = "Message Buffer 49 ID Register"]
    #[inline(always)]
    pub const fn mb49_8b_id(self) -> crate::common::Reg<regs::Mb498bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0394usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word7_h(
        self,
    ) -> crate::common::Reg<regs::Mb632bWord7H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0394usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb11_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb1116bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0398usize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word14_h(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord14H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0398usize) as _) }
    }
    #[doc = "Message Buffer 49 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb49_8b_word0(self) -> crate::common::Reg<regs::Mb498bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0398usize) as _) }
    }
    #[doc = "Message Buffer 7 CS Register"]
    #[inline(always)]
    pub const fn mb7_32b_cs_h(self) -> crate::common::Reg<regs::Mb732bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0398usize) as _) }
    }
    #[doc = "Message Buffer 49 WORD0 Register"]
    #[inline(always)]
    pub const fn word049(self) -> crate::common::Reg<regs::Word049, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0398usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb11_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb1116bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x039cusize) as _) }
    }
    #[doc = "Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word15_h(
        self,
    ) -> crate::common::Reg<regs::Mb364bWord15H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x039cusize) as _) }
    }
    #[doc = "Message Buffer 49 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb49_8b_word1(self) -> crate::common::Reg<regs::Mb498bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x039cusize) as _) }
    }
    #[doc = "Message Buffer 7 ID Register"]
    #[inline(always)]
    pub const fn mb7_32b_id_h(self) -> crate::common::Reg<regs::Mb732bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x039cusize) as _) }
    }
    #[doc = "Message Buffer 49 WORD1 Register"]
    #[inline(always)]
    pub const fn word149(self) -> crate::common::Reg<regs::Word149, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x039cusize) as _) }
    }
    #[doc = "Message Buffer 50 CS Register"]
    #[inline(always)]
    pub const fn cs50(self) -> crate::common::Reg<regs::Cs50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a0usize) as _) }
    }
    #[doc = "Message Buffer 12 CS Register"]
    #[inline(always)]
    pub const fn mb12_16b_cs_h(self) -> crate::common::Reg<regs::Mb1216bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a0usize) as _) }
    }
    #[doc = "Message Buffer 4 CS Register"]
    #[inline(always)]
    pub const fn mb4_64b_cs_h(self) -> crate::common::Reg<regs::Mb464bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a0usize) as _) }
    }
    #[doc = "Message Buffer 50 CS Register"]
    #[inline(always)]
    pub const fn mb50_8b_cs(self) -> crate::common::Reg<regs::Mb508bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a0usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb732bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a0usize) as _) }
    }
    #[doc = "Message Buffer 50 ID Register"]
    #[inline(always)]
    pub const fn id50(self) -> crate::common::Reg<regs::Id50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a4usize) as _) }
    }
    #[doc = "Message Buffer 12 ID Register"]
    #[inline(always)]
    pub const fn mb12_16b_id_h(self) -> crate::common::Reg<regs::Mb1216bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a4usize) as _) }
    }
    #[doc = "Message Buffer 4 ID Register"]
    #[inline(always)]
    pub const fn mb4_64b_id_h(self) -> crate::common::Reg<regs::Mb464bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a4usize) as _) }
    }
    #[doc = "Message Buffer 50 ID Register"]
    #[inline(always)]
    pub const fn mb50_8b_id(self) -> crate::common::Reg<regs::Mb508bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a4usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb732bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a4usize) as _) }
    }
    #[doc = "Message Buffer 12 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb12_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb1216bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a8usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a8usize) as _) }
    }
    #[doc = "Message Buffer 50 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb50_8b_word0(self) -> crate::common::Reg<regs::Mb508bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a8usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb732bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a8usize) as _) }
    }
    #[doc = "Message Buffer 50 WORD0 Register"]
    #[inline(always)]
    pub const fn word050(self) -> crate::common::Reg<regs::Word050, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a8usize) as _) }
    }
    #[doc = "Message Buffer 12 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb12_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb1216bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03acusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03acusize) as _) }
    }
    #[doc = "Message Buffer 50 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb50_8b_word1(self) -> crate::common::Reg<regs::Mb508bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03acusize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb732bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03acusize) as _) }
    }
    #[doc = "Message Buffer 50 WORD1 Register"]
    #[inline(always)]
    pub const fn word150(self) -> crate::common::Reg<regs::Word150, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03acusize) as _) }
    }
    #[doc = "Message Buffer 51 CS Register"]
    #[inline(always)]
    pub const fn cs51(self) -> crate::common::Reg<regs::Cs51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b0usize) as _) }
    }
    #[doc = "Message Buffer 12 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb12_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb1216bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b0usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b0usize) as _) }
    }
    #[doc = "Message Buffer 51 CS Register"]
    #[inline(always)]
    pub const fn mb51_8b_cs(self) -> crate::common::Reg<regs::Mb518bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b0usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word4_h(
        self,
    ) -> crate::common::Reg<regs::Mb732bWord4H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b0usize) as _) }
    }
    #[doc = "Message Buffer 51 ID Register"]
    #[inline(always)]
    pub const fn id51(self) -> crate::common::Reg<regs::Id51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b4usize) as _) }
    }
    #[doc = "Message Buffer 12 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb12_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb1216bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b4usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b4usize) as _) }
    }
    #[doc = "Message Buffer 51 ID Register"]
    #[inline(always)]
    pub const fn mb51_8b_id(self) -> crate::common::Reg<regs::Mb518bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b4usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word5_h(
        self,
    ) -> crate::common::Reg<regs::Mb732bWord5H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b4usize) as _) }
    }
    #[doc = "Message Buffer 13 CS Register"]
    #[inline(always)]
    pub const fn mb13_16b_cs_h(self) -> crate::common::Reg<regs::Mb1316bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b8usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word4_h(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord4H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b8usize) as _) }
    }
    #[doc = "Message Buffer 51 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb51_8b_word0(self) -> crate::common::Reg<regs::Mb518bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b8usize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word6_h(
        self,
    ) -> crate::common::Reg<regs::Mb732bWord6H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b8usize) as _) }
    }
    #[doc = "Message Buffer 51 WORD0 Register"]
    #[inline(always)]
    pub const fn word051(self) -> crate::common::Reg<regs::Word051, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b8usize) as _) }
    }
    #[doc = "Message Buffer 13 ID Register"]
    #[inline(always)]
    pub const fn mb13_16b_id_h(self) -> crate::common::Reg<regs::Mb1316bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03bcusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word5_h(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord5H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03bcusize) as _) }
    }
    #[doc = "Message Buffer 51 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb51_8b_word1(self) -> crate::common::Reg<regs::Mb518bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03bcusize) as _) }
    }
    #[doc = "Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word7_h(
        self,
    ) -> crate::common::Reg<regs::Mb732bWord7H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03bcusize) as _) }
    }
    #[doc = "Message Buffer 51 WORD1 Register"]
    #[inline(always)]
    pub const fn word151(self) -> crate::common::Reg<regs::Word151, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03bcusize) as _) }
    }
    #[doc = "Message Buffer 52 CS Register"]
    #[inline(always)]
    pub const fn cs52(self) -> crate::common::Reg<regs::Cs52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c0usize) as _) }
    }
    #[doc = "Message Buffer 13 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb13_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb1316bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c0usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word6_h(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord6H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c0usize) as _) }
    }
    #[doc = "Message Buffer 52 CS Register"]
    #[inline(always)]
    pub const fn mb52_8b_cs(self) -> crate::common::Reg<regs::Mb528bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c0usize) as _) }
    }
    #[doc = "Message Buffer 8 CS Register"]
    #[inline(always)]
    pub const fn mb8_32b_cs_h(self) -> crate::common::Reg<regs::Mb832bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c0usize) as _) }
    }
    #[doc = "Message Buffer 52 ID Register"]
    #[inline(always)]
    pub const fn id52(self) -> crate::common::Reg<regs::Id52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c4usize) as _) }
    }
    #[doc = "Message Buffer 13 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb13_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb1316bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c4usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word7_h(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord7H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c4usize) as _) }
    }
    #[doc = "Message Buffer 52 ID Register"]
    #[inline(always)]
    pub const fn mb52_8b_id(self) -> crate::common::Reg<regs::Mb528bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c4usize) as _) }
    }
    #[doc = "Message Buffer 8 ID Register"]
    #[inline(always)]
    pub const fn mb8_32b_id_h(self) -> crate::common::Reg<regs::Mb832bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c4usize) as _) }
    }
    #[doc = "Message Buffer 13 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb13_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb1316bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c8usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word8_h(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord8H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c8usize) as _) }
    }
    #[doc = "Message Buffer 52 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb52_8b_word0(self) -> crate::common::Reg<regs::Mb528bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c8usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb832bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c8usize) as _) }
    }
    #[doc = "Message Buffer 52 WORD0 Register"]
    #[inline(always)]
    pub const fn word052(self) -> crate::common::Reg<regs::Word052, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c8usize) as _) }
    }
    #[doc = "Message Buffer 13 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb13_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb1316bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ccusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word9_h(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord9H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ccusize) as _) }
    }
    #[doc = "Message Buffer 52 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb52_8b_word1(self) -> crate::common::Reg<regs::Mb528bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ccusize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb832bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ccusize) as _) }
    }
    #[doc = "Message Buffer 52 WORD1 Register"]
    #[inline(always)]
    pub const fn word152(self) -> crate::common::Reg<regs::Word152, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ccusize) as _) }
    }
    #[doc = "Message Buffer 53 CS Register"]
    #[inline(always)]
    pub const fn cs53(self) -> crate::common::Reg<regs::Cs53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d0usize) as _) }
    }
    #[doc = "Message Buffer 14 CS Register"]
    #[inline(always)]
    pub const fn mb14_16b_cs_h(self) -> crate::common::Reg<regs::Mb1416bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d0usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word10_h(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord10H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d0usize) as _) }
    }
    #[doc = "Message Buffer 53 CS Register"]
    #[inline(always)]
    pub const fn mb53_8b_cs(self) -> crate::common::Reg<regs::Mb538bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d0usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb832bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d0usize) as _) }
    }
    #[doc = "Message Buffer 53 ID Register"]
    #[inline(always)]
    pub const fn id53(self) -> crate::common::Reg<regs::Id53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d4usize) as _) }
    }
    #[doc = "Message Buffer 14 ID Register"]
    #[inline(always)]
    pub const fn mb14_16b_id_h(self) -> crate::common::Reg<regs::Mb1416bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d4usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word11_h(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord11H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d4usize) as _) }
    }
    #[doc = "Message Buffer 53 ID Register"]
    #[inline(always)]
    pub const fn mb53_8b_id(self) -> crate::common::Reg<regs::Mb538bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d4usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb832bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d4usize) as _) }
    }
    #[doc = "Message Buffer 14 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb14_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb1416bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d8usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word12_h(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord12H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d8usize) as _) }
    }
    #[doc = "Message Buffer 53 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb53_8b_word0(self) -> crate::common::Reg<regs::Mb538bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d8usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word4_h(
        self,
    ) -> crate::common::Reg<regs::Mb832bWord4H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d8usize) as _) }
    }
    #[doc = "Message Buffer 53 WORD0 Register"]
    #[inline(always)]
    pub const fn word053(self) -> crate::common::Reg<regs::Word053, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d8usize) as _) }
    }
    #[doc = "Message Buffer 14 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb14_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb1416bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03dcusize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word13_h(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord13H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03dcusize) as _) }
    }
    #[doc = "Message Buffer 53 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb53_8b_word1(self) -> crate::common::Reg<regs::Mb538bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03dcusize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word5_h(
        self,
    ) -> crate::common::Reg<regs::Mb832bWord5H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03dcusize) as _) }
    }
    #[doc = "Message Buffer 53 WORD1 Register"]
    #[inline(always)]
    pub const fn word153(self) -> crate::common::Reg<regs::Word153, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03dcusize) as _) }
    }
    #[doc = "Message Buffer 54 CS Register"]
    #[inline(always)]
    pub const fn cs54(self) -> crate::common::Reg<regs::Cs54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e0usize) as _) }
    }
    #[doc = "Message Buffer 14 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb14_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb1416bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e0usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word14_h(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord14H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e0usize) as _) }
    }
    #[doc = "Message Buffer 54 CS Register"]
    #[inline(always)]
    pub const fn mb54_8b_cs(self) -> crate::common::Reg<regs::Mb548bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e0usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word6_h(
        self,
    ) -> crate::common::Reg<regs::Mb832bWord6H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e0usize) as _) }
    }
    #[doc = "Message Buffer 54 ID Register"]
    #[inline(always)]
    pub const fn id54(self) -> crate::common::Reg<regs::Id54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "Message Buffer 14 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb14_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb1416bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word15_h(
        self,
    ) -> crate::common::Reg<regs::Mb464bWord15H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "Message Buffer 54 ID Register"]
    #[inline(always)]
    pub const fn mb54_8b_id(self) -> crate::common::Reg<regs::Mb548bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word7_h(
        self,
    ) -> crate::common::Reg<regs::Mb832bWord7H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "Message Buffer 15 CS Register"]
    #[inline(always)]
    pub const fn mb15_16b_cs_h(self) -> crate::common::Reg<regs::Mb1516bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e8usize) as _) }
    }
    #[doc = "Message Buffer 54 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb54_8b_word0(self) -> crate::common::Reg<regs::Mb548bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e8usize) as _) }
    }
    #[doc = "Message Buffer 5 CS Register"]
    #[inline(always)]
    pub const fn mb5_64b_cs_h(self) -> crate::common::Reg<regs::Mb564bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e8usize) as _) }
    }
    #[doc = "Message Buffer 9 CS Register"]
    #[inline(always)]
    pub const fn mb9_32b_cs_h(self) -> crate::common::Reg<regs::Mb932bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e8usize) as _) }
    }
    #[doc = "Message Buffer 54 WORD0 Register"]
    #[inline(always)]
    pub const fn word054(self) -> crate::common::Reg<regs::Word054, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e8usize) as _) }
    }
    #[doc = "Message Buffer 15 ID Register"]
    #[inline(always)]
    pub const fn mb15_16b_id_h(self) -> crate::common::Reg<regs::Mb1516bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
    #[doc = "Message Buffer 54 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb54_8b_word1(self) -> crate::common::Reg<regs::Mb548bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
    #[doc = "Message Buffer 5 ID Register"]
    #[inline(always)]
    pub const fn mb5_64b_id_h(self) -> crate::common::Reg<regs::Mb564bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
    #[doc = "Message Buffer 9 ID Register"]
    #[inline(always)]
    pub const fn mb9_32b_id_h(self) -> crate::common::Reg<regs::Mb932bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
    #[doc = "Message Buffer 54 WORD1 Register"]
    #[inline(always)]
    pub const fn word154(self) -> crate::common::Reg<regs::Word154, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
    #[doc = "Message Buffer 55 CS Register"]
    #[inline(always)]
    pub const fn cs55(self) -> crate::common::Reg<regs::Cs55, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "Message Buffer 15 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb15_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb1516bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "Message Buffer 55 CS Register"]
    #[inline(always)]
    pub const fn mb55_8b_cs(self) -> crate::common::Reg<regs::Mb558bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb932bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "Message Buffer 55 ID Register"]
    #[inline(always)]
    pub const fn id55(self) -> crate::common::Reg<regs::Id55, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "Message Buffer 15 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb15_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb1516bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "Message Buffer 55 ID Register"]
    #[inline(always)]
    pub const fn mb55_8b_id(self) -> crate::common::Reg<regs::Mb558bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb932bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "Message Buffer 15 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb15_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb1516bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "Message Buffer 55 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb55_8b_word0(self) -> crate::common::Reg<regs::Mb558bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb932bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "Message Buffer 55 WORD0 Register"]
    #[inline(always)]
    pub const fn word055(self) -> crate::common::Reg<regs::Word055, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "Message Buffer 15 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb15_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb1516bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "Message Buffer 55 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb55_8b_word1(self) -> crate::common::Reg<regs::Mb558bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb932bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "Message Buffer 55 WORD1 Register"]
    #[inline(always)]
    pub const fn word155(self) -> crate::common::Reg<regs::Word155, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "Message Buffer 56 CS Register"]
    #[inline(always)]
    pub const fn cs56(self) -> crate::common::Reg<regs::Cs56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "Message Buffer 16 CS Register"]
    #[inline(always)]
    pub const fn mb16_16b_cs_h(self) -> crate::common::Reg<regs::Mb1616bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "Message Buffer 56 CS Register"]
    #[inline(always)]
    pub const fn mb56_8b_cs(self) -> crate::common::Reg<regs::Mb568bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word4_h(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord4H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word4_h(
        self,
    ) -> crate::common::Reg<regs::Mb932bWord4H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "Message Buffer 56 ID Register"]
    #[inline(always)]
    pub const fn id56(self) -> crate::common::Reg<regs::Id56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "Message Buffer 16 ID Register"]
    #[inline(always)]
    pub const fn mb16_16b_id_h(self) -> crate::common::Reg<regs::Mb1616bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "Message Buffer 56 ID Register"]
    #[inline(always)]
    pub const fn mb56_8b_id(self) -> crate::common::Reg<regs::Mb568bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word5_h(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord5H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word5_h(
        self,
    ) -> crate::common::Reg<regs::Mb932bWord5H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "Message Buffer 16 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb16_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb1616bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "Message Buffer 56 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb56_8b_word0(self) -> crate::common::Reg<regs::Mb568bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word6_h(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord6H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word6_h(
        self,
    ) -> crate::common::Reg<regs::Mb932bWord6H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "Message Buffer 56 WORD0 Register"]
    #[inline(always)]
    pub const fn word056(self) -> crate::common::Reg<regs::Word056, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "Message Buffer 16 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb16_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb1616bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "Message Buffer 56 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb56_8b_word1(self) -> crate::common::Reg<regs::Mb568bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word7_h(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord7H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word7_h(
        self,
    ) -> crate::common::Reg<regs::Mb932bWord7H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "Message Buffer 56 WORD1 Register"]
    #[inline(always)]
    pub const fn word156(self) -> crate::common::Reg<regs::Word156, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "Message Buffer 57 CS Register"]
    #[inline(always)]
    pub const fn cs57(self) -> crate::common::Reg<regs::Cs57, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "Message Buffer 10 CS Register"]
    #[inline(always)]
    pub const fn mb10_32b_cs_h(self) -> crate::common::Reg<regs::Mb1032bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "Message Buffer 16 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb16_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb1616bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "Message Buffer 57 CS Register"]
    #[inline(always)]
    pub const fn mb57_8b_cs(self) -> crate::common::Reg<regs::Mb578bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word8_h(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord8H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "Message Buffer 57 ID Register"]
    #[inline(always)]
    pub const fn id57(self) -> crate::common::Reg<regs::Id57, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "Message Buffer 10 ID Register"]
    #[inline(always)]
    pub const fn mb10_32b_id_h(self) -> crate::common::Reg<regs::Mb1032bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "Message Buffer 16 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb16_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb1616bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "Message Buffer 57 ID Register"]
    #[inline(always)]
    pub const fn mb57_8b_id(self) -> crate::common::Reg<regs::Mb578bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word9_h(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord9H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb1032bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    #[doc = "Message Buffer 17 CS Register"]
    #[inline(always)]
    pub const fn mb17_16b_cs_h(self) -> crate::common::Reg<regs::Mb1716bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    #[doc = "Message Buffer 57 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb57_8b_word0(self) -> crate::common::Reg<regs::Mb578bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word10_h(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord10H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    #[doc = "Message Buffer 57 WORD0 Register"]
    #[inline(always)]
    pub const fn word057(self) -> crate::common::Reg<regs::Word057, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb1032bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
    }
    #[doc = "Message Buffer 17 ID Register"]
    #[inline(always)]
    pub const fn mb17_16b_id_h(self) -> crate::common::Reg<regs::Mb1716bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
    }
    #[doc = "Message Buffer 57 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb57_8b_word1(self) -> crate::common::Reg<regs::Mb578bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word11_h(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord11H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
    }
    #[doc = "Message Buffer 57 WORD1 Register"]
    #[inline(always)]
    pub const fn word157(self) -> crate::common::Reg<regs::Word157, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
    }
    #[doc = "Message Buffer 58 CS Register"]
    #[inline(always)]
    pub const fn cs58(self) -> crate::common::Reg<regs::Cs58, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb1032bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "Message Buffer 17 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb17_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb1716bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "Message Buffer 58 CS Register"]
    #[inline(always)]
    pub const fn mb58_8b_cs(self) -> crate::common::Reg<regs::Mb588bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word12_h(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord12H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "Message Buffer 58 ID Register"]
    #[inline(always)]
    pub const fn id58(self) -> crate::common::Reg<regs::Id58, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb1032bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "Message Buffer 17 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb17_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb1716bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "Message Buffer 58 ID Register"]
    #[inline(always)]
    pub const fn mb58_8b_id(self) -> crate::common::Reg<regs::Mb588bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word13_h(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord13H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word4_h(
        self,
    ) -> crate::common::Reg<regs::Mb1032bWord4H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0428usize) as _) }
    }
    #[doc = "Message Buffer 17 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb17_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb1716bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0428usize) as _) }
    }
    #[doc = "Message Buffer 58 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb58_8b_word0(self) -> crate::common::Reg<regs::Mb588bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0428usize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word14_h(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord14H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0428usize) as _) }
    }
    #[doc = "Message Buffer 58 WORD0 Register"]
    #[inline(always)]
    pub const fn word058(self) -> crate::common::Reg<regs::Word058, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0428usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word5_h(
        self,
    ) -> crate::common::Reg<regs::Mb1032bWord5H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x042cusize) as _) }
    }
    #[doc = "Message Buffer 17 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb17_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb1716bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x042cusize) as _) }
    }
    #[doc = "Message Buffer 58 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb58_8b_word1(self) -> crate::common::Reg<regs::Mb588bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x042cusize) as _) }
    }
    #[doc = "Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word15_h(
        self,
    ) -> crate::common::Reg<regs::Mb564bWord15H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x042cusize) as _) }
    }
    #[doc = "Message Buffer 58 WORD1 Register"]
    #[inline(always)]
    pub const fn word158(self) -> crate::common::Reg<regs::Word158, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x042cusize) as _) }
    }
    #[doc = "Message Buffer 59 CS Register"]
    #[inline(always)]
    pub const fn cs59(self) -> crate::common::Reg<regs::Cs59, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0430usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word6_h(
        self,
    ) -> crate::common::Reg<regs::Mb1032bWord6H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0430usize) as _) }
    }
    #[doc = "Message Buffer 18 CS Register"]
    #[inline(always)]
    pub const fn mb18_16b_cs_h(self) -> crate::common::Reg<regs::Mb1816bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0430usize) as _) }
    }
    #[doc = "Message Buffer 59 CS Register"]
    #[inline(always)]
    pub const fn mb59_8b_cs(self) -> crate::common::Reg<regs::Mb598bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0430usize) as _) }
    }
    #[doc = "Message Buffer 6 CS Register"]
    #[inline(always)]
    pub const fn mb6_64b_cs_h(self) -> crate::common::Reg<regs::Mb664bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0430usize) as _) }
    }
    #[doc = "Message Buffer 59 ID Register"]
    #[inline(always)]
    pub const fn id59(self) -> crate::common::Reg<regs::Id59, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0434usize) as _) }
    }
    #[doc = "Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word7_h(
        self,
    ) -> crate::common::Reg<regs::Mb1032bWord7H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0434usize) as _) }
    }
    #[doc = "Message Buffer 18 ID Register"]
    #[inline(always)]
    pub const fn mb18_16b_id_h(self) -> crate::common::Reg<regs::Mb1816bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0434usize) as _) }
    }
    #[doc = "Message Buffer 59 ID Register"]
    #[inline(always)]
    pub const fn mb59_8b_id(self) -> crate::common::Reg<regs::Mb598bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0434usize) as _) }
    }
    #[doc = "Message Buffer 6 ID Register"]
    #[inline(always)]
    pub const fn mb6_64b_id_h(self) -> crate::common::Reg<regs::Mb664bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0434usize) as _) }
    }
    #[doc = "Message Buffer 11 CS Register"]
    #[inline(always)]
    pub const fn mb11_32b_cs_h(self) -> crate::common::Reg<regs::Mb1132bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0438usize) as _) }
    }
    #[doc = "Message Buffer 18 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb18_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb1816bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0438usize) as _) }
    }
    #[doc = "Message Buffer 59 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb59_8b_word0(self) -> crate::common::Reg<regs::Mb598bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0438usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0438usize) as _) }
    }
    #[doc = "Message Buffer 59 WORD0 Register"]
    #[inline(always)]
    pub const fn word059(self) -> crate::common::Reg<regs::Word059, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0438usize) as _) }
    }
    #[doc = "Message Buffer 11 ID Register"]
    #[inline(always)]
    pub const fn mb11_32b_id_h(self) -> crate::common::Reg<regs::Mb1132bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x043cusize) as _) }
    }
    #[doc = "Message Buffer 18 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb18_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb1816bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x043cusize) as _) }
    }
    #[doc = "Message Buffer 59 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb59_8b_word1(self) -> crate::common::Reg<regs::Mb598bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x043cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x043cusize) as _) }
    }
    #[doc = "Message Buffer 59 WORD1 Register"]
    #[inline(always)]
    pub const fn word159(self) -> crate::common::Reg<regs::Word159, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x043cusize) as _) }
    }
    #[doc = "Message Buffer 60 CS Register"]
    #[inline(always)]
    pub const fn cs60(self) -> crate::common::Reg<regs::Cs60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb1132bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "Message Buffer 18 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb18_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb1816bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "Message Buffer 60 CS Register"]
    #[inline(always)]
    pub const fn mb60_8b_cs(self) -> crate::common::Reg<regs::Mb608bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "Message Buffer 60 ID Register"]
    #[inline(always)]
    pub const fn id60(self) -> crate::common::Reg<regs::Id60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0444usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb1132bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0444usize) as _) }
    }
    #[doc = "Message Buffer 18 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb18_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb1816bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0444usize) as _) }
    }
    #[doc = "Message Buffer 60 ID Register"]
    #[inline(always)]
    pub const fn mb60_8b_id(self) -> crate::common::Reg<regs::Mb608bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0444usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0444usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb1132bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0448usize) as _) }
    }
    #[doc = "Message Buffer 19 CS Register"]
    #[inline(always)]
    pub const fn mb19_16b_cs_h(self) -> crate::common::Reg<regs::Mb1916bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0448usize) as _) }
    }
    #[doc = "Message Buffer 60 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb60_8b_word0(self) -> crate::common::Reg<regs::Mb608bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0448usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word4_h(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord4H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0448usize) as _) }
    }
    #[doc = "Message Buffer 60 WORD0 Register"]
    #[inline(always)]
    pub const fn word060(self) -> crate::common::Reg<regs::Word060, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0448usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb1132bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x044cusize) as _) }
    }
    #[doc = "Message Buffer 19 ID Register"]
    #[inline(always)]
    pub const fn mb19_16b_id_h(self) -> crate::common::Reg<regs::Mb1916bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x044cusize) as _) }
    }
    #[doc = "Message Buffer 60 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb60_8b_word1(self) -> crate::common::Reg<regs::Mb608bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x044cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word5_h(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord5H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x044cusize) as _) }
    }
    #[doc = "Message Buffer 60 WORD1 Register"]
    #[inline(always)]
    pub const fn word160(self) -> crate::common::Reg<regs::Word160, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x044cusize) as _) }
    }
    #[doc = "Message Buffer 61 CS Register"]
    #[inline(always)]
    pub const fn cs61(self) -> crate::common::Reg<regs::Cs61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0450usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word4_h(
        self,
    ) -> crate::common::Reg<regs::Mb1132bWord4H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0450usize) as _) }
    }
    #[doc = "Message Buffer 19 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb19_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb1916bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0450usize) as _) }
    }
    #[doc = "Message Buffer 61 CS Register"]
    #[inline(always)]
    pub const fn mb61_8b_cs(self) -> crate::common::Reg<regs::Mb618bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0450usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word6_h(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord6H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0450usize) as _) }
    }
    #[doc = "Message Buffer 61 ID Register"]
    #[inline(always)]
    pub const fn id61(self) -> crate::common::Reg<regs::Id61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0454usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word5_h(
        self,
    ) -> crate::common::Reg<regs::Mb1132bWord5H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0454usize) as _) }
    }
    #[doc = "Message Buffer 19 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb19_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb1916bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0454usize) as _) }
    }
    #[doc = "Message Buffer 61 ID Register"]
    #[inline(always)]
    pub const fn mb61_8b_id(self) -> crate::common::Reg<regs::Mb618bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0454usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word7_h(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord7H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0454usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word6_h(
        self,
    ) -> crate::common::Reg<regs::Mb1132bWord6H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0458usize) as _) }
    }
    #[doc = "Message Buffer 19 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb19_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb1916bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0458usize) as _) }
    }
    #[doc = "Message Buffer 61 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb61_8b_word0(self) -> crate::common::Reg<regs::Mb618bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0458usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word8_h(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord8H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0458usize) as _) }
    }
    #[doc = "Message Buffer 61 WORD0 Register"]
    #[inline(always)]
    pub const fn word061(self) -> crate::common::Reg<regs::Word061, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0458usize) as _) }
    }
    #[doc = "Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word7_h(
        self,
    ) -> crate::common::Reg<regs::Mb1132bWord7H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x045cusize) as _) }
    }
    #[doc = "Message Buffer 19 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb19_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb1916bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x045cusize) as _) }
    }
    #[doc = "Message Buffer 61 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb61_8b_word1(self) -> crate::common::Reg<regs::Mb618bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x045cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word9_h(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord9H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x045cusize) as _) }
    }
    #[doc = "Message Buffer 61 WORD1 Register"]
    #[inline(always)]
    pub const fn word161(self) -> crate::common::Reg<regs::Word161, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x045cusize) as _) }
    }
    #[doc = "Message Buffer 62 CS Register"]
    #[inline(always)]
    pub const fn cs62(self) -> crate::common::Reg<regs::Cs62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0460usize) as _) }
    }
    #[doc = "Message Buffer 20 CS Register"]
    #[inline(always)]
    pub const fn mb20_16b_cs_h(self) -> crate::common::Reg<regs::Mb2016bCsH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0460usize) as _) }
    }
    #[doc = "Message Buffer 62 CS Register"]
    #[inline(always)]
    pub const fn mb62_8b_cs(self) -> crate::common::Reg<regs::Mb628bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0460usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word10_h(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord10H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0460usize) as _) }
    }
    #[doc = "Message Buffer 62 ID Register"]
    #[inline(always)]
    pub const fn id62(self) -> crate::common::Reg<regs::Id62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0464usize) as _) }
    }
    #[doc = "Message Buffer 20 ID Register"]
    #[inline(always)]
    pub const fn mb20_16b_id_h(self) -> crate::common::Reg<regs::Mb2016bIdH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0464usize) as _) }
    }
    #[doc = "Message Buffer 62 ID Register"]
    #[inline(always)]
    pub const fn mb62_8b_id(self) -> crate::common::Reg<regs::Mb628bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0464usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word11_h(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord11H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0464usize) as _) }
    }
    #[doc = "Message Buffer 20 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb20_16b_word0_h(
        self,
    ) -> crate::common::Reg<regs::Mb2016bWord0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0468usize) as _) }
    }
    #[doc = "Message Buffer 62 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb62_8b_word0(self) -> crate::common::Reg<regs::Mb628bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0468usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word12_h(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord12H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0468usize) as _) }
    }
    #[doc = "Message Buffer 62 WORD0 Register"]
    #[inline(always)]
    pub const fn word062(self) -> crate::common::Reg<regs::Word062, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0468usize) as _) }
    }
    #[doc = "Message Buffer 20 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb20_16b_word1_h(
        self,
    ) -> crate::common::Reg<regs::Mb2016bWord1H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x046cusize) as _) }
    }
    #[doc = "Message Buffer 62 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb62_8b_word1(self) -> crate::common::Reg<regs::Mb628bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x046cusize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word13_h(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord13H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x046cusize) as _) }
    }
    #[doc = "Message Buffer 62 WORD1 Register"]
    #[inline(always)]
    pub const fn word162(self) -> crate::common::Reg<regs::Word162, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x046cusize) as _) }
    }
    #[doc = "Message Buffer 63 CS Register"]
    #[inline(always)]
    pub const fn cs63(self) -> crate::common::Reg<regs::Cs63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0470usize) as _) }
    }
    #[doc = "Message Buffer 20 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb20_16b_word2_h(
        self,
    ) -> crate::common::Reg<regs::Mb2016bWord2H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0470usize) as _) }
    }
    #[doc = "Message Buffer 63 CS Register"]
    #[inline(always)]
    pub const fn mb63_8b_cs(self) -> crate::common::Reg<regs::Mb638bCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0470usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word14_h(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord14H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0470usize) as _) }
    }
    #[doc = "Message Buffer 63 ID Register"]
    #[inline(always)]
    pub const fn id63(self) -> crate::common::Reg<regs::Id63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0474usize) as _) }
    }
    #[doc = "Message Buffer 20 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb20_16b_word3_h(
        self,
    ) -> crate::common::Reg<regs::Mb2016bWord3H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0474usize) as _) }
    }
    #[doc = "Message Buffer 63 ID Register"]
    #[inline(always)]
    pub const fn mb63_8b_id(self) -> crate::common::Reg<regs::Mb638bId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0474usize) as _) }
    }
    #[doc = "Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word15_h(
        self,
    ) -> crate::common::Reg<regs::Mb664bWord15H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0474usize) as _) }
    }
    #[doc = "Message Buffer 63 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb63_8b_word0(self) -> crate::common::Reg<regs::Mb638bWord0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0478usize) as _) }
    }
    #[doc = "Message Buffer 63 WORD0 Register"]
    #[inline(always)]
    pub const fn word063(self) -> crate::common::Reg<regs::Word063, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0478usize) as _) }
    }
    #[doc = "Message Buffer 63 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb63_8b_word1(self) -> crate::common::Reg<regs::Mb638bWord1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x047cusize) as _) }
    }
    #[doc = "Message Buffer 63 WORD1 Register"]
    #[inline(always)]
    pub const fn word163(self) -> crate::common::Reg<regs::Word163, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x047cusize) as _) }
    }
    #[doc = "Rx Individual Mask Registers"]
    #[inline(always)]
    pub const fn rximr(self, n: usize) -> crate::common::Reg<regs::Rximr, crate::common::RW> {
        assert!(n < 64usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0880usize + n * 4usize) as _)
        }
    }
    #[doc = "CAN FD Control Register"]
    #[inline(always)]
    pub const fn fdctrl(self) -> crate::common::Reg<regs::Fdctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize) as _) }
    }
    #[doc = "CAN FD Bit Timing Register"]
    #[inline(always)]
    pub const fn fdcbt(self) -> crate::common::Reg<regs::Fdcbt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c04usize) as _) }
    }
    #[doc = "CAN FD CRC Register"]
    #[inline(always)]
    pub const fn fdcrc(self) -> crate::common::Reg<regs::Fdcrc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c08usize) as _) }
    }
}
pub mod regs;
pub mod vals;
