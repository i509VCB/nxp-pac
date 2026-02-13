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
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_14 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_14(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_13(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_12(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_11(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_10(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_09(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_08(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_07(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_06(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_05(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_04(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_03(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_02(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_01(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_00(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_14 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_14(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_13(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_12(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_11(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_10(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_09(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_08(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_07(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_06(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_05(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_04(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_03(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_02(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_01(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_00(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_13(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_12(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_11(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_10(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_09(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_08(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_07(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_06(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_05(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_04(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_03(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_02(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_01(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_00(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_14 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_14(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_13(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_12(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_11(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_10(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_09(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_08(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_07(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_06(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_05(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_04(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_03(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_02(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_01(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_00(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_14 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_14(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_13(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_12(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_11(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_10(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_09(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_08(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_07(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_06(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_05(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_04(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_03(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_02(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_01(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_00(
        self,
    ) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_13(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_12(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_11(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_10(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_09(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_08(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_07(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_06(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_05(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_04(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_03(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_02(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_01(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_00(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "USB_OTG_ID_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usb_otg_id_select_input(
        self,
    ) -> crate::common::Reg<regs::UsbOtgIdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMA_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwma_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1PwmaSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMA_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwma_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1PwmaSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMA_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwma_select_input_2(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1PwmaSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "FLEXPWM1_PWMA_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwma_select_input_3(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1PwmaSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMB_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwmb_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1PwmbSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMB_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwmb_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1PwmbSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMB_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwmb_select_input_2(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1PwmbSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "FLEXPWM1_PWMB_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwmb_select_input_3(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1PwmbSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "FLEXSPI_DQS_FA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi_dqs_fa_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspiDqsFaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "FLEXSPI_DQS_FB_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi_dqs_fb_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspiDqsFbSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "KPP_COL_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_col_select_input_0(
        self,
    ) -> crate::common::Reg<regs::KppColSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "KPP_COL_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_col_select_input_1(
        self,
    ) -> crate::common::Reg<regs::KppColSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "KPP_COL_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_col_select_input_2(
        self,
    ) -> crate::common::Reg<regs::KppColSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "KPP_COL_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_col_select_input_3(
        self,
    ) -> crate::common::Reg<regs::KppColSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "KPP_ROW_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_row_select_input_0(
        self,
    ) -> crate::common::Reg<regs::KppRowSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "KPP_ROW_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_row_select_input_1(
        self,
    ) -> crate::common::Reg<regs::KppRowSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "KPP_ROW_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_row_select_input_2(
        self,
    ) -> crate::common::Reg<regs::KppRowSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "KPP_ROW_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_row_select_input_3(
        self,
    ) -> crate::common::Reg<regs::KppRowSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "LPI2C1_HREQ_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c1_hreq_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c1HreqSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "LPI2C1_SCL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c1_scl_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c1SclSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "LPI2C1_SDA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c1_sda_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c1SdaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "LPI2C2_SCL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c2_scl_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c2SclSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "LPI2C2_SDA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c2_sda_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c2SdaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "LPSPI1_PCS_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_pcs_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Lpspi1PcsSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "LPSPI1_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi1SckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "LPSPI1_SDI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_sdi_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi1SdiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "LPSPI1_SDO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_sdo_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi1SdoSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "LPSPI2_PCS_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_pcs_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Lpspi2PcsSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "LPSPI2_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi2SckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "LPSPI2_SDI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_sdi_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi2SdiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "LPSPI2_SDO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_sdo_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi2SdoSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "LPUART1_RXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart1_rxd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart1RxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "LPUART1_TXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart1_txd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart1TxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "LPUART2_RXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart2_rxd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart2RxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "LPUART2_TXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart2_txd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart2TxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "LPUART3_RXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart3_rxd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart3RxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "LPUART3_TXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart3_txd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart3TxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "LPUART4_RXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart4_rxd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart4RxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "LPUART4_TXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart4_txd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart4TxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "NMI_GLUE_NMI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn nmi_glue_nmi_select_input(
        self,
    ) -> crate::common::Reg<regs::NmiGlueNmiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "SPDIF_IN1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn spdif_in1_select_input(
        self,
    ) -> crate::common::Reg<regs::SpdifIn1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "SPDIF_TX_CLK2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn spdif_tx_clk2_select_input(
        self,
    ) -> crate::common::Reg<regs::SpdifTxClk2SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "USB_OTG_OC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usb_otg_oc_select_input(
        self,
    ) -> crate::common::Reg<regs::UsbOtgOcSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "XEV_GLUE_RXEV_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xev_glue_rxev_select_input(
        self,
    ) -> crate::common::Reg<regs::XevGlueRxevSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
}
pub mod regs;
pub mod vals;
