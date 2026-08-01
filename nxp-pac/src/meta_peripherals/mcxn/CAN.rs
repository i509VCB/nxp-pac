#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "CAN."]
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
    #[doc = "Module Configuration."]
    #[inline(always)]
    pub const fn mcr(self) -> crate::pac::common::Reg<Mcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control 1."]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::pac::common::Reg<Ctrl1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Free-Running Timer."]
    #[inline(always)]
    pub const fn timer(self) -> crate::pac::common::Reg<Timer, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RX Message Buffers Global Mask."]
    #[inline(always)]
    pub const fn rxmgmask(self) -> crate::pac::common::Reg<Rxmgmask, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Receive 14 Mask."]
    #[inline(always)]
    pub const fn rx14mask(self) -> crate::pac::common::Reg<Rx14mask, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Receive 15 Mask."]
    #[inline(always)]
    pub const fn rx15mask(self) -> crate::pac::common::Reg<Rx15mask, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Error Counter."]
    #[inline(always)]
    pub const fn ecr(self) -> crate::pac::common::Reg<Ecr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Error and Status 1."]
    #[inline(always)]
    pub const fn esr1(self) -> crate::pac::common::Reg<Esr1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Interrupt Masks 1."]
    #[inline(always)]
    pub const fn imask1(self) -> crate::pac::common::Reg<Imask1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Interrupt Flags 1."]
    #[inline(always)]
    pub const fn iflag1(self) -> crate::pac::common::Reg<Iflag1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Control 2."]
    #[inline(always)]
    pub const fn ctrl2(self) -> crate::pac::common::Reg<Ctrl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Error and Status 2."]
    #[inline(always)]
    pub const fn esr2(self) -> crate::pac::common::Reg<Esr2, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Cyclic Redundancy Check."]
    #[inline(always)]
    pub const fn crcr(self) -> crate::pac::common::Reg<Crcr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Legacy RX FIFO Global Mask."]
    #[inline(always)]
    pub const fn rxfgmask(self) -> crate::pac::common::Reg<Rxfgmask, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Legacy RX FIFO Information."]
    #[inline(always)]
    pub const fn rxfir(self) -> crate::pac::common::Reg<Rxfir, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "CAN Bit Timing."]
    #[inline(always)]
    pub const fn cbt(self) -> crate::pac::common::Reg<Cbt, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Message Buffer CS Register."]
    #[inline(always)]
    pub const fn cs(self, n: usize) -> crate::pac::common::Reg<Cs, crate::pac::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 16usize) as _)
        }
    }
    #[doc = "Message Buffer CS Register."]
    #[inline(always)]
    pub const fn mb_16b_cs(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb16bCs, crate::pac::common::RW> {
        assert!(n < 21usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 24usize) as _)
        }
    }
    #[doc = "Message Buffer CS Register."]
    #[inline(always)]
    pub const fn mb_32b_cs(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb32bCs, crate::pac::common::RW> {
        assert!(n < 12usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 40usize) as _)
        }
    }
    #[doc = "Message Buffer CS Register."]
    #[inline(always)]
    pub const fn mb_64b_cs(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb64bCs, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 72usize) as _)
        }
    }
    #[doc = "Message Buffer CS Register."]
    #[inline(always)]
    pub const fn mb_8b_cs(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb8bCs, crate::pac::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 16usize) as _)
        }
    }
    #[doc = "Message Buffer ID Register."]
    #[inline(always)]
    pub const fn id(self, n: usize) -> crate::pac::common::Reg<Id, crate::pac::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize + n * 16usize) as _)
        }
    }
    #[doc = "Message Buffer ID Register."]
    #[inline(always)]
    pub const fn mb_16b_id(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb16bId, crate::pac::common::RW> {
        assert!(n < 21usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize + n * 24usize) as _)
        }
    }
    #[doc = "Message Buffer ID Register."]
    #[inline(always)]
    pub const fn mb_32b_id(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb32bId, crate::pac::common::RW> {
        assert!(n < 12usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize + n * 40usize) as _)
        }
    }
    #[doc = "Message Buffer ID Register."]
    #[inline(always)]
    pub const fn mb_64b_id(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb64bId, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize + n * 72usize) as _)
        }
    }
    #[doc = "Message Buffer ID Register."]
    #[inline(always)]
    pub const fn mb_8b_id(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb8bId, crate::pac::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize + n * 16usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_16B Register."]
    #[inline(always)]
    pub const fn mb_16b_word0(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb16bWord0, crate::pac::common::RW> {
        assert!(n < 21usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize + n * 24usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_32B Register."]
    #[inline(always)]
    pub const fn mb_32b_word0(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb32bWord0, crate::pac::common::RW> {
        assert!(n < 12usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize + n * 40usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_64B Register."]
    #[inline(always)]
    pub const fn mb_64b_word0(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb64bWord0, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize + n * 72usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_8B Register."]
    #[inline(always)]
    pub const fn mb_8b_word0(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb8bWord0, crate::pac::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize + n * 16usize) as _)
        }
    }
    #[doc = "Message Buffer WORD0 Register."]
    #[inline(always)]
    pub const fn word0(self, n: usize) -> crate::pac::common::Reg<Word0, crate::pac::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize + n * 16usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_16B Register."]
    #[inline(always)]
    pub const fn mb_16b_word1(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb16bWord1, crate::pac::common::RW> {
        assert!(n < 21usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize + n * 24usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_32B Register."]
    #[inline(always)]
    pub const fn mb_32b_word1(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb32bWord1, crate::pac::common::RW> {
        assert!(n < 12usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize + n * 40usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_64B Register."]
    #[inline(always)]
    pub const fn mb_64b_word1(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb64bWord1, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize + n * 72usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_8B Register."]
    #[inline(always)]
    pub const fn mb_8b_word1(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb8bWord1, crate::pac::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize + n * 16usize) as _)
        }
    }
    #[doc = "Message Buffer WORD1 Register."]
    #[inline(always)]
    pub const fn word1(self, n: usize) -> crate::pac::common::Reg<Word1, crate::pac::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize + n * 16usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_16B Register."]
    #[inline(always)]
    pub const fn mb_16b_word2(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb16bWord2, crate::pac::common::RW> {
        assert!(n < 21usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize + n * 24usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_32B Register."]
    #[inline(always)]
    pub const fn mb_32b_word2(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb32bWord2, crate::pac::common::RW> {
        assert!(n < 12usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize + n * 40usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_64B Register."]
    #[inline(always)]
    pub const fn mb_64b_word2(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb64bWord2, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize + n * 72usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_16B Register."]
    #[inline(always)]
    pub const fn mb_16b_word3(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb16bWord3, crate::pac::common::RW> {
        assert!(n < 21usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize + n * 24usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_32B Register."]
    #[inline(always)]
    pub const fn mb_32b_word3(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb32bWord3, crate::pac::common::RW> {
        assert!(n < 12usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize + n * 40usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_64B Register."]
    #[inline(always)]
    pub const fn mb_64b_word3(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb64bWord3, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize + n * 72usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_32B Register."]
    #[inline(always)]
    pub const fn mb_32b_word4(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb32bWord4, crate::pac::common::RW> {
        assert!(n < 12usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize + n * 40usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_64B Register."]
    #[inline(always)]
    pub const fn mb_64b_word4(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb64bWord4, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize + n * 72usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_32B Register."]
    #[inline(always)]
    pub const fn mb_32b_word5(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb32bWord5, crate::pac::common::RW> {
        assert!(n < 12usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize + n * 40usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_64B Register."]
    #[inline(always)]
    pub const fn mb_64b_word5(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb64bWord5, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize + n * 72usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_32B Register."]
    #[inline(always)]
    pub const fn mb_32b_word6(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb32bWord6, crate::pac::common::RW> {
        assert!(n < 12usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 40usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_64B Register."]
    #[inline(always)]
    pub const fn mb_64b_word6(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb64bWord6, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 72usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_32B Register."]
    #[inline(always)]
    pub const fn mb_32b_word7(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb32bWord7, crate::pac::common::RW> {
        assert!(n < 12usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize + n * 40usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_64B Register."]
    #[inline(always)]
    pub const fn mb_64b_word7(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb64bWord7, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize + n * 72usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_64B Register."]
    #[inline(always)]
    pub const fn mb_64b_word8(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb64bWord8, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize + n * 72usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_64B Register."]
    #[inline(always)]
    pub const fn mb_64b_word9(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb64bWord9, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize + n * 72usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_64B Register."]
    #[inline(always)]
    pub const fn mb_64b_word10(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb64bWord10, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize + n * 72usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_64B Register."]
    #[inline(always)]
    pub const fn mb_64b_word11(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb64bWord11, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize + n * 72usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_64B Register."]
    #[inline(always)]
    pub const fn mb_64b_word12(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb64bWord12, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize + n * 72usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_64B Register."]
    #[inline(always)]
    pub const fn mb_64b_word13(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb64bWord13, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize + n * 72usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_64B Register."]
    #[inline(always)]
    pub const fn mb_64b_word14(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb64bWord14, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 72usize) as _)
        }
    }
    #[doc = "Message Buffer WORD_64B Register."]
    #[inline(always)]
    pub const fn mb_64b_word15(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mb64bWord15, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize + n * 72usize) as _)
        }
    }
    #[doc = "Receive Individual Mask."]
    #[inline(always)]
    pub const fn rximr(self, n: usize) -> crate::pac::common::Reg<Rximr, crate::pac::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0880usize + n * 4usize) as _)
        }
    }
    #[doc = "Pretended Networking Control 1."]
    #[inline(always)]
    pub const fn ctrl1_pn(self) -> crate::pac::common::Reg<Ctrl1Pn, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b00usize) as _) }
    }
    #[doc = "Pretended Networking Control 2."]
    #[inline(always)]
    pub const fn ctrl2_pn(self) -> crate::pac::common::Reg<Ctrl2Pn, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b04usize) as _) }
    }
    #[doc = "Pretended Networking Wake-Up Match."]
    #[inline(always)]
    pub const fn wu_mtc(self) -> crate::pac::common::Reg<WuMtc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b08usize) as _) }
    }
    #[doc = "Pretended Networking ID Filter 1."]
    #[inline(always)]
    pub const fn flt_id1(self) -> crate::pac::common::Reg<FltId1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b0cusize) as _) }
    }
    #[doc = "Pretended Networking Data Length Code (DLC) Filter."]
    #[inline(always)]
    pub const fn flt_dlc(self) -> crate::pac::common::Reg<FltDlc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b10usize) as _) }
    }
    #[doc = "Pretended Networking Payload Low Filter 1."]
    #[inline(always)]
    pub const fn pl1_lo(self) -> crate::pac::common::Reg<Pl1Lo, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b14usize) as _) }
    }
    #[doc = "Pretended Networking Payload High Filter 1."]
    #[inline(always)]
    pub const fn pl1_hi(self) -> crate::pac::common::Reg<Pl1Hi, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b18usize) as _) }
    }
    #[doc = "Pretended Networking ID Filter 2 or ID Mask."]
    #[inline(always)]
    pub const fn flt_id2_idmask(
        self,
    ) -> crate::pac::common::Reg<FltId2Idmask, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b1cusize) as _) }
    }
    #[doc = "Pretended Networking Payload Low Filter 2 and Payload Low Mask."]
    #[inline(always)]
    pub const fn pl2_plmask_lo(
        self,
    ) -> crate::pac::common::Reg<Pl2PlmaskLo, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b20usize) as _) }
    }
    #[doc = "Pretended Networking Payload High Filter 2 and Payload High Mask."]
    #[inline(always)]
    pub const fn pl2_plmask_hi(
        self,
    ) -> crate::pac::common::Reg<Pl2PlmaskHi, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b24usize) as _) }
    }
    #[doc = "Array of registers: WMB_CS, WMB_ID, WMB_D03, WMB_D47."]
    #[inline(always)]
    pub const fn wmb(self, n: usize) -> Wmb {
        assert!(n < 4usize);
        unsafe { Wmb::from_ptr(self.ptr.wrapping_add(0x0b40usize + n * 16usize) as _) }
    }
    #[doc = "Enhanced CAN Bit Timing Prescalers."]
    #[inline(always)]
    pub const fn eprs(self) -> crate::pac::common::Reg<Eprs, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bf0usize) as _) }
    }
    #[doc = "Enhanced Nominal CAN Bit Timing."]
    #[inline(always)]
    pub const fn encbt(self) -> crate::pac::common::Reg<Encbt, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bf4usize) as _) }
    }
    #[doc = "Enhanced Data Phase CAN Bit Timing."]
    #[inline(always)]
    pub const fn edcbt(self) -> crate::pac::common::Reg<Edcbt, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bf8usize) as _) }
    }
    #[doc = "Enhanced Transceiver Delay Compensation."]
    #[inline(always)]
    pub const fn etdc(self) -> crate::pac::common::Reg<Etdc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bfcusize) as _) }
    }
    #[doc = "CAN FD Control."]
    #[inline(always)]
    pub const fn fdctrl(self) -> crate::pac::common::Reg<Fdctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize) as _) }
    }
    #[doc = "CAN FD Bit Timing."]
    #[inline(always)]
    pub const fn fdcbt(self) -> crate::pac::common::Reg<Fdcbt, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c04usize) as _) }
    }
    #[doc = "CAN FD CRC."]
    #[inline(always)]
    pub const fn fdcrc(self) -> crate::pac::common::Reg<Fdcrc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c08usize) as _) }
    }
    #[doc = "Enhanced RX FIFO Control."]
    #[inline(always)]
    pub const fn erfcr(self) -> crate::pac::common::Reg<Erfcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c0cusize) as _) }
    }
    #[doc = "Enhanced RX FIFO Interrupt Enable."]
    #[inline(always)]
    pub const fn erfier(self) -> crate::pac::common::Reg<Erfier, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c10usize) as _) }
    }
    #[doc = "Enhanced RX FIFO Status."]
    #[inline(always)]
    pub const fn erfsr(self) -> crate::pac::common::Reg<Erfsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c14usize) as _) }
    }
    #[doc = "Enhanced RX FIFO Filter Element."]
    #[inline(always)]
    pub const fn erffel(self, n: usize) -> crate::pac::common::Reg<Erffel, crate::pac::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3000usize + n * 4usize) as _)
        }
    }
}
#[doc = "Array of registers: WMB_CS, WMB_ID, WMB_D03, WMB_D47."]
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
    #[doc = "Wake-Up Message Buffer."]
    #[inline(always)]
    pub const fn wmb_cs(self) -> crate::pac::common::Reg<WmbCs, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Wake-Up Message Buffer for ID."]
    #[inline(always)]
    pub const fn wmb_id(self) -> crate::pac::common::Reg<WmbId, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Wake-Up Message Buffer for Data 0-3."]
    #[inline(always)]
    pub const fn wmb_d03(self) -> crate::pac::common::Reg<WmbD03, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Wake-Up Message Buffer Register Data 4-7."]
    #[inline(always)]
    pub const fn wmb_d47(self) -> crate::pac::common::Reg<WmbD47, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = "CAN Bit Timing."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbt(pub u32);
impl Cbt {
    #[doc = "Extended Phase Segment 2."]
    #[must_use]
    #[inline(always)]
    pub const fn epseg2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Extended Phase Segment 2."]
    #[inline(always)]
    pub const fn set_epseg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Extended Phase Segment 1."]
    #[must_use]
    #[inline(always)]
    pub const fn epseg1(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "Extended Phase Segment 1."]
    #[inline(always)]
    pub const fn set_epseg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "Extended Propagation Segment."]
    #[must_use]
    #[inline(always)]
    pub const fn epropseg(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "Extended Propagation Segment."]
    #[inline(always)]
    pub const fn set_epropseg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
    }
    #[doc = "Extended Resync Jump Width."]
    #[must_use]
    #[inline(always)]
    pub const fn erjw(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Extended Resync Jump Width."]
    #[inline(always)]
    pub const fn set_erjw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Extended Prescaler Division Factor."]
    #[must_use]
    #[inline(always)]
    pub const fn epresdiv(&self) -> u16 {
        let val = (self.0 >> 21usize) & 0x03ff;
        val as u16
    }
    #[doc = "Extended Prescaler Division Factor."]
    #[inline(always)]
    pub const fn set_epresdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 21usize)) | (((val as u32) & 0x03ff) << 21usize);
    }
    #[doc = "Bit Timing Format Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn btf(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Timing Format Enable."]
    #[inline(always)]
    pub const fn set_btf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cbt {
    #[inline(always)]
    fn default() -> Cbt {
        Cbt(0)
    }
}
impl core::fmt::Debug for Cbt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cbt")
            .field("epseg2", &self.epseg2())
            .field("epseg1", &self.epseg1())
            .field("epropseg", &self.epropseg())
            .field("erjw", &self.erjw())
            .field("epresdiv", &self.epresdiv())
            .field("btf", &self.btf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cbt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cbt {{ epseg2: {=u8:?}, epseg1: {=u8:?}, epropseg: {=u8:?}, erjw: {=u8:?}, epresdiv: {=u16:?}, btf: {=bool:?} }}",
            self.epseg2(),
            self.epseg1(),
            self.epropseg(),
            self.erjw(),
            self.epresdiv(),
            self.btf()
        )
    }
}
#[doc = "Cyclic Redundancy Check."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcr(pub u32);
impl Crcr {
    #[doc = "Transmitted CRC value."]
    #[must_use]
    #[inline(always)]
    pub const fn txcrc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Transmitted CRC value."]
    #[inline(always)]
    pub const fn set_txcrc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "CRC Message Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn mbcrc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "CRC Message Buffer."]
    #[inline(always)]
    pub const fn set_mbcrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Crcr {
    #[inline(always)]
    fn default() -> Crcr {
        Crcr(0)
    }
}
impl core::fmt::Debug for Crcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crcr")
            .field("txcrc", &self.txcrc())
            .field("mbcrc", &self.mbcrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Crcr {{ txcrc: {=u16:?}, mbcrc: {=u8:?} }}",
            self.txcrc(),
            self.mbcrc()
        )
    }
}
#[doc = "Message Buffer CS Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs(pub u32);
impl Cs {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    #[must_use]
    #[inline(always)]
    pub const fn time_stamp(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    #[inline(always)]
    pub const fn set_time_stamp(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Length of the data to be stored/transmitted."]
    #[must_use]
    #[inline(always)]
    pub const fn dlc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Length of the data to be stored/transmitted."]
    #[inline(always)]
    pub const fn set_dlc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    #[must_use]
    #[inline(always)]
    pub const fn rtr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    #[inline(always)]
    pub const fn set_rtr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    #[must_use]
    #[inline(always)]
    pub const fn ide(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    #[inline(always)]
    pub const fn set_ide(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    #[must_use]
    #[inline(always)]
    pub const fn srr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    #[inline(always)]
    pub const fn set_srr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    #[must_use]
    #[inline(always)]
    pub const fn code(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    #[inline(always)]
    pub const fn set_code(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    #[must_use]
    #[inline(always)]
    pub const fn esi(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    #[inline(always)]
    pub const fn set_esi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    #[must_use]
    #[inline(always)]
    pub const fn brs(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    #[inline(always)]
    pub const fn set_brs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    #[must_use]
    #[inline(always)]
    pub const fn edl(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    #[inline(always)]
    pub const fn set_edl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cs {
    #[inline(always)]
    fn default() -> Cs {
        Cs(0)
    }
}
impl core::fmt::Debug for Cs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cs")
            .field("time_stamp", &self.time_stamp())
            .field("dlc", &self.dlc())
            .field("rtr", &self.rtr())
            .field("ide", &self.ide())
            .field("srr", &self.srr())
            .field("code", &self.code())
            .field("esi", &self.esi())
            .field("brs", &self.brs())
            .field("edl", &self.edl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cs {{ time_stamp: {=u16:?}, dlc: {=u8:?}, rtr: {=bool:?}, ide: {=bool:?}, srr: {=bool:?}, code: {=u8:?}, esi: {=bool:?}, brs: {=bool:?}, edl: {=bool:?} }}",
            self.time_stamp(),
            self.dlc(),
            self.rtr(),
            self.ide(),
            self.srr(),
            self.code(),
            self.esi(),
            self.brs(),
            self.edl()
        )
    }
}
#[doc = "Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1(pub u32);
impl Ctrl1 {
    #[doc = "Propagation Segment."]
    #[must_use]
    #[inline(always)]
    pub const fn propseg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Propagation Segment."]
    #[inline(always)]
    pub const fn set_propseg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Listen-Only Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn lom(&self) -> Lom {
        let val = (self.0 >> 3usize) & 0x01;
        Lom::from_bits(val as u8)
    }
    #[doc = "Listen-Only Mode."]
    #[inline(always)]
    pub const fn set_lom(&mut self, val: Lom) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Lowest Buffer Transmitted First."]
    #[must_use]
    #[inline(always)]
    pub const fn lbuf(&self) -> Lbuf {
        let val = (self.0 >> 4usize) & 0x01;
        Lbuf::from_bits(val as u8)
    }
    #[doc = "Lowest Buffer Transmitted First."]
    #[inline(always)]
    pub const fn set_lbuf(&mut self, val: Lbuf) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Timer Sync."]
    #[must_use]
    #[inline(always)]
    pub const fn tsyn(&self) -> Tsyn {
        let val = (self.0 >> 5usize) & 0x01;
        Tsyn::from_bits(val as u8)
    }
    #[doc = "Timer Sync."]
    #[inline(always)]
    pub const fn set_tsyn(&mut self, val: Tsyn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Bus Off Recovery."]
    #[must_use]
    #[inline(always)]
    pub const fn boffrec(&self) -> Boffrec {
        let val = (self.0 >> 6usize) & 0x01;
        Boffrec::from_bits(val as u8)
    }
    #[doc = "Bus Off Recovery."]
    #[inline(always)]
    pub const fn set_boffrec(&mut self, val: Boffrec) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "CAN Bit Sampling."]
    #[must_use]
    #[inline(always)]
    pub const fn smp(&self) -> Smp {
        let val = (self.0 >> 7usize) & 0x01;
        Smp::from_bits(val as u8)
    }
    #[doc = "CAN Bit Sampling."]
    #[inline(always)]
    pub const fn set_smp(&mut self, val: Smp) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "RX Warning Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn rwrnmsk(&self) -> Rwrnmsk {
        let val = (self.0 >> 10usize) & 0x01;
        Rwrnmsk::from_bits(val as u8)
    }
    #[doc = "RX Warning Interrupt Mask."]
    #[inline(always)]
    pub const fn set_rwrnmsk(&mut self, val: Rwrnmsk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "TX Warning Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn twrnmsk(&self) -> Twrnmsk {
        let val = (self.0 >> 11usize) & 0x01;
        Twrnmsk::from_bits(val as u8)
    }
    #[doc = "TX Warning Interrupt Mask."]
    #[inline(always)]
    pub const fn set_twrnmsk(&mut self, val: Twrnmsk) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Loopback Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn lpb(&self) -> Lpb {
        let val = (self.0 >> 12usize) & 0x01;
        Lpb::from_bits(val as u8)
    }
    #[doc = "Loopback Mode."]
    #[inline(always)]
    pub const fn set_lpb(&mut self, val: Lpb) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Error Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn errmsk(&self) -> Errmsk {
        let val = (self.0 >> 14usize) & 0x01;
        Errmsk::from_bits(val as u8)
    }
    #[doc = "Error Interrupt Mask."]
    #[inline(always)]
    pub const fn set_errmsk(&mut self, val: Errmsk) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Bus Off Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn boffmsk(&self) -> Boffmsk {
        let val = (self.0 >> 15usize) & 0x01;
        Boffmsk::from_bits(val as u8)
    }
    #[doc = "Bus Off Interrupt Mask."]
    #[inline(always)]
    pub const fn set_boffmsk(&mut self, val: Boffmsk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Phase Segment 2."]
    #[must_use]
    #[inline(always)]
    pub const fn pseg2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Phase Segment 2."]
    #[inline(always)]
    pub const fn set_pseg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Phase Segment 1."]
    #[must_use]
    #[inline(always)]
    pub const fn pseg1(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x07;
        val as u8
    }
    #[doc = "Phase Segment 1."]
    #[inline(always)]
    pub const fn set_pseg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
    }
    #[doc = "Resync Jump Width."]
    #[must_use]
    #[inline(always)]
    pub const fn rjw(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Resync Jump Width."]
    #[inline(always)]
    pub const fn set_rjw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Prescaler Division Factor."]
    #[must_use]
    #[inline(always)]
    pub const fn presdiv(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Prescaler Division Factor."]
    #[inline(always)]
    pub const fn set_presdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ctrl1 {
    #[inline(always)]
    fn default() -> Ctrl1 {
        Ctrl1(0)
    }
}
impl core::fmt::Debug for Ctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1")
            .field("propseg", &self.propseg())
            .field("lom", &self.lom())
            .field("lbuf", &self.lbuf())
            .field("tsyn", &self.tsyn())
            .field("boffrec", &self.boffrec())
            .field("smp", &self.smp())
            .field("rwrnmsk", &self.rwrnmsk())
            .field("twrnmsk", &self.twrnmsk())
            .field("lpb", &self.lpb())
            .field("errmsk", &self.errmsk())
            .field("boffmsk", &self.boffmsk())
            .field("pseg2", &self.pseg2())
            .field("pseg1", &self.pseg1())
            .field("rjw", &self.rjw())
            .field("presdiv", &self.presdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl1 {{ propseg: {=u8:?}, lom: {:?}, lbuf: {:?}, tsyn: {:?}, boffrec: {:?}, smp: {:?}, rwrnmsk: {:?}, twrnmsk: {:?}, lpb: {:?}, errmsk: {:?}, boffmsk: {:?}, pseg2: {=u8:?}, pseg1: {=u8:?}, rjw: {=u8:?}, presdiv: {=u8:?} }}",
            self.propseg(),
            self.lom(),
            self.lbuf(),
            self.tsyn(),
            self.boffrec(),
            self.smp(),
            self.rwrnmsk(),
            self.twrnmsk(),
            self.lpb(),
            self.errmsk(),
            self.boffmsk(),
            self.pseg2(),
            self.pseg1(),
            self.rjw(),
            self.presdiv()
        )
    }
}
#[doc = "Pretended Networking Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1Pn(pub u32);
impl Ctrl1Pn {
    #[doc = "Filtering Combination Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn fcs(&self) -> Fcs {
        let val = (self.0 >> 0usize) & 0x03;
        Fcs::from_bits(val as u8)
    }
    #[doc = "Filtering Combination Selection."]
    #[inline(always)]
    pub const fn set_fcs(&mut self, val: Fcs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "ID Filtering Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn idfs(&self) -> Idfs {
        let val = (self.0 >> 2usize) & 0x03;
        Idfs::from_bits(val as u8)
    }
    #[doc = "ID Filtering Selection."]
    #[inline(always)]
    pub const fn set_idfs(&mut self, val: Idfs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Payload Filtering Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn plfs(&self) -> Plfs {
        let val = (self.0 >> 4usize) & 0x03;
        Plfs::from_bits(val as u8)
    }
    #[doc = "Payload Filtering Selection."]
    #[inline(always)]
    pub const fn set_plfs(&mut self, val: Plfs) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Number of Messages Matching the Same Filtering Criteria."]
    #[must_use]
    #[inline(always)]
    pub const fn nmatch(&self) -> Nmatch {
        let val = (self.0 >> 8usize) & 0xff;
        Nmatch::from_bits(val as u8)
    }
    #[doc = "Number of Messages Matching the Same Filtering Criteria."]
    #[inline(always)]
    pub const fn set_nmatch(&mut self, val: Nmatch) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Wake-up by Matching Flag Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn wumf_msk(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up by Matching Flag Mask."]
    #[inline(always)]
    pub const fn set_wumf_msk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-up by Timeout Flag Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn wtof_msk(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up by Timeout Flag Mask."]
    #[inline(always)]
    pub const fn set_wtof_msk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Ctrl1Pn {
    #[inline(always)]
    fn default() -> Ctrl1Pn {
        Ctrl1Pn(0)
    }
}
impl core::fmt::Debug for Ctrl1Pn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1Pn")
            .field("fcs", &self.fcs())
            .field("idfs", &self.idfs())
            .field("plfs", &self.plfs())
            .field("nmatch", &self.nmatch())
            .field("wumf_msk", &self.wumf_msk())
            .field("wtof_msk", &self.wtof_msk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1Pn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl1Pn {{ fcs: {:?}, idfs: {:?}, plfs: {:?}, nmatch: {:?}, wumf_msk: {=bool:?}, wtof_msk: {=bool:?} }}",
            self.fcs(),
            self.idfs(),
            self.plfs(),
            self.nmatch(),
            self.wumf_msk(),
            self.wtof_msk()
        )
    }
}
#[doc = "Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2(pub u32);
impl Ctrl2 {
    #[doc = "Edge Filter Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn edfltdis(&self) -> Edfltdis {
        let val = (self.0 >> 11usize) & 0x01;
        Edfltdis::from_bits(val as u8)
    }
    #[doc = "Edge Filter Disable."]
    #[inline(always)]
    pub const fn set_edfltdis(&mut self, val: Edfltdis) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "ISO CAN FD Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn isocanfden(&self) -> Isocanfden {
        let val = (self.0 >> 12usize) & 0x01;
        Isocanfden::from_bits(val as u8)
    }
    #[doc = "ISO CAN FD Enable."]
    #[inline(always)]
    pub const fn set_isocanfden(&mut self, val: Isocanfden) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Bit Timing Expansion Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bte(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Timing Expansion Enable."]
    #[inline(always)]
    pub const fn set_bte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Protocol Exception Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prexcen(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol Exception Enable."]
    #[inline(always)]
    pub const fn set_prexcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Entire Frame Arbitration Field Comparison Enable for RX Message Buffers."]
    #[must_use]
    #[inline(always)]
    pub const fn eacen(&self) -> Eacen {
        let val = (self.0 >> 16usize) & 0x01;
        Eacen::from_bits(val as u8)
    }
    #[doc = "Entire Frame Arbitration Field Comparison Enable for RX Message Buffers."]
    #[inline(always)]
    pub const fn set_eacen(&mut self, val: Eacen) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Remote Request Storing."]
    #[must_use]
    #[inline(always)]
    pub const fn rrs(&self) -> Rrs {
        let val = (self.0 >> 17usize) & 0x01;
        Rrs::from_bits(val as u8)
    }
    #[doc = "Remote Request Storing."]
    #[inline(always)]
    pub const fn set_rrs(&mut self, val: Rrs) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Message Buffers Reception Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn mrp(&self) -> Mrp {
        let val = (self.0 >> 18usize) & 0x01;
        Mrp::from_bits(val as u8)
    }
    #[doc = "Message Buffers Reception Priority."]
    #[inline(always)]
    pub const fn set_mrp(&mut self, val: Mrp) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Transmission Arbitration Start Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn tasd(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "Transmission Arbitration Start Delay."]
    #[inline(always)]
    pub const fn set_tasd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "Number of Legacy Receive FIFO Filters."]
    #[must_use]
    #[inline(always)]
    pub const fn rffn(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Legacy Receive FIFO Filters."]
    #[inline(always)]
    pub const fn set_rffn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Bus Off Done Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn boffdonemsk(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Off Done Interrupt Mask."]
    #[inline(always)]
    pub const fn set_boffdonemsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Error Interrupt Mask for Errors Detected in the Data Phase of Fast CAN FD Frames."]
    #[must_use]
    #[inline(always)]
    pub const fn errmsk_fast(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Error Interrupt Mask for Errors Detected in the Data Phase of Fast CAN FD Frames."]
    #[inline(always)]
    pub const fn set_errmsk_fast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl2 {
    #[inline(always)]
    fn default() -> Ctrl2 {
        Ctrl2(0)
    }
}
impl core::fmt::Debug for Ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2")
            .field("edfltdis", &self.edfltdis())
            .field("isocanfden", &self.isocanfden())
            .field("bte", &self.bte())
            .field("prexcen", &self.prexcen())
            .field("eacen", &self.eacen())
            .field("rrs", &self.rrs())
            .field("mrp", &self.mrp())
            .field("tasd", &self.tasd())
            .field("rffn", &self.rffn())
            .field("boffdonemsk", &self.boffdonemsk())
            .field("errmsk_fast", &self.errmsk_fast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl2 {{ edfltdis: {:?}, isocanfden: {:?}, bte: {=bool:?}, prexcen: {=bool:?}, eacen: {:?}, rrs: {:?}, mrp: {:?}, tasd: {=u8:?}, rffn: {=u8:?}, boffdonemsk: {=bool:?}, errmsk_fast: {=bool:?} }}",
            self.edfltdis(),
            self.isocanfden(),
            self.bte(),
            self.prexcen(),
            self.eacen(),
            self.rrs(),
            self.mrp(),
            self.tasd(),
            self.rffn(),
            self.boffdonemsk(),
            self.errmsk_fast()
        )
    }
}
#[doc = "Pretended Networking Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2Pn(pub u32);
impl Ctrl2Pn {
    #[doc = "Timeout for No Message Matching the Filtering Criteria."]
    #[must_use]
    #[inline(always)]
    pub const fn matchto(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timeout for No Message Matching the Filtering Criteria."]
    #[inline(always)]
    pub const fn set_matchto(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ctrl2Pn {
    #[inline(always)]
    fn default() -> Ctrl2Pn {
        Ctrl2Pn(0)
    }
}
impl core::fmt::Debug for Ctrl2Pn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2Pn")
            .field("matchto", &self.matchto())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2Pn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctrl2Pn {{ matchto: {=u16:?} }}", self.matchto())
    }
}
#[doc = "Error Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecr(pub u32);
impl Ecr {
    #[doc = "Transmit Error Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn txerrcnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Error Counter."]
    #[inline(always)]
    pub const fn set_txerrcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive Error Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn rxerrcnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Error Counter."]
    #[inline(always)]
    pub const fn set_rxerrcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Transmit Error Counter for Fast Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn txerrcnt_fast(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Error Counter for Fast Bits."]
    #[inline(always)]
    pub const fn set_txerrcnt_fast(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Receive Error Counter for Fast Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn rxerrcnt_fast(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Error Counter for Fast Bits."]
    #[inline(always)]
    pub const fn set_rxerrcnt_fast(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ecr {
    #[inline(always)]
    fn default() -> Ecr {
        Ecr(0)
    }
}
impl core::fmt::Debug for Ecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ecr")
            .field("txerrcnt", &self.txerrcnt())
            .field("rxerrcnt", &self.rxerrcnt())
            .field("txerrcnt_fast", &self.txerrcnt_fast())
            .field("rxerrcnt_fast", &self.rxerrcnt_fast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ecr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ecr {{ txerrcnt: {=u8:?}, rxerrcnt: {=u8:?}, txerrcnt_fast: {=u8:?}, rxerrcnt_fast: {=u8:?} }}",
            self.txerrcnt(),
            self.rxerrcnt(),
            self.txerrcnt_fast(),
            self.rxerrcnt_fast()
        )
    }
}
#[doc = "Enhanced Data Phase CAN Bit Timing."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edcbt(pub u32);
impl Edcbt {
    #[doc = "Data Phase Segment 1."]
    #[must_use]
    #[inline(always)]
    pub const fn dtseg1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Data Phase Segment 1."]
    #[inline(always)]
    pub const fn set_dtseg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Data Phase Time Segment 2."]
    #[must_use]
    #[inline(always)]
    pub const fn dtseg2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Data Phase Time Segment 2."]
    #[inline(always)]
    pub const fn set_dtseg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Data Phase Resynchronization Jump Width."]
    #[must_use]
    #[inline(always)]
    pub const fn drjw(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x0f;
        val as u8
    }
    #[doc = "Data Phase Resynchronization Jump Width."]
    #[inline(always)]
    pub const fn set_drjw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
    }
}
impl Default for Edcbt {
    #[inline(always)]
    fn default() -> Edcbt {
        Edcbt(0)
    }
}
impl core::fmt::Debug for Edcbt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Edcbt")
            .field("dtseg1", &self.dtseg1())
            .field("dtseg2", &self.dtseg2())
            .field("drjw", &self.drjw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Edcbt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Edcbt {{ dtseg1: {=u8:?}, dtseg2: {=u8:?}, drjw: {=u8:?} }}",
            self.dtseg1(),
            self.dtseg2(),
            self.drjw()
        )
    }
}
#[doc = "Enhanced Nominal CAN Bit Timing."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Encbt(pub u32);
impl Encbt {
    #[doc = "Nominal Time Segment 1."]
    #[must_use]
    #[inline(always)]
    pub const fn ntseg1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Nominal Time Segment 1."]
    #[inline(always)]
    pub const fn set_ntseg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Nominal Time Segment 2."]
    #[must_use]
    #[inline(always)]
    pub const fn ntseg2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x7f;
        val as u8
    }
    #[doc = "Nominal Time Segment 2."]
    #[inline(always)]
    pub const fn set_ntseg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 12usize)) | (((val as u32) & 0x7f) << 12usize);
    }
    #[doc = "Nominal Resynchronization Jump Width."]
    #[must_use]
    #[inline(always)]
    pub const fn nrjw(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x7f;
        val as u8
    }
    #[doc = "Nominal Resynchronization Jump Width."]
    #[inline(always)]
    pub const fn set_nrjw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
    }
}
impl Default for Encbt {
    #[inline(always)]
    fn default() -> Encbt {
        Encbt(0)
    }
}
impl core::fmt::Debug for Encbt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Encbt")
            .field("ntseg1", &self.ntseg1())
            .field("ntseg2", &self.ntseg2())
            .field("nrjw", &self.nrjw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Encbt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Encbt {{ ntseg1: {=u8:?}, ntseg2: {=u8:?}, nrjw: {=u8:?} }}",
            self.ntseg1(),
            self.ntseg2(),
            self.nrjw()
        )
    }
}
#[doc = "Enhanced CAN Bit Timing Prescalers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eprs(pub u32);
impl Eprs {
    #[doc = "Extended Nominal Prescaler Division Factor."]
    #[must_use]
    #[inline(always)]
    pub const fn enpresdiv(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Extended Nominal Prescaler Division Factor."]
    #[inline(always)]
    pub const fn set_enpresdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Extended Data Phase Prescaler Division Factor."]
    #[must_use]
    #[inline(always)]
    pub const fn edpresdiv(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Extended Data Phase Prescaler Division Factor."]
    #[inline(always)]
    pub const fn set_edpresdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Eprs {
    #[inline(always)]
    fn default() -> Eprs {
        Eprs(0)
    }
}
impl core::fmt::Debug for Eprs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eprs")
            .field("enpresdiv", &self.enpresdiv())
            .field("edpresdiv", &self.edpresdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eprs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eprs {{ enpresdiv: {=u16:?}, edpresdiv: {=u16:?} }}",
            self.enpresdiv(),
            self.edpresdiv()
        )
    }
}
#[doc = "Enhanced RX FIFO Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erfcr(pub u32);
impl Erfcr {
    #[doc = "Enhanced RX FIFO Watermark."]
    #[must_use]
    #[inline(always)]
    pub const fn erfwm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Enhanced RX FIFO Watermark."]
    #[inline(always)]
    pub const fn set_erfwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Number of Enhanced RX FIFO Filter Elements."]
    #[must_use]
    #[inline(always)]
    pub const fn nfe(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Number of Enhanced RX FIFO Filter Elements."]
    #[inline(always)]
    pub const fn set_nfe(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Number of Extended ID Filter Elements."]
    #[must_use]
    #[inline(always)]
    pub const fn nexif(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Number of Extended ID Filter Elements."]
    #[inline(always)]
    pub const fn set_nexif(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "DMA Last Word."]
    #[must_use]
    #[inline(always)]
    pub const fn dmalw(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "DMA Last Word."]
    #[inline(always)]
    pub const fn set_dmalw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
    #[doc = "Enhanced RX FIFO enable."]
    #[must_use]
    #[inline(always)]
    pub const fn erfen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO enable."]
    #[inline(always)]
    pub const fn set_erfen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Erfcr {
    #[inline(always)]
    fn default() -> Erfcr {
        Erfcr(0)
    }
}
impl core::fmt::Debug for Erfcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Erfcr")
            .field("erfwm", &self.erfwm())
            .field("nfe", &self.nfe())
            .field("nexif", &self.nexif())
            .field("dmalw", &self.dmalw())
            .field("erfen", &self.erfen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Erfcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Erfcr {{ erfwm: {=u8:?}, nfe: {=u8:?}, nexif: {=u8:?}, dmalw: {=u8:?}, erfen: {=bool:?} }}",
            self.erfwm(),
            self.nfe(),
            self.nexif(),
            self.dmalw(),
            self.erfen()
        )
    }
}
#[doc = "Enhanced RX FIFO Filter Element."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erffel(pub u32);
impl Erffel {
    #[doc = "Filter Element Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn fel(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Filter Element Bits."]
    #[inline(always)]
    pub const fn set_fel(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Erffel {
    #[inline(always)]
    fn default() -> Erffel {
        Erffel(0)
    }
}
impl core::fmt::Debug for Erffel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Erffel").field("fel", &self.fel()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Erffel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Erffel {{ fel: {=u32:?} }}", self.fel())
    }
}
#[doc = "Enhanced RX FIFO Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erfier(pub u32);
impl Erfier {
    #[doc = "Enhanced RX FIFO Data Available Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn erfdaie(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO Data Available Interrupt Enable."]
    #[inline(always)]
    pub const fn set_erfdaie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Enhanced RX FIFO Watermark Indication Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn erfwmiie(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO Watermark Indication Interrupt Enable."]
    #[inline(always)]
    pub const fn set_erfwmiie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Enhanced RX FIFO Overflow Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn erfovfie(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO Overflow Interrupt Enable."]
    #[inline(always)]
    pub const fn set_erfovfie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enhanced RX FIFO Underflow Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn erfufwie(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO Underflow Interrupt Enable."]
    #[inline(always)]
    pub const fn set_erfufwie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Erfier {
    #[inline(always)]
    fn default() -> Erfier {
        Erfier(0)
    }
}
impl core::fmt::Debug for Erfier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Erfier")
            .field("erfdaie", &self.erfdaie())
            .field("erfwmiie", &self.erfwmiie())
            .field("erfovfie", &self.erfovfie())
            .field("erfufwie", &self.erfufwie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Erfier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Erfier {{ erfdaie: {=bool:?}, erfwmiie: {=bool:?}, erfovfie: {=bool:?}, erfufwie: {=bool:?} }}",
            self.erfdaie(),
            self.erfwmiie(),
            self.erfovfie(),
            self.erfufwie()
        )
    }
}
#[doc = "Enhanced RX FIFO Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erfsr(pub u32);
impl Erfsr {
    #[doc = "Enhanced RX FIFO Elements."]
    #[must_use]
    #[inline(always)]
    pub const fn erfel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Enhanced RX FIFO Elements."]
    #[inline(always)]
    pub const fn set_erfel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Enhanced RX FIFO Full Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn erff(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO Full Flag."]
    #[inline(always)]
    pub const fn set_erff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enhanced RX FIFO Empty Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn erfe(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO Empty Flag."]
    #[inline(always)]
    pub const fn set_erfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enhanced RX FIFO Clear."]
    #[must_use]
    #[inline(always)]
    pub const fn erfclr(&self) -> Erfclr {
        let val = (self.0 >> 27usize) & 0x01;
        Erfclr::from_bits(val as u8)
    }
    #[doc = "Enhanced RX FIFO Clear."]
    #[inline(always)]
    pub const fn set_erfclr(&mut self, val: Erfclr) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Enhanced RX FIFO Data Available Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn erfda(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO Data Available Flag."]
    #[inline(always)]
    pub const fn set_erfda(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Enhanced RX FIFO Watermark Indication Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn erfwmi(&self) -> Erfwmi {
        let val = (self.0 >> 29usize) & 0x01;
        Erfwmi::from_bits(val as u8)
    }
    #[doc = "Enhanced RX FIFO Watermark Indication Flag."]
    #[inline(always)]
    pub const fn set_erfwmi(&mut self, val: Erfwmi) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Enhanced RX FIFO Overflow Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn erfovf(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO Overflow Flag."]
    #[inline(always)]
    pub const fn set_erfovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enhanced RX FIFO Underflow Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn erfufw(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO Underflow Flag."]
    #[inline(always)]
    pub const fn set_erfufw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Erfsr {
    #[inline(always)]
    fn default() -> Erfsr {
        Erfsr(0)
    }
}
impl core::fmt::Debug for Erfsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Erfsr")
            .field("erfel", &self.erfel())
            .field("erff", &self.erff())
            .field("erfe", &self.erfe())
            .field("erfclr", &self.erfclr())
            .field("erfda", &self.erfda())
            .field("erfwmi", &self.erfwmi())
            .field("erfovf", &self.erfovf())
            .field("erfufw", &self.erfufw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Erfsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Erfsr {{ erfel: {=u8:?}, erff: {=bool:?}, erfe: {=bool:?}, erfclr: {:?}, erfda: {=bool:?}, erfwmi: {:?}, erfovf: {=bool:?}, erfufw: {=bool:?} }}",
            self.erfel(),
            self.erff(),
            self.erfe(),
            self.erfclr(),
            self.erfda(),
            self.erfwmi(),
            self.erfovf(),
            self.erfufw()
        )
    }
}
#[doc = "Error and Status 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esr1(pub u32);
impl Esr1 {
    #[doc = "Wake-up Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn wakint(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Interrupt Flag."]
    #[inline(always)]
    pub const fn set_wakint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn errint(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_errint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bus Off Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn boffint(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Off Interrupt Flag."]
    #[inline(always)]
    pub const fn set_boffint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FlexCAN in Reception Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rx(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FlexCAN in Reception Flag."]
    #[inline(always)]
    pub const fn set_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Fault Confinement State."]
    #[must_use]
    #[inline(always)]
    pub const fn fltconf(&self) -> Fltconf {
        let val = (self.0 >> 4usize) & 0x03;
        Fltconf::from_bits(val as u8)
    }
    #[doc = "Fault Confinement State."]
    #[inline(always)]
    pub const fn set_fltconf(&mut self, val: Fltconf) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "FlexCAN In Transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn tx(&self) -> Tx {
        let val = (self.0 >> 6usize) & 0x01;
        Tx::from_bits(val as u8)
    }
    #[doc = "FlexCAN In Transmission."]
    #[inline(always)]
    pub const fn set_tx(&mut self, val: Tx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Idle."]
    #[must_use]
    #[inline(always)]
    pub const fn idle(&self) -> Idle {
        let val = (self.0 >> 7usize) & 0x01;
        Idle::from_bits(val as u8)
    }
    #[doc = "Idle."]
    #[inline(always)]
    pub const fn set_idle(&mut self, val: Idle) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "RX Error Warning Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxwrn(&self) -> Rxwrn {
        let val = (self.0 >> 8usize) & 0x01;
        Rxwrn::from_bits(val as u8)
    }
    #[doc = "RX Error Warning Flag."]
    #[inline(always)]
    pub const fn set_rxwrn(&mut self, val: Rxwrn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "TX Error Warning Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txwrn(&self) -> Txwrn {
        let val = (self.0 >> 9usize) & 0x01;
        Txwrn::from_bits(val as u8)
    }
    #[doc = "TX Error Warning Flag."]
    #[inline(always)]
    pub const fn set_txwrn(&mut self, val: Txwrn) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Stuffing Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn stferr(&self) -> Stferr {
        let val = (self.0 >> 10usize) & 0x01;
        Stferr::from_bits(val as u8)
    }
    #[doc = "Stuffing Error Flag."]
    #[inline(always)]
    pub const fn set_stferr(&mut self, val: Stferr) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Form Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn frmerr(&self) -> Frmerr {
        let val = (self.0 >> 11usize) & 0x01;
        Frmerr::from_bits(val as u8)
    }
    #[doc = "Form Error Flag."]
    #[inline(always)]
    pub const fn set_frmerr(&mut self, val: Frmerr) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Cyclic Redundancy Check Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn crcerr(&self) -> Crcerr {
        let val = (self.0 >> 12usize) & 0x01;
        Crcerr::from_bits(val as u8)
    }
    #[doc = "Cyclic Redundancy Check Error Flag."]
    #[inline(always)]
    pub const fn set_crcerr(&mut self, val: Crcerr) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Acknowledge Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ackerr(&self) -> Ackerr {
        let val = (self.0 >> 13usize) & 0x01;
        Ackerr::from_bits(val as u8)
    }
    #[doc = "Acknowledge Error Flag."]
    #[inline(always)]
    pub const fn set_ackerr(&mut self, val: Ackerr) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Bit0 Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn bit0err(&self) -> Bit0err {
        let val = (self.0 >> 14usize) & 0x01;
        Bit0err::from_bits(val as u8)
    }
    #[doc = "Bit0 Error Flag."]
    #[inline(always)]
    pub const fn set_bit0err(&mut self, val: Bit0err) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Bit1 Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn bit1err(&self) -> Bit1err {
        let val = (self.0 >> 15usize) & 0x01;
        Bit1err::from_bits(val as u8)
    }
    #[doc = "Bit1 Error Flag."]
    #[inline(always)]
    pub const fn set_bit1err(&mut self, val: Bit1err) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "RX Warning Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rwrnint(&self) -> Rwrnint {
        let val = (self.0 >> 16usize) & 0x01;
        Rwrnint::from_bits(val as u8)
    }
    #[doc = "RX Warning Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rwrnint(&mut self, val: Rwrnint) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Warning Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn twrnint(&self) -> Twrnint {
        let val = (self.0 >> 17usize) & 0x01;
        Twrnint::from_bits(val as u8)
    }
    #[doc = "TX Warning Interrupt Flag."]
    #[inline(always)]
    pub const fn set_twrnint(&mut self, val: Twrnint) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "CAN Synchronization Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn synch(&self) -> Synch {
        let val = (self.0 >> 18usize) & 0x01;
        Synch::from_bits(val as u8)
    }
    #[doc = "CAN Synchronization Status Flag."]
    #[inline(always)]
    pub const fn set_synch(&mut self, val: Synch) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Bus Off Done Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn boffdoneint(&self) -> Boffdoneint {
        let val = (self.0 >> 19usize) & 0x01;
        Boffdoneint::from_bits(val as u8)
    }
    #[doc = "Bus Off Done Interrupt Flag."]
    #[inline(always)]
    pub const fn set_boffdoneint(&mut self, val: Boffdoneint) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Fast Error Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn errint_fast(&self) -> ErrintFast {
        let val = (self.0 >> 20usize) & 0x01;
        ErrintFast::from_bits(val as u8)
    }
    #[doc = "Fast Error Interrupt Flag."]
    #[inline(always)]
    pub const fn set_errint_fast(&mut self, val: ErrintFast) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Error Overrun Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn errovr(&self) -> Errovr {
        let val = (self.0 >> 21usize) & 0x01;
        Errovr::from_bits(val as u8)
    }
    #[doc = "Error Overrun Flag."]
    #[inline(always)]
    pub const fn set_errovr(&mut self, val: Errovr) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Fast Stuffing Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn stferr_fast(&self) -> StferrFast {
        let val = (self.0 >> 26usize) & 0x01;
        StferrFast::from_bits(val as u8)
    }
    #[doc = "Fast Stuffing Error Flag."]
    #[inline(always)]
    pub const fn set_stferr_fast(&mut self, val: StferrFast) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Fast Form Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn frmerr_fast(&self) -> FrmerrFast {
        let val = (self.0 >> 27usize) & 0x01;
        FrmerrFast::from_bits(val as u8)
    }
    #[doc = "Fast Form Error Flag."]
    #[inline(always)]
    pub const fn set_frmerr_fast(&mut self, val: FrmerrFast) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Fast Cyclic Redundancy Check Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn crcerr_fast(&self) -> CrcerrFast {
        let val = (self.0 >> 28usize) & 0x01;
        CrcerrFast::from_bits(val as u8)
    }
    #[doc = "Fast Cyclic Redundancy Check Error Flag."]
    #[inline(always)]
    pub const fn set_crcerr_fast(&mut self, val: CrcerrFast) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Fast Bit0 Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn bit0err_fast(&self) -> Bit0errFast {
        let val = (self.0 >> 30usize) & 0x01;
        Bit0errFast::from_bits(val as u8)
    }
    #[doc = "Fast Bit0 Error Flag."]
    #[inline(always)]
    pub const fn set_bit0err_fast(&mut self, val: Bit0errFast) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Fast Bit1 Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn bit1err_fast(&self) -> Bit1errFast {
        let val = (self.0 >> 31usize) & 0x01;
        Bit1errFast::from_bits(val as u8)
    }
    #[doc = "Fast Bit1 Error Flag."]
    #[inline(always)]
    pub const fn set_bit1err_fast(&mut self, val: Bit1errFast) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Esr1 {
    #[inline(always)]
    fn default() -> Esr1 {
        Esr1(0)
    }
}
impl core::fmt::Debug for Esr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Esr1")
            .field("wakint", &self.wakint())
            .field("errint", &self.errint())
            .field("boffint", &self.boffint())
            .field("rx", &self.rx())
            .field("fltconf", &self.fltconf())
            .field("tx", &self.tx())
            .field("idle", &self.idle())
            .field("rxwrn", &self.rxwrn())
            .field("txwrn", &self.txwrn())
            .field("stferr", &self.stferr())
            .field("frmerr", &self.frmerr())
            .field("crcerr", &self.crcerr())
            .field("ackerr", &self.ackerr())
            .field("bit0err", &self.bit0err())
            .field("bit1err", &self.bit1err())
            .field("rwrnint", &self.rwrnint())
            .field("twrnint", &self.twrnint())
            .field("synch", &self.synch())
            .field("boffdoneint", &self.boffdoneint())
            .field("errint_fast", &self.errint_fast())
            .field("errovr", &self.errovr())
            .field("stferr_fast", &self.stferr_fast())
            .field("frmerr_fast", &self.frmerr_fast())
            .field("crcerr_fast", &self.crcerr_fast())
            .field("bit0err_fast", &self.bit0err_fast())
            .field("bit1err_fast", &self.bit1err_fast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Esr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Esr1 {{ wakint: {=bool:?}, errint: {=bool:?}, boffint: {=bool:?}, rx: {=bool:?}, fltconf: {:?}, tx: {:?}, idle: {:?}, rxwrn: {:?}, txwrn: {:?}, stferr: {:?}, frmerr: {:?}, crcerr: {:?}, ackerr: {:?}, bit0err: {:?}, bit1err: {:?}, rwrnint: {:?}, twrnint: {:?}, synch: {:?}, boffdoneint: {:?}, errint_fast: {:?}, errovr: {:?}, stferr_fast: {:?}, frmerr_fast: {:?}, crcerr_fast: {:?}, bit0err_fast: {:?}, bit1err_fast: {:?} }}",
            self.wakint(),
            self.errint(),
            self.boffint(),
            self.rx(),
            self.fltconf(),
            self.tx(),
            self.idle(),
            self.rxwrn(),
            self.txwrn(),
            self.stferr(),
            self.frmerr(),
            self.crcerr(),
            self.ackerr(),
            self.bit0err(),
            self.bit1err(),
            self.rwrnint(),
            self.twrnint(),
            self.synch(),
            self.boffdoneint(),
            self.errint_fast(),
            self.errovr(),
            self.stferr_fast(),
            self.frmerr_fast(),
            self.crcerr_fast(),
            self.bit0err_fast(),
            self.bit1err_fast()
        )
    }
}
#[doc = "Error and Status 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esr2(pub u32);
impl Esr2 {
    #[doc = "Inactive Message Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn imb(&self) -> Imb {
        let val = (self.0 >> 13usize) & 0x01;
        Imb::from_bits(val as u8)
    }
    #[doc = "Inactive Message Buffer."]
    #[inline(always)]
    pub const fn set_imb(&mut self, val: Imb) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Valid Priority Status."]
    #[must_use]
    #[inline(always)]
    pub const fn vps(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Valid Priority Status."]
    #[inline(always)]
    pub const fn set_vps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Lowest Priority TX Message Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn lptm(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Lowest Priority TX Message Buffer."]
    #[inline(always)]
    pub const fn set_lptm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Esr2 {
    #[inline(always)]
    fn default() -> Esr2 {
        Esr2(0)
    }
}
impl core::fmt::Debug for Esr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Esr2")
            .field("imb", &self.imb())
            .field("vps", &self.vps())
            .field("lptm", &self.lptm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Esr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Esr2 {{ imb: {:?}, vps: {=bool:?}, lptm: {=u8:?} }}",
            self.imb(),
            self.vps(),
            self.lptm()
        )
    }
}
#[doc = "Enhanced Transceiver Delay Compensation."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etdc(pub u32);
impl Etdc {
    #[doc = "Enhanced Transceiver Delay Compensation Value."]
    #[must_use]
    #[inline(always)]
    pub const fn etdcval(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Enhanced Transceiver Delay Compensation Value."]
    #[inline(always)]
    pub const fn set_etdcval(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Transceiver Delay Compensation Fail."]
    #[must_use]
    #[inline(always)]
    pub const fn etdcfail(&self) -> Etdcfail {
        let val = (self.0 >> 15usize) & 0x01;
        Etdcfail::from_bits(val as u8)
    }
    #[doc = "Transceiver Delay Compensation Fail."]
    #[inline(always)]
    pub const fn set_etdcfail(&mut self, val: Etdcfail) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Enhanced Transceiver Delay Compensation Offset."]
    #[must_use]
    #[inline(always)]
    pub const fn etdcoff(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Enhanced Transceiver Delay Compensation Offset."]
    #[inline(always)]
    pub const fn set_etdcoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Transceiver Delay Measurement Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn tdmdis(&self) -> Tdmdis {
        let val = (self.0 >> 30usize) & 0x01;
        Tdmdis::from_bits(val as u8)
    }
    #[doc = "Transceiver Delay Measurement Disable."]
    #[inline(always)]
    pub const fn set_tdmdis(&mut self, val: Tdmdis) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Transceiver Delay Compensation Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn etdcen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Transceiver Delay Compensation Enable."]
    #[inline(always)]
    pub const fn set_etdcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Etdc {
    #[inline(always)]
    fn default() -> Etdc {
        Etdc(0)
    }
}
impl core::fmt::Debug for Etdc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etdc")
            .field("etdcval", &self.etdcval())
            .field("etdcfail", &self.etdcfail())
            .field("etdcoff", &self.etdcoff())
            .field("tdmdis", &self.tdmdis())
            .field("etdcen", &self.etdcen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etdc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Etdc {{ etdcval: {=u8:?}, etdcfail: {:?}, etdcoff: {=u8:?}, tdmdis: {:?}, etdcen: {=bool:?} }}",
            self.etdcval(),
            self.etdcfail(),
            self.etdcoff(),
            self.tdmdis(),
            self.etdcen()
        )
    }
}
#[doc = "CAN FD Bit Timing."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdcbt(pub u32);
impl Fdcbt {
    #[doc = "Fast Phase Segment 2."]
    #[must_use]
    #[inline(always)]
    pub const fn fpseg2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Fast Phase Segment 2."]
    #[inline(always)]
    pub const fn set_fpseg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Fast Phase Segment 1."]
    #[must_use]
    #[inline(always)]
    pub const fn fpseg1(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Fast Phase Segment 1."]
    #[inline(always)]
    pub const fn set_fpseg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Fast Propagation Segment."]
    #[must_use]
    #[inline(always)]
    pub const fn fpropseg(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "Fast Propagation Segment."]
    #[inline(always)]
    pub const fn set_fpropseg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "Fast Resync Jump Width."]
    #[must_use]
    #[inline(always)]
    pub const fn frjw(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Fast Resync Jump Width."]
    #[inline(always)]
    pub const fn set_frjw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Fast Prescaler Division Factor."]
    #[must_use]
    #[inline(always)]
    pub const fn fpresdiv(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x03ff;
        val as u16
    }
    #[doc = "Fast Prescaler Division Factor."]
    #[inline(always)]
    pub const fn set_fpresdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
    }
}
impl Default for Fdcbt {
    #[inline(always)]
    fn default() -> Fdcbt {
        Fdcbt(0)
    }
}
impl core::fmt::Debug for Fdcbt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fdcbt")
            .field("fpseg2", &self.fpseg2())
            .field("fpseg1", &self.fpseg1())
            .field("fpropseg", &self.fpropseg())
            .field("frjw", &self.frjw())
            .field("fpresdiv", &self.fpresdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fdcbt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fdcbt {{ fpseg2: {=u8:?}, fpseg1: {=u8:?}, fpropseg: {=u8:?}, frjw: {=u8:?}, fpresdiv: {=u16:?} }}",
            self.fpseg2(),
            self.fpseg1(),
            self.fpropseg(),
            self.frjw(),
            self.fpresdiv()
        )
    }
}
#[doc = "CAN FD CRC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdcrc(pub u32);
impl Fdcrc {
    #[doc = "Extended Transmitted CRC value."]
    #[must_use]
    #[inline(always)]
    pub const fn fd_txcrc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x001f_ffff;
        val as u32
    }
    #[doc = "Extended Transmitted CRC value."]
    #[inline(always)]
    pub const fn set_fd_txcrc(&mut self, val: u32) {
        self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
    }
    #[doc = "CRC Message Buffer Number for FD_TXCRC."]
    #[must_use]
    #[inline(always)]
    pub const fn fd_mbcrc(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "CRC Message Buffer Number for FD_TXCRC."]
    #[inline(always)]
    pub const fn set_fd_mbcrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
}
impl Default for Fdcrc {
    #[inline(always)]
    fn default() -> Fdcrc {
        Fdcrc(0)
    }
}
impl core::fmt::Debug for Fdcrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fdcrc")
            .field("fd_txcrc", &self.fd_txcrc())
            .field("fd_mbcrc", &self.fd_mbcrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fdcrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fdcrc {{ fd_txcrc: {=u32:?}, fd_mbcrc: {=u8:?} }}",
            self.fd_txcrc(),
            self.fd_mbcrc()
        )
    }
}
#[doc = "CAN FD Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdctrl(pub u32);
impl Fdctrl {
    #[doc = "Transceiver Delay Compensation Value."]
    #[must_use]
    #[inline(always)]
    pub const fn tdcval(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Transceiver Delay Compensation Value."]
    #[inline(always)]
    pub const fn set_tdcval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Transceiver Delay Compensation Offset."]
    #[must_use]
    #[inline(always)]
    pub const fn tdcoff(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Transceiver Delay Compensation Offset."]
    #[inline(always)]
    pub const fn set_tdcoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Transceiver Delay Compensation Fail."]
    #[must_use]
    #[inline(always)]
    pub const fn tdcfail(&self) -> Tdcfail {
        let val = (self.0 >> 14usize) & 0x01;
        Tdcfail::from_bits(val as u8)
    }
    #[doc = "Transceiver Delay Compensation Fail."]
    #[inline(always)]
    pub const fn set_tdcfail(&mut self, val: Tdcfail) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Transceiver Delay Compensation Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tdcen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Transceiver Delay Compensation Enable."]
    #[inline(always)]
    pub const fn set_tdcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Message Buffer Data Size for Region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn mbdsr0(&self) -> Mbdsr0 {
        let val = (self.0 >> 16usize) & 0x03;
        Mbdsr0::from_bits(val as u8)
    }
    #[doc = "Message Buffer Data Size for Region 0."]
    #[inline(always)]
    pub const fn set_mbdsr0(&mut self, val: Mbdsr0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Bit Rate Switch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fdrate(&self) -> Fdrate {
        let val = (self.0 >> 31usize) & 0x01;
        Fdrate::from_bits(val as u8)
    }
    #[doc = "Bit Rate Switch Enable."]
    #[inline(always)]
    pub const fn set_fdrate(&mut self, val: Fdrate) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Fdctrl {
    #[inline(always)]
    fn default() -> Fdctrl {
        Fdctrl(0)
    }
}
impl core::fmt::Debug for Fdctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fdctrl")
            .field("tdcval", &self.tdcval())
            .field("tdcoff", &self.tdcoff())
            .field("tdcfail", &self.tdcfail())
            .field("tdcen", &self.tdcen())
            .field("mbdsr0", &self.mbdsr0())
            .field("fdrate", &self.fdrate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fdctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fdctrl {{ tdcval: {=u8:?}, tdcoff: {=u8:?}, tdcfail: {:?}, tdcen: {=bool:?}, mbdsr0: {:?}, fdrate: {:?} }}",
            self.tdcval(),
            self.tdcoff(),
            self.tdcfail(),
            self.tdcen(),
            self.mbdsr0(),
            self.fdrate()
        )
    }
}
#[doc = "Pretended Networking Data Length Code (DLC) Filter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FltDlc(pub u32);
impl FltDlc {
    #[doc = "Upper Limit for Length of Data Bytes Filter."]
    #[must_use]
    #[inline(always)]
    pub const fn flt_dlc_hi(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Upper Limit for Length of Data Bytes Filter."]
    #[inline(always)]
    pub const fn set_flt_dlc_hi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Lower Limit for Length of Data Bytes Filter."]
    #[must_use]
    #[inline(always)]
    pub const fn flt_dlc_lo(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Lower Limit for Length of Data Bytes Filter."]
    #[inline(always)]
    pub const fn set_flt_dlc_lo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for FltDlc {
    #[inline(always)]
    fn default() -> FltDlc {
        FltDlc(0)
    }
}
impl core::fmt::Debug for FltDlc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FltDlc")
            .field("flt_dlc_hi", &self.flt_dlc_hi())
            .field("flt_dlc_lo", &self.flt_dlc_lo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FltDlc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FltDlc {{ flt_dlc_hi: {=u8:?}, flt_dlc_lo: {=u8:?} }}",
            self.flt_dlc_hi(),
            self.flt_dlc_lo()
        )
    }
}
#[doc = "Pretended Networking ID Filter 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FltId1(pub u32);
impl FltId1 {
    #[doc = "ID Filter 1 for Pretended Networking filtering."]
    #[must_use]
    #[inline(always)]
    pub const fn flt_id1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "ID Filter 1 for Pretended Networking filtering."]
    #[inline(always)]
    pub const fn set_flt_id1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
    #[doc = "Remote Transmission Request Filter."]
    #[must_use]
    #[inline(always)]
    pub const fn flt_rtr(&self) -> FltRtr {
        let val = (self.0 >> 29usize) & 0x01;
        FltRtr::from_bits(val as u8)
    }
    #[doc = "Remote Transmission Request Filter."]
    #[inline(always)]
    pub const fn set_flt_rtr(&mut self, val: FltRtr) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "ID Extended Filter."]
    #[must_use]
    #[inline(always)]
    pub const fn flt_ide(&self) -> FltIde {
        let val = (self.0 >> 30usize) & 0x01;
        FltIde::from_bits(val as u8)
    }
    #[doc = "ID Extended Filter."]
    #[inline(always)]
    pub const fn set_flt_ide(&mut self, val: FltIde) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for FltId1 {
    #[inline(always)]
    fn default() -> FltId1 {
        FltId1(0)
    }
}
impl core::fmt::Debug for FltId1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FltId1")
            .field("flt_id1", &self.flt_id1())
            .field("flt_rtr", &self.flt_rtr())
            .field("flt_ide", &self.flt_ide())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FltId1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FltId1 {{ flt_id1: {=u32:?}, flt_rtr: {:?}, flt_ide: {:?} }}",
            self.flt_id1(),
            self.flt_rtr(),
            self.flt_ide()
        )
    }
}
#[doc = "Pretended Networking ID Filter 2 or ID Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FltId2Idmask(pub u32);
impl FltId2Idmask {
    #[doc = "ID Filter 2 for Pretended Networking Filtering or ID Mask Bits for Pretended Networking ID Filtering."]
    #[must_use]
    #[inline(always)]
    pub const fn flt_id2_idmask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "ID Filter 2 for Pretended Networking Filtering or ID Mask Bits for Pretended Networking ID Filtering."]
    #[inline(always)]
    pub const fn set_flt_id2_idmask(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
    #[doc = "Remote Transmission Request Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn rtr_msk(&self) -> RtrMsk {
        let val = (self.0 >> 29usize) & 0x01;
        RtrMsk::from_bits(val as u8)
    }
    #[doc = "Remote Transmission Request Mask."]
    #[inline(always)]
    pub const fn set_rtr_msk(&mut self, val: RtrMsk) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "ID Extended Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn ide_msk(&self) -> IdeMsk {
        let val = (self.0 >> 30usize) & 0x01;
        IdeMsk::from_bits(val as u8)
    }
    #[doc = "ID Extended Mask."]
    #[inline(always)]
    pub const fn set_ide_msk(&mut self, val: IdeMsk) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for FltId2Idmask {
    #[inline(always)]
    fn default() -> FltId2Idmask {
        FltId2Idmask(0)
    }
}
impl core::fmt::Debug for FltId2Idmask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FltId2Idmask")
            .field("flt_id2_idmask", &self.flt_id2_idmask())
            .field("rtr_msk", &self.rtr_msk())
            .field("ide_msk", &self.ide_msk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FltId2Idmask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FltId2Idmask {{ flt_id2_idmask: {=u32:?}, rtr_msk: {:?}, ide_msk: {:?} }}",
            self.flt_id2_idmask(),
            self.rtr_msk(),
            self.ide_msk()
        )
    }
}
#[doc = "Message Buffer ID Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id(pub u32);
impl Id {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn ext(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    #[inline(always)]
    pub const fn set_ext(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn std(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x07ff;
        val as u16
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    #[inline(always)]
    pub const fn set_std(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    #[must_use]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    #[inline(always)]
    pub const fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for Id {
    #[inline(always)]
    fn default() -> Id {
        Id(0)
    }
}
impl core::fmt::Debug for Id {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Id")
            .field("ext", &self.ext())
            .field("std", &self.std())
            .field("prio", &self.prio())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Id {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Id {{ ext: {=u32:?}, std: {=u16:?}, prio: {=u8:?} }}",
            self.ext(),
            self.std(),
            self.prio()
        )
    }
}
#[doc = "Interrupt Flags 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iflag1(pub u32);
impl Iflag1 {
    #[doc = "Buffer MB0 Interrupt or Clear Legacy FIFO bit."]
    #[must_use]
    #[inline(always)]
    pub const fn buf0i(&self) -> Buf0i {
        let val = (self.0 >> 0usize) & 0x01;
        Buf0i::from_bits(val as u8)
    }
    #[doc = "Buffer MB0 Interrupt or Clear Legacy FIFO bit."]
    #[inline(always)]
    pub const fn set_buf0i(&mut self, val: Buf0i) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Buffer MBi Interrupt or Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn buf4to1i(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "Buffer MBi Interrupt or Reserved."]
    #[inline(always)]
    pub const fn set_buf4to1i(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "Buffer MB5 Interrupt or Frames available in Legacy RX FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn buf5i(&self) -> Buf5i {
        let val = (self.0 >> 5usize) & 0x01;
        Buf5i::from_bits(val as u8)
    }
    #[doc = "Buffer MB5 Interrupt or Frames available in Legacy RX FIFO."]
    #[inline(always)]
    pub const fn set_buf5i(&mut self, val: Buf5i) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Buffer MB6 Interrupt or Legacy RX FIFO Warning."]
    #[must_use]
    #[inline(always)]
    pub const fn buf6i(&self) -> Buf6i {
        let val = (self.0 >> 6usize) & 0x01;
        Buf6i::from_bits(val as u8)
    }
    #[doc = "Buffer MB6 Interrupt or Legacy RX FIFO Warning."]
    #[inline(always)]
    pub const fn set_buf6i(&mut self, val: Buf6i) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Buffer MB7 Interrupt or Legacy RX FIFO Overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn buf7i(&self) -> Buf7i {
        let val = (self.0 >> 7usize) & 0x01;
        Buf7i::from_bits(val as u8)
    }
    #[doc = "Buffer MB7 Interrupt or Legacy RX FIFO Overflow."]
    #[inline(always)]
    pub const fn set_buf7i(&mut self, val: Buf7i) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Buffer MBi Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn buf31to8i(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Buffer MBi Interrupt."]
    #[inline(always)]
    pub const fn set_buf31to8i(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Iflag1 {
    #[inline(always)]
    fn default() -> Iflag1 {
        Iflag1(0)
    }
}
impl core::fmt::Debug for Iflag1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iflag1")
            .field("buf0i", &self.buf0i())
            .field("buf4to1i", &self.buf4to1i())
            .field("buf5i", &self.buf5i())
            .field("buf6i", &self.buf6i())
            .field("buf7i", &self.buf7i())
            .field("buf31to8i", &self.buf31to8i())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iflag1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Iflag1 {{ buf0i: {:?}, buf4to1i: {=u8:?}, buf5i: {:?}, buf6i: {:?}, buf7i: {:?}, buf31to8i: {=u32:?} }}",
            self.buf0i(),
            self.buf4to1i(),
            self.buf5i(),
            self.buf6i(),
            self.buf7i(),
            self.buf31to8i()
        )
    }
}
#[doc = "Interrupt Masks 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imask1(pub u32);
impl Imask1 {
    #[doc = "Buffer MBi Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn buf31to0m(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Buffer MBi Mask."]
    #[inline(always)]
    pub const fn set_buf31to0m(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Imask1 {
    #[inline(always)]
    fn default() -> Imask1 {
        Imask1(0)
    }
}
impl core::fmt::Debug for Imask1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imask1")
            .field("buf31to0m", &self.buf31to0m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imask1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Imask1 {{ buf31to0m: {=u32:?} }}", self.buf31to0m())
    }
}
#[doc = "Message Buffer CS Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb16bCs(pub u32);
impl Mb16bCs {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    #[must_use]
    #[inline(always)]
    pub const fn time_stamp(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    #[inline(always)]
    pub const fn set_time_stamp(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Length of the data to be stored/transmitted."]
    #[must_use]
    #[inline(always)]
    pub const fn dlc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Length of the data to be stored/transmitted."]
    #[inline(always)]
    pub const fn set_dlc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    #[must_use]
    #[inline(always)]
    pub const fn rtr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    #[inline(always)]
    pub const fn set_rtr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    #[must_use]
    #[inline(always)]
    pub const fn ide(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    #[inline(always)]
    pub const fn set_ide(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    #[must_use]
    #[inline(always)]
    pub const fn srr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    #[inline(always)]
    pub const fn set_srr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    #[must_use]
    #[inline(always)]
    pub const fn code(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    #[inline(always)]
    pub const fn set_code(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    #[must_use]
    #[inline(always)]
    pub const fn esi(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    #[inline(always)]
    pub const fn set_esi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    #[must_use]
    #[inline(always)]
    pub const fn brs(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    #[inline(always)]
    pub const fn set_brs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    #[must_use]
    #[inline(always)]
    pub const fn edl(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    #[inline(always)]
    pub const fn set_edl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mb16bCs {
    #[inline(always)]
    fn default() -> Mb16bCs {
        Mb16bCs(0)
    }
}
impl core::fmt::Debug for Mb16bCs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb16bCs")
            .field("time_stamp", &self.time_stamp())
            .field("dlc", &self.dlc())
            .field("rtr", &self.rtr())
            .field("ide", &self.ide())
            .field("srr", &self.srr())
            .field("code", &self.code())
            .field("esi", &self.esi())
            .field("brs", &self.brs())
            .field("edl", &self.edl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb16bCs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb16bCs {{ time_stamp: {=u16:?}, dlc: {=u8:?}, rtr: {=bool:?}, ide: {=bool:?}, srr: {=bool:?}, code: {=u8:?}, esi: {=bool:?}, brs: {=bool:?}, edl: {=bool:?} }}",
            self.time_stamp(),
            self.dlc(),
            self.rtr(),
            self.ide(),
            self.srr(),
            self.code(),
            self.esi(),
            self.brs(),
            self.edl()
        )
    }
}
#[doc = "Message Buffer ID Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb16bId(pub u32);
impl Mb16bId {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn ext(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    #[inline(always)]
    pub const fn set_ext(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn std(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x07ff;
        val as u16
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    #[inline(always)]
    pub const fn set_std(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    #[must_use]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    #[inline(always)]
    pub const fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for Mb16bId {
    #[inline(always)]
    fn default() -> Mb16bId {
        Mb16bId(0)
    }
}
impl core::fmt::Debug for Mb16bId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb16bId")
            .field("ext", &self.ext())
            .field("std", &self.std())
            .field("prio", &self.prio())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb16bId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb16bId {{ ext: {=u32:?}, std: {=u16:?}, prio: {=u8:?} }}",
            self.ext(),
            self.std(),
            self.prio()
        )
    }
}
#[doc = "Message Buffer WORD_16B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb16bWord0(pub u32);
impl Mb16bWord0 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_0(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb16bWord0 {
    #[inline(always)]
    fn default() -> Mb16bWord0 {
        Mb16bWord0(0)
    }
}
impl core::fmt::Debug for Mb16bWord0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb16bWord0")
            .field("data_byte_3", &self.data_byte_3())
            .field("data_byte_2", &self.data_byte_2())
            .field("data_byte_1", &self.data_byte_1())
            .field("data_byte_0", &self.data_byte_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb16bWord0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb16bWord0 {{ data_byte_3: {=u8:?}, data_byte_2: {=u8:?}, data_byte_1: {=u8:?}, data_byte_0: {=u8:?} }}",
            self.data_byte_3(),
            self.data_byte_2(),
            self.data_byte_1(),
            self.data_byte_0()
        )
    }
}
#[doc = "Message Buffer WORD_16B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb16bWord1(pub u32);
impl Mb16bWord1 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_7(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_6(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_5(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb16bWord1 {
    #[inline(always)]
    fn default() -> Mb16bWord1 {
        Mb16bWord1(0)
    }
}
impl core::fmt::Debug for Mb16bWord1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb16bWord1")
            .field("data_byte_7", &self.data_byte_7())
            .field("data_byte_6", &self.data_byte_6())
            .field("data_byte_5", &self.data_byte_5())
            .field("data_byte_4", &self.data_byte_4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb16bWord1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb16bWord1 {{ data_byte_7: {=u8:?}, data_byte_6: {=u8:?}, data_byte_5: {=u8:?}, data_byte_4: {=u8:?} }}",
            self.data_byte_7(),
            self.data_byte_6(),
            self.data_byte_5(),
            self.data_byte_4()
        )
    }
}
#[doc = "Message Buffer WORD_16B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb16bWord2(pub u32);
impl Mb16bWord2 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_11(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_11(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_10(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_10(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_9(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_9(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_8(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_8(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb16bWord2 {
    #[inline(always)]
    fn default() -> Mb16bWord2 {
        Mb16bWord2(0)
    }
}
impl core::fmt::Debug for Mb16bWord2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb16bWord2")
            .field("data_byte_11", &self.data_byte_11())
            .field("data_byte_10", &self.data_byte_10())
            .field("data_byte_9", &self.data_byte_9())
            .field("data_byte_8", &self.data_byte_8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb16bWord2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb16bWord2 {{ data_byte_11: {=u8:?}, data_byte_10: {=u8:?}, data_byte_9: {=u8:?}, data_byte_8: {=u8:?} }}",
            self.data_byte_11(),
            self.data_byte_10(),
            self.data_byte_9(),
            self.data_byte_8()
        )
    }
}
#[doc = "Message Buffer WORD_16B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb16bWord3(pub u32);
impl Mb16bWord3 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_15(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_15(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_14(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_14(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_13(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_13(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_12(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb16bWord3 {
    #[inline(always)]
    fn default() -> Mb16bWord3 {
        Mb16bWord3(0)
    }
}
impl core::fmt::Debug for Mb16bWord3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb16bWord3")
            .field("data_byte_15", &self.data_byte_15())
            .field("data_byte_14", &self.data_byte_14())
            .field("data_byte_13", &self.data_byte_13())
            .field("data_byte_12", &self.data_byte_12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb16bWord3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb16bWord3 {{ data_byte_15: {=u8:?}, data_byte_14: {=u8:?}, data_byte_13: {=u8:?}, data_byte_12: {=u8:?} }}",
            self.data_byte_15(),
            self.data_byte_14(),
            self.data_byte_13(),
            self.data_byte_12()
        )
    }
}
#[doc = "Message Buffer CS Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb32bCs(pub u32);
impl Mb32bCs {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    #[must_use]
    #[inline(always)]
    pub const fn time_stamp(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    #[inline(always)]
    pub const fn set_time_stamp(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Length of the data to be stored/transmitted."]
    #[must_use]
    #[inline(always)]
    pub const fn dlc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Length of the data to be stored/transmitted."]
    #[inline(always)]
    pub const fn set_dlc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    #[must_use]
    #[inline(always)]
    pub const fn rtr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    #[inline(always)]
    pub const fn set_rtr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    #[must_use]
    #[inline(always)]
    pub const fn ide(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    #[inline(always)]
    pub const fn set_ide(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    #[must_use]
    #[inline(always)]
    pub const fn srr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    #[inline(always)]
    pub const fn set_srr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    #[must_use]
    #[inline(always)]
    pub const fn code(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    #[inline(always)]
    pub const fn set_code(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    #[must_use]
    #[inline(always)]
    pub const fn esi(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    #[inline(always)]
    pub const fn set_esi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    #[must_use]
    #[inline(always)]
    pub const fn brs(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    #[inline(always)]
    pub const fn set_brs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    #[must_use]
    #[inline(always)]
    pub const fn edl(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    #[inline(always)]
    pub const fn set_edl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mb32bCs {
    #[inline(always)]
    fn default() -> Mb32bCs {
        Mb32bCs(0)
    }
}
impl core::fmt::Debug for Mb32bCs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb32bCs")
            .field("time_stamp", &self.time_stamp())
            .field("dlc", &self.dlc())
            .field("rtr", &self.rtr())
            .field("ide", &self.ide())
            .field("srr", &self.srr())
            .field("code", &self.code())
            .field("esi", &self.esi())
            .field("brs", &self.brs())
            .field("edl", &self.edl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb32bCs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb32bCs {{ time_stamp: {=u16:?}, dlc: {=u8:?}, rtr: {=bool:?}, ide: {=bool:?}, srr: {=bool:?}, code: {=u8:?}, esi: {=bool:?}, brs: {=bool:?}, edl: {=bool:?} }}",
            self.time_stamp(),
            self.dlc(),
            self.rtr(),
            self.ide(),
            self.srr(),
            self.code(),
            self.esi(),
            self.brs(),
            self.edl()
        )
    }
}
#[doc = "Message Buffer ID Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb32bId(pub u32);
impl Mb32bId {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn ext(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    #[inline(always)]
    pub const fn set_ext(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn std(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x07ff;
        val as u16
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    #[inline(always)]
    pub const fn set_std(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    #[must_use]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    #[inline(always)]
    pub const fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for Mb32bId {
    #[inline(always)]
    fn default() -> Mb32bId {
        Mb32bId(0)
    }
}
impl core::fmt::Debug for Mb32bId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb32bId")
            .field("ext", &self.ext())
            .field("std", &self.std())
            .field("prio", &self.prio())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb32bId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb32bId {{ ext: {=u32:?}, std: {=u16:?}, prio: {=u8:?} }}",
            self.ext(),
            self.std(),
            self.prio()
        )
    }
}
#[doc = "Message Buffer WORD_32B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb32bWord0(pub u32);
impl Mb32bWord0 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_0(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb32bWord0 {
    #[inline(always)]
    fn default() -> Mb32bWord0 {
        Mb32bWord0(0)
    }
}
impl core::fmt::Debug for Mb32bWord0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb32bWord0")
            .field("data_byte_3", &self.data_byte_3())
            .field("data_byte_2", &self.data_byte_2())
            .field("data_byte_1", &self.data_byte_1())
            .field("data_byte_0", &self.data_byte_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb32bWord0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb32bWord0 {{ data_byte_3: {=u8:?}, data_byte_2: {=u8:?}, data_byte_1: {=u8:?}, data_byte_0: {=u8:?} }}",
            self.data_byte_3(),
            self.data_byte_2(),
            self.data_byte_1(),
            self.data_byte_0()
        )
    }
}
#[doc = "Message Buffer WORD_32B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb32bWord1(pub u32);
impl Mb32bWord1 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_7(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_6(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_5(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb32bWord1 {
    #[inline(always)]
    fn default() -> Mb32bWord1 {
        Mb32bWord1(0)
    }
}
impl core::fmt::Debug for Mb32bWord1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb32bWord1")
            .field("data_byte_7", &self.data_byte_7())
            .field("data_byte_6", &self.data_byte_6())
            .field("data_byte_5", &self.data_byte_5())
            .field("data_byte_4", &self.data_byte_4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb32bWord1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb32bWord1 {{ data_byte_7: {=u8:?}, data_byte_6: {=u8:?}, data_byte_5: {=u8:?}, data_byte_4: {=u8:?} }}",
            self.data_byte_7(),
            self.data_byte_6(),
            self.data_byte_5(),
            self.data_byte_4()
        )
    }
}
#[doc = "Message Buffer WORD_32B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb32bWord2(pub u32);
impl Mb32bWord2 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_11(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_11(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_10(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_10(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_9(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_9(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_8(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_8(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb32bWord2 {
    #[inline(always)]
    fn default() -> Mb32bWord2 {
        Mb32bWord2(0)
    }
}
impl core::fmt::Debug for Mb32bWord2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb32bWord2")
            .field("data_byte_11", &self.data_byte_11())
            .field("data_byte_10", &self.data_byte_10())
            .field("data_byte_9", &self.data_byte_9())
            .field("data_byte_8", &self.data_byte_8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb32bWord2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb32bWord2 {{ data_byte_11: {=u8:?}, data_byte_10: {=u8:?}, data_byte_9: {=u8:?}, data_byte_8: {=u8:?} }}",
            self.data_byte_11(),
            self.data_byte_10(),
            self.data_byte_9(),
            self.data_byte_8()
        )
    }
}
#[doc = "Message Buffer WORD_32B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb32bWord3(pub u32);
impl Mb32bWord3 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_15(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_15(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_14(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_14(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_13(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_13(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_12(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb32bWord3 {
    #[inline(always)]
    fn default() -> Mb32bWord3 {
        Mb32bWord3(0)
    }
}
impl core::fmt::Debug for Mb32bWord3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb32bWord3")
            .field("data_byte_15", &self.data_byte_15())
            .field("data_byte_14", &self.data_byte_14())
            .field("data_byte_13", &self.data_byte_13())
            .field("data_byte_12", &self.data_byte_12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb32bWord3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb32bWord3 {{ data_byte_15: {=u8:?}, data_byte_14: {=u8:?}, data_byte_13: {=u8:?}, data_byte_12: {=u8:?} }}",
            self.data_byte_15(),
            self.data_byte_14(),
            self.data_byte_13(),
            self.data_byte_12()
        )
    }
}
#[doc = "Message Buffer WORD_32B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb32bWord4(pub u32);
impl Mb32bWord4 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_19(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_19(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_18(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_18(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_17(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_17(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_16(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_16(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb32bWord4 {
    #[inline(always)]
    fn default() -> Mb32bWord4 {
        Mb32bWord4(0)
    }
}
impl core::fmt::Debug for Mb32bWord4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb32bWord4")
            .field("data_byte_19", &self.data_byte_19())
            .field("data_byte_18", &self.data_byte_18())
            .field("data_byte_17", &self.data_byte_17())
            .field("data_byte_16", &self.data_byte_16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb32bWord4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb32bWord4 {{ data_byte_19: {=u8:?}, data_byte_18: {=u8:?}, data_byte_17: {=u8:?}, data_byte_16: {=u8:?} }}",
            self.data_byte_19(),
            self.data_byte_18(),
            self.data_byte_17(),
            self.data_byte_16()
        )
    }
}
#[doc = "Message Buffer WORD_32B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb32bWord5(pub u32);
impl Mb32bWord5 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_23(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_23(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_22(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_22(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_21(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_21(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_20(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_20(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb32bWord5 {
    #[inline(always)]
    fn default() -> Mb32bWord5 {
        Mb32bWord5(0)
    }
}
impl core::fmt::Debug for Mb32bWord5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb32bWord5")
            .field("data_byte_23", &self.data_byte_23())
            .field("data_byte_22", &self.data_byte_22())
            .field("data_byte_21", &self.data_byte_21())
            .field("data_byte_20", &self.data_byte_20())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb32bWord5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb32bWord5 {{ data_byte_23: {=u8:?}, data_byte_22: {=u8:?}, data_byte_21: {=u8:?}, data_byte_20: {=u8:?} }}",
            self.data_byte_23(),
            self.data_byte_22(),
            self.data_byte_21(),
            self.data_byte_20()
        )
    }
}
#[doc = "Message Buffer WORD_32B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb32bWord6(pub u32);
impl Mb32bWord6 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_27(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_27(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_26(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_26(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_25(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_25(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_24(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_24(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb32bWord6 {
    #[inline(always)]
    fn default() -> Mb32bWord6 {
        Mb32bWord6(0)
    }
}
impl core::fmt::Debug for Mb32bWord6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb32bWord6")
            .field("data_byte_27", &self.data_byte_27())
            .field("data_byte_26", &self.data_byte_26())
            .field("data_byte_25", &self.data_byte_25())
            .field("data_byte_24", &self.data_byte_24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb32bWord6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb32bWord6 {{ data_byte_27: {=u8:?}, data_byte_26: {=u8:?}, data_byte_25: {=u8:?}, data_byte_24: {=u8:?} }}",
            self.data_byte_27(),
            self.data_byte_26(),
            self.data_byte_25(),
            self.data_byte_24()
        )
    }
}
#[doc = "Message Buffer WORD_32B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb32bWord7(pub u32);
impl Mb32bWord7 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_31(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_31(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_30(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_30(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_29(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_29(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_28(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_28(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb32bWord7 {
    #[inline(always)]
    fn default() -> Mb32bWord7 {
        Mb32bWord7(0)
    }
}
impl core::fmt::Debug for Mb32bWord7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb32bWord7")
            .field("data_byte_31", &self.data_byte_31())
            .field("data_byte_30", &self.data_byte_30())
            .field("data_byte_29", &self.data_byte_29())
            .field("data_byte_28", &self.data_byte_28())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb32bWord7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb32bWord7 {{ data_byte_31: {=u8:?}, data_byte_30: {=u8:?}, data_byte_29: {=u8:?}, data_byte_28: {=u8:?} }}",
            self.data_byte_31(),
            self.data_byte_30(),
            self.data_byte_29(),
            self.data_byte_28()
        )
    }
}
#[doc = "Message Buffer CS Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb64bCs(pub u32);
impl Mb64bCs {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    #[must_use]
    #[inline(always)]
    pub const fn time_stamp(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    #[inline(always)]
    pub const fn set_time_stamp(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Length of the data to be stored/transmitted."]
    #[must_use]
    #[inline(always)]
    pub const fn dlc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Length of the data to be stored/transmitted."]
    #[inline(always)]
    pub const fn set_dlc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    #[must_use]
    #[inline(always)]
    pub const fn rtr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    #[inline(always)]
    pub const fn set_rtr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    #[must_use]
    #[inline(always)]
    pub const fn ide(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    #[inline(always)]
    pub const fn set_ide(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    #[must_use]
    #[inline(always)]
    pub const fn srr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    #[inline(always)]
    pub const fn set_srr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    #[must_use]
    #[inline(always)]
    pub const fn code(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    #[inline(always)]
    pub const fn set_code(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    #[must_use]
    #[inline(always)]
    pub const fn esi(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    #[inline(always)]
    pub const fn set_esi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    #[must_use]
    #[inline(always)]
    pub const fn brs(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    #[inline(always)]
    pub const fn set_brs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    #[must_use]
    #[inline(always)]
    pub const fn edl(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    #[inline(always)]
    pub const fn set_edl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mb64bCs {
    #[inline(always)]
    fn default() -> Mb64bCs {
        Mb64bCs(0)
    }
}
impl core::fmt::Debug for Mb64bCs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb64bCs")
            .field("time_stamp", &self.time_stamp())
            .field("dlc", &self.dlc())
            .field("rtr", &self.rtr())
            .field("ide", &self.ide())
            .field("srr", &self.srr())
            .field("code", &self.code())
            .field("esi", &self.esi())
            .field("brs", &self.brs())
            .field("edl", &self.edl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb64bCs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb64bCs {{ time_stamp: {=u16:?}, dlc: {=u8:?}, rtr: {=bool:?}, ide: {=bool:?}, srr: {=bool:?}, code: {=u8:?}, esi: {=bool:?}, brs: {=bool:?}, edl: {=bool:?} }}",
            self.time_stamp(),
            self.dlc(),
            self.rtr(),
            self.ide(),
            self.srr(),
            self.code(),
            self.esi(),
            self.brs(),
            self.edl()
        )
    }
}
#[doc = "Message Buffer ID Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb64bId(pub u32);
impl Mb64bId {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn ext(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    #[inline(always)]
    pub const fn set_ext(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn std(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x07ff;
        val as u16
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    #[inline(always)]
    pub const fn set_std(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    #[must_use]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    #[inline(always)]
    pub const fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for Mb64bId {
    #[inline(always)]
    fn default() -> Mb64bId {
        Mb64bId(0)
    }
}
impl core::fmt::Debug for Mb64bId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb64bId")
            .field("ext", &self.ext())
            .field("std", &self.std())
            .field("prio", &self.prio())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb64bId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb64bId {{ ext: {=u32:?}, std: {=u16:?}, prio: {=u8:?} }}",
            self.ext(),
            self.std(),
            self.prio()
        )
    }
}
#[doc = "Message Buffer WORD_64B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb64bWord0(pub u32);
impl Mb64bWord0 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_0(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb64bWord0 {
    #[inline(always)]
    fn default() -> Mb64bWord0 {
        Mb64bWord0(0)
    }
}
impl core::fmt::Debug for Mb64bWord0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb64bWord0")
            .field("data_byte_3", &self.data_byte_3())
            .field("data_byte_2", &self.data_byte_2())
            .field("data_byte_1", &self.data_byte_1())
            .field("data_byte_0", &self.data_byte_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb64bWord0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb64bWord0 {{ data_byte_3: {=u8:?}, data_byte_2: {=u8:?}, data_byte_1: {=u8:?}, data_byte_0: {=u8:?} }}",
            self.data_byte_3(),
            self.data_byte_2(),
            self.data_byte_1(),
            self.data_byte_0()
        )
    }
}
#[doc = "Message Buffer WORD_64B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb64bWord1(pub u32);
impl Mb64bWord1 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_7(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_6(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_5(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb64bWord1 {
    #[inline(always)]
    fn default() -> Mb64bWord1 {
        Mb64bWord1(0)
    }
}
impl core::fmt::Debug for Mb64bWord1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb64bWord1")
            .field("data_byte_7", &self.data_byte_7())
            .field("data_byte_6", &self.data_byte_6())
            .field("data_byte_5", &self.data_byte_5())
            .field("data_byte_4", &self.data_byte_4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb64bWord1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb64bWord1 {{ data_byte_7: {=u8:?}, data_byte_6: {=u8:?}, data_byte_5: {=u8:?}, data_byte_4: {=u8:?} }}",
            self.data_byte_7(),
            self.data_byte_6(),
            self.data_byte_5(),
            self.data_byte_4()
        )
    }
}
#[doc = "Message Buffer WORD_64B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb64bWord10(pub u32);
impl Mb64bWord10 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_43(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_43(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_42(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_42(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_41(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_41(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_40(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_40(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb64bWord10 {
    #[inline(always)]
    fn default() -> Mb64bWord10 {
        Mb64bWord10(0)
    }
}
impl core::fmt::Debug for Mb64bWord10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb64bWord10")
            .field("data_byte_43", &self.data_byte_43())
            .field("data_byte_42", &self.data_byte_42())
            .field("data_byte_41", &self.data_byte_41())
            .field("data_byte_40", &self.data_byte_40())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb64bWord10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb64bWord10 {{ data_byte_43: {=u8:?}, data_byte_42: {=u8:?}, data_byte_41: {=u8:?}, data_byte_40: {=u8:?} }}",
            self.data_byte_43(),
            self.data_byte_42(),
            self.data_byte_41(),
            self.data_byte_40()
        )
    }
}
#[doc = "Message Buffer WORD_64B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb64bWord11(pub u32);
impl Mb64bWord11 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_47(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_47(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_46(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_46(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_45(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_45(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_44(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_44(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb64bWord11 {
    #[inline(always)]
    fn default() -> Mb64bWord11 {
        Mb64bWord11(0)
    }
}
impl core::fmt::Debug for Mb64bWord11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb64bWord11")
            .field("data_byte_47", &self.data_byte_47())
            .field("data_byte_46", &self.data_byte_46())
            .field("data_byte_45", &self.data_byte_45())
            .field("data_byte_44", &self.data_byte_44())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb64bWord11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb64bWord11 {{ data_byte_47: {=u8:?}, data_byte_46: {=u8:?}, data_byte_45: {=u8:?}, data_byte_44: {=u8:?} }}",
            self.data_byte_47(),
            self.data_byte_46(),
            self.data_byte_45(),
            self.data_byte_44()
        )
    }
}
#[doc = "Message Buffer WORD_64B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb64bWord12(pub u32);
impl Mb64bWord12 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_51(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_51(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_50(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_50(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_49(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_49(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_48(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_48(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb64bWord12 {
    #[inline(always)]
    fn default() -> Mb64bWord12 {
        Mb64bWord12(0)
    }
}
impl core::fmt::Debug for Mb64bWord12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb64bWord12")
            .field("data_byte_51", &self.data_byte_51())
            .field("data_byte_50", &self.data_byte_50())
            .field("data_byte_49", &self.data_byte_49())
            .field("data_byte_48", &self.data_byte_48())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb64bWord12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb64bWord12 {{ data_byte_51: {=u8:?}, data_byte_50: {=u8:?}, data_byte_49: {=u8:?}, data_byte_48: {=u8:?} }}",
            self.data_byte_51(),
            self.data_byte_50(),
            self.data_byte_49(),
            self.data_byte_48()
        )
    }
}
#[doc = "Message Buffer WORD_64B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb64bWord13(pub u32);
impl Mb64bWord13 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_55(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_55(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_54(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_54(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_53(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_53(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_52(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_52(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb64bWord13 {
    #[inline(always)]
    fn default() -> Mb64bWord13 {
        Mb64bWord13(0)
    }
}
impl core::fmt::Debug for Mb64bWord13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb64bWord13")
            .field("data_byte_55", &self.data_byte_55())
            .field("data_byte_54", &self.data_byte_54())
            .field("data_byte_53", &self.data_byte_53())
            .field("data_byte_52", &self.data_byte_52())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb64bWord13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb64bWord13 {{ data_byte_55: {=u8:?}, data_byte_54: {=u8:?}, data_byte_53: {=u8:?}, data_byte_52: {=u8:?} }}",
            self.data_byte_55(),
            self.data_byte_54(),
            self.data_byte_53(),
            self.data_byte_52()
        )
    }
}
#[doc = "Message Buffer WORD_64B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb64bWord14(pub u32);
impl Mb64bWord14 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_59(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_59(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_58(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_58(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_57(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_57(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_56(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_56(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb64bWord14 {
    #[inline(always)]
    fn default() -> Mb64bWord14 {
        Mb64bWord14(0)
    }
}
impl core::fmt::Debug for Mb64bWord14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb64bWord14")
            .field("data_byte_59", &self.data_byte_59())
            .field("data_byte_58", &self.data_byte_58())
            .field("data_byte_57", &self.data_byte_57())
            .field("data_byte_56", &self.data_byte_56())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb64bWord14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb64bWord14 {{ data_byte_59: {=u8:?}, data_byte_58: {=u8:?}, data_byte_57: {=u8:?}, data_byte_56: {=u8:?} }}",
            self.data_byte_59(),
            self.data_byte_58(),
            self.data_byte_57(),
            self.data_byte_56()
        )
    }
}
#[doc = "Message Buffer WORD_64B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb64bWord15(pub u32);
impl Mb64bWord15 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_63(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_63(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_62(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_62(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_61(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_61(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_60(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_60(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb64bWord15 {
    #[inline(always)]
    fn default() -> Mb64bWord15 {
        Mb64bWord15(0)
    }
}
impl core::fmt::Debug for Mb64bWord15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb64bWord15")
            .field("data_byte_63", &self.data_byte_63())
            .field("data_byte_62", &self.data_byte_62())
            .field("data_byte_61", &self.data_byte_61())
            .field("data_byte_60", &self.data_byte_60())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb64bWord15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb64bWord15 {{ data_byte_63: {=u8:?}, data_byte_62: {=u8:?}, data_byte_61: {=u8:?}, data_byte_60: {=u8:?} }}",
            self.data_byte_63(),
            self.data_byte_62(),
            self.data_byte_61(),
            self.data_byte_60()
        )
    }
}
#[doc = "Message Buffer WORD_64B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb64bWord2(pub u32);
impl Mb64bWord2 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_11(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_11(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_10(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_10(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_9(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_9(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_8(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_8(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb64bWord2 {
    #[inline(always)]
    fn default() -> Mb64bWord2 {
        Mb64bWord2(0)
    }
}
impl core::fmt::Debug for Mb64bWord2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb64bWord2")
            .field("data_byte_11", &self.data_byte_11())
            .field("data_byte_10", &self.data_byte_10())
            .field("data_byte_9", &self.data_byte_9())
            .field("data_byte_8", &self.data_byte_8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb64bWord2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb64bWord2 {{ data_byte_11: {=u8:?}, data_byte_10: {=u8:?}, data_byte_9: {=u8:?}, data_byte_8: {=u8:?} }}",
            self.data_byte_11(),
            self.data_byte_10(),
            self.data_byte_9(),
            self.data_byte_8()
        )
    }
}
#[doc = "Message Buffer WORD_64B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb64bWord3(pub u32);
impl Mb64bWord3 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_15(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_15(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_14(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_14(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_13(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_13(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_12(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb64bWord3 {
    #[inline(always)]
    fn default() -> Mb64bWord3 {
        Mb64bWord3(0)
    }
}
impl core::fmt::Debug for Mb64bWord3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb64bWord3")
            .field("data_byte_15", &self.data_byte_15())
            .field("data_byte_14", &self.data_byte_14())
            .field("data_byte_13", &self.data_byte_13())
            .field("data_byte_12", &self.data_byte_12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb64bWord3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb64bWord3 {{ data_byte_15: {=u8:?}, data_byte_14: {=u8:?}, data_byte_13: {=u8:?}, data_byte_12: {=u8:?} }}",
            self.data_byte_15(),
            self.data_byte_14(),
            self.data_byte_13(),
            self.data_byte_12()
        )
    }
}
#[doc = "Message Buffer WORD_64B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb64bWord4(pub u32);
impl Mb64bWord4 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_19(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_19(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_18(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_18(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_17(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_17(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_16(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_16(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb64bWord4 {
    #[inline(always)]
    fn default() -> Mb64bWord4 {
        Mb64bWord4(0)
    }
}
impl core::fmt::Debug for Mb64bWord4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb64bWord4")
            .field("data_byte_19", &self.data_byte_19())
            .field("data_byte_18", &self.data_byte_18())
            .field("data_byte_17", &self.data_byte_17())
            .field("data_byte_16", &self.data_byte_16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb64bWord4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb64bWord4 {{ data_byte_19: {=u8:?}, data_byte_18: {=u8:?}, data_byte_17: {=u8:?}, data_byte_16: {=u8:?} }}",
            self.data_byte_19(),
            self.data_byte_18(),
            self.data_byte_17(),
            self.data_byte_16()
        )
    }
}
#[doc = "Message Buffer WORD_64B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb64bWord5(pub u32);
impl Mb64bWord5 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_23(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_23(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_22(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_22(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_21(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_21(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_20(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_20(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb64bWord5 {
    #[inline(always)]
    fn default() -> Mb64bWord5 {
        Mb64bWord5(0)
    }
}
impl core::fmt::Debug for Mb64bWord5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb64bWord5")
            .field("data_byte_23", &self.data_byte_23())
            .field("data_byte_22", &self.data_byte_22())
            .field("data_byte_21", &self.data_byte_21())
            .field("data_byte_20", &self.data_byte_20())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb64bWord5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb64bWord5 {{ data_byte_23: {=u8:?}, data_byte_22: {=u8:?}, data_byte_21: {=u8:?}, data_byte_20: {=u8:?} }}",
            self.data_byte_23(),
            self.data_byte_22(),
            self.data_byte_21(),
            self.data_byte_20()
        )
    }
}
#[doc = "Message Buffer WORD_64B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb64bWord6(pub u32);
impl Mb64bWord6 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_27(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_27(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_26(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_26(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_25(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_25(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_24(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_24(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb64bWord6 {
    #[inline(always)]
    fn default() -> Mb64bWord6 {
        Mb64bWord6(0)
    }
}
impl core::fmt::Debug for Mb64bWord6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb64bWord6")
            .field("data_byte_27", &self.data_byte_27())
            .field("data_byte_26", &self.data_byte_26())
            .field("data_byte_25", &self.data_byte_25())
            .field("data_byte_24", &self.data_byte_24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb64bWord6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb64bWord6 {{ data_byte_27: {=u8:?}, data_byte_26: {=u8:?}, data_byte_25: {=u8:?}, data_byte_24: {=u8:?} }}",
            self.data_byte_27(),
            self.data_byte_26(),
            self.data_byte_25(),
            self.data_byte_24()
        )
    }
}
#[doc = "Message Buffer WORD_64B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb64bWord7(pub u32);
impl Mb64bWord7 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_31(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_31(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_30(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_30(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_29(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_29(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_28(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_28(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb64bWord7 {
    #[inline(always)]
    fn default() -> Mb64bWord7 {
        Mb64bWord7(0)
    }
}
impl core::fmt::Debug for Mb64bWord7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb64bWord7")
            .field("data_byte_31", &self.data_byte_31())
            .field("data_byte_30", &self.data_byte_30())
            .field("data_byte_29", &self.data_byte_29())
            .field("data_byte_28", &self.data_byte_28())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb64bWord7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb64bWord7 {{ data_byte_31: {=u8:?}, data_byte_30: {=u8:?}, data_byte_29: {=u8:?}, data_byte_28: {=u8:?} }}",
            self.data_byte_31(),
            self.data_byte_30(),
            self.data_byte_29(),
            self.data_byte_28()
        )
    }
}
#[doc = "Message Buffer WORD_64B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb64bWord8(pub u32);
impl Mb64bWord8 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_35(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_35(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_34(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_34(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_33(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_33(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_32(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_32(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb64bWord8 {
    #[inline(always)]
    fn default() -> Mb64bWord8 {
        Mb64bWord8(0)
    }
}
impl core::fmt::Debug for Mb64bWord8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb64bWord8")
            .field("data_byte_35", &self.data_byte_35())
            .field("data_byte_34", &self.data_byte_34())
            .field("data_byte_33", &self.data_byte_33())
            .field("data_byte_32", &self.data_byte_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb64bWord8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb64bWord8 {{ data_byte_35: {=u8:?}, data_byte_34: {=u8:?}, data_byte_33: {=u8:?}, data_byte_32: {=u8:?} }}",
            self.data_byte_35(),
            self.data_byte_34(),
            self.data_byte_33(),
            self.data_byte_32()
        )
    }
}
#[doc = "Message Buffer WORD_64B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb64bWord9(pub u32);
impl Mb64bWord9 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_39(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_39(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_38(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_38(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_37(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_37(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_36(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_36(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb64bWord9 {
    #[inline(always)]
    fn default() -> Mb64bWord9 {
        Mb64bWord9(0)
    }
}
impl core::fmt::Debug for Mb64bWord9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb64bWord9")
            .field("data_byte_39", &self.data_byte_39())
            .field("data_byte_38", &self.data_byte_38())
            .field("data_byte_37", &self.data_byte_37())
            .field("data_byte_36", &self.data_byte_36())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb64bWord9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb64bWord9 {{ data_byte_39: {=u8:?}, data_byte_38: {=u8:?}, data_byte_37: {=u8:?}, data_byte_36: {=u8:?} }}",
            self.data_byte_39(),
            self.data_byte_38(),
            self.data_byte_37(),
            self.data_byte_36()
        )
    }
}
#[doc = "Message Buffer CS Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb8bCs(pub u32);
impl Mb8bCs {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    #[must_use]
    #[inline(always)]
    pub const fn time_stamp(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    #[inline(always)]
    pub const fn set_time_stamp(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Length of the data to be stored/transmitted."]
    #[must_use]
    #[inline(always)]
    pub const fn dlc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Length of the data to be stored/transmitted."]
    #[inline(always)]
    pub const fn set_dlc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    #[must_use]
    #[inline(always)]
    pub const fn rtr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    #[inline(always)]
    pub const fn set_rtr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    #[must_use]
    #[inline(always)]
    pub const fn ide(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    #[inline(always)]
    pub const fn set_ide(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    #[must_use]
    #[inline(always)]
    pub const fn srr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    #[inline(always)]
    pub const fn set_srr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    #[must_use]
    #[inline(always)]
    pub const fn code(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    #[inline(always)]
    pub const fn set_code(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    #[must_use]
    #[inline(always)]
    pub const fn esi(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    #[inline(always)]
    pub const fn set_esi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    #[must_use]
    #[inline(always)]
    pub const fn brs(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    #[inline(always)]
    pub const fn set_brs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    #[must_use]
    #[inline(always)]
    pub const fn edl(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    #[inline(always)]
    pub const fn set_edl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mb8bCs {
    #[inline(always)]
    fn default() -> Mb8bCs {
        Mb8bCs(0)
    }
}
impl core::fmt::Debug for Mb8bCs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb8bCs")
            .field("time_stamp", &self.time_stamp())
            .field("dlc", &self.dlc())
            .field("rtr", &self.rtr())
            .field("ide", &self.ide())
            .field("srr", &self.srr())
            .field("code", &self.code())
            .field("esi", &self.esi())
            .field("brs", &self.brs())
            .field("edl", &self.edl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb8bCs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb8bCs {{ time_stamp: {=u16:?}, dlc: {=u8:?}, rtr: {=bool:?}, ide: {=bool:?}, srr: {=bool:?}, code: {=u8:?}, esi: {=bool:?}, brs: {=bool:?}, edl: {=bool:?} }}",
            self.time_stamp(),
            self.dlc(),
            self.rtr(),
            self.ide(),
            self.srr(),
            self.code(),
            self.esi(),
            self.brs(),
            self.edl()
        )
    }
}
#[doc = "Message Buffer ID Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb8bId(pub u32);
impl Mb8bId {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn ext(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    #[inline(always)]
    pub const fn set_ext(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn std(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x07ff;
        val as u16
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    #[inline(always)]
    pub const fn set_std(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    #[must_use]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    #[inline(always)]
    pub const fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for Mb8bId {
    #[inline(always)]
    fn default() -> Mb8bId {
        Mb8bId(0)
    }
}
impl core::fmt::Debug for Mb8bId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb8bId")
            .field("ext", &self.ext())
            .field("std", &self.std())
            .field("prio", &self.prio())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb8bId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb8bId {{ ext: {=u32:?}, std: {=u16:?}, prio: {=u8:?} }}",
            self.ext(),
            self.std(),
            self.prio()
        )
    }
}
#[doc = "Message Buffer WORD_8B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb8bWord0(pub u32);
impl Mb8bWord0 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_0(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb8bWord0 {
    #[inline(always)]
    fn default() -> Mb8bWord0 {
        Mb8bWord0(0)
    }
}
impl core::fmt::Debug for Mb8bWord0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb8bWord0")
            .field("data_byte_3", &self.data_byte_3())
            .field("data_byte_2", &self.data_byte_2())
            .field("data_byte_1", &self.data_byte_1())
            .field("data_byte_0", &self.data_byte_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb8bWord0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb8bWord0 {{ data_byte_3: {=u8:?}, data_byte_2: {=u8:?}, data_byte_1: {=u8:?}, data_byte_0: {=u8:?} }}",
            self.data_byte_3(),
            self.data_byte_2(),
            self.data_byte_1(),
            self.data_byte_0()
        )
    }
}
#[doc = "Message Buffer WORD_8B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb8bWord1(pub u32);
impl Mb8bWord1 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_7(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_6(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_5(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mb8bWord1 {
    #[inline(always)]
    fn default() -> Mb8bWord1 {
        Mb8bWord1(0)
    }
}
impl core::fmt::Debug for Mb8bWord1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mb8bWord1")
            .field("data_byte_7", &self.data_byte_7())
            .field("data_byte_6", &self.data_byte_6())
            .field("data_byte_5", &self.data_byte_5())
            .field("data_byte_4", &self.data_byte_4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mb8bWord1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mb8bWord1 {{ data_byte_7: {=u8:?}, data_byte_6: {=u8:?}, data_byte_5: {=u8:?}, data_byte_4: {=u8:?} }}",
            self.data_byte_7(),
            self.data_byte_6(),
            self.data_byte_5(),
            self.data_byte_4()
        )
    }
}
#[doc = "Module Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc = "Number of the Last Message Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn maxmb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Number of the Last Message Buffer."]
    #[inline(always)]
    pub const fn set_maxmb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "ID Acceptance Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn idam(&self) -> Idam {
        let val = (self.0 >> 8usize) & 0x03;
        Idam::from_bits(val as u8)
    }
    #[doc = "ID Acceptance Mode."]
    #[inline(always)]
    pub const fn set_idam(&mut self, val: Idam) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CAN FD Operation Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fden(&self) -> Fden {
        let val = (self.0 >> 11usize) & 0x01;
        Fden::from_bits(val as u8)
    }
    #[doc = "CAN FD Operation Enable."]
    #[inline(always)]
    pub const fn set_fden(&mut self, val: Fden) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Abort Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aen(&self) -> Aen {
        let val = (self.0 >> 12usize) & 0x01;
        Aen::from_bits(val as u8)
    }
    #[doc = "Abort Enable."]
    #[inline(always)]
    pub const fn set_aen(&mut self, val: Aen) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Local Priority Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lprioen(&self) -> Lprioen {
        let val = (self.0 >> 13usize) & 0x01;
        Lprioen::from_bits(val as u8)
    }
    #[doc = "Local Priority Enable."]
    #[inline(always)]
    pub const fn set_lprioen(&mut self, val: Lprioen) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Pretended Networking Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pnet_en(&self) -> PnetEn {
        let val = (self.0 >> 14usize) & 0x01;
        PnetEn::from_bits(val as u8)
    }
    #[doc = "Pretended Networking Enable."]
    #[inline(always)]
    pub const fn set_pnet_en(&mut self, val: PnetEn) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dma(&self) -> Dma {
        let val = (self.0 >> 15usize) & 0x01;
        Dma::from_bits(val as u8)
    }
    #[doc = "DMA Enable."]
    #[inline(always)]
    pub const fn set_dma(&mut self, val: Dma) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Individual RX Masking and Queue Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn irmq(&self) -> Irmq {
        let val = (self.0 >> 16usize) & 0x01;
        Irmq::from_bits(val as u8)
    }
    #[doc = "Individual RX Masking and Queue Enable."]
    #[inline(always)]
    pub const fn set_irmq(&mut self, val: Irmq) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Self-Reception Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn srxdis(&self) -> Srxdis {
        let val = (self.0 >> 17usize) & 0x01;
        Srxdis::from_bits(val as u8)
    }
    #[doc = "Self-Reception Disable."]
    #[inline(always)]
    pub const fn set_srxdis(&mut self, val: Srxdis) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Wake-Up Source."]
    #[must_use]
    #[inline(always)]
    pub const fn waksrc(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Source."]
    #[inline(always)]
    pub const fn set_waksrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Low-Power Mode Acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn lpmack(&self) -> Lpmack {
        let val = (self.0 >> 20usize) & 0x01;
        Lpmack::from_bits(val as u8)
    }
    #[doc = "Low-Power Mode Acknowledge."]
    #[inline(always)]
    pub const fn set_lpmack(&mut self, val: Lpmack) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Warning Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wrnen(&self) -> Wrnen {
        let val = (self.0 >> 21usize) & 0x01;
        Wrnen::from_bits(val as u8)
    }
    #[doc = "Warning Interrupt Enable."]
    #[inline(always)]
    pub const fn set_wrnen(&mut self, val: Wrnen) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Self Wake-up."]
    #[must_use]
    #[inline(always)]
    pub const fn slfwak(&self) -> Slfwak {
        let val = (self.0 >> 22usize) & 0x01;
        Slfwak::from_bits(val as u8)
    }
    #[doc = "Self Wake-up."]
    #[inline(always)]
    pub const fn set_slfwak(&mut self, val: Slfwak) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Freeze Mode Acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn frzack(&self) -> Frzack {
        let val = (self.0 >> 24usize) & 0x01;
        Frzack::from_bits(val as u8)
    }
    #[doc = "Freeze Mode Acknowledge."]
    #[inline(always)]
    pub const fn set_frzack(&mut self, val: Frzack) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Soft Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn softrst(&self) -> Softrst {
        let val = (self.0 >> 25usize) & 0x01;
        Softrst::from_bits(val as u8)
    }
    #[doc = "Soft Reset."]
    #[inline(always)]
    pub const fn set_softrst(&mut self, val: Softrst) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Wake-up Interrupt Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn wakmsk(&self) -> Wakmsk {
        let val = (self.0 >> 26usize) & 0x01;
        Wakmsk::from_bits(val as u8)
    }
    #[doc = "Wake-up Interrupt Mask."]
    #[inline(always)]
    pub const fn set_wakmsk(&mut self, val: Wakmsk) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "FlexCAN Not Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn notrdy(&self) -> Notrdy {
        let val = (self.0 >> 27usize) & 0x01;
        Notrdy::from_bits(val as u8)
    }
    #[doc = "FlexCAN Not Ready."]
    #[inline(always)]
    pub const fn set_notrdy(&mut self, val: Notrdy) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Halt FlexCAN."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Halt {
        let val = (self.0 >> 28usize) & 0x01;
        Halt::from_bits(val as u8)
    }
    #[doc = "Halt FlexCAN."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Halt) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Legacy RX FIFO Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rfen(&self) -> Rfen {
        let val = (self.0 >> 29usize) & 0x01;
        Rfen::from_bits(val as u8)
    }
    #[doc = "Legacy RX FIFO Enable."]
    #[inline(always)]
    pub const fn set_rfen(&mut self, val: Rfen) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Freeze Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn frz(&self) -> Frz {
        let val = (self.0 >> 30usize) & 0x01;
        Frz::from_bits(val as u8)
    }
    #[doc = "Freeze Enable."]
    #[inline(always)]
    pub const fn set_frz(&mut self, val: Frz) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Module Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn mdis(&self) -> Mdis {
        let val = (self.0 >> 31usize) & 0x01;
        Mdis::from_bits(val as u8)
    }
    #[doc = "Module Disable."]
    #[inline(always)]
    pub const fn set_mdis(&mut self, val: Mdis) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mcr {
    #[inline(always)]
    fn default() -> Mcr {
        Mcr(0)
    }
}
impl core::fmt::Debug for Mcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcr")
            .field("maxmb", &self.maxmb())
            .field("idam", &self.idam())
            .field("fden", &self.fden())
            .field("aen", &self.aen())
            .field("lprioen", &self.lprioen())
            .field("pnet_en", &self.pnet_en())
            .field("dma", &self.dma())
            .field("irmq", &self.irmq())
            .field("srxdis", &self.srxdis())
            .field("waksrc", &self.waksrc())
            .field("lpmack", &self.lpmack())
            .field("wrnen", &self.wrnen())
            .field("slfwak", &self.slfwak())
            .field("frzack", &self.frzack())
            .field("softrst", &self.softrst())
            .field("wakmsk", &self.wakmsk())
            .field("notrdy", &self.notrdy())
            .field("halt", &self.halt())
            .field("rfen", &self.rfen())
            .field("frz", &self.frz())
            .field("mdis", &self.mdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcr {{ maxmb: {=u8:?}, idam: {:?}, fden: {:?}, aen: {:?}, lprioen: {:?}, pnet_en: {:?}, dma: {:?}, irmq: {:?}, srxdis: {:?}, waksrc: {=bool:?}, lpmack: {:?}, wrnen: {:?}, slfwak: {:?}, frzack: {:?}, softrst: {:?}, wakmsk: {:?}, notrdy: {:?}, halt: {:?}, rfen: {:?}, frz: {:?}, mdis: {:?} }}",
            self.maxmb(),
            self.idam(),
            self.fden(),
            self.aen(),
            self.lprioen(),
            self.pnet_en(),
            self.dma(),
            self.irmq(),
            self.srxdis(),
            self.waksrc(),
            self.lpmack(),
            self.wrnen(),
            self.slfwak(),
            self.frzack(),
            self.softrst(),
            self.wakmsk(),
            self.notrdy(),
            self.halt(),
            self.rfen(),
            self.frz(),
            self.mdis()
        )
    }
}
#[doc = "Pretended Networking Payload High Filter 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pl1Hi(pub u32);
impl Pl1Hi {
    #[doc = "Data byte 7."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_7(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 7."]
    #[inline(always)]
    pub const fn set_data_byte_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 6."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_6(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 6."]
    #[inline(always)]
    pub const fn set_data_byte_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 5."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_5(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 5."]
    #[inline(always)]
    pub const fn set_data_byte_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 4."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 4."]
    #[inline(always)]
    pub const fn set_data_byte_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Pl1Hi {
    #[inline(always)]
    fn default() -> Pl1Hi {
        Pl1Hi(0)
    }
}
impl core::fmt::Debug for Pl1Hi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pl1Hi")
            .field("data_byte_7", &self.data_byte_7())
            .field("data_byte_6", &self.data_byte_6())
            .field("data_byte_5", &self.data_byte_5())
            .field("data_byte_4", &self.data_byte_4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pl1Hi {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pl1Hi {{ data_byte_7: {=u8:?}, data_byte_6: {=u8:?}, data_byte_5: {=u8:?}, data_byte_4: {=u8:?} }}",
            self.data_byte_7(),
            self.data_byte_6(),
            self.data_byte_5(),
            self.data_byte_4()
        )
    }
}
#[doc = "Pretended Networking Payload Low Filter 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pl1Lo(pub u32);
impl Pl1Lo {
    #[doc = "Data byte 3."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3."]
    #[inline(always)]
    pub const fn set_data_byte_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 2."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2."]
    #[inline(always)]
    pub const fn set_data_byte_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 1."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1."]
    #[inline(always)]
    pub const fn set_data_byte_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 0."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_0(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0."]
    #[inline(always)]
    pub const fn set_data_byte_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Pl1Lo {
    #[inline(always)]
    fn default() -> Pl1Lo {
        Pl1Lo(0)
    }
}
impl core::fmt::Debug for Pl1Lo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pl1Lo")
            .field("data_byte_3", &self.data_byte_3())
            .field("data_byte_2", &self.data_byte_2())
            .field("data_byte_1", &self.data_byte_1())
            .field("data_byte_0", &self.data_byte_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pl1Lo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pl1Lo {{ data_byte_3: {=u8:?}, data_byte_2: {=u8:?}, data_byte_1: {=u8:?}, data_byte_0: {=u8:?} }}",
            self.data_byte_3(),
            self.data_byte_2(),
            self.data_byte_1(),
            self.data_byte_0()
        )
    }
}
#[doc = "Pretended Networking Payload High Filter 2 and Payload High Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pl2PlmaskHi(pub u32);
impl Pl2PlmaskHi {
    #[doc = "Data Byte 7."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_7(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 7."]
    #[inline(always)]
    pub const fn set_data_byte_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data Byte 6."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_6(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 6."]
    #[inline(always)]
    pub const fn set_data_byte_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data Byte 5."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_5(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 5."]
    #[inline(always)]
    pub const fn set_data_byte_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data Byte 4."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 4."]
    #[inline(always)]
    pub const fn set_data_byte_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Pl2PlmaskHi {
    #[inline(always)]
    fn default() -> Pl2PlmaskHi {
        Pl2PlmaskHi(0)
    }
}
impl core::fmt::Debug for Pl2PlmaskHi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pl2PlmaskHi")
            .field("data_byte_7", &self.data_byte_7())
            .field("data_byte_6", &self.data_byte_6())
            .field("data_byte_5", &self.data_byte_5())
            .field("data_byte_4", &self.data_byte_4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pl2PlmaskHi {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pl2PlmaskHi {{ data_byte_7: {=u8:?}, data_byte_6: {=u8:?}, data_byte_5: {=u8:?}, data_byte_4: {=u8:?} }}",
            self.data_byte_7(),
            self.data_byte_6(),
            self.data_byte_5(),
            self.data_byte_4()
        )
    }
}
#[doc = "Pretended Networking Payload Low Filter 2 and Payload Low Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pl2PlmaskLo(pub u32);
impl Pl2PlmaskLo {
    #[doc = "Data Byte 3."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 3."]
    #[inline(always)]
    pub const fn set_data_byte_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data Byte 2."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 2."]
    #[inline(always)]
    pub const fn set_data_byte_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data Byte 1."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 1."]
    #[inline(always)]
    pub const fn set_data_byte_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data Byte 0."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_0(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 0."]
    #[inline(always)]
    pub const fn set_data_byte_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Pl2PlmaskLo {
    #[inline(always)]
    fn default() -> Pl2PlmaskLo {
        Pl2PlmaskLo(0)
    }
}
impl core::fmt::Debug for Pl2PlmaskLo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pl2PlmaskLo")
            .field("data_byte_3", &self.data_byte_3())
            .field("data_byte_2", &self.data_byte_2())
            .field("data_byte_1", &self.data_byte_1())
            .field("data_byte_0", &self.data_byte_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pl2PlmaskLo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pl2PlmaskLo {{ data_byte_3: {=u8:?}, data_byte_2: {=u8:?}, data_byte_1: {=u8:?}, data_byte_0: {=u8:?} }}",
            self.data_byte_3(),
            self.data_byte_2(),
            self.data_byte_1(),
            self.data_byte_0()
        )
    }
}
#[doc = "Receive 14 Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rx14mask(pub u32);
impl Rx14mask {
    #[doc = "RX Buffer 14 Mask Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn rx14m(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "RX Buffer 14 Mask Bits."]
    #[inline(always)]
    pub const fn set_rx14m(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rx14mask {
    #[inline(always)]
    fn default() -> Rx14mask {
        Rx14mask(0)
    }
}
impl core::fmt::Debug for Rx14mask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rx14mask")
            .field("rx14m", &self.rx14m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rx14mask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rx14mask {{ rx14m: {=u32:?} }}", self.rx14m())
    }
}
#[doc = "Receive 15 Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rx15mask(pub u32);
impl Rx15mask {
    #[doc = "RX Buffer 15 Mask Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn rx15m(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "RX Buffer 15 Mask Bits."]
    #[inline(always)]
    pub const fn set_rx15m(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rx15mask {
    #[inline(always)]
    fn default() -> Rx15mask {
        Rx15mask(0)
    }
}
impl core::fmt::Debug for Rx15mask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rx15mask")
            .field("rx15m", &self.rx15m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rx15mask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rx15mask {{ rx15m: {=u32:?} }}", self.rx15m())
    }
}
#[doc = "Legacy RX FIFO Global Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxfgmask(pub u32);
impl Rxfgmask {
    #[doc = "Legacy RX FIFO Global Mask Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn fgm(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Legacy RX FIFO Global Mask Bits."]
    #[inline(always)]
    pub const fn set_fgm(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rxfgmask {
    #[inline(always)]
    fn default() -> Rxfgmask {
        Rxfgmask(0)
    }
}
impl core::fmt::Debug for Rxfgmask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxfgmask")
            .field("fgm", &self.fgm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxfgmask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxfgmask {{ fgm: {=u32:?} }}", self.fgm())
    }
}
#[doc = "Legacy RX FIFO Information."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxfir(pub u32);
impl Rxfir {
    #[doc = "Identifier Acceptance Filter Hit Indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn idhit(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Identifier Acceptance Filter Hit Indicator."]
    #[inline(always)]
    pub const fn set_idhit(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Rxfir {
    #[inline(always)]
    fn default() -> Rxfir {
        Rxfir(0)
    }
}
impl core::fmt::Debug for Rxfir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxfir")
            .field("idhit", &self.idhit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxfir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxfir {{ idhit: {=u16:?} }}", self.idhit())
    }
}
#[doc = "Receive Individual Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rximr(pub u32);
impl Rximr {
    #[doc = "Individual Mask Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn mi(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Individual Mask Bits."]
    #[inline(always)]
    pub const fn set_mi(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rximr {
    #[inline(always)]
    fn default() -> Rximr {
        Rximr(0)
    }
}
impl core::fmt::Debug for Rximr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rximr").field("mi", &self.mi()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rximr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rximr {{ mi: {=u32:?} }}", self.mi())
    }
}
#[doc = "RX Message Buffers Global Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxmgmask(pub u32);
impl Rxmgmask {
    #[doc = "Global Mask for RX Message Buffers."]
    #[must_use]
    #[inline(always)]
    pub const fn mg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Global Mask for RX Message Buffers."]
    #[inline(always)]
    pub const fn set_mg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rxmgmask {
    #[inline(always)]
    fn default() -> Rxmgmask {
        Rxmgmask(0)
    }
}
impl core::fmt::Debug for Rxmgmask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxmgmask").field("mg", &self.mg()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxmgmask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxmgmask {{ mg: {=u32:?} }}", self.mg())
    }
}
#[doc = "Free-Running Timer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer(pub u32);
impl Timer {
    #[doc = "Timer Value."]
    #[must_use]
    #[inline(always)]
    pub const fn timer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timer Value."]
    #[inline(always)]
    pub const fn set_timer(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Timer {
    #[inline(always)]
    fn default() -> Timer {
        Timer(0)
    }
}
impl core::fmt::Debug for Timer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer")
            .field("timer", &self.timer())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer {{ timer: {=u16:?} }}", self.timer())
    }
}
#[doc = "Wake-Up Message Buffer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WmbCs(pub u32);
impl WmbCs {
    #[doc = "Length of Data in Bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn dlc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Length of Data in Bytes."]
    #[inline(always)]
    pub const fn set_dlc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Remote Transmission Request."]
    #[must_use]
    #[inline(always)]
    pub const fn rtr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Transmission Request."]
    #[inline(always)]
    pub const fn set_rtr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ID Extended Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ide(&self) -> Ide {
        let val = (self.0 >> 21usize) & 0x01;
        Ide::from_bits(val as u8)
    }
    #[doc = "ID Extended Bit."]
    #[inline(always)]
    pub const fn set_ide(&mut self, val: Ide) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Substitute Remote Request."]
    #[must_use]
    #[inline(always)]
    pub const fn srr(&self) -> Srr {
        let val = (self.0 >> 22usize) & 0x01;
        Srr::from_bits(val as u8)
    }
    #[doc = "Substitute Remote Request."]
    #[inline(always)]
    pub const fn set_srr(&mut self, val: Srr) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
}
impl Default for WmbCs {
    #[inline(always)]
    fn default() -> WmbCs {
        WmbCs(0)
    }
}
impl core::fmt::Debug for WmbCs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WmbCs")
            .field("dlc", &self.dlc())
            .field("rtr", &self.rtr())
            .field("ide", &self.ide())
            .field("srr", &self.srr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WmbCs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WmbCs {{ dlc: {=u8:?}, rtr: {=bool:?}, ide: {:?}, srr: {:?} }}",
            self.dlc(),
            self.rtr(),
            self.ide(),
            self.srr()
        )
    }
}
#[doc = "Wake-Up Message Buffer for Data 0-3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WmbD03(pub u32);
impl WmbD03 {
    #[doc = "Data Byte 3."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 3."]
    #[inline(always)]
    pub const fn set_data_byte_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data Byte 2."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 2."]
    #[inline(always)]
    pub const fn set_data_byte_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data Byte 1."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 1."]
    #[inline(always)]
    pub const fn set_data_byte_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data Byte 0."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_0(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 0."]
    #[inline(always)]
    pub const fn set_data_byte_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for WmbD03 {
    #[inline(always)]
    fn default() -> WmbD03 {
        WmbD03(0)
    }
}
impl core::fmt::Debug for WmbD03 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WmbD03")
            .field("data_byte_3", &self.data_byte_3())
            .field("data_byte_2", &self.data_byte_2())
            .field("data_byte_1", &self.data_byte_1())
            .field("data_byte_0", &self.data_byte_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WmbD03 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WmbD03 {{ data_byte_3: {=u8:?}, data_byte_2: {=u8:?}, data_byte_1: {=u8:?}, data_byte_0: {=u8:?} }}",
            self.data_byte_3(),
            self.data_byte_2(),
            self.data_byte_1(),
            self.data_byte_0()
        )
    }
}
#[doc = "Wake-Up Message Buffer Register Data 4-7."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WmbD47(pub u32);
impl WmbD47 {
    #[doc = "Data Byte 7."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_7(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 7."]
    #[inline(always)]
    pub const fn set_data_byte_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data Byte 6."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_6(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 6."]
    #[inline(always)]
    pub const fn set_data_byte_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data Byte 5."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_5(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 5."]
    #[inline(always)]
    pub const fn set_data_byte_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data Byte 4."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 4."]
    #[inline(always)]
    pub const fn set_data_byte_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for WmbD47 {
    #[inline(always)]
    fn default() -> WmbD47 {
        WmbD47(0)
    }
}
impl core::fmt::Debug for WmbD47 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WmbD47")
            .field("data_byte_7", &self.data_byte_7())
            .field("data_byte_6", &self.data_byte_6())
            .field("data_byte_5", &self.data_byte_5())
            .field("data_byte_4", &self.data_byte_4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WmbD47 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WmbD47 {{ data_byte_7: {=u8:?}, data_byte_6: {=u8:?}, data_byte_5: {=u8:?}, data_byte_4: {=u8:?} }}",
            self.data_byte_7(),
            self.data_byte_6(),
            self.data_byte_5(),
            self.data_byte_4()
        )
    }
}
#[doc = "Wake-Up Message Buffer for ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WmbId(pub u32);
impl WmbId {
    #[doc = "Received ID in Pretended Networking Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Received ID in Pretended Networking Mode."]
    #[inline(always)]
    pub const fn set_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
}
impl Default for WmbId {
    #[inline(always)]
    fn default() -> WmbId {
        WmbId(0)
    }
}
impl core::fmt::Debug for WmbId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WmbId").field("id", &self.id()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WmbId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "WmbId {{ id: {=u32:?} }}", self.id())
    }
}
#[doc = "Message Buffer WORD0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Word0(pub u32);
impl Word0 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_0(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Word0 {
    #[inline(always)]
    fn default() -> Word0 {
        Word0(0)
    }
}
impl core::fmt::Debug for Word0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Word0")
            .field("data_byte_3", &self.data_byte_3())
            .field("data_byte_2", &self.data_byte_2())
            .field("data_byte_1", &self.data_byte_1())
            .field("data_byte_0", &self.data_byte_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Word0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Word0 {{ data_byte_3: {=u8:?}, data_byte_2: {=u8:?}, data_byte_1: {=u8:?}, data_byte_0: {=u8:?} }}",
            self.data_byte_3(),
            self.data_byte_2(),
            self.data_byte_1(),
            self.data_byte_0()
        )
    }
}
#[doc = "Message Buffer WORD1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Word1(pub u32);
impl Word1 {
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_7(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_6(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_5(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub const fn set_data_byte_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Word1 {
    #[inline(always)]
    fn default() -> Word1 {
        Word1(0)
    }
}
impl core::fmt::Debug for Word1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Word1")
            .field("data_byte_7", &self.data_byte_7())
            .field("data_byte_6", &self.data_byte_6())
            .field("data_byte_5", &self.data_byte_5())
            .field("data_byte_4", &self.data_byte_4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Word1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Word1 {{ data_byte_7: {=u8:?}, data_byte_6: {=u8:?}, data_byte_5: {=u8:?}, data_byte_4: {=u8:?} }}",
            self.data_byte_7(),
            self.data_byte_6(),
            self.data_byte_5(),
            self.data_byte_4()
        )
    }
}
#[doc = "Pretended Networking Wake-Up Match."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WuMtc(pub u32);
impl WuMtc {
    #[doc = "Number of Matches in Pretended Networking."]
    #[must_use]
    #[inline(always)]
    pub const fn mcounter(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Number of Matches in Pretended Networking."]
    #[inline(always)]
    pub const fn set_mcounter(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Wake-up by Match Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn wumf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up by Match Flag."]
    #[inline(always)]
    pub const fn set_wumf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-up by Timeout Flag Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn wtof(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up by Timeout Flag Bit."]
    #[inline(always)]
    pub const fn set_wtof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for WuMtc {
    #[inline(always)]
    fn default() -> WuMtc {
        WuMtc(0)
    }
}
impl core::fmt::Debug for WuMtc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WuMtc")
            .field("mcounter", &self.mcounter())
            .field("wumf", &self.wumf())
            .field("wtof", &self.wtof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WuMtc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WuMtc {{ mcounter: {=u8:?}, wumf: {=bool:?}, wtof: {=bool:?} }}",
            self.mcounter(),
            self.wumf(),
            self.wtof()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ackerr {
    #[doc = "No error."]
    AckErrorNo = 0x0,
    #[doc = "Error occurred since last read of this register."]
    AckErrorYes = 0x01,
}
impl Ackerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ackerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ackerr {
    #[inline(always)]
    fn from(val: u8) -> Ackerr {
        Ackerr::from_bits(val)
    }
}
impl From<Ackerr> for u8 {
    #[inline(always)]
    fn from(val: Ackerr) -> u8 {
        Ackerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aen {
    #[doc = "Disabled."]
    AbortDisabled = 0x0,
    #[doc = "Enabled."]
    AbortEnabled = 0x01,
}
impl Aen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aen {
    #[inline(always)]
    fn from(val: u8) -> Aen {
        Aen::from_bits(val)
    }
}
impl From<Aen> for u8 {
    #[inline(always)]
    fn from(val: Aen) -> u8 {
        Aen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bit0err {
    #[doc = "No such occurrence."]
    Bit0ErrorNo = 0x0,
    #[doc = "At least one bit sent as dominant is received as recessive."]
    Bit0ErrorYes = 0x01,
}
impl Bit0err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bit0err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bit0err {
    #[inline(always)]
    fn from(val: u8) -> Bit0err {
        Bit0err::from_bits(val)
    }
}
impl From<Bit0err> for u8 {
    #[inline(always)]
    fn from(val: Bit0err) -> u8 {
        Bit0err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bit0errFast {
    #[doc = "No such occurrence."]
    Bit0ErrorNo = 0x0,
    #[doc = "At least one bit transmitted as dominant is received as recessive."]
    Bit0ErrorYes = 0x01,
}
impl Bit0errFast {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bit0errFast {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bit0errFast {
    #[inline(always)]
    fn from(val: u8) -> Bit0errFast {
        Bit0errFast::from_bits(val)
    }
}
impl From<Bit0errFast> for u8 {
    #[inline(always)]
    fn from(val: Bit0errFast) -> u8 {
        Bit0errFast::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bit1err {
    #[doc = "No such occurrence."]
    Bit1ErrorNo = 0x0,
    #[doc = "At least one bit sent as recessive is received as dominant."]
    Bit1ErrorYes = 0x01,
}
impl Bit1err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bit1err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bit1err {
    #[inline(always)]
    fn from(val: u8) -> Bit1err {
        Bit1err::from_bits(val)
    }
}
impl From<Bit1err> for u8 {
    #[inline(always)]
    fn from(val: Bit1err) -> u8 {
        Bit1err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bit1errFast {
    #[doc = "No such occurrence."]
    Bit1ErrorNo = 0x0,
    #[doc = "At least one bit transmitted as recessive is received as dominant."]
    Bit1ErrorYes = 0x01,
}
impl Bit1errFast {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bit1errFast {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bit1errFast {
    #[inline(always)]
    fn from(val: u8) -> Bit1errFast {
        Bit1errFast::from_bits(val)
    }
}
impl From<Bit1errFast> for u8 {
    #[inline(always)]
    fn from(val: Bit1errFast) -> u8 {
        Bit1errFast::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Boffdoneint {
    #[doc = "No such occurrence."]
    BusOffNotDone = 0x0,
    #[doc = "FlexCAN module has completed Bus Off process."]
    BusOffDone = 0x01,
}
impl Boffdoneint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Boffdoneint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Boffdoneint {
    #[inline(always)]
    fn from(val: u8) -> Boffdoneint {
        Boffdoneint::from_bits(val)
    }
}
impl From<Boffdoneint> for u8 {
    #[inline(always)]
    fn from(val: Boffdoneint) -> u8 {
        Boffdoneint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Boffmsk {
    #[doc = "Interrupt disabled."]
    BusOffIntDisabled = 0x0,
    #[doc = "Interrupt enabled."]
    BusOffIntEnabled = 0x01,
}
impl Boffmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Boffmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Boffmsk {
    #[inline(always)]
    fn from(val: u8) -> Boffmsk {
        Boffmsk::from_bits(val)
    }
}
impl From<Boffmsk> for u8 {
    #[inline(always)]
    fn from(val: Boffmsk) -> u8 {
        Boffmsk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Boffrec {
    #[doc = "Enabled."]
    AutoRecoverEnabled = 0x0,
    #[doc = "Disabled."]
    AutoRecoverDisabled = 0x01,
}
impl Boffrec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Boffrec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Boffrec {
    #[inline(always)]
    fn from(val: u8) -> Boffrec {
        Boffrec::from_bits(val)
    }
}
impl From<Boffrec> for u8 {
    #[inline(always)]
    fn from(val: Boffrec) -> u8 {
        Boffrec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Buf0i {
    #[doc = "MB0 has no occurrence of successfully completed transmission or reception."]
    BufferTxRxNotComplete = 0x0,
    #[doc = "MB0 has successfully completed transmission or reception."]
    BufferTxRxComplete = 0x01,
}
impl Buf0i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Buf0i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Buf0i {
    #[inline(always)]
    fn from(val: u8) -> Buf0i {
        Buf0i::from_bits(val)
    }
}
impl From<Buf0i> for u8 {
    #[inline(always)]
    fn from(val: Buf0i) -> u8 {
        Buf0i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Buf5i {
    #[doc = "No occurrence of completed transmission or reception, or no frames available."]
    Id1 = 0x0,
    #[doc = "MB5 completed transmission or reception, or frames available."]
    Id2 = 0x01,
}
impl Buf5i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Buf5i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Buf5i {
    #[inline(always)]
    fn from(val: u8) -> Buf5i {
        Buf5i::from_bits(val)
    }
}
impl From<Buf5i> for u8 {
    #[inline(always)]
    fn from(val: Buf5i) -> u8 {
        Buf5i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Buf6i {
    #[doc = "No occurrence of MB6 completing transmission or reception, or FIFO not almost full."]
    Id1 = 0x0,
    #[doc = "MB6 completed transmission or reception, or FIFO almost full."]
    Id2 = 0x01,
}
impl Buf6i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Buf6i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Buf6i {
    #[inline(always)]
    fn from(val: u8) -> Buf6i {
        Buf6i::from_bits(val)
    }
}
impl From<Buf6i> for u8 {
    #[inline(always)]
    fn from(val: Buf6i) -> u8 {
        Buf6i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Buf7i {
    #[doc = "No occurrence of MB7 completing transmission or reception, or no FIFO overflow."]
    Id1 = 0x0,
    #[doc = "MB7 completed transmission or reception, or FIFO overflow."]
    Id2 = 0x01,
}
impl Buf7i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Buf7i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Buf7i {
    #[inline(always)]
    fn from(val: u8) -> Buf7i {
        Buf7i::from_bits(val)
    }
}
impl From<Buf7i> for u8 {
    #[inline(always)]
    fn from(val: Buf7i) -> u8 {
        Buf7i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crcerr {
    #[doc = "No error."]
    CrcErrorNo = 0x0,
    #[doc = "Error occurred since last read of this register."]
    CrcErrorYes = 0x01,
}
impl Crcerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crcerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crcerr {
    #[inline(always)]
    fn from(val: u8) -> Crcerr {
        Crcerr::from_bits(val)
    }
}
impl From<Crcerr> for u8 {
    #[inline(always)]
    fn from(val: Crcerr) -> u8 {
        Crcerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrcerrFast {
    #[doc = "No such occurrence."]
    CrcErrorNo = 0x0,
    #[doc = "A CRC error occurred since last read of this register."]
    CrcErrorYes = 0x01,
}
impl CrcerrFast {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrcerrFast {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrcerrFast {
    #[inline(always)]
    fn from(val: u8) -> CrcerrFast {
        CrcerrFast::from_bits(val)
    }
}
impl From<CrcerrFast> for u8 {
    #[inline(always)]
    fn from(val: CrcerrFast) -> u8 {
        CrcerrFast::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma {
    #[doc = "Disable."]
    Id1 = 0x0,
    #[doc = "Enable."]
    Id2 = 0x01,
}
impl Dma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma {
    #[inline(always)]
    fn from(val: u8) -> Dma {
        Dma::from_bits(val)
    }
}
impl From<Dma> for u8 {
    #[inline(always)]
    fn from(val: Dma) -> u8 {
        Dma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eacen {
    #[doc = "Disable."]
    RtrCompareNo = 0x0,
    #[doc = "Enable."]
    RtrCompareYes = 0x01,
}
impl Eacen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eacen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eacen {
    #[inline(always)]
    fn from(val: u8) -> Eacen {
        Eacen::from_bits(val)
    }
}
impl From<Eacen> for u8 {
    #[inline(always)]
    fn from(val: Eacen) -> u8 {
        Eacen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edfltdis {
    #[doc = "Enabled."]
    Enable = 0x0,
    #[doc = "Disabled."]
    Disable = 0x01,
}
impl Edfltdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edfltdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edfltdis {
    #[inline(always)]
    fn from(val: u8) -> Edfltdis {
        Edfltdis::from_bits(val)
    }
}
impl From<Edfltdis> for u8 {
    #[inline(always)]
    fn from(val: Edfltdis) -> u8 {
        Edfltdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Erfclr {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "Clear enhanced RX FIFO content."]
    Clear = 0x01,
}
impl Erfclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erfclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erfclr {
    #[inline(always)]
    fn from(val: u8) -> Erfclr {
        Erfclr::from_bits(val)
    }
}
impl From<Erfclr> for u8 {
    #[inline(always)]
    fn from(val: Erfclr) -> u8 {
        Erfclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Erfwmi {
    #[doc = "No such occurrence."]
    WatermarkNo = 0x0,
    #[doc = "Number of messages in FIFO is greater than the watermark."]
    WatermarkYes = 0x01,
}
impl Erfwmi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erfwmi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erfwmi {
    #[inline(always)]
    fn from(val: u8) -> Erfwmi {
        Erfwmi::from_bits(val)
    }
}
impl From<Erfwmi> for u8 {
    #[inline(always)]
    fn from(val: Erfwmi) -> u8 {
        Erfwmi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ErrintFast {
    #[doc = "No such occurrence."]
    ErrorsDataPhaseNo = 0x0,
    #[doc = "Error flag set in the data phase of CAN FD frames that have BRS = 1."]
    ErrorsDataPhaseYes = 0x01,
}
impl ErrintFast {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ErrintFast {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ErrintFast {
    #[inline(always)]
    fn from(val: u8) -> ErrintFast {
        ErrintFast::from_bits(val)
    }
}
impl From<ErrintFast> for u8 {
    #[inline(always)]
    fn from(val: ErrintFast) -> u8 {
        ErrintFast::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Errmsk {
    #[doc = "Interrupt disabled."]
    ErrorIntDisabled = 0x0,
    #[doc = "Interrupt enabled."]
    ErrorIntEnabled = 0x01,
}
impl Errmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Errmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Errmsk {
    #[inline(always)]
    fn from(val: u8) -> Errmsk {
        Errmsk::from_bits(val)
    }
}
impl From<Errmsk> for u8 {
    #[inline(always)]
    fn from(val: Errmsk) -> u8 {
        Errmsk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Errovr {
    #[doc = "No overrun."]
    OverrunNotOccurred = 0x0,
    #[doc = "Overrun."]
    OverrunOccurred = 0x01,
}
impl Errovr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Errovr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Errovr {
    #[inline(always)]
    fn from(val: u8) -> Errovr {
        Errovr::from_bits(val)
    }
}
impl From<Errovr> for u8 {
    #[inline(always)]
    fn from(val: Errovr) -> u8 {
        Errovr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Etdcfail {
    #[doc = "In range."]
    InRange = 0x0,
    #[doc = "Out of range."]
    OutOfRange = 0x01,
}
impl Etdcfail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Etdcfail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Etdcfail {
    #[inline(always)]
    fn from(val: u8) -> Etdcfail {
        Etdcfail::from_bits(val)
    }
}
impl From<Etdcfail> for u8 {
    #[inline(always)]
    fn from(val: Etdcfail) -> u8 {
        Etdcfail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fcs {
    #[doc = "Message ID filtering only."]
    IdFiltering = 0x0,
    #[doc = "Message ID filtering and payload filtering."]
    IdPayloadFiltering = 0x01,
    #[doc = "Message ID filtering occurring a specified number of times."]
    IdFilteringNumber = 0x02,
    #[doc = "Message ID filtering and payload filtering a specified number of times."]
    IdPayloadFilteringNumber = 0x03,
}
impl Fcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fcs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fcs {
    #[inline(always)]
    fn from(val: u8) -> Fcs {
        Fcs::from_bits(val)
    }
}
impl From<Fcs> for u8 {
    #[inline(always)]
    fn from(val: Fcs) -> u8 {
        Fcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fden {
    #[doc = "Disable."]
    CanFdDisabled = 0x0,
    #[doc = "Enable."]
    CanFdEnabled = 0x01,
}
impl Fden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fden {
    #[inline(always)]
    fn from(val: u8) -> Fden {
        Fden::from_bits(val)
    }
}
impl From<Fden> for u8 {
    #[inline(always)]
    fn from(val: Fden) -> u8 {
        Fden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fdrate {
    #[doc = "Disable."]
    Nominal = 0x0,
    #[doc = "Enable."]
    BitRateSwitching = 0x01,
}
impl Fdrate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fdrate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fdrate {
    #[inline(always)]
    fn from(val: u8) -> Fdrate {
        Fdrate::from_bits(val)
    }
}
impl From<Fdrate> for u8 {
    #[inline(always)]
    fn from(val: Fdrate) -> u8 {
        Fdrate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FltIde {
    #[doc = "Standard."]
    Standard = 0x0,
    #[doc = "Extended."]
    Extended = 0x01,
}
impl FltIde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FltIde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FltIde {
    #[inline(always)]
    fn from(val: u8) -> FltIde {
        FltIde::from_bits(val)
    }
}
impl From<FltIde> for u8 {
    #[inline(always)]
    fn from(val: FltIde) -> u8 {
        FltIde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FltRtr {
    #[doc = "Reject remote frame (accept data frame)."]
    Reject = 0x0,
    #[doc = "Accept remote frame."]
    Accept = 0x01,
}
impl FltRtr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FltRtr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FltRtr {
    #[inline(always)]
    fn from(val: u8) -> FltRtr {
        FltRtr::from_bits(val)
    }
}
impl From<FltRtr> for u8 {
    #[inline(always)]
    fn from(val: FltRtr) -> u8 {
        FltRtr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fltconf {
    #[doc = "Error Active."]
    ErrorActive = 0x0,
    #[doc = "Error Passive."]
    ErrorPassive = 0x01,
    #[doc = "Bus Off."]
    BusOff = 0x02,
    _RESERVED_3 = 0x03,
}
impl Fltconf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fltconf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fltconf {
    #[inline(always)]
    fn from(val: u8) -> Fltconf {
        Fltconf::from_bits(val)
    }
}
impl From<Fltconf> for u8 {
    #[inline(always)]
    fn from(val: Fltconf) -> u8 {
        Fltconf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frmerr {
    #[doc = "No error."]
    FormErrorNo = 0x0,
    #[doc = "Error occurred since last read of this register."]
    FormErrorYes = 0x01,
}
impl Frmerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frmerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frmerr {
    #[inline(always)]
    fn from(val: u8) -> Frmerr {
        Frmerr::from_bits(val)
    }
}
impl From<Frmerr> for u8 {
    #[inline(always)]
    fn from(val: Frmerr) -> u8 {
        Frmerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrmerrFast {
    #[doc = "No such occurrence."]
    FormErrorNo = 0x0,
    #[doc = "A form error occurred since last read of this register."]
    FormErrorYes = 0x01,
}
impl FrmerrFast {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrmerrFast {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrmerrFast {
    #[inline(always)]
    fn from(val: u8) -> FrmerrFast {
        FrmerrFast::from_bits(val)
    }
}
impl From<FrmerrFast> for u8 {
    #[inline(always)]
    fn from(val: FrmerrFast) -> u8 {
        FrmerrFast::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frz {
    #[doc = "Disable."]
    FreezeModeDisabled = 0x0,
    #[doc = "Enable."]
    FreezeModeEnabled = 0x01,
}
impl Frz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frz {
    #[inline(always)]
    fn from(val: u8) -> Frz {
        Frz::from_bits(val)
    }
}
impl From<Frz> for u8 {
    #[inline(always)]
    fn from(val: Frz) -> u8 {
        Frz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frzack {
    #[doc = "Not in Freeze mode, prescaler running."]
    FreezeModeNo = 0x0,
    #[doc = "In Freeze mode, prescaler stopped."]
    FreezeModeYes = 0x01,
}
impl Frzack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frzack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frzack {
    #[inline(always)]
    fn from(val: u8) -> Frzack {
        Frzack::from_bits(val)
    }
}
impl From<Frzack> for u8 {
    #[inline(always)]
    fn from(val: Frzack) -> u8 {
        Frzack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Halt {
    #[doc = "No request."]
    HaltDisable = 0x0,
    #[doc = "Enter Freeze mode, if MCR\\[FRZ\\] = 1."]
    HaltEnable = 0x01,
}
impl Halt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Halt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Halt {
    #[inline(always)]
    fn from(val: u8) -> Halt {
        Halt::from_bits(val)
    }
}
impl From<Halt> for u8 {
    #[inline(always)]
    fn from(val: Halt) -> u8 {
        Halt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idam {
    #[doc = "Format A: One full ID (standard and extended) per ID filter table element."]
    OneFullId = 0x0,
    #[doc = "Format B: Two full standard IDs or two partial 14-bit (standard and extended) IDs per ID filter table element."]
    TwoFullId = 0x01,
    #[doc = "Format C: Four partial 8-bit standard IDs per ID filter table element."]
    FourPartialId = 0x02,
    #[doc = "Format D: All frames rejected."]
    AllFramesRejected = 0x03,
}
impl Idam {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idam {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idam {
    #[inline(always)]
    fn from(val: u8) -> Idam {
        Idam::from_bits(val)
    }
}
impl From<Idam> for u8 {
    #[inline(always)]
    fn from(val: Idam) -> u8 {
        Idam::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ide {
    #[doc = "Standard."]
    Standard = 0x0,
    #[doc = "Extended."]
    Extended = 0x01,
}
impl Ide {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ide {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ide {
    #[inline(always)]
    fn from(val: u8) -> Ide {
        Ide::from_bits(val)
    }
}
impl From<Ide> for u8 {
    #[inline(always)]
    fn from(val: Ide) -> u8 {
        Ide::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IdeMsk {
    #[doc = "The corresponding bit in the filter is \"don't care.\"."]
    FrameFormatNo = 0x0,
    #[doc = "The corresponding bit in the filter is checked."]
    FrameFormatYes = 0x01,
}
impl IdeMsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdeMsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdeMsk {
    #[inline(always)]
    fn from(val: u8) -> IdeMsk {
        IdeMsk::from_bits(val)
    }
}
impl From<IdeMsk> for u8 {
    #[inline(always)]
    fn from(val: IdeMsk) -> u8 {
        IdeMsk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idfs {
    #[doc = "Match ID contents to an exact target value."]
    MatchExact = 0x0,
    #[doc = "Match an ID value greater than or equal to a specified target value."]
    MatchGte = 0x01,
    #[doc = "Match an ID value smaller than or equal to a specified target value."]
    MatchLte = 0x02,
    #[doc = "Match an ID value within a range of values, inclusive."]
    MatchRange = 0x03,
}
impl Idfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idfs {
    #[inline(always)]
    fn from(val: u8) -> Idfs {
        Idfs::from_bits(val)
    }
}
impl From<Idfs> for u8 {
    #[inline(always)]
    fn from(val: Idfs) -> u8 {
        Idfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idle {
    #[doc = "Not IDLE."]
    CanBusNotIdle = 0x0,
    #[doc = "IDLE."]
    CanBusIdle = 0x01,
}
impl Idle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idle {
    #[inline(always)]
    fn from(val: u8) -> Idle {
        Idle::from_bits(val)
    }
}
impl From<Idle> for u8 {
    #[inline(always)]
    fn from(val: Idle) -> u8 {
        Idle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Imb {
    #[doc = "Message buffer indicated by ESR2\\[LPTM\\] is not inactive."]
    InactiveMailboxNo = 0x0,
    #[doc = "At least one message buffer is inactive."]
    InactiveMailboxYes = 0x01,
}
impl Imb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Imb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Imb {
    #[inline(always)]
    fn from(val: u8) -> Imb {
        Imb::from_bits(val)
    }
}
impl From<Imb> for u8 {
    #[inline(always)]
    fn from(val: Imb) -> u8 {
        Imb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irmq {
    #[doc = "Disable."]
    IndividualRxMaskingDisabled = 0x0,
    #[doc = "Enable."]
    IndividualRxMaskingEnabled = 0x01,
}
impl Irmq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irmq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irmq {
    #[inline(always)]
    fn from(val: u8) -> Irmq {
        Irmq::from_bits(val)
    }
}
impl From<Irmq> for u8 {
    #[inline(always)]
    fn from(val: Irmq) -> u8 {
        Irmq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isocanfden {
    #[doc = "Disable."]
    NonIso = 0x0,
    #[doc = "Enable."]
    Iso = 0x01,
}
impl Isocanfden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isocanfden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isocanfden {
    #[inline(always)]
    fn from(val: u8) -> Isocanfden {
        Isocanfden::from_bits(val)
    }
}
impl From<Isocanfden> for u8 {
    #[inline(always)]
    fn from(val: Isocanfden) -> u8 {
        Isocanfden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lbuf {
    #[doc = "Buffer with highest priority is transmitted first."]
    HighestBufferFirst = 0x0,
    #[doc = "Lowest number buffer is transmitted first."]
    LowestBufferFirst = 0x01,
}
impl Lbuf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lbuf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lbuf {
    #[inline(always)]
    fn from(val: u8) -> Lbuf {
        Lbuf::from_bits(val)
    }
}
impl From<Lbuf> for u8 {
    #[inline(always)]
    fn from(val: Lbuf) -> u8 {
        Lbuf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lom {
    #[doc = "Listen-Only mode is deactivated."]
    ListenOnlyModeDisabled = 0x0,
    #[doc = "FlexCAN module operates in Listen-Only mode."]
    ListenOnlyModeEnabled = 0x01,
}
impl Lom {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lom {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lom {
    #[inline(always)]
    fn from(val: u8) -> Lom {
        Lom::from_bits(val)
    }
}
impl From<Lom> for u8 {
    #[inline(always)]
    fn from(val: Lom) -> u8 {
        Lom::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpb {
    #[doc = "Disabled."]
    LoopbackDisabled = 0x0,
    #[doc = "Enabled."]
    LoopbackEnabled = 0x01,
}
impl Lpb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpb {
    #[inline(always)]
    fn from(val: u8) -> Lpb {
        Lpb::from_bits(val)
    }
}
impl From<Lpb> for u8 {
    #[inline(always)]
    fn from(val: Lpb) -> u8 {
        Lpb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpmack {
    #[doc = "Not in a low-power mode."]
    LowPowerNo = 0x0,
    #[doc = "In a low-power mode."]
    LowPowerYes = 0x01,
}
impl Lpmack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpmack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpmack {
    #[inline(always)]
    fn from(val: u8) -> Lpmack {
        Lpmack::from_bits(val)
    }
}
impl From<Lpmack> for u8 {
    #[inline(always)]
    fn from(val: Lpmack) -> u8 {
        Lpmack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lprioen {
    #[doc = "Disable."]
    LocalPriorityDisabled = 0x0,
    #[doc = "Enable."]
    LocalPriorityEnabled = 0x01,
}
impl Lprioen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lprioen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lprioen {
    #[inline(always)]
    fn from(val: u8) -> Lprioen {
        Lprioen::from_bits(val)
    }
}
impl From<Lprioen> for u8 {
    #[inline(always)]
    fn from(val: Lprioen) -> u8 {
        Lprioen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbdsr0 {
    #[doc = "8 bytes."]
    R08Bytes = 0x0,
    #[doc = "16 bytes."]
    R016Bytes = 0x01,
    #[doc = "32 bytes."]
    R032Bytes = 0x02,
    #[doc = "64 bytes."]
    R064Bytes = 0x03,
}
impl Mbdsr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbdsr0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbdsr0 {
    #[inline(always)]
    fn from(val: u8) -> Mbdsr0 {
        Mbdsr0::from_bits(val)
    }
}
impl From<Mbdsr0> for u8 {
    #[inline(always)]
    fn from(val: Mbdsr0) -> u8 {
        Mbdsr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mdis {
    #[doc = "Enable."]
    FlexcanEnabled = 0x0,
    #[doc = "Disable."]
    FlexcanDisabled = 0x01,
}
impl Mdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mdis {
    #[inline(always)]
    fn from(val: u8) -> Mdis {
        Mdis::from_bits(val)
    }
}
impl From<Mdis> for u8 {
    #[inline(always)]
    fn from(val: Mdis) -> u8 {
        Mdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mrp {
    #[doc = "Matching starts from Legacy RX FIFO or Enhanced RX FIFO and continues on message buffers."]
    Id1 = 0x0,
    #[doc = "Matching starts from message buffers and continues on Legacy RX FIFO or Enhanced RX FIFO."]
    Id3 = 0x01,
}
impl Mrp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrp {
    #[inline(always)]
    fn from(val: u8) -> Mrp {
        Mrp::from_bits(val)
    }
}
impl From<Mrp> for u8 {
    #[inline(always)]
    fn from(val: Mrp) -> u8 {
        Mrp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Nmatch(u8);
impl Nmatch {
    #[doc = "Once."]
    pub const Match1: Self = Self(0x01);
    #[doc = "Twice."]
    pub const Match2: Self = Self(0x02);
    #[doc = "255 times."]
    pub const Match255: Self = Self(0xff);
}
impl Nmatch {
    pub const fn from_bits(val: u8) -> Nmatch {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Nmatch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Match1"),
            0x02 => f.write_str("Match2"),
            0xff => f.write_str("Match255"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nmatch {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Match1"),
            0x02 => defmt::write!(f, "Match2"),
            0xff => defmt::write!(f, "Match255"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Nmatch {
    #[inline(always)]
    fn from(val: u8) -> Nmatch {
        Nmatch::from_bits(val)
    }
}
impl From<Nmatch> for u8 {
    #[inline(always)]
    fn from(val: Nmatch) -> u8 {
        Nmatch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Notrdy {
    #[doc = "FlexCAN is in Normal mode, Listen-Only mode, or Loopback mode."]
    Id1 = 0x0,
    #[doc = "FlexCAN is in Disable mode, Stop mode, or Freeze mode."]
    Id4 = 0x01,
}
impl Notrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Notrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Notrdy {
    #[inline(always)]
    fn from(val: u8) -> Notrdy {
        Notrdy::from_bits(val)
    }
}
impl From<Notrdy> for u8 {
    #[inline(always)]
    fn from(val: Notrdy) -> u8 {
        Notrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Plfs {
    #[doc = "Match payload contents to an exact target value."]
    MatchExact = 0x0,
    #[doc = "Match a payload value greater than or equal to a specified target value."]
    MatchGte = 0x01,
    #[doc = "Match a payload value smaller than or equal to a specified target value."]
    MatchLte = 0x02,
    #[doc = "Match upon a payload value within a range of values, inclusive."]
    MatchRange = 0x03,
}
impl Plfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Plfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Plfs {
    #[inline(always)]
    fn from(val: u8) -> Plfs {
        Plfs::from_bits(val)
    }
}
impl From<Plfs> for u8 {
    #[inline(always)]
    fn from(val: Plfs) -> u8 {
        Plfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PnetEn {
    #[doc = "Disable."]
    PnDisabled = 0x0,
    #[doc = "Enable."]
    PnEnabled = 0x01,
}
impl PnetEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PnetEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PnetEn {
    #[inline(always)]
    fn from(val: u8) -> PnetEn {
        PnetEn::from_bits(val)
    }
}
impl From<PnetEn> for u8 {
    #[inline(always)]
    fn from(val: PnetEn) -> u8 {
        PnetEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rfen {
    #[doc = "Disable."]
    Id1 = 0x0,
    #[doc = "Enable."]
    Id2 = 0x01,
}
impl Rfen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rfen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rfen {
    #[inline(always)]
    fn from(val: u8) -> Rfen {
        Rfen::from_bits(val)
    }
}
impl From<Rfen> for u8 {
    #[inline(always)]
    fn from(val: Rfen) -> u8 {
        Rfen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rrs {
    #[doc = "Generated."]
    RemoteResponseFrameNotGenerated = 0x0,
    #[doc = "Stored."]
    RemoteResponseFrameGenerated = 0x01,
}
impl Rrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rrs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rrs {
    #[inline(always)]
    fn from(val: u8) -> Rrs {
        Rrs::from_bits(val)
    }
}
impl From<Rrs> for u8 {
    #[inline(always)]
    fn from(val: Rrs) -> u8 {
        Rrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtrMsk {
    #[doc = "The corresponding bit in the filter is \"don't care.\"."]
    FrameTypeNo = 0x0,
    #[doc = "The corresponding bit in the filter is checked."]
    FrameTypeYes = 0x01,
}
impl RtrMsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtrMsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtrMsk {
    #[inline(always)]
    fn from(val: u8) -> RtrMsk {
        RtrMsk::from_bits(val)
    }
}
impl From<RtrMsk> for u8 {
    #[inline(always)]
    fn from(val: RtrMsk) -> u8 {
        RtrMsk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwrnint {
    #[doc = "No such occurrence."]
    RxWarningIntNo = 0x0,
    #[doc = "RX error counter changed from less than 96 to greater than or equal to 96."]
    RxWarningIntYes = 0x01,
}
impl Rwrnint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwrnint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwrnint {
    #[inline(always)]
    fn from(val: u8) -> Rwrnint {
        Rwrnint::from_bits(val)
    }
}
impl From<Rwrnint> for u8 {
    #[inline(always)]
    fn from(val: Rwrnint) -> u8 {
        Rwrnint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwrnmsk {
    #[doc = "Disabled."]
    RxWarningIntDisabled = 0x0,
    #[doc = "Enabled."]
    RxWarningIntEnabled = 0x01,
}
impl Rwrnmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwrnmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwrnmsk {
    #[inline(always)]
    fn from(val: u8) -> Rwrnmsk {
        Rwrnmsk::from_bits(val)
    }
}
impl From<Rwrnmsk> for u8 {
    #[inline(always)]
    fn from(val: Rwrnmsk) -> u8 {
        Rwrnmsk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxwrn {
    #[doc = "No such occurrence."]
    RxerrcntLt96 = 0x0,
    #[doc = "RXERRCNT is greater than or equal to 96."]
    RxerrcntGte96 = 0x01,
}
impl Rxwrn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxwrn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxwrn {
    #[inline(always)]
    fn from(val: u8) -> Rxwrn {
        Rxwrn::from_bits(val)
    }
}
impl From<Rxwrn> for u8 {
    #[inline(always)]
    fn from(val: Rxwrn) -> u8 {
        Rxwrn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slfwak {
    #[doc = "Disable."]
    SelfWakeupDisabled = 0x0,
    #[doc = "Enable."]
    SelfWakeupEnabled = 0x01,
}
impl Slfwak {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slfwak {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slfwak {
    #[inline(always)]
    fn from(val: u8) -> Slfwak {
        Slfwak::from_bits(val)
    }
}
impl From<Slfwak> for u8 {
    #[inline(always)]
    fn from(val: Slfwak) -> u8 {
        Slfwak::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smp {
    #[doc = "One sample is used to determine the bit value."]
    OneSample = 0x0,
    #[doc = "Three samples are used to determine the value of the received bit: the regular one (sample point) and two preceding samples. A majority rule is used."]
    ThreeSample = 0x01,
}
impl Smp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smp {
    #[inline(always)]
    fn from(val: u8) -> Smp {
        Smp::from_bits(val)
    }
}
impl From<Smp> for u8 {
    #[inline(always)]
    fn from(val: Smp) -> u8 {
        Smp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Softrst {
    #[doc = "No reset."]
    SoftrstNoResetRequest = 0x0,
    #[doc = "Soft reset affects reset registers."]
    SoftrstResetRegisters = 0x01,
}
impl Softrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Softrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Softrst {
    #[inline(always)]
    fn from(val: u8) -> Softrst {
        Softrst::from_bits(val)
    }
}
impl From<Softrst> for u8 {
    #[inline(always)]
    fn from(val: Softrst) -> u8 {
        Softrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srr {
    #[doc = "Dominant."]
    Dominant = 0x0,
    #[doc = "Recessive."]
    Recessive = 0x01,
}
impl Srr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srr {
    #[inline(always)]
    fn from(val: u8) -> Srr {
        Srr::from_bits(val)
    }
}
impl From<Srr> for u8 {
    #[inline(always)]
    fn from(val: Srr) -> u8 {
        Srr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srxdis {
    #[doc = "Enable."]
    SelfReceptionEnabled = 0x0,
    #[doc = "Disable."]
    SelfReceptionDisabled = 0x01,
}
impl Srxdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srxdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srxdis {
    #[inline(always)]
    fn from(val: u8) -> Srxdis {
        Srxdis::from_bits(val)
    }
}
impl From<Srxdis> for u8 {
    #[inline(always)]
    fn from(val: Srxdis) -> u8 {
        Srxdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stferr {
    #[doc = "No error."]
    StuffingErrorNo = 0x0,
    #[doc = "Error occurred since last read of this register."]
    StuffingErrorYes = 0x01,
}
impl Stferr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stferr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stferr {
    #[inline(always)]
    fn from(val: u8) -> Stferr {
        Stferr::from_bits(val)
    }
}
impl From<Stferr> for u8 {
    #[inline(always)]
    fn from(val: Stferr) -> u8 {
        Stferr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StferrFast {
    #[doc = "No such occurrence."]
    StuffingErrorNo = 0x0,
    #[doc = "A stuffing error occurred since last read of this register."]
    StuffingErrorYes = 0x01,
}
impl StferrFast {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StferrFast {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StferrFast {
    #[inline(always)]
    fn from(val: u8) -> StferrFast {
        StferrFast::from_bits(val)
    }
}
impl From<StferrFast> for u8 {
    #[inline(always)]
    fn from(val: StferrFast) -> u8 {
        StferrFast::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Synch {
    #[doc = "Not synchronized."]
    CanBusSyncNo = 0x0,
    #[doc = "Synchronized."]
    CanBusSyncYes = 0x01,
}
impl Synch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Synch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Synch {
    #[inline(always)]
    fn from(val: u8) -> Synch {
        Synch::from_bits(val)
    }
}
impl From<Synch> for u8 {
    #[inline(always)]
    fn from(val: Synch) -> u8 {
        Synch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdcfail {
    #[doc = "In range."]
    InRange = 0x0,
    #[doc = "Out of range."]
    OutOfRange = 0x01,
}
impl Tdcfail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdcfail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdcfail {
    #[inline(always)]
    fn from(val: u8) -> Tdcfail {
        Tdcfail::from_bits(val)
    }
}
impl From<Tdcfail> for u8 {
    #[inline(always)]
    fn from(val: Tdcfail) -> u8 {
        Tdcfail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdmdis {
    #[doc = "Enable."]
    Enable = 0x0,
    #[doc = "Disable."]
    Disable = 0x01,
}
impl Tdmdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdmdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdmdis {
    #[inline(always)]
    fn from(val: u8) -> Tdmdis {
        Tdmdis::from_bits(val)
    }
}
impl From<Tdmdis> for u8 {
    #[inline(always)]
    fn from(val: Tdmdis) -> u8 {
        Tdmdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsyn {
    #[doc = "Disable."]
    TimerSyncDisabled = 0x0,
    #[doc = "Enable."]
    TimerSyncEnabled = 0x01,
}
impl Tsyn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsyn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsyn {
    #[inline(always)]
    fn from(val: u8) -> Tsyn {
        Tsyn::from_bits(val)
    }
}
impl From<Tsyn> for u8 {
    #[inline(always)]
    fn from(val: Tsyn) -> u8 {
        Tsyn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Twrnint {
    #[doc = "No such occurrence."]
    TxWarningIntNo = 0x0,
    #[doc = "TX error counter changed from less than 96 to greater than or equal to 96."]
    TxWarningIntYes = 0x01,
}
impl Twrnint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Twrnint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Twrnint {
    #[inline(always)]
    fn from(val: u8) -> Twrnint {
        Twrnint::from_bits(val)
    }
}
impl From<Twrnint> for u8 {
    #[inline(always)]
    fn from(val: Twrnint) -> u8 {
        Twrnint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Twrnmsk {
    #[doc = "Disabled."]
    TxWarningIntDisabled = 0x0,
    #[doc = "Enabled."]
    TxWarningIntEnabled = 0x01,
}
impl Twrnmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Twrnmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Twrnmsk {
    #[inline(always)]
    fn from(val: u8) -> Twrnmsk {
        Twrnmsk::from_bits(val)
    }
}
impl From<Twrnmsk> for u8 {
    #[inline(always)]
    fn from(val: Twrnmsk) -> u8 {
        Twrnmsk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tx {
    #[doc = "Not transmitting."]
    TransmitMessageNo = 0x0,
    #[doc = "Transmitting."]
    TransmitMessageYes = 0x01,
}
impl Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tx {
    #[inline(always)]
    fn from(val: u8) -> Tx {
        Tx::from_bits(val)
    }
}
impl From<Tx> for u8 {
    #[inline(always)]
    fn from(val: Tx) -> u8 {
        Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txwrn {
    #[doc = "No such occurrence."]
    TxerrcntLt96 = 0x0,
    #[doc = "TXERRCNT is 96 or greater."]
    TxerrcntGte96 = 0x01,
}
impl Txwrn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txwrn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txwrn {
    #[inline(always)]
    fn from(val: u8) -> Txwrn {
        Txwrn::from_bits(val)
    }
}
impl From<Txwrn> for u8 {
    #[inline(always)]
    fn from(val: Txwrn) -> u8 {
        Txwrn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wakmsk {
    #[doc = "Disabled."]
    WakeupInterruptDisabled = 0x0,
    #[doc = "Enabled."]
    WakeupInterruptEnabled = 0x01,
}
impl Wakmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wakmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wakmsk {
    #[inline(always)]
    fn from(val: u8) -> Wakmsk {
        Wakmsk::from_bits(val)
    }
}
impl From<Wakmsk> for u8 {
    #[inline(always)]
    fn from(val: Wakmsk) -> u8 {
        Wakmsk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wrnen {
    #[doc = "Disable."]
    TwrnintRwrnintInactive = 0x0,
    #[doc = "Enable."]
    TwrnintRwrnintActive = 0x01,
}
impl Wrnen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wrnen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wrnen {
    #[inline(always)]
    fn from(val: u8) -> Wrnen {
        Wrnen::from_bits(val)
    }
}
impl From<Wrnen> for u8 {
    #[inline(always)]
    fn from(val: Wrnen) -> u8 {
        Wrnen::to_bits(val)
    }
}
