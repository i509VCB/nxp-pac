#[doc = "IOMUXC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iomuxc {
    ptr: *mut u8,
}
unsafe impl Send for Iomuxc {}
unsafe impl Sync for Iomuxc {}
impl Iomuxc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_00(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_01(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_02(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_03(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_04(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_05(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_06(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_07(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_08(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_09(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_10(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_11(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_12(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_13(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_14 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_14(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_15 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_15(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_16 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_16(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_17 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_17(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_18 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_18(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_19 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_19(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_20 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_20(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_21 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_21(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_22 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_22(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_23 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_23(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_24 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_24(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_25 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_25(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_26 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_26(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_27 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_27(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_28 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_28(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_29 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_29(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_30 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_30(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_31 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_31(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_32 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_32(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_33 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_33(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_34 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_34(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_35 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_35(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_36 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_36(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_37 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_37(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_38 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_38(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_39 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_39(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_40 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_40(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_41 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_41(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_00(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_01(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_02(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_03(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_04(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_05(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_06(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_07(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_08(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_09(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_10(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_11(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_12(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_13(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_14 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_14(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_15 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_15(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_00(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_01(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_02(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_03(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_04(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_05(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_06(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_07(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_08(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_09(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_10(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_11(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_12(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_13(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_14 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_14(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_15 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_15(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b0_00(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b0_01(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b0_02(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b0_03(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b0_04(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b0_05(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b0_06(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b0_07(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b0_08(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b0_09(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b0_10(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b0_11(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b0_12(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b0_13(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_14 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b0_14(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_15 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b0_15(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_00(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_01(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_02(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_03(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_04(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_05(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_06(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_07(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_08(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_09(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_10(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_11(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_12(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_13(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_14 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_14(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_15 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_15(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b0_00(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b0_01(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b0_02(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b0_03(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b0_04(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b0_05(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_00(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_01(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_02(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_03(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_04(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_05(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_06(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_07(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_08(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_09(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_10(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_11(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_00(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_01(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_02(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_03(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_04(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_05(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_06(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_07(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_08(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_09(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_10(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_11(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_12(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_13(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_14 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_14(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_15 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_15(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_16 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_16(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_17 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_17(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_18 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_18(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_19 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_19(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_20 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_20(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_21 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_21(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_22 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_22(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_23 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_23(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_24 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_24(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_25 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_25(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_26 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_26(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_27 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_27(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_28 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_28(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_29 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_29(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0278usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_30 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_30(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x027cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_31 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_31(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_32 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_32(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_33 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_33(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_34 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_34(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x028cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_35 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_35(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_36 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_36(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_37 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_37(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0298usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_38 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_38(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x029cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_39 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_39(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_40 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_40(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_41 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_41(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_00(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02acusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_01(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_02(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_03(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_04(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02bcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_05(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_06(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_07(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_08(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ccusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_09(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_10(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_11(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_12(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02dcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_13(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_14 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_14(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_15 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_15(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_00(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ecusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_01(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_02(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_03(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_04(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02fcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_05(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_06(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_07(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_08(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_09(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_10(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0314usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_11(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_12(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x031cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_13(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_14 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_14(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0324usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_15 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_15(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0328usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_00(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x032cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_01(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_02(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0334usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_03(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0338usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_04(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x033cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_05(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0340usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_06(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0344usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_07(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0348usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_08(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x034cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_09(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0350usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_10(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0354usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_11(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0358usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_12(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x035cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_13(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_14 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_14(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0364usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_15 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_15(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0368usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_00(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x036cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_01(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0370usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_02(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0374usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_03(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0378usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_04(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x037cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_05(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_06(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0384usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_07(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_08(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_09(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0390usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_10(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0394usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_11(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0398usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_12(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x039cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_13(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_14 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_14(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_15 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_15(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b0_00(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03acusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b0_01(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b0_02(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b0_03(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b0_04(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03bcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b0_05(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_00(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_01(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_02(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ccusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_03(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_04(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_05(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_06(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03dcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_07(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_08(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_09(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_10(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_11(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "ANATOP_USB_OTG1_ID_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn anatop_usb_otg1_id_select_input(
        self,
    ) -> crate::common::Reg<regs::AnatopUsbOtg1IdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "ANATOP_USB_OTG2_ID_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn anatop_usb_otg2_id_select_input(
        self,
    ) -> crate::common::Reg<regs::AnatopUsbOtg2IdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "CCM_PMIC_READY_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn ccm_pmic_ready_select_input(
        self,
    ) -> crate::common::Reg<regs::CcmPmicReadySelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "CSI_DATA02_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_data02_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiData02SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "CSI_DATA03_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_data03_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiData03SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "CSI_DATA04_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_data04_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiData04SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "CSI_DATA05_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_data05_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiData05SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "CSI_DATA06_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_data06_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiData06SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "CSI_DATA07_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_data07_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiData07SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "CSI_DATA08_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_data08_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiData08SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    #[doc = "CSI_DATA09_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_data09_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiData09SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
    }
    #[doc = "CSI_HSYNC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_hsync_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiHsyncSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "CSI_PIXCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_pixclk_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiPixclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "CSI_VSYNC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_vsync_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiVsyncSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0428usize) as _) }
    }
    #[doc = "ENET_IPG_CLK_RMII_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet_ipg_clk_rmii_select_input(
        self,
    ) -> crate::common::Reg<regs::EnetIpgClkRmiiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x042cusize) as _) }
    }
    #[doc = "ENET_MDIO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet_mdio_select_input(
        self,
    ) -> crate::common::Reg<regs::EnetMdioSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0430usize) as _) }
    }
    #[doc = "ENET0_RXDATA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet0_rxdata_select_input(
        self,
    ) -> crate::common::Reg<regs::Enet0RxdataSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0434usize) as _) }
    }
    #[doc = "ENET1_RXDATA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet1_rxdata_select_input(
        self,
    ) -> crate::common::Reg<regs::Enet1RxdataSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0438usize) as _) }
    }
    #[doc = "ENET_RXEN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet_rxen_select_input(
        self,
    ) -> crate::common::Reg<regs::EnetRxenSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x043cusize) as _) }
    }
    #[doc = "ENET_RXERR_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet_rxerr_select_input(
        self,
    ) -> crate::common::Reg<regs::EnetRxerrSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "ENET0_TIMER_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet0_timer_select_input(
        self,
    ) -> crate::common::Reg<regs::Enet0TimerSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0444usize) as _) }
    }
    #[doc = "ENET_TXCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet_txclk_select_input(
        self,
    ) -> crate::common::Reg<regs::EnetTxclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0448usize) as _) }
    }
    #[doc = "FLEXCAN1_RX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexcan1_rx_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexcan1RxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x044cusize) as _) }
    }
    #[doc = "FLEXCAN2_RX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexcan2_rx_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexcan2RxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0450usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMA3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwma3_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1Pwma3SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0454usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMA0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwma0_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1Pwma0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0458usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMA1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwma1_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1Pwma1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x045cusize) as _) }
    }
    #[doc = "FLEXPWM1_PWMA2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwma2_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1Pwma2SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0460usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMB3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwmb3_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1Pwmb3SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0464usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMB0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwmb0_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1Pwmb0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0468usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMB1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwmb1_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1Pwmb1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x046cusize) as _) }
    }
    #[doc = "FLEXPWM1_PWMB2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwmb2_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1Pwmb2SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0470usize) as _) }
    }
    #[doc = "FLEXPWM2_PWMA3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_pwma3_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2Pwma3SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0474usize) as _) }
    }
    #[doc = "FLEXPWM2_PWMA0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_pwma0_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2Pwma0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0478usize) as _) }
    }
    #[doc = "FLEXPWM2_PWMA1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_pwma1_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2Pwma1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x047cusize) as _) }
    }
    #[doc = "FLEXPWM2_PWMA2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_pwma2_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2Pwma2SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0480usize) as _) }
    }
    #[doc = "FLEXPWM2_PWMB3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_pwmb3_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2Pwmb3SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0484usize) as _) }
    }
    #[doc = "FLEXPWM2_PWMB0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_pwmb0_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2Pwmb0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0488usize) as _) }
    }
    #[doc = "FLEXPWM2_PWMB1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_pwmb1_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2Pwmb1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x048cusize) as _) }
    }
    #[doc = "FLEXPWM2_PWMB2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_pwmb2_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2Pwmb2SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0490usize) as _) }
    }
    #[doc = "FLEXPWM4_PWMA0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm4_pwma0_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm4Pwma0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0494usize) as _) }
    }
    #[doc = "FLEXPWM4_PWMA1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm4_pwma1_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm4Pwma1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0498usize) as _) }
    }
    #[doc = "FLEXPWM4_PWMA2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm4_pwma2_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm4Pwma2SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x049cusize) as _) }
    }
    #[doc = "FLEXPWM4_PWMA3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm4_pwma3_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm4Pwma3SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a0usize) as _) }
    }
    #[doc = "FLEXSPIA_DQS_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspia_dqs_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspiaDqsSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a4usize) as _) }
    }
    #[doc = "FLEXSPIA_DATA0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspia_data0_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspiaData0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a8usize) as _) }
    }
    #[doc = "FLEXSPIA_DATA1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspia_data1_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspiaData1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04acusize) as _) }
    }
    #[doc = "FLEXSPIA_DATA2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspia_data2_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspiaData2SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b0usize) as _) }
    }
    #[doc = "FLEXSPIA_DATA3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspia_data3_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspiaData3SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b4usize) as _) }
    }
    #[doc = "FLEXSPIB_DATA0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspib_data0_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspibData0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b8usize) as _) }
    }
    #[doc = "FLEXSPIB_DATA1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspib_data1_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspibData1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04bcusize) as _) }
    }
    #[doc = "FLEXSPIB_DATA2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspib_data2_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspibData2SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c0usize) as _) }
    }
    #[doc = "FLEXSPIB_DATA3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspib_data3_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspibData3SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c4usize) as _) }
    }
    #[doc = "FLEXSPIA_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspia_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspiaSckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c8usize) as _) }
    }
    #[doc = "LPI2C1_SCL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c1_scl_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c1SclSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ccusize) as _) }
    }
    #[doc = "LPI2C1_SDA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c1_sda_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c1SdaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d0usize) as _) }
    }
    #[doc = "LPI2C2_SCL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c2_scl_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c2SclSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d4usize) as _) }
    }
    #[doc = "LPI2C2_SDA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c2_sda_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c2SdaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d8usize) as _) }
    }
    #[doc = "LPI2C3_SCL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c3_scl_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c3SclSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04dcusize) as _) }
    }
    #[doc = "LPI2C3_SDA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c3_sda_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c3SdaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e0usize) as _) }
    }
    #[doc = "LPI2C4_SCL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c4_scl_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c4SclSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e4usize) as _) }
    }
    #[doc = "LPI2C4_SDA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c4_sda_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c4SdaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e8usize) as _) }
    }
    #[doc = "LPSPI1_PCS0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_pcs0_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi1Pcs0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ecusize) as _) }
    }
    #[doc = "LPSPI1_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi1SckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f0usize) as _) }
    }
    #[doc = "LPSPI1_SDI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_sdi_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi1SdiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f4usize) as _) }
    }
    #[doc = "LPSPI1_SDO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_sdo_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi1SdoSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f8usize) as _) }
    }
    #[doc = "LPSPI2_PCS0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_pcs0_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi2Pcs0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04fcusize) as _) }
    }
    #[doc = "LPSPI2_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi2SckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "LPSPI2_SDI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_sdi_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi2SdiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "LPSPI2_SDO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_sdo_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi2SdoSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
    }
    #[doc = "LPSPI3_PCS0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi3_pcs0_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi3Pcs0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x050cusize) as _) }
    }
    #[doc = "LPSPI3_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi3_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi3SckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
    }
    #[doc = "LPSPI3_SDI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi3_sdi_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi3SdiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0514usize) as _) }
    }
    #[doc = "LPSPI3_SDO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi3_sdo_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi3SdoSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize) as _) }
    }
    #[doc = "LPSPI4_PCS0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi4_pcs0_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi4Pcs0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x051cusize) as _) }
    }
    #[doc = "LPSPI4_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi4_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi4SckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
    }
    #[doc = "LPSPI4_SDI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi4_sdi_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi4SdiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
    }
    #[doc = "LPSPI4_SDO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi4_sdo_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi4SdoSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0528usize) as _) }
    }
    #[doc = "LPUART2_RX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart2_rx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart2RxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x052cusize) as _) }
    }
    #[doc = "LPUART2_TX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart2_tx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart2TxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0530usize) as _) }
    }
    #[doc = "LPUART3_CTS_B_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart3_cts_b_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart3CtsBSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
    }
    #[doc = "LPUART3_RX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart3_rx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart3RxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0538usize) as _) }
    }
    #[doc = "LPUART3_TX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart3_tx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart3TxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x053cusize) as _) }
    }
    #[doc = "LPUART4_RX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart4_rx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart4RxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "LPUART4_TX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart4_tx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart4TxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
    }
    #[doc = "LPUART5_RX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart5_rx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart5RxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0548usize) as _) }
    }
    #[doc = "LPUART5_TX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart5_tx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart5TxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x054cusize) as _) }
    }
    #[doc = "LPUART6_RX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart6_rx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart6RxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0550usize) as _) }
    }
    #[doc = "LPUART6_TX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart6_tx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart6TxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0554usize) as _) }
    }
    #[doc = "LPUART7_RX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart7_rx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart7RxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0558usize) as _) }
    }
    #[doc = "LPUART7_TX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart7_tx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart7TxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x055cusize) as _) }
    }
    #[doc = "LPUART8_RX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart8_rx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart8RxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "LPUART8_TX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart8_tx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart8TxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0564usize) as _) }
    }
    #[doc = "NMI_GLUE_NMI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn nmi_select_input(
        self,
    ) -> crate::common::Reg<regs::NmiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0568usize) as _) }
    }
    #[doc = "QTIMER2_TIMER0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer2_timer0_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer2Timer0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x056cusize) as _) }
    }
    #[doc = "QTIMER2_TIMER1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer2_timer1_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer2Timer1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0570usize) as _) }
    }
    #[doc = "QTIMER2_TIMER2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer2_timer2_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer2Timer2SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0574usize) as _) }
    }
    #[doc = "QTIMER2_TIMER3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer2_timer3_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer2Timer3SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0578usize) as _) }
    }
    #[doc = "QTIMER3_TIMER0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer3_timer0_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer3Timer0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x057cusize) as _) }
    }
    #[doc = "QTIMER3_TIMER1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer3_timer1_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer3Timer1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    #[doc = "QTIMER3_TIMER2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer3_timer2_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer3Timer2SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0584usize) as _) }
    }
    #[doc = "QTIMER3_TIMER3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer3_timer3_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer3Timer3SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0588usize) as _) }
    }
    #[doc = "SAI1_MCLK2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_mclk2_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1Mclk2SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x058cusize) as _) }
    }
    #[doc = "SAI1_RX_BCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_rx_bclk_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1RxBclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0590usize) as _) }
    }
    #[doc = "SAI1_RX_DATA0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_rx_data0_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1RxData0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0594usize) as _) }
    }
    #[doc = "SAI1_RX_DATA1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_rx_data1_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1RxData1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0598usize) as _) }
    }
    #[doc = "SAI1_RX_DATA2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_rx_data2_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1RxData2SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x059cusize) as _) }
    }
    #[doc = "SAI1_RX_DATA3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_rx_data3_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1RxData3SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "SAI1_RX_SYNC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_rx_sync_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1RxSyncSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a4usize) as _) }
    }
    #[doc = "SAI1_TX_BCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_tx_bclk_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1TxBclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a8usize) as _) }
    }
    #[doc = "SAI1_TX_SYNC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_tx_sync_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1TxSyncSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05acusize) as _) }
    }
    #[doc = "SAI2_MCLK2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai2_mclk2_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai2Mclk2SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b0usize) as _) }
    }
    #[doc = "SAI2_RX_BCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai2_rx_bclk_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai2RxBclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b4usize) as _) }
    }
    #[doc = "SAI2_RX_DATA0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai2_rx_data0_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai2RxData0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b8usize) as _) }
    }
    #[doc = "SAI2_RX_SYNC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai2_rx_sync_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai2RxSyncSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05bcusize) as _) }
    }
    #[doc = "SAI2_TX_BCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai2_tx_bclk_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai2TxBclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "SAI2_TX_SYNC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai2_tx_sync_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai2TxSyncSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c4usize) as _) }
    }
    #[doc = "SPDIF_IN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn spdif_in_select_input(
        self,
    ) -> crate::common::Reg<regs::SpdifInSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c8usize) as _) }
    }
    #[doc = "USB_OTG2_OC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usb_otg2_oc_select_input(
        self,
    ) -> crate::common::Reg<regs::UsbOtg2OcSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ccusize) as _) }
    }
    #[doc = "USB_OTG1_OC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usb_otg1_oc_select_input(
        self,
    ) -> crate::common::Reg<regs::UsbOtg1OcSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d0usize) as _) }
    }
    #[doc = "USDHC1_CD_B_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc1_cd_b_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc1CdBSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d4usize) as _) }
    }
    #[doc = "USDHC1_WP_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc1_wp_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc1WpSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d8usize) as _) }
    }
    #[doc = "USDHC2_CLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_clk_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2ClkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05dcusize) as _) }
    }
    #[doc = "USDHC2_CD_B_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_cd_b_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2CdBSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "USDHC2_CMD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_cmd_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2CmdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e4usize) as _) }
    }
    #[doc = "USDHC2_DATA0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_data0_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2Data0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e8usize) as _) }
    }
    #[doc = "USDHC2_DATA1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_data1_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2Data1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ecusize) as _) }
    }
    #[doc = "USDHC2_DATA2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_data2_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2Data2SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f0usize) as _) }
    }
    #[doc = "USDHC2_DATA3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_data3_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2Data3SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f4usize) as _) }
    }
    #[doc = "USDHC2_DATA4_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_data4_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2Data4SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f8usize) as _) }
    }
    #[doc = "USDHC2_DATA5_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_data5_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2Data5SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05fcusize) as _) }
    }
    #[doc = "USDHC2_DATA6_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_data6_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2Data6SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "USDHC2_DATA7_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_data7_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2Data7SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0604usize) as _) }
    }
    #[doc = "USDHC2_WP_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_wp_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2WpSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0608usize) as _) }
    }
    #[doc = "XBAR1_IN02_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in02_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1In02SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x060cusize) as _) }
    }
    #[doc = "XBAR1_IN03_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in03_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1In03SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0610usize) as _) }
    }
    #[doc = "XBAR1_IN04_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in04_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1In04SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0614usize) as _) }
    }
    #[doc = "XBAR1_IN05_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in05_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1In05SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0618usize) as _) }
    }
    #[doc = "XBAR1_IN06_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in06_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1In06SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x061cusize) as _) }
    }
    #[doc = "XBAR1_IN07_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in07_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1In07SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize) as _) }
    }
    #[doc = "XBAR1_IN08_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in08_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1In08SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0624usize) as _) }
    }
    #[doc = "XBAR1_IN09_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in09_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1In09SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0628usize) as _) }
    }
    #[doc = "XBAR1_IN17_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in17_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1In17SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x062cusize) as _) }
    }
    #[doc = "XBAR1_IN18_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in18_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1In18SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0630usize) as _) }
    }
    #[doc = "XBAR1_IN20_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in20_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1In20SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0634usize) as _) }
    }
    #[doc = "XBAR1_IN22_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in22_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1In22SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0638usize) as _) }
    }
    #[doc = "XBAR1_IN23_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in23_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1In23SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x063cusize) as _) }
    }
    #[doc = "XBAR1_IN24_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in24_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1In24SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0640usize) as _) }
    }
    #[doc = "XBAR1_IN14_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in14_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1In14SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0644usize) as _) }
    }
    #[doc = "XBAR1_IN15_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in15_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1In15SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0648usize) as _) }
    }
    #[doc = "XBAR1_IN16_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in16_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1In16SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x064cusize) as _) }
    }
    #[doc = "XBAR1_IN25_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in25_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1In25SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0650usize) as _) }
    }
    #[doc = "XBAR1_IN19_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in19_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1In19SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0654usize) as _) }
    }
    #[doc = "XBAR1_IN23_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in21_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1In21SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0658usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_00(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x065cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_01(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0660usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_02(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0664usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_03(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0668usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_04(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x066cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_05(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0670usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_06(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0674usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_07(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0678usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_08(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x067cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_09(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0680usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_10(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0684usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_11(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0688usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_12(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x068cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_13(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0690usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b1_00(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0694usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b1_01(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0698usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b1_02(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x069cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b1_03(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06a0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b1_04(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06a4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b1_05(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06a8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b1_06(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06acusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b1_07(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06b0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_00(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06b4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_01(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06b8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_02(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06bcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_03(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06c0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_04(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06c4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_05(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06c8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_06(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06ccusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_07(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06d0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_08(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06d4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_09(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06d8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_10(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06dcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_11(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_12(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_13(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b1_00(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06ecusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b1_01(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06f0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b1_02(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06f4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b1_03(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06f8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b1_04(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06fcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b1_05(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b1_06(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0704usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b1_07(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0708usize) as _) }
    }
    #[doc = "ENET2_IPG_CLK_RMII_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet2_ipg_clk_rmii_select_input(
        self,
    ) -> crate::common::Reg<regs::Enet2IpgClkRmiiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x070cusize) as _) }
    }
    #[doc = "ENET2_IPP_IND_MAC0_MDIO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet2_ipp_ind_mac0_mdio_select_input(
        self,
    ) -> crate::common::Reg<regs::Enet2IppIndMac0MdioSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0710usize) as _) }
    }
    #[doc = "ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn enet2_ipp_ind_mac0_rxdata_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Enet2IppIndMac0RxdataSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0714usize) as _) }
    }
    #[doc = "ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn enet2_ipp_ind_mac0_rxdata_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Enet2IppIndMac0RxdataSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0718usize) as _) }
    }
    #[doc = "ENET2_IPP_IND_MAC0_RXEN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet2_ipp_ind_mac0_rxen_select_input(
        self,
    ) -> crate::common::Reg<regs::Enet2IppIndMac0RxenSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x071cusize) as _) }
    }
    #[doc = "ENET2_IPP_IND_MAC0_RXERR_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet2_ipp_ind_mac0_rxerr_select_input(
        self,
    ) -> crate::common::Reg<regs::Enet2IppIndMac0RxerrSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0720usize) as _) }
    }
    #[doc = "ENET2_IPP_IND_MAC0_TIMER_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn enet2_ipp_ind_mac0_timer_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Enet2IppIndMac0TimerSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0724usize) as _) }
    }
    #[doc = "ENET2_IPP_IND_MAC0_TXCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet2_ipp_ind_mac0_txclk_select_input(
        self,
    ) -> crate::common::Reg<regs::Enet2IppIndMac0TxclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0728usize) as _) }
    }
    #[doc = "GPT1_IPP_IND_CAPIN1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn gpt1_ipp_ind_capin1_select_input(
        self,
    ) -> crate::common::Reg<regs::Gpt1IppIndCapin1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0758usize) as _) }
    }
    #[doc = "GPT1_IPP_IND_CAPIN2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn gpt1_ipp_ind_capin2_select_input(
        self,
    ) -> crate::common::Reg<regs::Gpt1IppIndCapin2SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x075cusize) as _) }
    }
    #[doc = "GPT1_IPP_IND_CLKIN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn gpt1_ipp_ind_clkin_select_input(
        self,
    ) -> crate::common::Reg<regs::Gpt1IppIndClkinSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0760usize) as _) }
    }
    #[doc = "GPT2_IPP_IND_CAPIN1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn gpt2_ipp_ind_capin1_select_input(
        self,
    ) -> crate::common::Reg<regs::Gpt2IppIndCapin1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0764usize) as _) }
    }
    #[doc = "GPT2_IPP_IND_CAPIN2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn gpt2_ipp_ind_capin2_select_input(
        self,
    ) -> crate::common::Reg<regs::Gpt2IppIndCapin2SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0768usize) as _) }
    }
    #[doc = "GPT2_IPP_IND_CLKIN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn gpt2_ipp_ind_clkin_select_input(
        self,
    ) -> crate::common::Reg<regs::Gpt2IppIndClkinSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x076cusize) as _) }
    }
    #[doc = "SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn sai3_ipg_clk_sai_mclk_select_input_2(
        self,
    ) -> crate::common::Reg<regs::Sai3IpgClkSaiMclkSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0770usize) as _) }
    }
    #[doc = "SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai3_ipp_ind_sai_rxbclk_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai3IppIndSaiRxbclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0774usize) as _) }
    }
    #[doc = "SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn sai3_ipp_ind_sai_rxdata_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Sai3IppIndSaiRxdataSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0778usize) as _) }
    }
    #[doc = "SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai3_ipp_ind_sai_rxsync_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai3IppIndSaiRxsyncSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x077cusize) as _) }
    }
    #[doc = "SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai3_ipp_ind_sai_txbclk_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai3IppIndSaiTxbclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0780usize) as _) }
    }
    #[doc = "SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai3_ipp_ind_sai_txsync_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai3IppIndSaiTxsyncSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0784usize) as _) }
    }
    #[doc = "SEMC_I_IPP_IND_DQS4_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn semc_i_ipp_ind_dqs4_select_input(
        self,
    ) -> crate::common::Reg<regs::SemcIIppIndDqs4SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0788usize) as _) }
    }
    #[doc = "CANFD_IPP_IND_CANRX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn canfd_ipp_ind_canrx_select_input(
        self,
    ) -> crate::common::Reg<regs::CanfdIppIndCanrxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x078cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
