#[doc = "ADC_ETC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcEtc {
    ptr: *mut u8,
}
unsafe impl Send for AdcEtc {}
unsafe impl Sync for AdcEtc {}
impl AdcEtc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ADC_ETC Global Control Register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "ETC DONE0 and DONE1 IRQ State Register"]
    #[inline(always)]
    pub const fn done0_1_irq(self) -> crate::common::Reg<regs::Done01Irq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "ETC DONE_2, DONE_3 and DONE_ERR IRQ State Register"]
    #[inline(always)]
    pub const fn done2_3_err_irq(
        self,
    ) -> crate::common::Reg<regs::Done23ErrIrq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "ETC DMA control Register"]
    #[inline(always)]
    pub const fn dma_ctrl(self) -> crate::common::Reg<regs::DmaCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "ETC_TRIG Control Register"]
    #[inline(always)]
    pub const fn trig0_ctrl(self) -> crate::common::Reg<regs::Trig0Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "ETC_TRIG Counter Register"]
    #[inline(always)]
    pub const fn trig0_counter(self) -> crate::common::Reg<regs::Trig0Counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    #[inline(always)]
    pub const fn trig0_chain_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig0Chain10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    #[inline(always)]
    pub const fn trig0_chain_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig0Chain32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    #[inline(always)]
    pub const fn trig0_chain_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig0Chain54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    #[inline(always)]
    pub const fn trig0_chain_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig0Chain76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    #[inline(always)]
    pub const fn trig0_result_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig0Result10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    #[inline(always)]
    pub const fn trig0_result_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig0Result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    #[inline(always)]
    pub const fn trig0_result_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig0Result54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    #[inline(always)]
    pub const fn trig0_result_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig0Result76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "ETC_TRIG Control Register"]
    #[inline(always)]
    pub const fn trig1_ctrl(self) -> crate::common::Reg<regs::Trig1Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "ETC_TRIG Counter Register"]
    #[inline(always)]
    pub const fn trig1_counter(self) -> crate::common::Reg<regs::Trig1Counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    #[inline(always)]
    pub const fn trig1_chain_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig1Chain10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    #[inline(always)]
    pub const fn trig1_chain_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig1Chain32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    #[inline(always)]
    pub const fn trig1_chain_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig1Chain54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    #[inline(always)]
    pub const fn trig1_chain_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig1Chain76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    #[inline(always)]
    pub const fn trig1_result_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig1Result10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    #[inline(always)]
    pub const fn trig1_result_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig1Result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    #[inline(always)]
    pub const fn trig1_result_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig1Result54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    #[inline(always)]
    pub const fn trig1_result_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig1Result76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "ETC_TRIG Control Register"]
    #[inline(always)]
    pub const fn trig2_ctrl(self) -> crate::common::Reg<regs::Trig2Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "ETC_TRIG Counter Register"]
    #[inline(always)]
    pub const fn trig2_counter(self) -> crate::common::Reg<regs::Trig2Counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    #[inline(always)]
    pub const fn trig2_chain_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig2Chain10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    #[inline(always)]
    pub const fn trig2_chain_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig2Chain32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    #[inline(always)]
    pub const fn trig2_chain_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig2Chain54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    #[inline(always)]
    pub const fn trig2_chain_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig2Chain76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    #[inline(always)]
    pub const fn trig2_result_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig2Result10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    #[inline(always)]
    pub const fn trig2_result_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig2Result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    #[inline(always)]
    pub const fn trig2_result_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig2Result54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    #[inline(always)]
    pub const fn trig2_result_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig2Result76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "ETC_TRIG Control Register"]
    #[inline(always)]
    pub const fn trig3_ctrl(self) -> crate::common::Reg<regs::Trig3Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "ETC_TRIG Counter Register"]
    #[inline(always)]
    pub const fn trig3_counter(self) -> crate::common::Reg<regs::Trig3Counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    #[inline(always)]
    pub const fn trig3_chain_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig3Chain10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    #[inline(always)]
    pub const fn trig3_chain_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig3Chain32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    #[inline(always)]
    pub const fn trig3_chain_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig3Chain54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    #[inline(always)]
    pub const fn trig3_chain_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig3Chain76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    #[inline(always)]
    pub const fn trig3_result_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig3Result10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    #[inline(always)]
    pub const fn trig3_result_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig3Result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    #[inline(always)]
    pub const fn trig3_result_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig3Result54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    #[inline(always)]
    pub const fn trig3_result_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig3Result76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
}
pub mod regs;
pub mod vals;
