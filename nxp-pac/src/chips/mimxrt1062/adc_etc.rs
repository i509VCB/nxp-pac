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
    #[doc = "ETC DONE_2 and DONE_ERR IRQ State Register"]
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
    #[doc = "ETC_TRIG Control Register"]
    #[inline(always)]
    pub const fn trig4_ctrl(self) -> crate::common::Reg<regs::Trig4Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "ETC_TRIG Counter Register"]
    #[inline(always)]
    pub const fn trig4_counter(self) -> crate::common::Reg<regs::Trig4Counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    #[inline(always)]
    pub const fn trig4_chain_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig4Chain10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    #[inline(always)]
    pub const fn trig4_chain_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig4Chain32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    #[inline(always)]
    pub const fn trig4_chain_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig4Chain54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    #[inline(always)]
    pub const fn trig4_chain_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig4Chain76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    #[inline(always)]
    pub const fn trig4_result_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig4Result10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    #[inline(always)]
    pub const fn trig4_result_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig4Result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    #[inline(always)]
    pub const fn trig4_result_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig4Result54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    #[inline(always)]
    pub const fn trig4_result_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig4Result76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "ETC_TRIG Control Register"]
    #[inline(always)]
    pub const fn trig5_ctrl(self) -> crate::common::Reg<regs::Trig5Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "ETC_TRIG Counter Register"]
    #[inline(always)]
    pub const fn trig5_counter(self) -> crate::common::Reg<regs::Trig5Counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    #[inline(always)]
    pub const fn trig5_chain_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig5Chain10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    #[inline(always)]
    pub const fn trig5_chain_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig5Chain32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    #[inline(always)]
    pub const fn trig5_chain_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig5Chain54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    #[inline(always)]
    pub const fn trig5_chain_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig5Chain76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    #[inline(always)]
    pub const fn trig5_result_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig5Result10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    #[inline(always)]
    pub const fn trig5_result_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig5Result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    #[inline(always)]
    pub const fn trig5_result_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig5Result54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    #[inline(always)]
    pub const fn trig5_result_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig5Result76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "ETC_TRIG Control Register"]
    #[inline(always)]
    pub const fn trig6_ctrl(self) -> crate::common::Reg<regs::Trig6Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "ETC_TRIG Counter Register"]
    #[inline(always)]
    pub const fn trig6_counter(self) -> crate::common::Reg<regs::Trig6Counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    #[inline(always)]
    pub const fn trig6_chain_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig6Chain10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    #[inline(always)]
    pub const fn trig6_chain_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig6Chain32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    #[inline(always)]
    pub const fn trig6_chain_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig6Chain54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    #[inline(always)]
    pub const fn trig6_chain_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig6Chain76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    #[inline(always)]
    pub const fn trig6_result_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig6Result10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    #[inline(always)]
    pub const fn trig6_result_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig6Result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    #[inline(always)]
    pub const fn trig6_result_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig6Result54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    #[inline(always)]
    pub const fn trig6_result_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig6Result76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "ETC_TRIG Control Register"]
    #[inline(always)]
    pub const fn trig7_ctrl(self) -> crate::common::Reg<regs::Trig7Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "ETC_TRIG Counter Register"]
    #[inline(always)]
    pub const fn trig7_counter(self) -> crate::common::Reg<regs::Trig7Counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    #[inline(always)]
    pub const fn trig7_chain_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig7Chain10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    #[inline(always)]
    pub const fn trig7_chain_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig7Chain32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    #[inline(always)]
    pub const fn trig7_chain_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig7Chain54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    #[inline(always)]
    pub const fn trig7_chain_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig7Chain76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    #[inline(always)]
    pub const fn trig7_result_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig7Result10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    #[inline(always)]
    pub const fn trig7_result_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig7Result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    #[inline(always)]
    pub const fn trig7_result_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig7Result54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    #[inline(always)]
    pub const fn trig7_result_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig7Result76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
