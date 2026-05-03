#[doc = "AHB secure controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AHB_SECURE_CTRL {
    ptr: *mut u8,
}
unsafe impl Send for AHB_SECURE_CTRL {}
unsafe impl Sync for AHB_SECURE_CTRL {}
impl AHB_SECURE_CTRL {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Security access rules for Flash and ROM slaves."]
    #[inline(always)]
    pub const fn SEC_CTRL_FLASH_ROM_SLAVE_RULE(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_FLASH_ROM_SLAVE_RULE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Security access rules for FLASH sector 0 to sector 7. Each Flash sector is 32 Kbytes. There are 8 FLASH sectors in total."]
    #[inline(always)]
    pub const fn SEC_CTRL_FLASH_MEM_RULE0(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_FLASH_MEM_RULE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    #[inline(always)]
    pub const fn SEC_CTRL_ROM_MEM_RULE0(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_ROM_MEM_RULE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    #[inline(always)]
    pub const fn SEC_CTRL_ROM_MEM_RULE1(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_ROM_MEM_RULE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    #[inline(always)]
    pub const fn SEC_CTRL_ROM_MEM_RULE2(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_ROM_MEM_RULE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    #[inline(always)]
    pub const fn SEC_CTRL_ROM_MEM_RULE3(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_ROM_MEM_RULE3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Security access rules for RAMX slaves."]
    #[inline(always)]
    pub const fn SEC_CTRL_RAMX_SLAVE_RULE(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_RAMX_SLAVE_RULE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Security access rules for RAMX slaves."]
    #[inline(always)]
    pub const fn SEC_CTRL_RAMX_MEM_RULE0(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_RAMX_MEM_RULE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Security access rules for RAM0 slaves."]
    #[inline(always)]
    pub const fn SEC_CTRL_RAM0_SLAVE_RULE(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_RAM0_SLAVE_RULE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Security access rules for RAM0 slaves."]
    #[inline(always)]
    pub const fn SEC_CTRL_RAM0_MEM_RULE0(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_RAM0_MEM_RULE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Security access rules for RAM1 slaves."]
    #[inline(always)]
    pub const fn SEC_CTRL_RAM1_SLAVE_RULE(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_RAM1_SLAVE_RULE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Security access rules for RAM1 slaves."]
    #[inline(always)]
    pub const fn SEC_CTRL_RAM1_MEM_RULE0(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_RAM1_MEM_RULE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Security access rules for RAM2 slaves."]
    #[inline(always)]
    pub const fn SEC_CTRL_RAM2_SLAVE_RULE(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_RAM2_SLAVE_RULE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Security access rules for RAM2 slaves."]
    #[inline(always)]
    pub const fn SEC_CTRL_RAM2_MEM_RULE0(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_RAM2_MEM_RULE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Security access rules for USB High speed RAM slaves."]
    #[inline(always)]
    pub const fn SEC_CTRL_USB_HS_SLAVE_RULE(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_USB_HS_SLAVE_RULE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Security access rules for RAM_USB_HS."]
    #[inline(always)]
    pub const fn SEC_CTRL_USB_HS_MEM_RULE(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_USB_HS_MEM_RULE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Security access rules for both APB Bridges slaves."]
    #[inline(always)]
    pub const fn SEC_CTRL_APB_BRIDGE_SLAVE_RULE(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_APB_BRIDGE_SLAVE_RULE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
    #[inline(always)]
    pub const fn SEC_CTRL_APB_BRIDGE0_MEM_CTRL0(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_APB_BRIDGE0_MEM_CTRL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
    #[inline(always)]
    pub const fn SEC_CTRL_APB_BRIDGE0_MEM_CTRL1(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_APB_BRIDGE0_MEM_CTRL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
    #[inline(always)]
    pub const fn SEC_CTRL_APB_BRIDGE0_MEM_CTRL2(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_APB_BRIDGE0_MEM_CTRL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    #[inline(always)]
    pub const fn SEC_CTRL_APB_BRIDGE1_MEM_CTRL0(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_APB_BRIDGE1_MEM_CTRL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    #[inline(always)]
    pub const fn SEC_CTRL_APB_BRIDGE1_MEM_CTRL1(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_APB_BRIDGE1_MEM_CTRL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    #[inline(always)]
    pub const fn SEC_CTRL_APB_BRIDGE1_MEM_CTRL2(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_APB_BRIDGE1_MEM_CTRL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    #[inline(always)]
    pub const fn SEC_CTRL_APB_BRIDGE1_MEM_CTRL3(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_APB_BRIDGE1_MEM_CTRL3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Security access rules for AHB peripherals."]
    #[inline(always)]
    pub const fn SEC_CTRL_AHB_PORT7_SLAVE0_RULE(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_AHB_PORT7_SLAVE0_RULE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Security access rules for AHB peripherals."]
    #[inline(always)]
    pub const fn SEC_CTRL_AHB_PORT7_SLAVE1_RULE(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_AHB_PORT7_SLAVE1_RULE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Security access rules for AHB peripherals."]
    #[inline(always)]
    pub const fn SEC_CTRL_AHB_PORT8_SLAVE0_RULE(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_AHB_PORT8_SLAVE0_RULE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Security access rules for AHB peripherals."]
    #[inline(always)]
    pub const fn SEC_CTRL_AHB_PORT8_SLAVE1_RULE(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_AHB_PORT8_SLAVE1_RULE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Security access rules for AHB peripherals."]
    #[inline(always)]
    pub const fn SEC_CTRL_AHB_PORT9_SLAVE0_RULE(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_AHB_PORT9_SLAVE0_RULE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Security access rules for AHB peripherals."]
    #[inline(always)]
    pub const fn SEC_CTRL_AHB_PORT9_SLAVE1_RULE(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_AHB_PORT9_SLAVE1_RULE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Security access rules for AHB_SEC_CTRL_AHB."]
    #[inline(always)]
    pub const fn SEC_CTRL_AHB_SEC_CTRL_MEM_RULE(
        self,
    ) -> crate::common::Reg<regs::SEC_CTRL_AHB_SEC_CTRL_MEM_RULE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "most recent security violation address for AHB layer n."]
    #[inline(always)]
    pub const fn sec_vio_addr(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::sec_vio_addr, crate::common::R> {
        assert!(n < 10usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e00usize + n * 4usize) as _)
        }
    }
    #[doc = "most recent security violation miscellaneous information for AHB layer n."]
    #[inline(always)]
    pub const fn sec_vio_misc_info(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::sec_vio_misc_info, crate::common::R> {
        assert!(n < 10usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e80usize + n * 4usize) as _)
        }
    }
    #[doc = "security violation address/information registers valid flags."]
    #[inline(always)]
    pub const fn SEC_VIO_INFO_VALID(
        self,
    ) -> crate::common::Reg<regs::SEC_VIO_INFO_VALID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f00usize) as _) }
    }
    #[doc = "Secure GPIO mask for port 0 pins."]
    #[inline(always)]
    pub const fn SEC_GPIO_MASK0(
        self,
    ) -> crate::common::Reg<regs::SEC_GPIO_MASK0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f80usize) as _) }
    }
    #[doc = "Secure GPIO mask for port 1 pins."]
    #[inline(always)]
    pub const fn SEC_GPIO_MASK1(
        self,
    ) -> crate::common::Reg<regs::SEC_GPIO_MASK1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f84usize) as _) }
    }
    #[doc = "Security General Purpose register access control."]
    #[inline(always)]
    pub const fn SEC_MASK_LOCK(self) -> crate::common::Reg<regs::SEC_MASK_LOCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fbcusize) as _) }
    }
    #[doc = "master secure level register."]
    #[inline(always)]
    pub const fn MASTER_SEC_LEVEL(
        self,
    ) -> crate::common::Reg<regs::MASTER_SEC_LEVEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd0usize) as _) }
    }
    #[doc = "master secure level anti-pole register."]
    #[inline(always)]
    pub const fn MASTER_SEC_ANTI_POL_REG(
        self,
    ) -> crate::common::Reg<regs::MASTER_SEC_ANTI_POL_REG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd4usize) as _) }
    }
    #[doc = "Miscalleneous control signals for in Cortex M33 (CPU0)."]
    #[inline(always)]
    pub const fn CPU0_LOCK_REG(self) -> crate::common::Reg<regs::CPU0_LOCK_REG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
    }
    #[doc = "secure control duplicate register."]
    #[inline(always)]
    pub const fn MISC_CTRL_DP_REG(
        self,
    ) -> crate::common::Reg<regs::MISC_CTRL_DP_REG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "secure control register."]
    #[inline(always)]
    pub const fn MISC_CTRL_REG(self) -> crate::common::Reg<regs::MISC_CTRL_REG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
