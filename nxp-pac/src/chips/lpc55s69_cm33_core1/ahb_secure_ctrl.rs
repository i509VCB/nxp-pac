#[doc = "AHB secure controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbSecureCtrl {
    ptr: *mut u8,
}
unsafe impl Send for AhbSecureCtrl {}
unsafe impl Sync for AhbSecureCtrl {}
impl AhbSecureCtrl {
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
    pub const fn sec_ctrl_flash_rom_slave_rule(
        self,
    ) -> crate::common::Reg<regs::SecCtrlFlashRomSlaveRule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_flash_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::SecCtrlFlashMemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_flash_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::SecCtrlFlashMemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_flash_mem_rule2(
        self,
    ) -> crate::common::Reg<regs::SecCtrlFlashMemRule2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_rom_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::SecCtrlRomMemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_rom_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::SecCtrlRomMemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_rom_mem_rule2(
        self,
    ) -> crate::common::Reg<regs::SecCtrlRomMemRule2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_rom_mem_rule3(
        self,
    ) -> crate::common::Reg<regs::SecCtrlRomMemRule3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Security access rules for RAMX slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ramx_slave_rule(
        self,
    ) -> crate::common::Reg<regs::SecCtrlRamxSlaveRule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Security access rules for RAMX slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ramx_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::SecCtrlRamxMemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Security access rules for RAM0 slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ram0_slave_rule(
        self,
    ) -> crate::common::Reg<regs::SecCtrlRam0SlaveRule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Security access rules for RAM0 slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ram0_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::SecCtrlRam0MemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Security access rules for RAM0 slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ram0_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::SecCtrlRam0MemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Security access rules for RAM1 slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ram1_slave_rule(
        self,
    ) -> crate::common::Reg<regs::SecCtrlRam1SlaveRule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Security access rules for RAM1 slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ram1_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::SecCtrlRam1MemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Security access rules for RAM1 slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ram1_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::SecCtrlRam1MemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Security access rules for RAM2 slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ram2_slave_rule(
        self,
    ) -> crate::common::Reg<regs::SecCtrlRam2SlaveRule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Security access rules for RAM2 slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ram2_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::SecCtrlRam2MemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Security access rules for RAM2 slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ram2_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::SecCtrlRam2MemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Security access rules for RAM3 slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ram3_slave_rule(
        self,
    ) -> crate::common::Reg<regs::SecCtrlRam3SlaveRule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Security access rules for RAM3 slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ram3_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::SecCtrlRam3MemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Security access rules for RAM3 slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ram3_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::SecCtrlRam3MemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Security access rules for RAM4 slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ram4_slave_rule(
        self,
    ) -> crate::common::Reg<regs::SecCtrlRam4SlaveRule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Security access rules for RAM4 slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_ram4_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::SecCtrlRam4MemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Security access rules for both APB Bridges slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_apb_bridge_slave_rule(
        self,
    ) -> crate::common::Reg<regs::SecCtrlApbBridgeSlaveRule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_apb_bridge0_mem_ctrl0(
        self,
    ) -> crate::common::Reg<regs::SecCtrlApbBridge0MemCtrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_apb_bridge0_mem_ctrl1(
        self,
    ) -> crate::common::Reg<regs::SecCtrlApbBridge0MemCtrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_apb_bridge0_mem_ctrl2(
        self,
    ) -> crate::common::Reg<regs::SecCtrlApbBridge0MemCtrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_apb_bridge1_mem_ctrl0(
        self,
    ) -> crate::common::Reg<regs::SecCtrlApbBridge1MemCtrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_apb_bridge1_mem_ctrl1(
        self,
    ) -> crate::common::Reg<regs::SecCtrlApbBridge1MemCtrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_apb_bridge1_mem_ctrl2(
        self,
    ) -> crate::common::Reg<regs::SecCtrlApbBridge1MemCtrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    #[inline(always)]
    pub const fn sec_ctrl_apb_bridge1_mem_ctrl3(
        self,
    ) -> crate::common::Reg<regs::SecCtrlApbBridge1MemCtrl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Security access rules for AHB peripherals."]
    #[inline(always)]
    pub const fn sec_ctrl_ahb_port8_slave0_rule(
        self,
    ) -> crate::common::Reg<regs::SecCtrlAhbPort8Slave0Rule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Security access rules for AHB peripherals."]
    #[inline(always)]
    pub const fn sec_ctrl_ahb_port8_slave1_rule(
        self,
    ) -> crate::common::Reg<regs::SecCtrlAhbPort8Slave1Rule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Security access rules for AHB peripherals."]
    #[inline(always)]
    pub const fn sec_ctrl_ahb_port9_slave0_rule(
        self,
    ) -> crate::common::Reg<regs::SecCtrlAhbPort9Slave0Rule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Security access rules for AHB peripherals."]
    #[inline(always)]
    pub const fn sec_ctrl_ahb_port9_slave1_rule(
        self,
    ) -> crate::common::Reg<regs::SecCtrlAhbPort9Slave1Rule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Security access rules for AHB peripherals."]
    #[inline(always)]
    pub const fn sec_ctrl_ahb_port10_slave0_rule(
        self,
    ) -> crate::common::Reg<regs::SecCtrlAhbPort10Slave0Rule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Security access rules for AHB peripherals."]
    #[inline(always)]
    pub const fn sec_ctrl_ahb_port10_slave1_rule(
        self,
    ) -> crate::common::Reg<regs::SecCtrlAhbPort10Slave1Rule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Security access rules for AHB_SEC_CTRL_AHB."]
    #[inline(always)]
    pub const fn sec_ctrl_ahb_sec_ctrl_mem_rule(
        self,
    ) -> crate::common::Reg<regs::SecCtrlAhbSecCtrlMemRule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Security access rules for USB High speed RAM slaves."]
    #[inline(always)]
    pub const fn sec_ctrl_usb_hs_slave_rule(
        self,
    ) -> crate::common::Reg<regs::SecCtrlUsbHsSlaveRule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Security access rules for RAM_USB_HS."]
    #[inline(always)]
    pub const fn sec_ctrl_usb_hs_mem_rule(
        self,
    ) -> crate::common::Reg<regs::SecCtrlUsbHsMemRule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "most recent security violation address for AHB port n"]
    #[inline(always)]
    pub const fn sec_vio_addr(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SecVioAddr, crate::common::R> {
        assert!(n < 12usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e00usize + n * 4usize) as _)
        }
    }
    #[doc = "most recent security violation miscellaneous information for AHB port n"]
    #[inline(always)]
    pub const fn sec_vio_misc_info(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SecVioMiscInfo, crate::common::R> {
        assert!(n < 12usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e80usize + n * 4usize) as _)
        }
    }
    #[doc = "security violation address/information registers valid flags"]
    #[inline(always)]
    pub const fn sec_vio_info_valid(
        self,
    ) -> crate::common::Reg<regs::SecVioInfoValid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f00usize) as _) }
    }
    #[doc = "Secure GPIO mask for port 0 pins."]
    #[inline(always)]
    pub const fn sec_gpio_mask0(self) -> crate::common::Reg<regs::SecGpioMask0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f80usize) as _) }
    }
    #[doc = "Secure GPIO mask for port 1 pins."]
    #[inline(always)]
    pub const fn sec_gpio_mask1(self) -> crate::common::Reg<regs::SecGpioMask1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f84usize) as _) }
    }
    #[doc = "Secure Interrupt mask for CPU1"]
    #[inline(always)]
    pub const fn sec_cpu_int_mask0(
        self,
    ) -> crate::common::Reg<regs::SecCpuIntMask0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f90usize) as _) }
    }
    #[doc = "Secure Interrupt mask for CPU1"]
    #[inline(always)]
    pub const fn sec_cpu_int_mask1(
        self,
    ) -> crate::common::Reg<regs::SecCpuIntMask1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f94usize) as _) }
    }
    #[doc = "Security General Purpose register access control."]
    #[inline(always)]
    pub const fn sec_mask_lock(self) -> crate::common::Reg<regs::SecMaskLock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fbcusize) as _) }
    }
    #[doc = "master secure level register"]
    #[inline(always)]
    pub const fn master_sec_level(
        self,
    ) -> crate::common::Reg<regs::MasterSecLevel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd0usize) as _) }
    }
    #[doc = "master secure level anti-pole register"]
    #[inline(always)]
    pub const fn master_sec_anti_pol_reg(
        self,
    ) -> crate::common::Reg<regs::MasterSecAntiPolReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd4usize) as _) }
    }
    #[doc = "Miscalleneous control signals for in Cortex M33 (CPU0)"]
    #[inline(always)]
    pub const fn cpu0_lock_reg(self) -> crate::common::Reg<regs::Cpu0LockReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
    }
    #[doc = "Miscalleneous control signals for in micro-Cortex M33 (CPU1)"]
    #[inline(always)]
    pub const fn cpu1_lock_reg(self) -> crate::common::Reg<regs::Cpu1LockReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff0usize) as _) }
    }
    #[doc = "secure control duplicate register"]
    #[inline(always)]
    pub const fn misc_ctrl_dp_reg(
        self,
    ) -> crate::common::Reg<regs::MiscCtrlDpReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "secure control register"]
    #[inline(always)]
    pub const fn misc_ctrl_reg(self) -> crate::common::Reg<regs::MiscCtrlReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
