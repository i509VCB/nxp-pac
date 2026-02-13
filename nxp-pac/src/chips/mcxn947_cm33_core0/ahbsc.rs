#[doc = "AHBSC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbsc {
    ptr: *mut u8,
}
unsafe impl Send for Ahbsc {}
unsafe impl Sync for Ahbsc {}
impl Ahbsc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Flash Memory Rule"]
    #[inline(always)]
    pub const fn flash00_mem_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Flash00MemRule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _) }
    }
    #[doc = "Flash Memory Rule"]
    #[inline(always)]
    pub const fn flash01_mem_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Flash01MemRule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "Flash Memory Rule"]
    #[inline(always)]
    pub const fn flash02_mem_rule(
        self,
    ) -> crate::common::Reg<regs::Flash02MemRule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Flash Memory Rule"]
    #[inline(always)]
    pub const fn flash03_mem_rule(
        self,
    ) -> crate::common::Reg<regs::Flash03MemRule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "ROM Memory Rule"]
    #[inline(always)]
    pub const fn rom_mem_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::RomMemRule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "RAMX Memory Rule"]
    #[inline(always)]
    pub const fn ramx_mem_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::RamxMemRule, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "RAMA Memory Rule 0"]
    #[inline(always)]
    pub const fn rama_mem_rule(self) -> crate::common::Reg<regs::RamaMemRule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "RAMB Memory Rule"]
    #[inline(always)]
    pub const fn ramb_mem_rule(self) -> crate::common::Reg<regs::RambMemRule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "RAMC Memory Rule"]
    #[inline(always)]
    pub const fn ramc_mem_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::RamcMemRule, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize + n * 4usize) as _) }
    }
    #[doc = "RAMD Memory Rule"]
    #[inline(always)]
    pub const fn ramd_mem_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::RamdMemRule, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "RAME Memory Rule"]
    #[inline(always)]
    pub const fn rame_mem_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::RameMemRule, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize + n * 4usize) as _)
        }
    }
    #[doc = "RAMF Memory Rule"]
    #[inline(always)]
    pub const fn ramf_mem_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::RamfMemRule, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize + n * 4usize) as _)
        }
    }
    #[doc = "RAMG Memory Rule"]
    #[inline(always)]
    pub const fn ramg_mem_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::RamgMemRule, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize + n * 4usize) as _)
        }
    }
    #[doc = "RAMH Memory Rule"]
    #[inline(always)]
    pub const fn ramh_mem_rule(self) -> crate::common::Reg<regs::RamhMemRule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "APB Bridge Group 0 Memory Rule 0"]
    #[inline(always)]
    pub const fn apb_peripheral_group0_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::ApbPeripheralGroup0MemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "APB Bridge Group 0 Memory Rule 1"]
    #[inline(always)]
    pub const fn apb_peripheral_group0_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::ApbPeripheralGroup0MemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "APB Bridge Group 0 Rule 2"]
    #[inline(always)]
    pub const fn apb_peripheral_group0_mem_rule2(
        self,
    ) -> crate::common::Reg<regs::ApbPeripheralGroup0MemRule2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "APB Bridge Group 0 Memory Rule 3"]
    #[inline(always)]
    pub const fn apb_peripheral_group0_mem_rule3(
        self,
    ) -> crate::common::Reg<regs::ApbPeripheralGroup0MemRule3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "APB Bridge Group 1 Memory Rule 0"]
    #[inline(always)]
    pub const fn apb_peripheral_group1_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::ApbPeripheralGroup1MemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "APB Bridge Group 1 Memory Rule 1"]
    #[inline(always)]
    pub const fn apb_peripheral_group1_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::ApbPeripheralGroup1MemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "APB Bridge Group 1 Memory Rule 2"]
    #[inline(always)]
    pub const fn apb_peripheral_group1_mem_rule2(
        self,
    ) -> crate::common::Reg<regs::ApbPeripheralGroup1MemRule2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 0"]
    #[inline(always)]
    pub const fn aips_bridge_group0_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup0MemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 1"]
    #[inline(always)]
    pub const fn aips_bridge_group0_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup0MemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 2"]
    #[inline(always)]
    pub const fn aips_bridge_group0_mem_rule2(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup0MemRule2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 3"]
    #[inline(always)]
    pub const fn aips_bridge_group0_mem_rule3(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup0MemRule3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "AHB Peripheral 0 Slave Port 12 Slave Rule 0"]
    #[inline(always)]
    pub const fn ahb_peripheral0_slave_port_p12_slave_rule0(
        self,
    ) -> crate::common::Reg<regs::AhbPeripheral0SlavePortP12SlaveRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "AHB Peripheral 0 Slave Port 12 Slave Rule 1"]
    #[inline(always)]
    pub const fn ahb_peripheral0_slave_port_p12_slave_rule1(
        self,
    ) -> crate::common::Reg<regs::AhbPeripheral0SlavePortP12SlaveRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "AHB Peripheral 0 Slave Port 12 Slave Rule 2"]
    #[inline(always)]
    pub const fn ahb_peripheral0_slave_port_p12_slave_rule2(
        self,
    ) -> crate::common::Reg<regs::AhbPeripheral0SlavePortP12SlaveRule2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "AIPS Bridge Group 1 Rule 0"]
    #[inline(always)]
    pub const fn aips_bridge_group1_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup1MemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "AIPS Bridge Group 1 Rule 1"]
    #[inline(always)]
    pub const fn aips_bridge_group1_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup1MemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "AHB Peripheral 1 Slave Port 13 Slave Rule 0"]
    #[inline(always)]
    pub const fn ahb_peripheral1_slave_port_p13_slave_rule0(
        self,
    ) -> crate::common::Reg<regs::AhbPeripheral1SlavePortP13SlaveRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "AHB Peripheral 1 Slave Port 13 Slave Rule 1"]
    #[inline(always)]
    pub const fn ahb_peripheral1_slave_port_p13_slave_rule1(
        self,
    ) -> crate::common::Reg<regs::AhbPeripheral1SlavePortP13SlaveRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "AHB Peripheral 1 Slave Port 13 Slave Rule 2"]
    #[inline(always)]
    pub const fn ahb_peripheral1_slave_port_p13_slave_rule2(
        self,
    ) -> crate::common::Reg<regs::AhbPeripheral1SlavePortP13SlaveRule2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 0"]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup2MemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Memory Rule 1"]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup2MemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "AIPS Bridge Group 3 Rule 0"]
    #[inline(always)]
    pub const fn aips_bridge_group3_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup3MemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "AIPS Bridge Group 3 Memory Rule 1"]
    #[inline(always)]
    pub const fn aips_bridge_group3_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup3MemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "AIPS Bridge Group 3 Rule 2"]
    #[inline(always)]
    pub const fn aips_bridge_group3_mem_rule2(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup3MemRule2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "AIPS Bridge Group 3 Rule 3"]
    #[inline(always)]
    pub const fn aips_bridge_group3_mem_rule3(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup3MemRule3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "AIPS Bridge Group 4 Rule 0"]
    #[inline(always)]
    pub const fn aips_bridge_group4_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup4MemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "AIPS Bridge Group 4 Rule 1"]
    #[inline(always)]
    pub const fn aips_bridge_group4_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup4MemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "AIPS Bridge Group 4 Rule 2"]
    #[inline(always)]
    pub const fn aips_bridge_group4_mem_rule2(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup4MemRule2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "AIPS Bridge Group 4 Rule 3"]
    #[inline(always)]
    pub const fn aips_bridge_group4_mem_rule3(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup4MemRule3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "AHB Secure Control Peripheral Rule 0"]
    #[inline(always)]
    pub const fn ahb_secure_ctrl_peripheral_rule0(
        self,
    ) -> crate::common::Reg<regs::AhbSecureCtrlPeripheralRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "FLEXSPI0 Region 0 Memory Rule"]
    #[inline(always)]
    pub const fn flexspi0_region0_mem_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Flexspi0Region0MemRule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize + n * 4usize) as _)
        }
    }
    #[doc = "Array of registers: FLEXSPI0_REGION_MEM_RULE0"]
    #[inline(always)]
    pub const fn flexspi0_region1_6_mem_rule(self, n: usize) -> Flexspi0Region16MemRule {
        assert!(n < 6usize);
        unsafe {
            Flexspi0Region16MemRule::from_ptr(self.ptr.wrapping_add(0x0280usize + n * 16usize) as _)
        }
    }
    #[doc = "FLEXSPI0 Region 7 Memory Rule"]
    #[inline(always)]
    pub const fn flexspi0_region7_mem_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Flexspi0Region7MemRule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e0usize + n * 4usize) as _)
        }
    }
    #[doc = "Array of registers: FLEXSPI0_REGION_MEM_RULE0"]
    #[inline(always)]
    pub const fn flexspi0_region8_13_mem_rule(self, n: usize) -> Flexspi0Region813MemRule {
        assert!(n < 6usize);
        unsafe {
            Flexspi0Region813MemRule::from_ptr(self.ptr.wrapping_add(0x02f0usize + n * 16usize) as _)
        }
    }
    #[doc = "Security Violation Address"]
    #[inline(always)]
    pub const fn sec_vio_addr(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SecVioAddr, crate::common::R> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e00usize + n * 4usize) as _)
        }
    }
    #[doc = "Security Violation Miscellaneous Information at Address"]
    #[inline(always)]
    pub const fn sec_vio_misc_info(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SecVioMiscInfo, crate::common::R> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e80usize + n * 4usize) as _)
        }
    }
    #[doc = "Security Violation Info Validity for Address"]
    #[inline(always)]
    pub const fn sec_vio_info_valid(
        self,
    ) -> crate::common::Reg<regs::SecVioInfoValid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f00usize) as _) }
    }
    #[doc = "GPIO Mask for Port index"]
    #[inline(always)]
    pub const fn sec_gpio_mask(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SecGpioMask, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f80usize + n * 4usize) as _)
        }
    }
    #[doc = "Secure Interrupt Mask 0 for CPU1"]
    #[inline(always)]
    pub const fn sec_cpu1_int_mask0(
        self,
    ) -> crate::common::Reg<regs::SecCpu1IntMask0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f98usize) as _) }
    }
    #[doc = "Secure Interrupt Mask 1 for CPU1"]
    #[inline(always)]
    pub const fn sec_cpu1_int_mask1(
        self,
    ) -> crate::common::Reg<regs::SecCpu1IntMask1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f9cusize) as _) }
    }
    #[doc = "Secure Interrupt Mask 2 for CPU1"]
    #[inline(always)]
    pub const fn sec_cpu1_int_mask2(
        self,
    ) -> crate::common::Reg<regs::SecCpu1IntMask2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa0usize) as _) }
    }
    #[doc = "Secure Interrupt Mask 3 for CPU1"]
    #[inline(always)]
    pub const fn sec_cpu1_int_mask3(
        self,
    ) -> crate::common::Reg<regs::SecCpu1IntMask3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa4usize) as _) }
    }
    #[doc = "Secure Interrupt Mask 4 for CPU1"]
    #[inline(always)]
    pub const fn sec_cpu1_int_mask4(
        self,
    ) -> crate::common::Reg<regs::SecCpu1IntMask4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa8usize) as _) }
    }
    #[doc = "Secure Mask Lock"]
    #[inline(always)]
    pub const fn sec_gp_reg_lock(
        self,
    ) -> crate::common::Reg<regs::SecGpRegLock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fbcusize) as _) }
    }
    #[doc = "Master Secure Level"]
    #[inline(always)]
    pub const fn master_sec_level(
        self,
    ) -> crate::common::Reg<regs::MasterSecLevel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd0usize) as _) }
    }
    #[doc = "Master Secure Level"]
    #[inline(always)]
    pub const fn master_sec_anti_pol_reg(
        self,
    ) -> crate::common::Reg<regs::MasterSecAntiPolReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd4usize) as _) }
    }
    #[doc = "Miscellaneous CPU0 Control Signals"]
    #[inline(always)]
    pub const fn cpu0_lock_reg(self) -> crate::common::Reg<regs::Cpu0LockReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
    }
    #[doc = "Miscellaneous CPU1 Control Signals"]
    #[inline(always)]
    pub const fn cpu1_lock_reg(self) -> crate::common::Reg<regs::Cpu1LockReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff0usize) as _) }
    }
    #[doc = "Secure Control Duplicate"]
    #[inline(always)]
    pub const fn misc_ctrl_dp_reg(
        self,
    ) -> crate::common::Reg<regs::MiscCtrlDpReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "Secure Control"]
    #[inline(always)]
    pub const fn misc_ctrl_reg(self) -> crate::common::Reg<regs::MiscCtrlReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
#[doc = "Array of registers: FLEXSPI0_REGION_MEM_RULE0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi0Region16MemRule {
    ptr: *mut u8,
}
unsafe impl Send for Flexspi0Region16MemRule {}
unsafe impl Sync for Flexspi0Region16MemRule {}
impl Flexspi0Region16MemRule {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "FLEXSPI0 Region index Memory Rule 0"]
    #[inline(always)]
    pub const fn flexspi0_region_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::Flexspi0Region16MemRuleFlexspi0RegionMemRule0, crate::common::RW>
    {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
#[doc = "Array of registers: FLEXSPI0_REGION_MEM_RULE0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi0Region813MemRule {
    ptr: *mut u8,
}
unsafe impl Send for Flexspi0Region813MemRule {}
unsafe impl Sync for Flexspi0Region813MemRule {}
impl Flexspi0Region813MemRule {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "FLEXSPI0 Region index Memory Rule 0"]
    #[inline(always)]
    pub const fn flexspi0_region_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::Flexspi0Region813MemRuleFlexspi0RegionMemRule0, crate::common::RW>
    {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
