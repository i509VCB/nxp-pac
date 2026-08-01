#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "AHBSC."]
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
    #[doc = "Flash Memory Rule."]
    #[inline(always)]
    pub const fn flash00_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Flash00MemRule, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _)
        }
    }
    #[doc = "Flash Memory Rule."]
    #[inline(always)]
    pub const fn flash01_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Flash01MemRule, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _)
        }
    }
    #[doc = "Flash Memory Rule."]
    #[inline(always)]
    pub const fn flash02_mem_rule(
        self,
    ) -> crate::pac::common::Reg<Flash02MemRule, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Flash Memory Rule."]
    #[inline(always)]
    pub const fn flash03_mem_rule(
        self,
    ) -> crate::pac::common::Reg<Flash03MemRule, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "ROM Memory Rule."]
    #[inline(always)]
    pub const fn rom_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<RomMemRule, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _)
        }
    }
    #[doc = "RAMX Memory Rule."]
    #[inline(always)]
    pub const fn ramx_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<RamxMemRule, crate::pac::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _)
        }
    }
    #[doc = "RAMA Memory Rule 0."]
    #[inline(always)]
    pub const fn rama_mem_rule(
        self,
    ) -> crate::pac::common::Reg<RamaMemRule, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "RAMB Memory Rule."]
    #[inline(always)]
    pub const fn ramb_mem_rule(
        self,
    ) -> crate::pac::common::Reg<RambMemRule, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "RAMC Memory Rule."]
    #[inline(always)]
    pub const fn ramc_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<RamcMemRule, crate::pac::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize + n * 4usize) as _)
        }
    }
    #[doc = "RAMD Memory Rule."]
    #[inline(always)]
    pub const fn ramd_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<RamdMemRule, crate::pac::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "RAME Memory Rule."]
    #[inline(always)]
    pub const fn rame_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<RameMemRule, crate::pac::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize + n * 4usize) as _)
        }
    }
    #[doc = "RAMF Memory Rule."]
    #[inline(always)]
    pub const fn ramf_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<RamfMemRule, crate::pac::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize + n * 4usize) as _)
        }
    }
    #[doc = "RAMG Memory Rule."]
    #[inline(always)]
    pub const fn ramg_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<RamgMemRule, crate::pac::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize + n * 4usize) as _)
        }
    }
    #[doc = "RAMH Memory Rule."]
    #[inline(always)]
    pub const fn ramh_mem_rule(
        self,
    ) -> crate::pac::common::Reg<RamhMemRule, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "APB Bridge Group 0 Memory Rule 0."]
    #[inline(always)]
    pub const fn apb_peripheral_group0_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<ApbPeripheralGroup0MemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "APB Bridge Group 0 Memory Rule 1."]
    #[inline(always)]
    pub const fn apb_peripheral_group0_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<ApbPeripheralGroup0MemRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "APB Bridge Group 0 Rule 2."]
    #[inline(always)]
    pub const fn apb_peripheral_group0_mem_rule2(
        self,
    ) -> crate::pac::common::Reg<ApbPeripheralGroup0MemRule2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "APB Bridge Group 0 Memory Rule 3."]
    #[inline(always)]
    pub const fn apb_peripheral_group0_mem_rule3(
        self,
    ) -> crate::pac::common::Reg<ApbPeripheralGroup0MemRule3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "APB Bridge Group 1 Memory Rule 0."]
    #[inline(always)]
    pub const fn apb_peripheral_group1_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<ApbPeripheralGroup1MemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "APB Bridge Group 1 Memory Rule 1."]
    #[inline(always)]
    pub const fn apb_peripheral_group1_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<ApbPeripheralGroup1MemRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "APB Bridge Group 1 Memory Rule 2."]
    #[inline(always)]
    pub const fn apb_peripheral_group1_mem_rule2(
        self,
    ) -> crate::pac::common::Reg<ApbPeripheralGroup1MemRule2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 0."]
    #[inline(always)]
    pub const fn aips_bridge_group0_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup0MemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 1."]
    #[inline(always)]
    pub const fn aips_bridge_group0_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup0MemRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 2."]
    #[inline(always)]
    pub const fn aips_bridge_group0_mem_rule2(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup0MemRule2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 3."]
    #[inline(always)]
    pub const fn aips_bridge_group0_mem_rule3(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup0MemRule3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "AHB Peripheral 0 Slave Port 12 Slave Rule 0."]
    #[inline(always)]
    pub const fn ahb_peripheral0_slave_port_p12_slave_rule0(
        self,
    ) -> crate::pac::common::Reg<AhbPeripheral0SlavePortP12SlaveRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "AHB Peripheral 0 Slave Port 12 Slave Rule 1."]
    #[inline(always)]
    pub const fn ahb_peripheral0_slave_port_p12_slave_rule1(
        self,
    ) -> crate::pac::common::Reg<AhbPeripheral0SlavePortP12SlaveRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "AHB Peripheral 0 Slave Port 12 Slave Rule 2."]
    #[inline(always)]
    pub const fn ahb_peripheral0_slave_port_p12_slave_rule2(
        self,
    ) -> crate::pac::common::Reg<AhbPeripheral0SlavePortP12SlaveRule2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "AIPS Bridge Group 1 Rule 0."]
    #[inline(always)]
    pub const fn aips_bridge_group1_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup1MemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "AIPS Bridge Group 1 Rule 1."]
    #[inline(always)]
    pub const fn aips_bridge_group1_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup1MemRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "AHB Peripheral 1 Slave Port 13 Slave Rule 0."]
    #[inline(always)]
    pub const fn ahb_peripheral1_slave_port_p13_slave_rule0(
        self,
    ) -> crate::pac::common::Reg<AhbPeripheral1SlavePortP13SlaveRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "AHB Peripheral 1 Slave Port 13 Slave Rule 1."]
    #[inline(always)]
    pub const fn ahb_peripheral1_slave_port_p13_slave_rule1(
        self,
    ) -> crate::pac::common::Reg<AhbPeripheral1SlavePortP13SlaveRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "AHB Peripheral 1 Slave Port 13 Slave Rule 2."]
    #[inline(always)]
    pub const fn ahb_peripheral1_slave_port_p13_slave_rule2(
        self,
    ) -> crate::pac::common::Reg<AhbPeripheral1SlavePortP13SlaveRule2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 0."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2MemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Memory Rule 1."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2MemRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "AIPS Bridge Group 3 Rule 0."]
    #[inline(always)]
    pub const fn aips_bridge_group3_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup3MemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "AIPS Bridge Group 3 Memory Rule 1."]
    #[inline(always)]
    pub const fn aips_bridge_group3_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup3MemRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "AIPS Bridge Group 3 Rule 2."]
    #[inline(always)]
    pub const fn aips_bridge_group3_mem_rule2(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup3MemRule2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "AIPS Bridge Group 3 Rule 3."]
    #[inline(always)]
    pub const fn aips_bridge_group3_mem_rule3(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup3MemRule3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "AIPS Bridge Group 4 Rule 0."]
    #[inline(always)]
    pub const fn aips_bridge_group4_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup4MemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "AIPS Bridge Group 4 Rule 1."]
    #[inline(always)]
    pub const fn aips_bridge_group4_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup4MemRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "AIPS Bridge Group 4 Rule 2."]
    #[inline(always)]
    pub const fn aips_bridge_group4_mem_rule2(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup4MemRule2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "AIPS Bridge Group 4 Rule 3."]
    #[inline(always)]
    pub const fn aips_bridge_group4_mem_rule3(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup4MemRule3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "AHB Secure Control Peripheral Rule 0."]
    #[inline(always)]
    pub const fn ahb_secure_ctrl_peripheral_rule0(
        self,
    ) -> crate::pac::common::Reg<AhbSecureCtrlPeripheralRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "FLEXSPI0 Region 0 Memory Rule."]
    #[inline(always)]
    pub const fn flexspi0_region0_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Flexspi0Region0MemRule, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize + n * 4usize) as _)
        }
    }
    #[doc = "Array of registers: FLEXSPI0_REGION_MEM_RULE0."]
    #[inline(always)]
    pub const fn flexspi0_region1_6_mem_rule(self, n: usize) -> Flexspi0Region16MemRule {
        assert!(n < 6usize);
        unsafe {
            Flexspi0Region16MemRule::from_ptr(self.ptr.wrapping_add(0x0280usize + n * 16usize) as _)
        }
    }
    #[doc = "FLEXSPI0 Region 7 Memory Rule."]
    #[inline(always)]
    pub const fn flexspi0_region7_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Flexspi0Region7MemRule, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e0usize + n * 4usize) as _)
        }
    }
    #[doc = "Array of registers: FLEXSPI0_REGION_MEM_RULE0."]
    #[inline(always)]
    pub const fn flexspi0_region8_13_mem_rule(self, n: usize) -> Flexspi0Region813MemRule {
        assert!(n < 6usize);
        unsafe {
            Flexspi0Region813MemRule::from_ptr(self.ptr.wrapping_add(0x02f0usize + n * 16usize) as _)
        }
    }
    #[doc = "Security Violation Address."]
    #[inline(always)]
    pub const fn sec_vio_addr(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<SecVioAddr, crate::pac::common::R> {
        assert!(n < 32usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e00usize + n * 4usize) as _)
        }
    }
    #[doc = "Security Violation Miscellaneous Information at Address."]
    #[inline(always)]
    pub const fn sec_vio_misc_info(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<SecVioMiscInfo, crate::pac::common::R> {
        assert!(n < 32usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e80usize + n * 4usize) as _)
        }
    }
    #[doc = "Security Violation Info Validity for Address."]
    #[inline(always)]
    pub const fn sec_vio_info_valid(
        self,
    ) -> crate::pac::common::Reg<SecVioInfoValid, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f00usize) as _) }
    }
    #[doc = "GPIO Mask for Port index."]
    #[inline(always)]
    pub const fn sec_gpio_mask(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<SecGpioMask, crate::pac::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f80usize + n * 4usize) as _)
        }
    }
    #[doc = "Secure Interrupt Mask for CPU1."]
    #[inline(always)]
    pub const fn sec_cpu1_int_mask(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<SecCpu1IntMask, crate::pac::common::RW> {
        assert!(n < 5usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f98usize + n * 4usize) as _)
        }
    }
    #[doc = "Secure Mask Lock."]
    #[inline(always)]
    pub const fn sec_gp_reg_lock(
        self,
    ) -> crate::pac::common::Reg<SecGpRegLock, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fbcusize) as _) }
    }
    #[doc = "Master Secure Level."]
    #[inline(always)]
    pub const fn master_sec_level(
        self,
    ) -> crate::pac::common::Reg<MasterSecLevel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd0usize) as _) }
    }
    #[doc = "Master Secure Level."]
    #[inline(always)]
    pub const fn master_sec_anti_pol_reg(
        self,
    ) -> crate::pac::common::Reg<MasterSecAntiPolReg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd4usize) as _) }
    }
    #[doc = "Miscellaneous CPU0 Control Signals."]
    #[inline(always)]
    pub const fn cpu0_lock_reg(
        self,
    ) -> crate::pac::common::Reg<Cpu0LockReg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
    }
    #[doc = "Miscellaneous CPU1 Control Signals."]
    #[inline(always)]
    pub const fn cpu1_lock_reg(
        self,
    ) -> crate::pac::common::Reg<Cpu1LockReg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff0usize) as _) }
    }
    #[doc = "Secure Control Duplicate."]
    #[inline(always)]
    pub const fn misc_ctrl_dp_reg(
        self,
    ) -> crate::pac::common::Reg<MiscCtrlDpReg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "Secure Control."]
    #[inline(always)]
    pub const fn misc_ctrl_reg(
        self,
    ) -> crate::pac::common::Reg<MiscCtrlReg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
#[doc = "Array of registers: FLEXSPI0_REGION_MEM_RULE0."]
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
    #[doc = "FLEXSPI0 Region index Memory Rule 0."]
    #[inline(always)]
    pub const fn flexspi0_region_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<
        Flexspi0Region16MemRuleFlexspi0RegionMemRule0,
        crate::pac::common::RW,
    > {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
#[doc = "Array of registers: FLEXSPI0_REGION_MEM_RULE0."]
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
    #[doc = "FLEXSPI0 Region index Memory Rule 0."]
    #[inline(always)]
    pub const fn flexspi0_region_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<
        Flexspi0Region813MemRuleFlexspi0RegionMemRule0,
        crate::pac::common::RW,
    > {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
#[doc = "AHB Peripheral 0 Slave Port 12 Slave Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0SlavePortP12SlaveRule0(pub u32);
impl AhbPeripheral0SlavePortP12SlaveRule0 {
    #[doc = "eDMA0_CH15."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch15(&self) -> EDma0Ch15 {
        let val = (self.0 >> 4usize) & 0x03;
        EDma0Ch15::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH15."]
    #[inline(always)]
    pub const fn set_e_dma0_ch15(&mut self, val: EDma0Ch15) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "SCT0."]
    #[must_use]
    #[inline(always)]
    pub const fn sct0(&self) -> Sct0 {
        let val = (self.0 >> 8usize) & 0x03;
        Sct0::from_bits(val as u8)
    }
    #[doc = "SCT0."]
    #[inline(always)]
    pub const fn set_sct0(&mut self, val: Sct0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LP_FLEXCOMM."]
    #[must_use]
    #[inline(always)]
    pub const fn lp_flexcomm(&self, n: usize) -> LpFlexcomm {
        assert!(n < 4usize);
        let offs = 12usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        LpFlexcomm::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM."]
    #[inline(always)]
    pub const fn set_lp_flexcomm(&mut self, n: usize, val: LpFlexcomm) {
        assert!(n < 4usize);
        let offs = 12usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
    #[doc = "GPIO0_ALIAS0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0_alias0(&self) -> Gpio0Alias0 {
        let val = (self.0 >> 28usize) & 0x03;
        Gpio0Alias0::from_bits(val as u8)
    }
    #[doc = "GPIO0_ALIAS0."]
    #[inline(always)]
    pub const fn set_gpio0_alias0(&mut self, val: Gpio0Alias0) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AhbPeripheral0SlavePortP12SlaveRule0 {
    #[inline(always)]
    fn default() -> AhbPeripheral0SlavePortP12SlaveRule0 {
        AhbPeripheral0SlavePortP12SlaveRule0(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0SlavePortP12SlaveRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0SlavePortP12SlaveRule0")
            .field("e_dma0_ch15", &self.e_dma0_ch15())
            .field("sct0", &self.sct0())
            .field("lp_flexcomm[0]", &self.lp_flexcomm(0usize))
            .field("lp_flexcomm[1]", &self.lp_flexcomm(1usize))
            .field("lp_flexcomm[2]", &self.lp_flexcomm(2usize))
            .field("lp_flexcomm[3]", &self.lp_flexcomm(3usize))
            .field("gpio0_alias0", &self.gpio0_alias0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0SlavePortP12SlaveRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0SlavePortP12SlaveRule0 {{ e_dma0_ch15: {:?}, sct0: {:?}, lp_flexcomm[0]: {:?}, lp_flexcomm[1]: {:?}, lp_flexcomm[2]: {:?}, lp_flexcomm[3]: {:?}, gpio0_alias0: {:?} }}",
            self.e_dma0_ch15(),
            self.sct0(),
            self.lp_flexcomm(0usize),
            self.lp_flexcomm(1usize),
            self.lp_flexcomm(2usize),
            self.lp_flexcomm(3usize),
            self.gpio0_alias0()
        )
    }
}
#[doc = "AHB Peripheral 0 Slave Port 12 Slave Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0SlavePortP12SlaveRule1(pub u32);
impl AhbPeripheral0SlavePortP12SlaveRule1 {
    #[doc = "GPIO0_ALIAS1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0_alias1(&self) -> Gpio0Alias1 {
        let val = (self.0 >> 0usize) & 0x03;
        Gpio0Alias1::from_bits(val as u8)
    }
    #[doc = "GPIO0_ALIAS1."]
    #[inline(always)]
    pub const fn set_gpio0_alias1(&mut self, val: Gpio0Alias1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO1_ALIAS0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1_alias0(&self) -> Gpio1Alias0 {
        let val = (self.0 >> 4usize) & 0x03;
        Gpio1Alias0::from_bits(val as u8)
    }
    #[doc = "GPIO1_ALIAS0."]
    #[inline(always)]
    pub const fn set_gpio1_alias0(&mut self, val: Gpio1Alias0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "GPIO1_ALIAS1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1_alias1(&self) -> Gpio1Alias1 {
        let val = (self.0 >> 8usize) & 0x03;
        Gpio1Alias1::from_bits(val as u8)
    }
    #[doc = "GPIO1_ALIAS1."]
    #[inline(always)]
    pub const fn set_gpio1_alias1(&mut self, val: Gpio1Alias1) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "GPIO2_ALIAS0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2_alias0(&self) -> Gpio2Alias0 {
        let val = (self.0 >> 12usize) & 0x03;
        Gpio2Alias0::from_bits(val as u8)
    }
    #[doc = "GPIO2_ALIAS0."]
    #[inline(always)]
    pub const fn set_gpio2_alias0(&mut self, val: Gpio2Alias0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "GPIO2_ALIAS1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2_alias1(&self) -> Gpio2Alias1 {
        let val = (self.0 >> 16usize) & 0x03;
        Gpio2Alias1::from_bits(val as u8)
    }
    #[doc = "GPIO2_ALIAS1."]
    #[inline(always)]
    pub const fn set_gpio2_alias1(&mut self, val: Gpio2Alias1) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "GPIO3_ALIAS0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3_alias0(&self) -> Gpio3Alias0 {
        let val = (self.0 >> 20usize) & 0x03;
        Gpio3Alias0::from_bits(val as u8)
    }
    #[doc = "GPIO3_ALIAS0."]
    #[inline(always)]
    pub const fn set_gpio3_alias0(&mut self, val: Gpio3Alias0) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "GPIO3_ALIAS1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3_alias1(&self) -> Gpio3Alias1 {
        let val = (self.0 >> 24usize) & 0x03;
        Gpio3Alias1::from_bits(val as u8)
    }
    #[doc = "GPIO3_ALIAS1."]
    #[inline(always)]
    pub const fn set_gpio3_alias1(&mut self, val: Gpio3Alias1) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "GPIO4_ALIAS0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4_alias0(&self) -> Gpio4Alias0 {
        let val = (self.0 >> 28usize) & 0x03;
        Gpio4Alias0::from_bits(val as u8)
    }
    #[doc = "GPIO4_ALIAS0."]
    #[inline(always)]
    pub const fn set_gpio4_alias0(&mut self, val: Gpio4Alias0) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AhbPeripheral0SlavePortP12SlaveRule1 {
    #[inline(always)]
    fn default() -> AhbPeripheral0SlavePortP12SlaveRule1 {
        AhbPeripheral0SlavePortP12SlaveRule1(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0SlavePortP12SlaveRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0SlavePortP12SlaveRule1")
            .field("gpio0_alias1", &self.gpio0_alias1())
            .field("gpio1_alias0", &self.gpio1_alias0())
            .field("gpio1_alias1", &self.gpio1_alias1())
            .field("gpio2_alias0", &self.gpio2_alias0())
            .field("gpio2_alias1", &self.gpio2_alias1())
            .field("gpio3_alias0", &self.gpio3_alias0())
            .field("gpio3_alias1", &self.gpio3_alias1())
            .field("gpio4_alias0", &self.gpio4_alias0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0SlavePortP12SlaveRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0SlavePortP12SlaveRule1 {{ gpio0_alias1: {:?}, gpio1_alias0: {:?}, gpio1_alias1: {:?}, gpio2_alias0: {:?}, gpio2_alias1: {:?}, gpio3_alias0: {:?}, gpio3_alias1: {:?}, gpio4_alias0: {:?} }}",
            self.gpio0_alias1(),
            self.gpio1_alias0(),
            self.gpio1_alias1(),
            self.gpio2_alias0(),
            self.gpio2_alias1(),
            self.gpio3_alias0(),
            self.gpio3_alias1(),
            self.gpio4_alias0()
        )
    }
}
#[doc = "AHB Peripheral 0 Slave Port 12 Slave Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0SlavePortP12SlaveRule2(pub u32);
impl AhbPeripheral0SlavePortP12SlaveRule2 {
    #[doc = "GPIO4_ALIAS1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4_alias1(&self) -> Gpio4Alias1 {
        let val = (self.0 >> 0usize) & 0x03;
        Gpio4Alias1::from_bits(val as u8)
    }
    #[doc = "GPIO4_ALIAS1."]
    #[inline(always)]
    pub const fn set_gpio4_alias1(&mut self, val: Gpio4Alias1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for AhbPeripheral0SlavePortP12SlaveRule2 {
    #[inline(always)]
    fn default() -> AhbPeripheral0SlavePortP12SlaveRule2 {
        AhbPeripheral0SlavePortP12SlaveRule2(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0SlavePortP12SlaveRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0SlavePortP12SlaveRule2")
            .field("gpio4_alias1", &self.gpio4_alias1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0SlavePortP12SlaveRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0SlavePortP12SlaveRule2 {{ gpio4_alias1: {:?} }}",
            self.gpio4_alias1()
        )
    }
}
#[doc = "AHB Peripheral 1 Slave Port 13 Slave Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral1SlavePortP13SlaveRule0(pub u32);
impl AhbPeripheral1SlavePortP13SlaveRule0 {
    #[doc = "eDMA1_CH15."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma1_ch15(&self) -> EDma1Ch15 {
        let val = (self.0 >> 4usize) & 0x03;
        EDma1Ch15::from_bits(val as u8)
    }
    #[doc = "eDMA1_CH15."]
    #[inline(always)]
    pub const fn set_e_dma1_ch15(&mut self, val: EDma1Ch15) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "SEMA42."]
    #[must_use]
    #[inline(always)]
    pub const fn sema42(&self) -> Sema42 {
        let val = (self.0 >> 8usize) & 0x03;
        Sema42::from_bits(val as u8)
    }
    #[doc = "SEMA42."]
    #[inline(always)]
    pub const fn set_sema42(&mut self, val: Sema42) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "MAILBOX."]
    #[must_use]
    #[inline(always)]
    pub const fn mailbox(&self) -> Mailbox {
        let val = (self.0 >> 12usize) & 0x03;
        Mailbox::from_bits(val as u8)
    }
    #[doc = "MAILBOX."]
    #[inline(always)]
    pub const fn set_mailbox(&mut self, val: Mailbox) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "PKC_RAM."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc_ram(&self) -> PkcRam {
        let val = (self.0 >> 16usize) & 0x03;
        PkcRam::from_bits(val as u8)
    }
    #[doc = "PKC_RAM."]
    #[inline(always)]
    pub const fn set_pkc_ram(&mut self, val: PkcRam) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "FLEXCOMM."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm(&self, n: usize) -> Flexcomm {
        assert!(n < 3usize);
        let offs = 20usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Flexcomm::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM."]
    #[inline(always)]
    pub const fn set_flexcomm(&mut self, n: usize, val: Flexcomm) {
        assert!(n < 3usize);
        let offs = 20usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for AhbPeripheral1SlavePortP13SlaveRule0 {
    #[inline(always)]
    fn default() -> AhbPeripheral1SlavePortP13SlaveRule0 {
        AhbPeripheral1SlavePortP13SlaveRule0(0)
    }
}
impl core::fmt::Debug for AhbPeripheral1SlavePortP13SlaveRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral1SlavePortP13SlaveRule0")
            .field("e_dma1_ch15", &self.e_dma1_ch15())
            .field("sema42", &self.sema42())
            .field("mailbox", &self.mailbox())
            .field("pkc_ram", &self.pkc_ram())
            .field("flexcomm[0]", &self.flexcomm(0usize))
            .field("flexcomm[1]", &self.flexcomm(1usize))
            .field("flexcomm[2]", &self.flexcomm(2usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral1SlavePortP13SlaveRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral1SlavePortP13SlaveRule0 {{ e_dma1_ch15: {:?}, sema42: {:?}, mailbox: {:?}, pkc_ram: {:?}, flexcomm[0]: {:?}, flexcomm[1]: {:?}, flexcomm[2]: {:?} }}",
            self.e_dma1_ch15(),
            self.sema42(),
            self.mailbox(),
            self.pkc_ram(),
            self.flexcomm(0usize),
            self.flexcomm(1usize),
            self.flexcomm(2usize)
        )
    }
}
#[doc = "AHB Peripheral 1 Slave Port 13 Slave Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral1SlavePortP13SlaveRule1(pub u32);
impl AhbPeripheral1SlavePortP13SlaveRule1 {
    #[doc = "FLEXCOMM."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm(&self, n: usize) -> Flexcomm {
        assert!(n < 3usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Flexcomm::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM."]
    #[inline(always)]
    pub const fn set_flexcomm(&mut self, n: usize, val: Flexcomm) {
        assert!(n < 3usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
    #[doc = "USB FS OTG RAM."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_fs_otg_ram(&self) -> UsbFsOtgRam {
        let val = (self.0 >> 12usize) & 0x03;
        UsbFsOtgRam::from_bits(val as u8)
    }
    #[doc = "USB FS OTG RAM."]
    #[inline(always)]
    pub const fn set_usb_fs_otg_ram(&mut self, val: UsbFsOtgRam) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "CDOG0."]
    #[must_use]
    #[inline(always)]
    pub const fn cdog0(&self) -> Cdog0 {
        let val = (self.0 >> 16usize) & 0x03;
        Cdog0::from_bits(val as u8)
    }
    #[doc = "CDOG0."]
    #[inline(always)]
    pub const fn set_cdog0(&mut self, val: Cdog0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "CDOG1."]
    #[must_use]
    #[inline(always)]
    pub const fn cdog1(&self) -> Cdog1 {
        let val = (self.0 >> 20usize) & 0x03;
        Cdog1::from_bits(val as u8)
    }
    #[doc = "CDOG1."]
    #[inline(always)]
    pub const fn set_cdog1(&mut self, val: Cdog1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "DEBUG_MAILBOX."]
    #[must_use]
    #[inline(always)]
    pub const fn debug_mailbox(&self) -> DebugMailbox {
        let val = (self.0 >> 24usize) & 0x03;
        DebugMailbox::from_bits(val as u8)
    }
    #[doc = "DEBUG_MAILBOX."]
    #[inline(always)]
    pub const fn set_debug_mailbox(&mut self, val: DebugMailbox) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "NPU."]
    #[must_use]
    #[inline(always)]
    pub const fn npu(&self) -> Npu {
        let val = (self.0 >> 28usize) & 0x03;
        Npu::from_bits(val as u8)
    }
    #[doc = "NPU."]
    #[inline(always)]
    pub const fn set_npu(&mut self, val: Npu) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AhbPeripheral1SlavePortP13SlaveRule1 {
    #[inline(always)]
    fn default() -> AhbPeripheral1SlavePortP13SlaveRule1 {
        AhbPeripheral1SlavePortP13SlaveRule1(0)
    }
}
impl core::fmt::Debug for AhbPeripheral1SlavePortP13SlaveRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral1SlavePortP13SlaveRule1")
            .field("flexcomm[0]", &self.flexcomm(0usize))
            .field("flexcomm[1]", &self.flexcomm(1usize))
            .field("flexcomm[2]", &self.flexcomm(2usize))
            .field("usb_fs_otg_ram", &self.usb_fs_otg_ram())
            .field("cdog0", &self.cdog0())
            .field("cdog1", &self.cdog1())
            .field("debug_mailbox", &self.debug_mailbox())
            .field("npu", &self.npu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral1SlavePortP13SlaveRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral1SlavePortP13SlaveRule1 {{ flexcomm[0]: {:?}, flexcomm[1]: {:?}, flexcomm[2]: {:?}, usb_fs_otg_ram: {:?}, cdog0: {:?}, cdog1: {:?}, debug_mailbox: {:?}, npu: {:?} }}",
            self.flexcomm(0usize),
            self.flexcomm(1usize),
            self.flexcomm(2usize),
            self.usb_fs_otg_ram(),
            self.cdog0(),
            self.cdog1(),
            self.debug_mailbox(),
            self.npu()
        )
    }
}
#[doc = "AHB Peripheral 1 Slave Port 13 Slave Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral1SlavePortP13SlaveRule2(pub u32);
impl AhbPeripheral1SlavePortP13SlaveRule2 {
    #[doc = "POWERQUAD."]
    #[must_use]
    #[inline(always)]
    pub const fn powerquad(&self) -> Powerquad {
        let val = (self.0 >> 0usize) & 0x03;
        Powerquad::from_bits(val as u8)
    }
    #[doc = "POWERQUAD."]
    #[inline(always)]
    pub const fn set_powerquad(&mut self, val: Powerquad) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for AhbPeripheral1SlavePortP13SlaveRule2 {
    #[inline(always)]
    fn default() -> AhbPeripheral1SlavePortP13SlaveRule2 {
        AhbPeripheral1SlavePortP13SlaveRule2(0)
    }
}
impl core::fmt::Debug for AhbPeripheral1SlavePortP13SlaveRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral1SlavePortP13SlaveRule2")
            .field("powerquad", &self.powerquad())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral1SlavePortP13SlaveRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral1SlavePortP13SlaveRule2 {{ powerquad: {:?} }}",
            self.powerquad()
        )
    }
}
#[doc = "AHB Secure Control Peripheral Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbSecureCtrlPeripheralRule0(pub u32);
impl AhbSecureCtrlPeripheralRule0 {
    #[doc = "Rule."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> AhbSecureCtrlPeripheralRule0Rule {
        assert!(n < 4usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        AhbSecureCtrlPeripheralRule0Rule::from_bits(val as u8)
    }
    #[doc = "Rule."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: AhbSecureCtrlPeripheralRule0Rule) {
        assert!(n < 4usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for AhbSecureCtrlPeripheralRule0 {
    #[inline(always)]
    fn default() -> AhbSecureCtrlPeripheralRule0 {
        AhbSecureCtrlPeripheralRule0(0)
    }
}
impl core::fmt::Debug for AhbSecureCtrlPeripheralRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbSecureCtrlPeripheralRule0")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbSecureCtrlPeripheralRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbSecureCtrlPeripheralRule0 {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize)
        )
    }
}
#[doc = "AIPS Bridge Group 0 Memory Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup0MemRule0(pub u32);
impl AipsBridgeGroup0MemRule0 {
    #[doc = "GPIO5_ALIAS0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio5_alias0(&self) -> Gpio5Alias0 {
        let val = (self.0 >> 0usize) & 0x03;
        Gpio5Alias0::from_bits(val as u8)
    }
    #[doc = "GPIO5_ALIAS0."]
    #[inline(always)]
    pub const fn set_gpio5_alias0(&mut self, val: Gpio5Alias0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO5_ALIAS2."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio5_alias1(&self) -> Gpio5Alias1 {
        let val = (self.0 >> 4usize) & 0x03;
        Gpio5Alias1::from_bits(val as u8)
    }
    #[doc = "GPIO5_ALIAS2."]
    #[inline(always)]
    pub const fn set_gpio5_alias1(&mut self, val: Gpio5Alias1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "PORT5."]
    #[must_use]
    #[inline(always)]
    pub const fn port5(&self) -> Port5 {
        let val = (self.0 >> 8usize) & 0x03;
        Port5::from_bits(val as u8)
    }
    #[doc = "PORT5."]
    #[inline(always)]
    pub const fn set_port5(&mut self, val: Port5) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "FMU0."]
    #[must_use]
    #[inline(always)]
    pub const fn fmu0(&self) -> Fmu0 {
        let val = (self.0 >> 12usize) & 0x03;
        Fmu0::from_bits(val as u8)
    }
    #[doc = "FMU0."]
    #[inline(always)]
    pub const fn set_fmu0(&mut self, val: Fmu0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "SCG0."]
    #[must_use]
    #[inline(always)]
    pub const fn scg0(&self) -> Scg0 {
        let val = (self.0 >> 16usize) & 0x03;
        Scg0::from_bits(val as u8)
    }
    #[doc = "SCG0."]
    #[inline(always)]
    pub const fn set_scg0(&mut self, val: Scg0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "SPC0."]
    #[must_use]
    #[inline(always)]
    pub const fn spc0(&self) -> Spc0 {
        let val = (self.0 >> 20usize) & 0x03;
        Spc0::from_bits(val as u8)
    }
    #[doc = "SPC0."]
    #[inline(always)]
    pub const fn set_spc0(&mut self, val: Spc0) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "WUU0."]
    #[must_use]
    #[inline(always)]
    pub const fn wuu0(&self) -> Wuu0 {
        let val = (self.0 >> 24usize) & 0x03;
        Wuu0::from_bits(val as u8)
    }
    #[doc = "WUU0."]
    #[inline(always)]
    pub const fn set_wuu0(&mut self, val: Wuu0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "TRO0."]
    #[must_use]
    #[inline(always)]
    pub const fn tro0(&self) -> Tro0 {
        let val = (self.0 >> 28usize) & 0x03;
        Tro0::from_bits(val as u8)
    }
    #[doc = "TRO0."]
    #[inline(always)]
    pub const fn set_tro0(&mut self, val: Tro0) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup0MemRule0 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup0MemRule0 {
        AipsBridgeGroup0MemRule0(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup0MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup0MemRule0")
            .field("gpio5_alias0", &self.gpio5_alias0())
            .field("gpio5_alias1", &self.gpio5_alias1())
            .field("port5", &self.port5())
            .field("fmu0", &self.fmu0())
            .field("scg0", &self.scg0())
            .field("spc0", &self.spc0())
            .field("wuu0", &self.wuu0())
            .field("tro0", &self.tro0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup0MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup0MemRule0 {{ gpio5_alias0: {:?}, gpio5_alias1: {:?}, port5: {:?}, fmu0: {:?}, scg0: {:?}, spc0: {:?}, wuu0: {:?}, tro0: {:?} }}",
            self.gpio5_alias0(),
            self.gpio5_alias1(),
            self.port5(),
            self.fmu0(),
            self.scg0(),
            self.spc0(),
            self.wuu0(),
            self.tro0()
        )
    }
}
#[doc = "AIPS Bridge Group 0 Memory Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup0MemRule1(pub u32);
impl AipsBridgeGroup0MemRule1 {
    #[doc = "LPTMR0."]
    #[must_use]
    #[inline(always)]
    pub const fn lptmr0(&self) -> Lptmr0 {
        let val = (self.0 >> 8usize) & 0x03;
        Lptmr0::from_bits(val as u8)
    }
    #[doc = "LPTMR0."]
    #[inline(always)]
    pub const fn set_lptmr0(&mut self, val: Lptmr0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LPTMR1."]
    #[must_use]
    #[inline(always)]
    pub const fn lptmr1(&self) -> Lptmr1 {
        let val = (self.0 >> 12usize) & 0x03;
        Lptmr1::from_bits(val as u8)
    }
    #[doc = "LPTMR1."]
    #[inline(always)]
    pub const fn set_lptmr1(&mut self, val: Lptmr1) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "RTC."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc(&self) -> Rtc {
        let val = (self.0 >> 16usize) & 0x03;
        Rtc::from_bits(val as u8)
    }
    #[doc = "RTC."]
    #[inline(always)]
    pub const fn set_rtc(&mut self, val: Rtc) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "FMU_TEST."]
    #[must_use]
    #[inline(always)]
    pub const fn fmu_test(&self) -> FmuTest {
        let val = (self.0 >> 24usize) & 0x03;
        FmuTest::from_bits(val as u8)
    }
    #[doc = "FMU_TEST."]
    #[inline(always)]
    pub const fn set_fmu_test(&mut self, val: FmuTest) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for AipsBridgeGroup0MemRule1 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup0MemRule1 {
        AipsBridgeGroup0MemRule1(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup0MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup0MemRule1")
            .field("lptmr0", &self.lptmr0())
            .field("lptmr1", &self.lptmr1())
            .field("rtc", &self.rtc())
            .field("fmu_test", &self.fmu_test())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup0MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup0MemRule1 {{ lptmr0: {:?}, lptmr1: {:?}, rtc: {:?}, fmu_test: {:?} }}",
            self.lptmr0(),
            self.lptmr1(),
            self.rtc(),
            self.fmu_test()
        )
    }
}
#[doc = "AIPS Bridge Group 0 Memory Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup0MemRule2(pub u32);
impl AipsBridgeGroup0MemRule2 {
    #[doc = "TSI."]
    #[must_use]
    #[inline(always)]
    pub const fn tsi(&self) -> Tsi {
        let val = (self.0 >> 0usize) & 0x03;
        Tsi::from_bits(val as u8)
    }
    #[doc = "TSI."]
    #[inline(always)]
    pub const fn set_tsi(&mut self, val: Tsi) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CMP."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp(&self, n: usize) -> Cmp {
        assert!(n < 3usize);
        let offs = 4usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Cmp::from_bits(val as u8)
    }
    #[doc = "CMP."]
    #[inline(always)]
    pub const fn set_cmp(&mut self, n: usize, val: Cmp) {
        assert!(n < 3usize);
        let offs = 4usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
    #[doc = "ELS."]
    #[must_use]
    #[inline(always)]
    pub const fn els(&self) -> Els {
        let val = (self.0 >> 16usize) & 0x03;
        Els::from_bits(val as u8)
    }
    #[doc = "ELS."]
    #[inline(always)]
    pub const fn set_els(&mut self, val: Els) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "ELS_ALIAS."]
    #[must_use]
    #[inline(always)]
    pub const fn els_alias(&self, n: usize) -> ElsAlias {
        assert!(n < 3usize);
        let offs = 20usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        ElsAlias::from_bits(val as u8)
    }
    #[doc = "ELS_ALIAS."]
    #[inline(always)]
    pub const fn set_els_alias(&mut self, n: usize, val: ElsAlias) {
        assert!(n < 3usize);
        let offs = 20usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for AipsBridgeGroup0MemRule2 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup0MemRule2 {
        AipsBridgeGroup0MemRule2(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup0MemRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup0MemRule2")
            .field("tsi", &self.tsi())
            .field("cmp[0]", &self.cmp(0usize))
            .field("cmp[1]", &self.cmp(1usize))
            .field("cmp[2]", &self.cmp(2usize))
            .field("els", &self.els())
            .field("els_alias[0]", &self.els_alias(0usize))
            .field("els_alias[1]", &self.els_alias(1usize))
            .field("els_alias[2]", &self.els_alias(2usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup0MemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup0MemRule2 {{ tsi: {:?}, cmp[0]: {:?}, cmp[1]: {:?}, cmp[2]: {:?}, els: {:?}, els_alias[0]: {:?}, els_alias[1]: {:?}, els_alias[2]: {:?} }}",
            self.tsi(),
            self.cmp(0usize),
            self.cmp(1usize),
            self.cmp(2usize),
            self.els(),
            self.els_alias(0usize),
            self.els_alias(1usize),
            self.els_alias(2usize)
        )
    }
}
#[doc = "AIPS Bridge Group 0 Memory Rule 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup0MemRule3(pub u32);
impl AipsBridgeGroup0MemRule3 {
    #[doc = "DIGTMP."]
    #[must_use]
    #[inline(always)]
    pub const fn digtmp(&self) -> Digtmp {
        let val = (self.0 >> 0usize) & 0x03;
        Digtmp::from_bits(val as u8)
    }
    #[doc = "DIGTMP."]
    #[inline(always)]
    pub const fn set_digtmp(&mut self, val: Digtmp) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "VBAT."]
    #[must_use]
    #[inline(always)]
    pub const fn vbat(&self) -> Vbat {
        let val = (self.0 >> 4usize) & 0x03;
        Vbat::from_bits(val as u8)
    }
    #[doc = "VBAT."]
    #[inline(always)]
    pub const fn set_vbat(&mut self, val: Vbat) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "TRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn trng(&self) -> Trng {
        let val = (self.0 >> 8usize) & 0x03;
        Trng::from_bits(val as u8)
    }
    #[doc = "TRNG."]
    #[inline(always)]
    pub const fn set_trng(&mut self, val: Trng) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "EIM0."]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> Eim0 {
        let val = (self.0 >> 12usize) & 0x03;
        Eim0::from_bits(val as u8)
    }
    #[doc = "EIM0."]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: Eim0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "ERM0."]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> Erm0 {
        let val = (self.0 >> 16usize) & 0x03;
        Erm0::from_bits(val as u8)
    }
    #[doc = "ERM0."]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: Erm0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "INTM0."]
    #[must_use]
    #[inline(always)]
    pub const fn intm0(&self) -> Intm0 {
        let val = (self.0 >> 20usize) & 0x03;
        Intm0::from_bits(val as u8)
    }
    #[doc = "INTM0."]
    #[inline(always)]
    pub const fn set_intm0(&mut self, val: Intm0) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
}
impl Default for AipsBridgeGroup0MemRule3 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup0MemRule3 {
        AipsBridgeGroup0MemRule3(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup0MemRule3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup0MemRule3")
            .field("digtmp", &self.digtmp())
            .field("vbat", &self.vbat())
            .field("trng", &self.trng())
            .field("eim0", &self.eim0())
            .field("erm0", &self.erm0())
            .field("intm0", &self.intm0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup0MemRule3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup0MemRule3 {{ digtmp: {:?}, vbat: {:?}, trng: {:?}, eim0: {:?}, erm0: {:?}, intm0: {:?} }}",
            self.digtmp(),
            self.vbat(),
            self.trng(),
            self.eim0(),
            self.erm0(),
            self.intm0()
        )
    }
}
#[doc = "AIPS Bridge Group 1 Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup1MemRule0(pub u32);
impl AipsBridgeGroup1MemRule0 {
    #[doc = "eDMA0_MP."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_mp(&self) -> EDma0Mp {
        let val = (self.0 >> 0usize) & 0x03;
        EDma0Mp::from_bits(val as u8)
    }
    #[doc = "eDMA0_MP."]
    #[inline(always)]
    pub const fn set_e_dma0_mp(&mut self, val: EDma0Mp) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "eDMA0_CH0."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch0(&self) -> EDma0Ch0 {
        let val = (self.0 >> 4usize) & 0x03;
        EDma0Ch0::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH0."]
    #[inline(always)]
    pub const fn set_e_dma0_ch0(&mut self, val: EDma0Ch0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "eDMA0_CH1."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch1(&self) -> EDma0Ch1 {
        let val = (self.0 >> 8usize) & 0x03;
        EDma0Ch1::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH1."]
    #[inline(always)]
    pub const fn set_e_dma0_ch1(&mut self, val: EDma0Ch1) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "eDMA0_CH2."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch2(&self) -> EDma0Ch2 {
        let val = (self.0 >> 12usize) & 0x03;
        EDma0Ch2::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH2."]
    #[inline(always)]
    pub const fn set_e_dma0_ch2(&mut self, val: EDma0Ch2) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "FLEXSPI0 Registers."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch3(&self) -> EDma0Ch3 {
        let val = (self.0 >> 16usize) & 0x03;
        EDma0Ch3::from_bits(val as u8)
    }
    #[doc = "FLEXSPI0 Registers."]
    #[inline(always)]
    pub const fn set_e_dma0_ch3(&mut self, val: EDma0Ch3) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "eDMA0_CH4."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch4(&self) -> EDma0Ch4 {
        let val = (self.0 >> 20usize) & 0x03;
        EDma0Ch4::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH4."]
    #[inline(always)]
    pub const fn set_e_dma0_ch4(&mut self, val: EDma0Ch4) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "eDMA0_CH5."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch5(&self) -> EDma0Ch5 {
        let val = (self.0 >> 24usize) & 0x03;
        EDma0Ch5::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH5."]
    #[inline(always)]
    pub const fn set_e_dma0_ch5(&mut self, val: EDma0Ch5) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "eDMA0_CH6."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch6(&self) -> EDma0Ch6 {
        let val = (self.0 >> 28usize) & 0x03;
        EDma0Ch6::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH6."]
    #[inline(always)]
    pub const fn set_e_dma0_ch6(&mut self, val: EDma0Ch6) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup1MemRule0 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup1MemRule0 {
        AipsBridgeGroup1MemRule0(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup1MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup1MemRule0")
            .field("e_dma0_mp", &self.e_dma0_mp())
            .field("e_dma0_ch0", &self.e_dma0_ch0())
            .field("e_dma0_ch1", &self.e_dma0_ch1())
            .field("e_dma0_ch2", &self.e_dma0_ch2())
            .field("e_dma0_ch3", &self.e_dma0_ch3())
            .field("e_dma0_ch4", &self.e_dma0_ch4())
            .field("e_dma0_ch5", &self.e_dma0_ch5())
            .field("e_dma0_ch6", &self.e_dma0_ch6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup1MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup1MemRule0 {{ e_dma0_mp: {:?}, e_dma0_ch0: {:?}, e_dma0_ch1: {:?}, e_dma0_ch2: {:?}, e_dma0_ch3: {:?}, e_dma0_ch4: {:?}, e_dma0_ch5: {:?}, e_dma0_ch6: {:?} }}",
            self.e_dma0_mp(),
            self.e_dma0_ch0(),
            self.e_dma0_ch1(),
            self.e_dma0_ch2(),
            self.e_dma0_ch3(),
            self.e_dma0_ch4(),
            self.e_dma0_ch5(),
            self.e_dma0_ch6()
        )
    }
}
#[doc = "AIPS Bridge Group 1 Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup1MemRule1(pub u32);
impl AipsBridgeGroup1MemRule1 {
    #[doc = "eDMA0_CH7."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch7(&self) -> EDma0Ch7 {
        let val = (self.0 >> 0usize) & 0x03;
        EDma0Ch7::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH7."]
    #[inline(always)]
    pub const fn set_e_dma0_ch7(&mut self, val: EDma0Ch7) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "eDMA0_CH8."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch8(&self) -> EDma0Ch8 {
        let val = (self.0 >> 4usize) & 0x03;
        EDma0Ch8::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH8."]
    #[inline(always)]
    pub const fn set_e_dma0_ch8(&mut self, val: EDma0Ch8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "eDMA0_CH9."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch9(&self) -> EDma0Ch9 {
        let val = (self.0 >> 8usize) & 0x03;
        EDma0Ch9::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH9."]
    #[inline(always)]
    pub const fn set_e_dma0_ch9(&mut self, val: EDma0Ch9) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "eDMA0_CH10."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch10(&self) -> EDma0Ch10 {
        let val = (self.0 >> 12usize) & 0x03;
        EDma0Ch10::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH10."]
    #[inline(always)]
    pub const fn set_e_dma0_ch10(&mut self, val: EDma0Ch10) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "FLEXSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch11(&self) -> EDma0Ch11 {
        let val = (self.0 >> 16usize) & 0x03;
        EDma0Ch11::from_bits(val as u8)
    }
    #[doc = "FLEXSPI0."]
    #[inline(always)]
    pub const fn set_e_dma0_ch11(&mut self, val: EDma0Ch11) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "eDMA0_CH12."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch12(&self) -> EDma0Ch12 {
        let val = (self.0 >> 20usize) & 0x03;
        EDma0Ch12::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH12."]
    #[inline(always)]
    pub const fn set_e_dma0_ch12(&mut self, val: EDma0Ch12) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "eDMA0_CH13."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch13(&self) -> EDma0Ch13 {
        let val = (self.0 >> 24usize) & 0x03;
        EDma0Ch13::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH13."]
    #[inline(always)]
    pub const fn set_e_dma0_ch13(&mut self, val: EDma0Ch13) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "eDMA0_CH14."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0_ch14(&self) -> EDma0Ch14 {
        let val = (self.0 >> 28usize) & 0x03;
        EDma0Ch14::from_bits(val as u8)
    }
    #[doc = "eDMA0_CH14."]
    #[inline(always)]
    pub const fn set_e_dma0_ch14(&mut self, val: EDma0Ch14) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup1MemRule1 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup1MemRule1 {
        AipsBridgeGroup1MemRule1(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup1MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup1MemRule1")
            .field("e_dma0_ch7", &self.e_dma0_ch7())
            .field("e_dma0_ch8", &self.e_dma0_ch8())
            .field("e_dma0_ch9", &self.e_dma0_ch9())
            .field("e_dma0_ch10", &self.e_dma0_ch10())
            .field("e_dma0_ch11", &self.e_dma0_ch11())
            .field("e_dma0_ch12", &self.e_dma0_ch12())
            .field("e_dma0_ch13", &self.e_dma0_ch13())
            .field("e_dma0_ch14", &self.e_dma0_ch14())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup1MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup1MemRule1 {{ e_dma0_ch7: {:?}, e_dma0_ch8: {:?}, e_dma0_ch9: {:?}, e_dma0_ch10: {:?}, e_dma0_ch11: {:?}, e_dma0_ch12: {:?}, e_dma0_ch13: {:?}, e_dma0_ch14: {:?} }}",
            self.e_dma0_ch7(),
            self.e_dma0_ch8(),
            self.e_dma0_ch9(),
            self.e_dma0_ch10(),
            self.e_dma0_ch11(),
            self.e_dma0_ch12(),
            self.e_dma0_ch13(),
            self.e_dma0_ch14()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule0(pub u32);
impl AipsBridgeGroup2MemRule0 {
    #[doc = "eDMA1_MP."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma1_mp(&self) -> EDma1Mp {
        let val = (self.0 >> 0usize) & 0x03;
        EDma1Mp::from_bits(val as u8)
    }
    #[doc = "eDMA1_MP."]
    #[inline(always)]
    pub const fn set_e_dma1_mp(&mut self, val: EDma1Mp) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "eDMA1_CH."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma1_ch(&self, n: usize) -> EDma1Ch {
        assert!(n < 7usize);
        let offs = 4usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        EDma1Ch::from_bits(val as u8)
    }
    #[doc = "eDMA1_CH."]
    #[inline(always)]
    pub const fn set_e_dma1_ch(&mut self, n: usize, val: EDma1Ch) {
        assert!(n < 7usize);
        let offs = 4usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for AipsBridgeGroup2MemRule0 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule0 {
        AipsBridgeGroup2MemRule0(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule0")
            .field("e_dma1_mp", &self.e_dma1_mp())
            .field("e_dma1_ch[0]", &self.e_dma1_ch(0usize))
            .field("e_dma1_ch[1]", &self.e_dma1_ch(1usize))
            .field("e_dma1_ch[2]", &self.e_dma1_ch(2usize))
            .field("e_dma1_ch[3]", &self.e_dma1_ch(3usize))
            .field("e_dma1_ch[4]", &self.e_dma1_ch(4usize))
            .field("e_dma1_ch[5]", &self.e_dma1_ch(5usize))
            .field("e_dma1_ch[6]", &self.e_dma1_ch(6usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule0 {{ e_dma1_mp: {:?}, e_dma1_ch[0]: {:?}, e_dma1_ch[1]: {:?}, e_dma1_ch[2]: {:?}, e_dma1_ch[3]: {:?}, e_dma1_ch[4]: {:?}, e_dma1_ch[5]: {:?}, e_dma1_ch[6]: {:?} }}",
            self.e_dma1_mp(),
            self.e_dma1_ch(0usize),
            self.e_dma1_ch(1usize),
            self.e_dma1_ch(2usize),
            self.e_dma1_ch(3usize),
            self.e_dma1_ch(4usize),
            self.e_dma1_ch(5usize),
            self.e_dma1_ch(6usize)
        )
    }
}
#[doc = "AIPS Bridge Group 2 Memory Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule1(pub u32);
impl AipsBridgeGroup2MemRule1 {
    #[doc = "eDMA1_CH."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma1_ch(&self, n: usize) -> EDma1Ch {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        EDma1Ch::from_bits(val as u8)
    }
    #[doc = "eDMA1_CH."]
    #[inline(always)]
    pub const fn set_e_dma1_ch(&mut self, n: usize, val: EDma1Ch) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for AipsBridgeGroup2MemRule1 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule1 {
        AipsBridgeGroup2MemRule1(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule1")
            .field("e_dma1_ch[0]", &self.e_dma1_ch(0usize))
            .field("e_dma1_ch[1]", &self.e_dma1_ch(1usize))
            .field("e_dma1_ch[2]", &self.e_dma1_ch(2usize))
            .field("e_dma1_ch[3]", &self.e_dma1_ch(3usize))
            .field("e_dma1_ch[4]", &self.e_dma1_ch(4usize))
            .field("e_dma1_ch[5]", &self.e_dma1_ch(5usize))
            .field("e_dma1_ch[6]", &self.e_dma1_ch(6usize))
            .field("e_dma1_ch[7]", &self.e_dma1_ch(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule1 {{ e_dma1_ch[0]: {:?}, e_dma1_ch[1]: {:?}, e_dma1_ch[2]: {:?}, e_dma1_ch[3]: {:?}, e_dma1_ch[4]: {:?}, e_dma1_ch[5]: {:?}, e_dma1_ch[6]: {:?}, e_dma1_ch[7]: {:?} }}",
            self.e_dma1_ch(0usize),
            self.e_dma1_ch(1usize),
            self.e_dma1_ch(2usize),
            self.e_dma1_ch(3usize),
            self.e_dma1_ch(4usize),
            self.e_dma1_ch(5usize),
            self.e_dma1_ch(6usize),
            self.e_dma1_ch(7usize)
        )
    }
}
#[doc = "AIPS Bridge Group 3 Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup3MemRule0(pub u32);
impl AipsBridgeGroup3MemRule0 {
    #[doc = "EWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn ewm0(&self) -> Ewm0 {
        let val = (self.0 >> 0usize) & 0x03;
        Ewm0::from_bits(val as u8)
    }
    #[doc = "EWM0."]
    #[inline(always)]
    pub const fn set_ewm0(&mut self, val: Ewm0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "LPCAC."]
    #[must_use]
    #[inline(always)]
    pub const fn lpcac(&self) -> Lpcac {
        let val = (self.0 >> 4usize) & 0x03;
        Lpcac::from_bits(val as u8)
    }
    #[doc = "LPCAC."]
    #[inline(always)]
    pub const fn set_lpcac(&mut self, val: Lpcac) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "FLEXSPI_CMX."]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_cmx(&self) -> FlexspiCmx {
        let val = (self.0 >> 8usize) & 0x03;
        FlexspiCmx::from_bits(val as u8)
    }
    #[doc = "FLEXSPI_CMX."]
    #[inline(always)]
    pub const fn set_flexspi_cmx(&mut self, val: FlexspiCmx) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "SFA."]
    #[must_use]
    #[inline(always)]
    pub const fn sfa(&self) -> Sfa {
        let val = (self.0 >> 20usize) & 0x03;
        Sfa::from_bits(val as u8)
    }
    #[doc = "SFA."]
    #[inline(always)]
    pub const fn set_sfa(&mut self, val: Sfa) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "MBC."]
    #[must_use]
    #[inline(always)]
    pub const fn mbc(&self) -> Mbc {
        let val = (self.0 >> 28usize) & 0x03;
        Mbc::from_bits(val as u8)
    }
    #[doc = "MBC."]
    #[inline(always)]
    pub const fn set_mbc(&mut self, val: Mbc) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup3MemRule0 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup3MemRule0 {
        AipsBridgeGroup3MemRule0(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup3MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup3MemRule0")
            .field("ewm0", &self.ewm0())
            .field("lpcac", &self.lpcac())
            .field("flexspi_cmx", &self.flexspi_cmx())
            .field("sfa", &self.sfa())
            .field("mbc", &self.mbc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup3MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup3MemRule0 {{ ewm0: {:?}, lpcac: {:?}, flexspi_cmx: {:?}, sfa: {:?}, mbc: {:?} }}",
            self.ewm0(),
            self.lpcac(),
            self.flexspi_cmx(),
            self.sfa(),
            self.mbc()
        )
    }
}
#[doc = "AIPS Bridge Group 3 Memory Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup3MemRule1(pub u32);
impl AipsBridgeGroup3MemRule1 {
    #[doc = "FLEXSPI."]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi(&self) -> Flexspi {
        let val = (self.0 >> 0usize) & 0x03;
        Flexspi::from_bits(val as u8)
    }
    #[doc = "FLEXSPI."]
    #[inline(always)]
    pub const fn set_flexspi(&mut self, val: Flexspi) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "OTPC."]
    #[must_use]
    #[inline(always)]
    pub const fn otpc(&self) -> Otpc {
        let val = (self.0 >> 4usize) & 0x03;
        Otpc::from_bits(val as u8)
    }
    #[doc = "OTPC."]
    #[inline(always)]
    pub const fn set_otpc(&mut self, val: Otpc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CRC."]
    #[must_use]
    #[inline(always)]
    pub const fn crc(&self) -> Crc {
        let val = (self.0 >> 12usize) & 0x03;
        Crc::from_bits(val as u8)
    }
    #[doc = "CRC."]
    #[inline(always)]
    pub const fn set_crc(&mut self, val: Crc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "NPX."]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> Npx {
        let val = (self.0 >> 16usize) & 0x03;
        Npx::from_bits(val as u8)
    }
    #[doc = "NPX."]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: Npx) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "PWM."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm(&self) -> Pwm {
        let val = (self.0 >> 24usize) & 0x03;
        Pwm::from_bits(val as u8)
    }
    #[doc = "PWM."]
    #[inline(always)]
    pub const fn set_pwm(&mut self, val: Pwm) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "ENC."]
    #[must_use]
    #[inline(always)]
    pub const fn enc(&self) -> Enc {
        let val = (self.0 >> 28usize) & 0x03;
        Enc::from_bits(val as u8)
    }
    #[doc = "ENC."]
    #[inline(always)]
    pub const fn set_enc(&mut self, val: Enc) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup3MemRule1 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup3MemRule1 {
        AipsBridgeGroup3MemRule1(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup3MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup3MemRule1")
            .field("flexspi", &self.flexspi())
            .field("otpc", &self.otpc())
            .field("crc", &self.crc())
            .field("npx", &self.npx())
            .field("pwm", &self.pwm())
            .field("enc", &self.enc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup3MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup3MemRule1 {{ flexspi: {:?}, otpc: {:?}, crc: {:?}, npx: {:?}, pwm: {:?}, enc: {:?} }}",
            self.flexspi(),
            self.otpc(),
            self.crc(),
            self.npx(),
            self.pwm(),
            self.enc()
        )
    }
}
#[doc = "AIPS Bridge Group 3 Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup3MemRule2(pub u32);
impl AipsBridgeGroup3MemRule2 {
    #[doc = "PWM1."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm1(&self) -> Pwm1 {
        let val = (self.0 >> 0usize) & 0x03;
        Pwm1::from_bits(val as u8)
    }
    #[doc = "PWM1."]
    #[inline(always)]
    pub const fn set_pwm1(&mut self, val: Pwm1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "ENC1."]
    #[must_use]
    #[inline(always)]
    pub const fn enc1(&self) -> Enc1 {
        let val = (self.0 >> 4usize) & 0x03;
        Enc1::from_bits(val as u8)
    }
    #[doc = "ENC1."]
    #[inline(always)]
    pub const fn set_enc1(&mut self, val: Enc1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "EVTG."]
    #[must_use]
    #[inline(always)]
    pub const fn evtg(&self) -> Evtg {
        let val = (self.0 >> 8usize) & 0x03;
        Evtg::from_bits(val as u8)
    }
    #[doc = "EVTG."]
    #[inline(always)]
    pub const fn set_evtg(&mut self, val: Evtg) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CAN0 RULE."]
    #[must_use]
    #[inline(always)]
    pub const fn can0_rule(&self, n: usize) -> Can0Rule {
        assert!(n < 4usize);
        let offs = 16usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Can0Rule::from_bits(val as u8)
    }
    #[doc = "CAN0 RULE."]
    #[inline(always)]
    pub const fn set_can0_rule(&mut self, n: usize, val: Can0Rule) {
        assert!(n < 4usize);
        let offs = 16usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for AipsBridgeGroup3MemRule2 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup3MemRule2 {
        AipsBridgeGroup3MemRule2(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup3MemRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup3MemRule2")
            .field("pwm1", &self.pwm1())
            .field("enc1", &self.enc1())
            .field("evtg", &self.evtg())
            .field("can0_rule[0]", &self.can0_rule(0usize))
            .field("can0_rule[1]", &self.can0_rule(1usize))
            .field("can0_rule[2]", &self.can0_rule(2usize))
            .field("can0_rule[3]", &self.can0_rule(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup3MemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup3MemRule2 {{ pwm1: {:?}, enc1: {:?}, evtg: {:?}, can0_rule[0]: {:?}, can0_rule[1]: {:?}, can0_rule[2]: {:?}, can0_rule[3]: {:?} }}",
            self.pwm1(),
            self.enc1(),
            self.evtg(),
            self.can0_rule(0usize),
            self.can0_rule(1usize),
            self.can0_rule(2usize),
            self.can0_rule(3usize)
        )
    }
}
#[doc = "AIPS Bridge Group 3 Rule 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup3MemRule3(pub u32);
impl AipsBridgeGroup3MemRule3 {
    #[doc = "CAN1 RULE."]
    #[must_use]
    #[inline(always)]
    pub const fn can1_rule(&self, n: usize) -> Can1Rule {
        assert!(n < 4usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Can1Rule::from_bits(val as u8)
    }
    #[doc = "CAN1 RULE."]
    #[inline(always)]
    pub const fn set_can1_rule(&mut self, n: usize, val: Can1Rule) {
        assert!(n < 4usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
    #[doc = "USBDCD."]
    #[must_use]
    #[inline(always)]
    pub const fn usbdcd(&self) -> Usbdcd {
        let val = (self.0 >> 16usize) & 0x03;
        Usbdcd::from_bits(val as u8)
    }
    #[doc = "USBDCD."]
    #[inline(always)]
    pub const fn set_usbdcd(&mut self, val: Usbdcd) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "USBFS."]
    #[must_use]
    #[inline(always)]
    pub const fn usbfs(&self) -> Usbfs {
        let val = (self.0 >> 20usize) & 0x03;
        Usbfs::from_bits(val as u8)
    }
    #[doc = "USBFS."]
    #[inline(always)]
    pub const fn set_usbfs(&mut self, val: Usbfs) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
}
impl Default for AipsBridgeGroup3MemRule3 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup3MemRule3 {
        AipsBridgeGroup3MemRule3(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup3MemRule3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup3MemRule3")
            .field("can1_rule[0]", &self.can1_rule(0usize))
            .field("can1_rule[1]", &self.can1_rule(1usize))
            .field("can1_rule[2]", &self.can1_rule(2usize))
            .field("can1_rule[3]", &self.can1_rule(3usize))
            .field("usbdcd", &self.usbdcd())
            .field("usbfs", &self.usbfs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup3MemRule3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup3MemRule3 {{ can1_rule[0]: {:?}, can1_rule[1]: {:?}, can1_rule[2]: {:?}, can1_rule[3]: {:?}, usbdcd: {:?}, usbfs: {:?} }}",
            self.can1_rule(0usize),
            self.can1_rule(1usize),
            self.can1_rule(2usize),
            self.can1_rule(3usize),
            self.usbdcd(),
            self.usbfs()
        )
    }
}
#[doc = "AIPS Bridge Group 4 Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup4MemRule0(pub u32);
impl AipsBridgeGroup4MemRule0 {
    #[doc = "ENET."]
    #[must_use]
    #[inline(always)]
    pub const fn enet(&self) -> Enet {
        let val = (self.0 >> 0usize) & 0x0f;
        Enet::from_bits(val as u8)
    }
    #[doc = "ENET."]
    #[inline(always)]
    pub const fn set_enet(&mut self, val: Enet) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "EMVSIM0."]
    #[must_use]
    #[inline(always)]
    pub const fn emvsim0(&self) -> Emvsim0 {
        let val = (self.0 >> 12usize) & 0x03;
        Emvsim0::from_bits(val as u8)
    }
    #[doc = "EMVSIM0."]
    #[inline(always)]
    pub const fn set_emvsim0(&mut self, val: Emvsim0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "EMVSIM1."]
    #[must_use]
    #[inline(always)]
    pub const fn emvsim1(&self) -> Emvsim1 {
        let val = (self.0 >> 16usize) & 0x03;
        Emvsim1::from_bits(val as u8)
    }
    #[doc = "EMVSIM1."]
    #[inline(always)]
    pub const fn set_emvsim1(&mut self, val: Emvsim1) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "FLEXIO."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio(&self) -> Flexio {
        let val = (self.0 >> 20usize) & 0x03;
        Flexio::from_bits(val as u8)
    }
    #[doc = "FLEXIO."]
    #[inline(always)]
    pub const fn set_flexio(&mut self, val: Flexio) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "SAI0."]
    #[must_use]
    #[inline(always)]
    pub const fn sai0(&self) -> Sai0 {
        let val = (self.0 >> 24usize) & 0x03;
        Sai0::from_bits(val as u8)
    }
    #[doc = "SAI0."]
    #[inline(always)]
    pub const fn set_sai0(&mut self, val: Sai0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "SAI1."]
    #[must_use]
    #[inline(always)]
    pub const fn sai1(&self) -> Sai1 {
        let val = (self.0 >> 28usize) & 0x03;
        Sai1::from_bits(val as u8)
    }
    #[doc = "SAI1."]
    #[inline(always)]
    pub const fn set_sai1(&mut self, val: Sai1) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup4MemRule0 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup4MemRule0 {
        AipsBridgeGroup4MemRule0(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup4MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup4MemRule0")
            .field("enet", &self.enet())
            .field("emvsim0", &self.emvsim0())
            .field("emvsim1", &self.emvsim1())
            .field("flexio", &self.flexio())
            .field("sai0", &self.sai0())
            .field("sai1", &self.sai1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup4MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup4MemRule0 {{ enet: {:?}, emvsim0: {:?}, emvsim1: {:?}, flexio: {:?}, sai0: {:?}, sai1: {:?} }}",
            self.enet(),
            self.emvsim0(),
            self.emvsim1(),
            self.flexio(),
            self.sai0(),
            self.sai1()
        )
    }
}
#[doc = "AIPS Bridge Group 4 Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup4MemRule1(pub u32);
impl AipsBridgeGroup4MemRule1 {
    #[doc = "SINC0."]
    #[must_use]
    #[inline(always)]
    pub const fn sinc0(&self) -> Sinc0 {
        let val = (self.0 >> 0usize) & 0x03;
        Sinc0::from_bits(val as u8)
    }
    #[doc = "SINC0."]
    #[inline(always)]
    pub const fn set_sinc0(&mut self, val: Sinc0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "uSDHC0."]
    #[must_use]
    #[inline(always)]
    pub const fn u_sdhc0(&self) -> USdhc0 {
        let val = (self.0 >> 4usize) & 0x03;
        USdhc0::from_bits(val as u8)
    }
    #[doc = "uSDHC0."]
    #[inline(always)]
    pub const fn set_u_sdhc0(&mut self, val: USdhc0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "USBHSPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usbhsphy(&self) -> Usbhsphy {
        let val = (self.0 >> 8usize) & 0x03;
        Usbhsphy::from_bits(val as u8)
    }
    #[doc = "USBHSPHY."]
    #[inline(always)]
    pub const fn set_usbhsphy(&mut self, val: Usbhsphy) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "USBHS."]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs(&self) -> Usbhs {
        let val = (self.0 >> 12usize) & 0x03;
        Usbhs::from_bits(val as u8)
    }
    #[doc = "USBHS."]
    #[inline(always)]
    pub const fn set_usbhs(&mut self, val: Usbhs) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "MICD."]
    #[must_use]
    #[inline(always)]
    pub const fn micd(&self) -> Micd {
        let val = (self.0 >> 16usize) & 0x03;
        Micd::from_bits(val as u8)
    }
    #[doc = "MICD."]
    #[inline(always)]
    pub const fn set_micd(&mut self, val: Micd) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "ADC0."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> Adc0 {
        let val = (self.0 >> 20usize) & 0x03;
        Adc0::from_bits(val as u8)
    }
    #[doc = "ADC0."]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: Adc0) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "ADC1."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> Adc1 {
        let val = (self.0 >> 24usize) & 0x03;
        Adc1::from_bits(val as u8)
    }
    #[doc = "ADC1."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: Adc1) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "DAC0."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> Dac0 {
        let val = (self.0 >> 28usize) & 0x03;
        Dac0::from_bits(val as u8)
    }
    #[doc = "DAC0."]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: Dac0) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup4MemRule1 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup4MemRule1 {
        AipsBridgeGroup4MemRule1(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup4MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup4MemRule1")
            .field("sinc0", &self.sinc0())
            .field("u_sdhc0", &self.u_sdhc0())
            .field("usbhsphy", &self.usbhsphy())
            .field("usbhs", &self.usbhs())
            .field("micd", &self.micd())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("dac0", &self.dac0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup4MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup4MemRule1 {{ sinc0: {:?}, u_sdhc0: {:?}, usbhsphy: {:?}, usbhs: {:?}, micd: {:?}, adc0: {:?}, adc1: {:?}, dac0: {:?} }}",
            self.sinc0(),
            self.u_sdhc0(),
            self.usbhsphy(),
            self.usbhs(),
            self.micd(),
            self.adc0(),
            self.adc1(),
            self.dac0()
        )
    }
}
#[doc = "AIPS Bridge Group 4 Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup4MemRule2(pub u32);
impl AipsBridgeGroup4MemRule2 {
    #[doc = "OPAMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp0(&self) -> Opamp0 {
        let val = (self.0 >> 0usize) & 0x03;
        Opamp0::from_bits(val as u8)
    }
    #[doc = "OPAMP0."]
    #[inline(always)]
    pub const fn set_opamp0(&mut self, val: Opamp0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "VREF."]
    #[must_use]
    #[inline(always)]
    pub const fn vref(&self) -> Vref {
        let val = (self.0 >> 4usize) & 0x03;
        Vref::from_bits(val as u8)
    }
    #[doc = "VREF."]
    #[inline(always)]
    pub const fn set_vref(&mut self, val: Vref) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "DAC."]
    #[must_use]
    #[inline(always)]
    pub const fn dac(&self) -> Dac {
        let val = (self.0 >> 8usize) & 0x03;
        Dac::from_bits(val as u8)
    }
    #[doc = "DAC."]
    #[inline(always)]
    pub const fn set_dac(&mut self, val: Dac) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "OPAMP1."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp1(&self) -> Opamp1 {
        let val = (self.0 >> 12usize) & 0x03;
        Opamp1::from_bits(val as u8)
    }
    #[doc = "OPAMP1."]
    #[inline(always)]
    pub const fn set_opamp1(&mut self, val: Opamp1) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "HPDAC0."]
    #[must_use]
    #[inline(always)]
    pub const fn hpdac0(&self) -> Hpdac0 {
        let val = (self.0 >> 16usize) & 0x03;
        Hpdac0::from_bits(val as u8)
    }
    #[doc = "HPDAC0."]
    #[inline(always)]
    pub const fn set_hpdac0(&mut self, val: Hpdac0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "OPAMP2."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp2(&self) -> Opamp2 {
        let val = (self.0 >> 20usize) & 0x03;
        Opamp2::from_bits(val as u8)
    }
    #[doc = "OPAMP2."]
    #[inline(always)]
    pub const fn set_opamp2(&mut self, val: Opamp2) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "PORT0."]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> Port0 {
        let val = (self.0 >> 24usize) & 0x03;
        Port0::from_bits(val as u8)
    }
    #[doc = "PORT0."]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: Port0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "PORT1."]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> Port1 {
        let val = (self.0 >> 28usize) & 0x03;
        Port1::from_bits(val as u8)
    }
    #[doc = "PORT1."]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: Port1) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup4MemRule2 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup4MemRule2 {
        AipsBridgeGroup4MemRule2(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup4MemRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup4MemRule2")
            .field("opamp0", &self.opamp0())
            .field("vref", &self.vref())
            .field("dac", &self.dac())
            .field("opamp1", &self.opamp1())
            .field("hpdac0", &self.hpdac0())
            .field("opamp2", &self.opamp2())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup4MemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup4MemRule2 {{ opamp0: {:?}, vref: {:?}, dac: {:?}, opamp1: {:?}, hpdac0: {:?}, opamp2: {:?}, port0: {:?}, port1: {:?} }}",
            self.opamp0(),
            self.vref(),
            self.dac(),
            self.opamp1(),
            self.hpdac0(),
            self.opamp2(),
            self.port0(),
            self.port1()
        )
    }
}
#[doc = "AIPS Bridge Group 4 Rule 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup4MemRule3(pub u32);
impl AipsBridgeGroup4MemRule3 {
    #[doc = "PORT."]
    #[must_use]
    #[inline(always)]
    pub const fn port(&self, n: usize) -> Port {
        assert!(n < 3usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Port::from_bits(val as u8)
    }
    #[doc = "PORT."]
    #[inline(always)]
    pub const fn set_port(&mut self, n: usize, val: Port) {
        assert!(n < 3usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
    #[doc = "MTR0."]
    #[must_use]
    #[inline(always)]
    pub const fn mtr0(&self) -> Mtr0 {
        let val = (self.0 >> 24usize) & 0x03;
        Mtr0::from_bits(val as u8)
    }
    #[doc = "MTR0."]
    #[inline(always)]
    pub const fn set_mtr0(&mut self, val: Mtr0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "ATX0."]
    #[must_use]
    #[inline(always)]
    pub const fn atx0(&self) -> Atx0 {
        let val = (self.0 >> 28usize) & 0x03;
        Atx0::from_bits(val as u8)
    }
    #[doc = "ATX0."]
    #[inline(always)]
    pub const fn set_atx0(&mut self, val: Atx0) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup4MemRule3 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup4MemRule3 {
        AipsBridgeGroup4MemRule3(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup4MemRule3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup4MemRule3")
            .field("port[0]", &self.port(0usize))
            .field("port[1]", &self.port(1usize))
            .field("port[2]", &self.port(2usize))
            .field("mtr0", &self.mtr0())
            .field("atx0", &self.atx0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup4MemRule3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup4MemRule3 {{ port[0]: {:?}, port[1]: {:?}, port[2]: {:?}, mtr0: {:?}, atx0: {:?} }}",
            self.port(0usize),
            self.port(1usize),
            self.port(2usize),
            self.mtr0(),
            self.atx0()
        )
    }
}
#[doc = "APB Bridge Group 0 Memory Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup0MemRule0(pub u32);
impl ApbPeripheralGroup0MemRule0 {
    #[doc = "SYSCON."]
    #[must_use]
    #[inline(always)]
    pub const fn syscon(&self) -> Syscon {
        let val = (self.0 >> 0usize) & 0x03;
        Syscon::from_bits(val as u8)
    }
    #[doc = "SYSCON."]
    #[inline(always)]
    pub const fn set_syscon(&mut self, val: Syscon) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "PINT0."]
    #[must_use]
    #[inline(always)]
    pub const fn pint0(&self) -> Pint0 {
        let val = (self.0 >> 16usize) & 0x03;
        Pint0::from_bits(val as u8)
    }
    #[doc = "PINT0."]
    #[inline(always)]
    pub const fn set_pint0(&mut self, val: Pint0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "INPUTMUX."]
    #[must_use]
    #[inline(always)]
    pub const fn inputmux(&self) -> Inputmux {
        let val = (self.0 >> 24usize) & 0x03;
        Inputmux::from_bits(val as u8)
    }
    #[doc = "INPUTMUX."]
    #[inline(always)]
    pub const fn set_inputmux(&mut self, val: Inputmux) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for ApbPeripheralGroup0MemRule0 {
    #[inline(always)]
    fn default() -> ApbPeripheralGroup0MemRule0 {
        ApbPeripheralGroup0MemRule0(0)
    }
}
impl core::fmt::Debug for ApbPeripheralGroup0MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbPeripheralGroup0MemRule0")
            .field("syscon", &self.syscon())
            .field("pint0", &self.pint0())
            .field("inputmux", &self.inputmux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup0MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup0MemRule0 {{ syscon: {:?}, pint0: {:?}, inputmux: {:?} }}",
            self.syscon(),
            self.pint0(),
            self.inputmux()
        )
    }
}
#[doc = "APB Bridge Group 0 Memory Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup0MemRule1(pub u32);
impl ApbPeripheralGroup0MemRule1 {
    #[doc = "CTIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer(&self, n: usize) -> Ctimer {
        assert!(n < 4usize);
        let offs = 16usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Ctimer::from_bits(val as u8)
    }
    #[doc = "CTIMER."]
    #[inline(always)]
    pub const fn set_ctimer(&mut self, n: usize, val: Ctimer) {
        assert!(n < 4usize);
        let offs = 16usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for ApbPeripheralGroup0MemRule1 {
    #[inline(always)]
    fn default() -> ApbPeripheralGroup0MemRule1 {
        ApbPeripheralGroup0MemRule1(0)
    }
}
impl core::fmt::Debug for ApbPeripheralGroup0MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbPeripheralGroup0MemRule1")
            .field("ctimer[0]", &self.ctimer(0usize))
            .field("ctimer[1]", &self.ctimer(1usize))
            .field("ctimer[2]", &self.ctimer(2usize))
            .field("ctimer[3]", &self.ctimer(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup0MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup0MemRule1 {{ ctimer[0]: {:?}, ctimer[1]: {:?}, ctimer[2]: {:?}, ctimer[3]: {:?} }}",
            self.ctimer(0usize),
            self.ctimer(1usize),
            self.ctimer(2usize),
            self.ctimer(3usize)
        )
    }
}
#[doc = "APB Bridge Group 0 Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup0MemRule2(pub u32);
impl ApbPeripheralGroup0MemRule2 {
    #[doc = "CTIMER4."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4(&self) -> Ctimer4 {
        let val = (self.0 >> 0usize) & 0x03;
        Ctimer4::from_bits(val as u8)
    }
    #[doc = "CTIMER4."]
    #[inline(always)]
    pub const fn set_ctimer4(&mut self, val: Ctimer4) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "FREQME0."]
    #[must_use]
    #[inline(always)]
    pub const fn freqme0(&self) -> Freqme0 {
        let val = (self.0 >> 4usize) & 0x03;
        Freqme0::from_bits(val as u8)
    }
    #[doc = "FREQME0."]
    #[inline(always)]
    pub const fn set_freqme0(&mut self, val: Freqme0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "UTCIK0."]
    #[must_use]
    #[inline(always)]
    pub const fn utcik0(&self) -> Utcik0 {
        let val = (self.0 >> 8usize) & 0x03;
        Utcik0::from_bits(val as u8)
    }
    #[doc = "UTCIK0."]
    #[inline(always)]
    pub const fn set_utcik0(&mut self, val: Utcik0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "MRT0."]
    #[must_use]
    #[inline(always)]
    pub const fn mrt0(&self) -> Mrt0 {
        let val = (self.0 >> 12usize) & 0x03;
        Mrt0::from_bits(val as u8)
    }
    #[doc = "MRT0."]
    #[inline(always)]
    pub const fn set_mrt0(&mut self, val: Mrt0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "OSTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> Ostimer0 {
        let val = (self.0 >> 16usize) & 0x03;
        Ostimer0::from_bits(val as u8)
    }
    #[doc = "OSTIMER0."]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: Ostimer0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "WWDT0."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> Wwdt0 {
        let val = (self.0 >> 24usize) & 0x03;
        Wwdt0::from_bits(val as u8)
    }
    #[doc = "WWDT0."]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: Wwdt0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "WWDT1."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt1(&self) -> Wwdt1 {
        let val = (self.0 >> 28usize) & 0x03;
        Wwdt1::from_bits(val as u8)
    }
    #[doc = "WWDT1."]
    #[inline(always)]
    pub const fn set_wwdt1(&mut self, val: Wwdt1) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for ApbPeripheralGroup0MemRule2 {
    #[inline(always)]
    fn default() -> ApbPeripheralGroup0MemRule2 {
        ApbPeripheralGroup0MemRule2(0)
    }
}
impl core::fmt::Debug for ApbPeripheralGroup0MemRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbPeripheralGroup0MemRule2")
            .field("ctimer4", &self.ctimer4())
            .field("freqme0", &self.freqme0())
            .field("utcik0", &self.utcik0())
            .field("mrt0", &self.mrt0())
            .field("ostimer0", &self.ostimer0())
            .field("wwdt0", &self.wwdt0())
            .field("wwdt1", &self.wwdt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup0MemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup0MemRule2 {{ ctimer4: {:?}, freqme0: {:?}, utcik0: {:?}, mrt0: {:?}, ostimer0: {:?}, wwdt0: {:?}, wwdt1: {:?} }}",
            self.ctimer4(),
            self.freqme0(),
            self.utcik0(),
            self.mrt0(),
            self.ostimer0(),
            self.wwdt0(),
            self.wwdt1()
        )
    }
}
#[doc = "APB Bridge Group 0 Memory Rule 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup0MemRule3(pub u32);
impl ApbPeripheralGroup0MemRule3 {
    #[doc = "CACHE64_POLSEL0."]
    #[must_use]
    #[inline(always)]
    pub const fn cache64_polsel0(&self) -> Cache64Polsel0 {
        let val = (self.0 >> 12usize) & 0x03;
        Cache64Polsel0::from_bits(val as u8)
    }
    #[doc = "CACHE64_POLSEL0."]
    #[inline(always)]
    pub const fn set_cache64_polsel0(&mut self, val: Cache64Polsel0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for ApbPeripheralGroup0MemRule3 {
    #[inline(always)]
    fn default() -> ApbPeripheralGroup0MemRule3 {
        ApbPeripheralGroup0MemRule3(0)
    }
}
impl core::fmt::Debug for ApbPeripheralGroup0MemRule3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbPeripheralGroup0MemRule3")
            .field("cache64_polsel0", &self.cache64_polsel0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup0MemRule3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup0MemRule3 {{ cache64_polsel0: {:?} }}",
            self.cache64_polsel0()
        )
    }
}
#[doc = "APB Bridge Group 1 Memory Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup1MemRule0(pub u32);
impl ApbPeripheralGroup1MemRule0 {
    #[doc = "I3C0."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> I3c0 {
        let val = (self.0 >> 4usize) & 0x03;
        I3c0::from_bits(val as u8)
    }
    #[doc = "I3C0."]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: I3c0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "I3C1."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c1(&self) -> I3c1 {
        let val = (self.0 >> 8usize) & 0x03;
        I3c1::from_bits(val as u8)
    }
    #[doc = "I3C1."]
    #[inline(always)]
    pub const fn set_i3c1(&mut self, val: I3c1) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "GDET."]
    #[must_use]
    #[inline(always)]
    pub const fn gdet(&self) -> Gdet {
        let val = (self.0 >> 20usize) & 0x03;
        Gdet::from_bits(val as u8)
    }
    #[doc = "GDET."]
    #[inline(always)]
    pub const fn set_gdet(&mut self, val: Gdet) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "ITRC."]
    #[must_use]
    #[inline(always)]
    pub const fn itrc(&self) -> Itrc {
        let val = (self.0 >> 24usize) & 0x03;
        Itrc::from_bits(val as u8)
    }
    #[doc = "ITRC."]
    #[inline(always)]
    pub const fn set_itrc(&mut self, val: Itrc) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for ApbPeripheralGroup1MemRule0 {
    #[inline(always)]
    fn default() -> ApbPeripheralGroup1MemRule0 {
        ApbPeripheralGroup1MemRule0(0)
    }
}
impl core::fmt::Debug for ApbPeripheralGroup1MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbPeripheralGroup1MemRule0")
            .field("i3c0", &self.i3c0())
            .field("i3c1", &self.i3c1())
            .field("gdet", &self.gdet())
            .field("itrc", &self.itrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup1MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup1MemRule0 {{ i3c0: {:?}, i3c1: {:?}, gdet: {:?}, itrc: {:?} }}",
            self.i3c0(),
            self.i3c1(),
            self.gdet(),
            self.itrc()
        )
    }
}
#[doc = "APB Bridge Group 1 Memory Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup1MemRule1(pub u32);
impl ApbPeripheralGroup1MemRule1 {
    #[doc = "PKC."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc(&self) -> ApbPeripheralGroup1MemRule1Pkc {
        let val = (self.0 >> 12usize) & 0x03;
        ApbPeripheralGroup1MemRule1Pkc::from_bits(val as u8)
    }
    #[doc = "PKC."]
    #[inline(always)]
    pub const fn set_pkc(&mut self, val: ApbPeripheralGroup1MemRule1Pkc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "PUF_ALIAS."]
    #[must_use]
    #[inline(always)]
    pub const fn puf_alias(&self, n: usize) -> PufAlias {
        assert!(n < 4usize);
        let offs = 16usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        PufAlias::from_bits(val as u8)
    }
    #[doc = "PUF_ALIAS."]
    #[inline(always)]
    pub const fn set_puf_alias(&mut self, n: usize, val: PufAlias) {
        assert!(n < 4usize);
        let offs = 16usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for ApbPeripheralGroup1MemRule1 {
    #[inline(always)]
    fn default() -> ApbPeripheralGroup1MemRule1 {
        ApbPeripheralGroup1MemRule1(0)
    }
}
impl core::fmt::Debug for ApbPeripheralGroup1MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbPeripheralGroup1MemRule1")
            .field("pkc", &self.pkc())
            .field("puf_alias[0]", &self.puf_alias(0usize))
            .field("puf_alias[1]", &self.puf_alias(1usize))
            .field("puf_alias[2]", &self.puf_alias(2usize))
            .field("puf_alias[3]", &self.puf_alias(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup1MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup1MemRule1 {{ pkc: {:?}, puf_alias[0]: {:?}, puf_alias[1]: {:?}, puf_alias[2]: {:?}, puf_alias[3]: {:?} }}",
            self.pkc(),
            self.puf_alias(0usize),
            self.puf_alias(1usize),
            self.puf_alias(2usize),
            self.puf_alias(3usize)
        )
    }
}
#[doc = "APB Bridge Group 1 Memory Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup1MemRule2(pub u32);
impl ApbPeripheralGroup1MemRule2 {
    #[doc = "SM3."]
    #[must_use]
    #[inline(always)]
    pub const fn sm3(&self) -> Sm3 {
        let val = (self.0 >> 4usize) & 0x03;
        Sm3::from_bits(val as u8)
    }
    #[doc = "SM3."]
    #[inline(always)]
    pub const fn set_sm3(&mut self, val: Sm3) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "COOLFLUX."]
    #[must_use]
    #[inline(always)]
    pub const fn coolflux(&self) -> Coolflux {
        let val = (self.0 >> 8usize) & 0x03;
        Coolflux::from_bits(val as u8)
    }
    #[doc = "COOLFLUX."]
    #[inline(always)]
    pub const fn set_coolflux(&mut self, val: Coolflux) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "SmartDMA."]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma(&self) -> ApbPeripheralGroup1MemRule2Smartdma {
        let val = (self.0 >> 12usize) & 0x03;
        ApbPeripheralGroup1MemRule2Smartdma::from_bits(val as u8)
    }
    #[doc = "SmartDMA."]
    #[inline(always)]
    pub const fn set_smartdma(&mut self, val: ApbPeripheralGroup1MemRule2Smartdma) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "PLU."]
    #[must_use]
    #[inline(always)]
    pub const fn plu(&self) -> Plu {
        let val = (self.0 >> 16usize) & 0x03;
        Plu::from_bits(val as u8)
    }
    #[doc = "PLU."]
    #[inline(always)]
    pub const fn set_plu(&mut self, val: Plu) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
}
impl Default for ApbPeripheralGroup1MemRule2 {
    #[inline(always)]
    fn default() -> ApbPeripheralGroup1MemRule2 {
        ApbPeripheralGroup1MemRule2(0)
    }
}
impl core::fmt::Debug for ApbPeripheralGroup1MemRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbPeripheralGroup1MemRule2")
            .field("sm3", &self.sm3())
            .field("coolflux", &self.coolflux())
            .field("smartdma", &self.smartdma())
            .field("plu", &self.plu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup1MemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup1MemRule2 {{ sm3: {:?}, coolflux: {:?}, smartdma: {:?}, plu: {:?} }}",
            self.sm3(),
            self.coolflux(),
            self.smartdma(),
            self.plu()
        )
    }
}
#[doc = "Miscellaneous CPU0 Control Signals."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0LockReg(pub u32);
impl Cpu0LockReg {
    #[doc = "LOCK_NS_VTOR."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ns_vtor(&self) -> Cpu0LockRegLockNsVtor {
        let val = (self.0 >> 0usize) & 0x03;
        Cpu0LockRegLockNsVtor::from_bits(val as u8)
    }
    #[doc = "LOCK_NS_VTOR."]
    #[inline(always)]
    pub const fn set_lock_ns_vtor(&mut self, val: Cpu0LockRegLockNsVtor) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "LOCK_NS_MPU."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ns_mpu(&self) -> Cpu0LockRegLockNsMpu {
        let val = (self.0 >> 2usize) & 0x03;
        Cpu0LockRegLockNsMpu::from_bits(val as u8)
    }
    #[doc = "LOCK_NS_MPU."]
    #[inline(always)]
    pub const fn set_lock_ns_mpu(&mut self, val: Cpu0LockRegLockNsMpu) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "LOCK_S_VTAIRCR."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_s_vtaircr(&self) -> LockSVtaircr {
        let val = (self.0 >> 4usize) & 0x03;
        LockSVtaircr::from_bits(val as u8)
    }
    #[doc = "LOCK_S_VTAIRCR."]
    #[inline(always)]
    pub const fn set_lock_s_vtaircr(&mut self, val: LockSVtaircr) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "LOCK_S_MPU."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_s_mpu(&self) -> LockSMpu {
        let val = (self.0 >> 6usize) & 0x03;
        LockSMpu::from_bits(val as u8)
    }
    #[doc = "LOCK_S_MPU."]
    #[inline(always)]
    pub const fn set_lock_s_mpu(&mut self, val: LockSMpu) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "LOCK_SAU."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_sau(&self) -> LockSau {
        let val = (self.0 >> 8usize) & 0x03;
        LockSau::from_bits(val as u8)
    }
    #[doc = "LOCK_SAU."]
    #[inline(always)]
    pub const fn set_lock_sau(&mut self, val: LockSau) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CM33_LOCK_REG_LOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn cm33_lock_reg_lock(&self) -> Cm33LockRegLock {
        let val = (self.0 >> 30usize) & 0x03;
        Cm33LockRegLock::from_bits(val as u8)
    }
    #[doc = "CM33_LOCK_REG_LOCK."]
    #[inline(always)]
    pub const fn set_cm33_lock_reg_lock(&mut self, val: Cm33LockRegLock) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Cpu0LockReg {
    #[inline(always)]
    fn default() -> Cpu0LockReg {
        Cpu0LockReg(0)
    }
}
impl core::fmt::Debug for Cpu0LockReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpu0LockReg")
            .field("lock_ns_vtor", &self.lock_ns_vtor())
            .field("lock_ns_mpu", &self.lock_ns_mpu())
            .field("lock_s_vtaircr", &self.lock_s_vtaircr())
            .field("lock_s_mpu", &self.lock_s_mpu())
            .field("lock_sau", &self.lock_sau())
            .field("cm33_lock_reg_lock", &self.cm33_lock_reg_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpu0LockReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpu0LockReg {{ lock_ns_vtor: {:?}, lock_ns_mpu: {:?}, lock_s_vtaircr: {:?}, lock_s_mpu: {:?}, lock_sau: {:?}, cm33_lock_reg_lock: {:?} }}",
            self.lock_ns_vtor(),
            self.lock_ns_mpu(),
            self.lock_s_vtaircr(),
            self.lock_s_mpu(),
            self.lock_sau(),
            self.cm33_lock_reg_lock()
        )
    }
}
#[doc = "Miscellaneous CPU1 Control Signals."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu1LockReg(pub u32);
impl Cpu1LockReg {
    #[doc = "LOCK_NS_VTOR."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ns_vtor(&self) -> Cpu1LockRegLockNsVtor {
        let val = (self.0 >> 0usize) & 0x03;
        Cpu1LockRegLockNsVtor::from_bits(val as u8)
    }
    #[doc = "LOCK_NS_VTOR."]
    #[inline(always)]
    pub const fn set_lock_ns_vtor(&mut self, val: Cpu1LockRegLockNsVtor) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "LOCK_NS_MPU."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ns_mpu(&self) -> Cpu1LockRegLockNsMpu {
        let val = (self.0 >> 2usize) & 0x03;
        Cpu1LockRegLockNsMpu::from_bits(val as u8)
    }
    #[doc = "LOCK_NS_MPU."]
    #[inline(always)]
    pub const fn set_lock_ns_mpu(&mut self, val: Cpu1LockRegLockNsMpu) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for Cpu1LockReg {
    #[inline(always)]
    fn default() -> Cpu1LockReg {
        Cpu1LockReg(0)
    }
}
impl core::fmt::Debug for Cpu1LockReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpu1LockReg")
            .field("lock_ns_vtor", &self.lock_ns_vtor())
            .field("lock_ns_mpu", &self.lock_ns_mpu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpu1LockReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpu1LockReg {{ lock_ns_vtor: {:?}, lock_ns_mpu: {:?} }}",
            self.lock_ns_vtor(),
            self.lock_ns_mpu()
        )
    }
}
#[doc = "Flash Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash00MemRule(pub u32);
impl Flash00MemRule {
    #[doc = "Rule."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Flash00MemRuleRule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Flash00MemRuleRule::from_bits(val as u8)
    }
    #[doc = "Rule."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Flash00MemRuleRule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Flash00MemRule {
    #[inline(always)]
    fn default() -> Flash00MemRule {
        Flash00MemRule(0)
    }
}
impl core::fmt::Debug for Flash00MemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flash00MemRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flash00MemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flash00MemRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "Flash Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash01MemRule(pub u32);
impl Flash01MemRule {
    #[doc = "Rule."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Flash01MemRuleRule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Flash01MemRuleRule::from_bits(val as u8)
    }
    #[doc = "Rule."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Flash01MemRuleRule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Flash01MemRule {
    #[inline(always)]
    fn default() -> Flash01MemRule {
        Flash01MemRule(0)
    }
}
impl core::fmt::Debug for Flash01MemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flash01MemRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flash01MemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flash01MemRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "Flash Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash02MemRule(pub u32);
impl Flash02MemRule {
    #[doc = "Rule."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Flash02MemRuleRule {
        assert!(n < 4usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Flash02MemRuleRule::from_bits(val as u8)
    }
    #[doc = "Rule."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Flash02MemRuleRule) {
        assert!(n < 4usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Flash02MemRule {
    #[inline(always)]
    fn default() -> Flash02MemRule {
        Flash02MemRule(0)
    }
}
impl core::fmt::Debug for Flash02MemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flash02MemRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flash02MemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flash02MemRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize)
        )
    }
}
#[doc = "Flash Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash03MemRule(pub u32);
impl Flash03MemRule {
    #[doc = "Rule."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Flash03MemRuleRule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Flash03MemRuleRule::from_bits(val as u8)
    }
    #[doc = "Rule."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Flash03MemRuleRule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Flash03MemRule {
    #[inline(always)]
    fn default() -> Flash03MemRule {
        Flash03MemRule(0)
    }
}
impl core::fmt::Debug for Flash03MemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flash03MemRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flash03MemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flash03MemRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "FLEXSPI0 Region 0 Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi0Region0MemRule(pub u32);
impl Flexspi0Region0MemRule {
    #[doc = "Rule."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Flexspi0Region0MemRuleRule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Flexspi0Region0MemRuleRule::from_bits(val as u8)
    }
    #[doc = "Rule."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Flexspi0Region0MemRuleRule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Flexspi0Region0MemRule {
    #[inline(always)]
    fn default() -> Flexspi0Region0MemRule {
        Flexspi0Region0MemRule(0)
    }
}
impl core::fmt::Debug for Flexspi0Region0MemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi0Region0MemRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi0Region0MemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexspi0Region0MemRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "FLEXSPI0 Region index Memory Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi0Region16MemRuleFlexspi0RegionMemRule0(pub u32);
impl Flexspi0Region16MemRuleFlexspi0RegionMemRule0 {
    #[doc = "Rule."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Flexspi0Region16MemRuleFlexspi0RegionMemRule0Rule {
        assert!(n < 6usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Flexspi0Region16MemRuleFlexspi0RegionMemRule0Rule::from_bits(val as u8)
    }
    #[doc = "Rule."]
    #[inline(always)]
    pub const fn set_rule(
        &mut self,
        n: usize,
        val: Flexspi0Region16MemRuleFlexspi0RegionMemRule0Rule,
    ) {
        assert!(n < 6usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Flexspi0Region16MemRuleFlexspi0RegionMemRule0 {
    #[inline(always)]
    fn default() -> Flexspi0Region16MemRuleFlexspi0RegionMemRule0 {
        Flexspi0Region16MemRuleFlexspi0RegionMemRule0(0)
    }
}
impl core::fmt::Debug for Flexspi0Region16MemRuleFlexspi0RegionMemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi0Region16MemRuleFlexspi0RegionMemRule0")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi0Region16MemRuleFlexspi0RegionMemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexspi0Region16MemRuleFlexspi0RegionMemRule0 {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize)
        )
    }
}
#[doc = "FLEXSPI0 Region 7 Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi0Region7MemRule(pub u32);
impl Flexspi0Region7MemRule {
    #[doc = "Rule."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Flexspi0Region7MemRuleRule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Flexspi0Region7MemRuleRule::from_bits(val as u8)
    }
    #[doc = "Rule."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Flexspi0Region7MemRuleRule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Flexspi0Region7MemRule {
    #[inline(always)]
    fn default() -> Flexspi0Region7MemRule {
        Flexspi0Region7MemRule(0)
    }
}
impl core::fmt::Debug for Flexspi0Region7MemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi0Region7MemRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi0Region7MemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexspi0Region7MemRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "FLEXSPI0 Region index Memory Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi0Region813MemRuleFlexspi0RegionMemRule0(pub u32);
impl Flexspi0Region813MemRuleFlexspi0RegionMemRule0 {
    #[doc = "Rule."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Flexspi0Region813MemRuleFlexspi0RegionMemRule0Rule {
        assert!(n < 6usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Flexspi0Region813MemRuleFlexspi0RegionMemRule0Rule::from_bits(val as u8)
    }
    #[doc = "Rule."]
    #[inline(always)]
    pub const fn set_rule(
        &mut self,
        n: usize,
        val: Flexspi0Region813MemRuleFlexspi0RegionMemRule0Rule,
    ) {
        assert!(n < 6usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Flexspi0Region813MemRuleFlexspi0RegionMemRule0 {
    #[inline(always)]
    fn default() -> Flexspi0Region813MemRuleFlexspi0RegionMemRule0 {
        Flexspi0Region813MemRuleFlexspi0RegionMemRule0(0)
    }
}
impl core::fmt::Debug for Flexspi0Region813MemRuleFlexspi0RegionMemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi0Region813MemRuleFlexspi0RegionMemRule0")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi0Region813MemRuleFlexspi0RegionMemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexspi0Region813MemRuleFlexspi0RegionMemRule0 {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize)
        )
    }
}
#[doc = "Master Secure Level."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MasterSecAntiPolReg(pub u32);
impl MasterSecAntiPolReg {
    #[doc = "CPU1."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1(&self) -> MasterSecAntiPolRegCpu1 {
        let val = (self.0 >> 2usize) & 0x03;
        MasterSecAntiPolRegCpu1::from_bits(val as u8)
    }
    #[doc = "CPU1."]
    #[inline(always)]
    pub const fn set_cpu1(&mut self, val: MasterSecAntiPolRegCpu1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "SMARTDMA Data."]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma(&self) -> MasterSecAntiPolRegSmartdma {
        let val = (self.0 >> 4usize) & 0x03;
        MasterSecAntiPolRegSmartdma::from_bits(val as u8)
    }
    #[doc = "SMARTDMA Data."]
    #[inline(always)]
    pub const fn set_smartdma(&mut self, val: MasterSecAntiPolRegSmartdma) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "eDMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0(&self) -> MasterSecAntiPolRegEDma0 {
        let val = (self.0 >> 6usize) & 0x03;
        MasterSecAntiPolRegEDma0::from_bits(val as u8)
    }
    #[doc = "eDMA0."]
    #[inline(always)]
    pub const fn set_e_dma0(&mut self, val: MasterSecAntiPolRegEDma0) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "eDMA1."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma1(&self) -> MasterSecAntiPolRegEDma1 {
        let val = (self.0 >> 8usize) & 0x03;
        MasterSecAntiPolRegEDma1::from_bits(val as u8)
    }
    #[doc = "eDMA1."]
    #[inline(always)]
    pub const fn set_e_dma1(&mut self, val: MasterSecAntiPolRegEDma1) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "PKC."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc(&self) -> MasterSecAntiPolRegPkc {
        let val = (self.0 >> 10usize) & 0x03;
        MasterSecAntiPolRegPkc::from_bits(val as u8)
    }
    #[doc = "PKC."]
    #[inline(always)]
    pub const fn set_pkc(&mut self, val: MasterSecAntiPolRegPkc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "PowerQuad."]
    #[must_use]
    #[inline(always)]
    pub const fn pq(&self) -> MasterSecAntiPolRegPq {
        let val = (self.0 >> 14usize) & 0x03;
        MasterSecAntiPolRegPq::from_bits(val as u8)
    }
    #[doc = "PowerQuad."]
    #[inline(always)]
    pub const fn set_pq(&mut self, val: MasterSecAntiPolRegPq) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "NPU Operands."]
    #[must_use]
    #[inline(always)]
    pub const fn npuo(&self) -> MasterSecAntiPolRegNpuo {
        let val = (self.0 >> 16usize) & 0x03;
        MasterSecAntiPolRegNpuo::from_bits(val as u8)
    }
    #[doc = "NPU Operands."]
    #[inline(always)]
    pub const fn set_npuo(&mut self, val: MasterSecAntiPolRegNpuo) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Coolflux Instruction."]
    #[must_use]
    #[inline(always)]
    pub const fn coolfluxi(&self) -> MasterSecAntiPolRegCoolfluxi {
        let val = (self.0 >> 18usize) & 0x03;
        MasterSecAntiPolRegCoolfluxi::from_bits(val as u8)
    }
    #[doc = "Coolflux Instruction."]
    #[inline(always)]
    pub const fn set_coolfluxi(&mut self, val: MasterSecAntiPolRegCoolfluxi) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "USB_FS."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_fs(&self) -> MasterSecAntiPolRegUsbFs {
        let val = (self.0 >> 22usize) & 0x03;
        MasterSecAntiPolRegUsbFs::from_bits(val as u8)
    }
    #[doc = "USB_FS."]
    #[inline(always)]
    pub const fn set_usb_fs(&mut self, val: MasterSecAntiPolRegUsbFs) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Ethernet."]
    #[must_use]
    #[inline(always)]
    pub const fn ethernet(&self) -> MasterSecAntiPolRegEthernet {
        let val = (self.0 >> 24usize) & 0x03;
        MasterSecAntiPolRegEthernet::from_bits(val as u8)
    }
    #[doc = "Ethernet."]
    #[inline(always)]
    pub const fn set_ethernet(&mut self, val: MasterSecAntiPolRegEthernet) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "USB HS."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs(&self) -> MasterSecAntiPolRegUsbHs {
        let val = (self.0 >> 26usize) & 0x03;
        MasterSecAntiPolRegUsbHs::from_bits(val as u8)
    }
    #[doc = "USB HS."]
    #[inline(always)]
    pub const fn set_usb_hs(&mut self, val: MasterSecAntiPolRegUsbHs) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "uSDHC."]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc(&self) -> MasterSecAntiPolRegUsdhc {
        let val = (self.0 >> 28usize) & 0x03;
        MasterSecAntiPolRegUsdhc::from_bits(val as u8)
    }
    #[doc = "uSDHC."]
    #[inline(always)]
    pub const fn set_usdhc(&mut self, val: MasterSecAntiPolRegUsdhc) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Master SEC Level Antipol Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn master_sec_level_antipol_lock(&self) -> MasterSecLevelAntipolLock {
        let val = (self.0 >> 30usize) & 0x03;
        MasterSecLevelAntipolLock::from_bits(val as u8)
    }
    #[doc = "Master SEC Level Antipol Lock."]
    #[inline(always)]
    pub const fn set_master_sec_level_antipol_lock(&mut self, val: MasterSecLevelAntipolLock) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for MasterSecAntiPolReg {
    #[inline(always)]
    fn default() -> MasterSecAntiPolReg {
        MasterSecAntiPolReg(0)
    }
}
impl core::fmt::Debug for MasterSecAntiPolReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MasterSecAntiPolReg")
            .field("cpu1", &self.cpu1())
            .field("smartdma", &self.smartdma())
            .field("e_dma0", &self.e_dma0())
            .field("e_dma1", &self.e_dma1())
            .field("pkc", &self.pkc())
            .field("pq", &self.pq())
            .field("npuo", &self.npuo())
            .field("coolfluxi", &self.coolfluxi())
            .field("usb_fs", &self.usb_fs())
            .field("ethernet", &self.ethernet())
            .field("usb_hs", &self.usb_hs())
            .field("usdhc", &self.usdhc())
            .field(
                "master_sec_level_antipol_lock",
                &self.master_sec_level_antipol_lock(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MasterSecAntiPolReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MasterSecAntiPolReg {{ cpu1: {:?}, smartdma: {:?}, e_dma0: {:?}, e_dma1: {:?}, pkc: {:?}, pq: {:?}, npuo: {:?}, coolfluxi: {:?}, usb_fs: {:?}, ethernet: {:?}, usb_hs: {:?}, usdhc: {:?}, master_sec_level_antipol_lock: {:?} }}",
            self.cpu1(),
            self.smartdma(),
            self.e_dma0(),
            self.e_dma1(),
            self.pkc(),
            self.pq(),
            self.npuo(),
            self.coolfluxi(),
            self.usb_fs(),
            self.ethernet(),
            self.usb_hs(),
            self.usdhc(),
            self.master_sec_level_antipol_lock()
        )
    }
}
#[doc = "Master Secure Level."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MasterSecLevel(pub u32);
impl MasterSecLevel {
    #[doc = "CPU1."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1(&self) -> MasterSecLevelCpu1 {
        let val = (self.0 >> 2usize) & 0x03;
        MasterSecLevelCpu1::from_bits(val as u8)
    }
    #[doc = "CPU1."]
    #[inline(always)]
    pub const fn set_cpu1(&mut self, val: MasterSecLevelCpu1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "SMARTDMA Data."]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma(&self) -> MasterSecLevelSmartdma {
        let val = (self.0 >> 4usize) & 0x03;
        MasterSecLevelSmartdma::from_bits(val as u8)
    }
    #[doc = "SMARTDMA Data."]
    #[inline(always)]
    pub const fn set_smartdma(&mut self, val: MasterSecLevelSmartdma) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "eDMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma0(&self) -> MasterSecLevelEDma0 {
        let val = (self.0 >> 6usize) & 0x03;
        MasterSecLevelEDma0::from_bits(val as u8)
    }
    #[doc = "eDMA0."]
    #[inline(always)]
    pub const fn set_e_dma0(&mut self, val: MasterSecLevelEDma0) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "eDMA1."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma1(&self) -> MasterSecLevelEDma1 {
        let val = (self.0 >> 8usize) & 0x03;
        MasterSecLevelEDma1::from_bits(val as u8)
    }
    #[doc = "eDMA1."]
    #[inline(always)]
    pub const fn set_e_dma1(&mut self, val: MasterSecLevelEDma1) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "PKC."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc(&self) -> MasterSecLevelPkc {
        let val = (self.0 >> 10usize) & 0x03;
        MasterSecLevelPkc::from_bits(val as u8)
    }
    #[doc = "PKC."]
    #[inline(always)]
    pub const fn set_pkc(&mut self, val: MasterSecLevelPkc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "PowerQuad."]
    #[must_use]
    #[inline(always)]
    pub const fn pq(&self) -> MasterSecLevelPq {
        let val = (self.0 >> 14usize) & 0x03;
        MasterSecLevelPq::from_bits(val as u8)
    }
    #[doc = "PowerQuad."]
    #[inline(always)]
    pub const fn set_pq(&mut self, val: MasterSecLevelPq) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "NPU Operands."]
    #[must_use]
    #[inline(always)]
    pub const fn npuo(&self) -> MasterSecLevelNpuo {
        let val = (self.0 >> 16usize) & 0x03;
        MasterSecLevelNpuo::from_bits(val as u8)
    }
    #[doc = "NPU Operands."]
    #[inline(always)]
    pub const fn set_npuo(&mut self, val: MasterSecLevelNpuo) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Coolflux Instruction."]
    #[must_use]
    #[inline(always)]
    pub const fn coolfluxi(&self) -> MasterSecLevelCoolfluxi {
        let val = (self.0 >> 18usize) & 0x03;
        MasterSecLevelCoolfluxi::from_bits(val as u8)
    }
    #[doc = "Coolflux Instruction."]
    #[inline(always)]
    pub const fn set_coolfluxi(&mut self, val: MasterSecLevelCoolfluxi) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "USB_FS."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_fs(&self) -> MasterSecLevelUsbFs {
        let val = (self.0 >> 22usize) & 0x03;
        MasterSecLevelUsbFs::from_bits(val as u8)
    }
    #[doc = "USB_FS."]
    #[inline(always)]
    pub const fn set_usb_fs(&mut self, val: MasterSecLevelUsbFs) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Ethernet."]
    #[must_use]
    #[inline(always)]
    pub const fn ethernet(&self) -> MasterSecLevelEthernet {
        let val = (self.0 >> 24usize) & 0x03;
        MasterSecLevelEthernet::from_bits(val as u8)
    }
    #[doc = "Ethernet."]
    #[inline(always)]
    pub const fn set_ethernet(&mut self, val: MasterSecLevelEthernet) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "USB HS."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs(&self) -> MasterSecLevelUsbHs {
        let val = (self.0 >> 26usize) & 0x03;
        MasterSecLevelUsbHs::from_bits(val as u8)
    }
    #[doc = "USB HS."]
    #[inline(always)]
    pub const fn set_usb_hs(&mut self, val: MasterSecLevelUsbHs) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "uSDHC."]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc(&self) -> MasterSecLevelUsdhc {
        let val = (self.0 >> 28usize) & 0x03;
        MasterSecLevelUsdhc::from_bits(val as u8)
    }
    #[doc = "uSDHC."]
    #[inline(always)]
    pub const fn set_usdhc(&mut self, val: MasterSecLevelUsdhc) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Master SEC Level Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn master_sec_level_lock(&self) -> MasterSecLevelLock {
        let val = (self.0 >> 30usize) & 0x03;
        MasterSecLevelLock::from_bits(val as u8)
    }
    #[doc = "Master SEC Level Lock."]
    #[inline(always)]
    pub const fn set_master_sec_level_lock(&mut self, val: MasterSecLevelLock) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for MasterSecLevel {
    #[inline(always)]
    fn default() -> MasterSecLevel {
        MasterSecLevel(0)
    }
}
impl core::fmt::Debug for MasterSecLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MasterSecLevel")
            .field("cpu1", &self.cpu1())
            .field("smartdma", &self.smartdma())
            .field("e_dma0", &self.e_dma0())
            .field("e_dma1", &self.e_dma1())
            .field("pkc", &self.pkc())
            .field("pq", &self.pq())
            .field("npuo", &self.npuo())
            .field("coolfluxi", &self.coolfluxi())
            .field("usb_fs", &self.usb_fs())
            .field("ethernet", &self.ethernet())
            .field("usb_hs", &self.usb_hs())
            .field("usdhc", &self.usdhc())
            .field("master_sec_level_lock", &self.master_sec_level_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MasterSecLevel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MasterSecLevel {{ cpu1: {:?}, smartdma: {:?}, e_dma0: {:?}, e_dma1: {:?}, pkc: {:?}, pq: {:?}, npuo: {:?}, coolfluxi: {:?}, usb_fs: {:?}, ethernet: {:?}, usb_hs: {:?}, usdhc: {:?}, master_sec_level_lock: {:?} }}",
            self.cpu1(),
            self.smartdma(),
            self.e_dma0(),
            self.e_dma1(),
            self.pkc(),
            self.pq(),
            self.npuo(),
            self.coolfluxi(),
            self.usb_fs(),
            self.ethernet(),
            self.usb_hs(),
            self.usdhc(),
            self.master_sec_level_lock()
        )
    }
}
#[doc = "Secure Control Duplicate."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiscCtrlDpReg(pub u32);
impl MiscCtrlDpReg {
    #[doc = "Write Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn write_lock(&self) -> MiscCtrlDpRegWriteLock {
        let val = (self.0 >> 0usize) & 0x03;
        MiscCtrlDpRegWriteLock::from_bits(val as u8)
    }
    #[doc = "Write Lock."]
    #[inline(always)]
    pub const fn set_write_lock(&mut self, val: MiscCtrlDpRegWriteLock) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Enable Secure Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_secure_checking(&self) -> MiscCtrlDpRegEnableSecureChecking {
        let val = (self.0 >> 2usize) & 0x03;
        MiscCtrlDpRegEnableSecureChecking::from_bits(val as u8)
    }
    #[doc = "Enable Secure Checking."]
    #[inline(always)]
    pub const fn set_enable_secure_checking(&mut self, val: MiscCtrlDpRegEnableSecureChecking) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Secure Privilege Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_s_priv_check(&self) -> MiscCtrlDpRegEnableSPrivCheck {
        let val = (self.0 >> 4usize) & 0x03;
        MiscCtrlDpRegEnableSPrivCheck::from_bits(val as u8)
    }
    #[doc = "Enable Secure Privilege Checking."]
    #[inline(always)]
    pub const fn set_enable_s_priv_check(&mut self, val: MiscCtrlDpRegEnableSPrivCheck) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Enable Non-Secure Privilege Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ns_priv_check(&self) -> MiscCtrlDpRegEnableNsPrivCheck {
        let val = (self.0 >> 6usize) & 0x03;
        MiscCtrlDpRegEnableNsPrivCheck::from_bits(val as u8)
    }
    #[doc = "Enable Non-Secure Privilege Checking."]
    #[inline(always)]
    pub const fn set_enable_ns_priv_check(&mut self, val: MiscCtrlDpRegEnableNsPrivCheck) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Disable Violation Abort."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_violation_abort(&self) -> MiscCtrlDpRegDisableViolationAbort {
        let val = (self.0 >> 8usize) & 0x03;
        MiscCtrlDpRegDisableViolationAbort::from_bits(val as u8)
    }
    #[doc = "Disable Violation Abort."]
    #[inline(always)]
    pub const fn set_disable_violation_abort(&mut self, val: MiscCtrlDpRegDisableViolationAbort) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Disable Strict Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_strict_mode(&self) -> MiscCtrlDpRegDisableStrictMode {
        let val = (self.0 >> 10usize) & 0x03;
        MiscCtrlDpRegDisableStrictMode::from_bits(val as u8)
    }
    #[doc = "Disable Strict Mode."]
    #[inline(always)]
    pub const fn set_disable_strict_mode(&mut self, val: MiscCtrlDpRegDisableStrictMode) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "IDAU All Non-Secure."]
    #[must_use]
    #[inline(always)]
    pub const fn idau_all_ns(&self) -> MiscCtrlDpRegIdauAllNs {
        let val = (self.0 >> 14usize) & 0x03;
        MiscCtrlDpRegIdauAllNs::from_bits(val as u8)
    }
    #[doc = "IDAU All Non-Secure."]
    #[inline(always)]
    pub const fn set_idau_all_ns(&mut self, val: MiscCtrlDpRegIdauAllNs) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for MiscCtrlDpReg {
    #[inline(always)]
    fn default() -> MiscCtrlDpReg {
        MiscCtrlDpReg(0)
    }
}
impl core::fmt::Debug for MiscCtrlDpReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiscCtrlDpReg")
            .field("write_lock", &self.write_lock())
            .field("enable_secure_checking", &self.enable_secure_checking())
            .field("enable_s_priv_check", &self.enable_s_priv_check())
            .field("enable_ns_priv_check", &self.enable_ns_priv_check())
            .field("disable_violation_abort", &self.disable_violation_abort())
            .field("disable_strict_mode", &self.disable_strict_mode())
            .field("idau_all_ns", &self.idau_all_ns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiscCtrlDpReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MiscCtrlDpReg {{ write_lock: {:?}, enable_secure_checking: {:?}, enable_s_priv_check: {:?}, enable_ns_priv_check: {:?}, disable_violation_abort: {:?}, disable_strict_mode: {:?}, idau_all_ns: {:?} }}",
            self.write_lock(),
            self.enable_secure_checking(),
            self.enable_s_priv_check(),
            self.enable_ns_priv_check(),
            self.disable_violation_abort(),
            self.disable_strict_mode(),
            self.idau_all_ns()
        )
    }
}
#[doc = "Secure Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiscCtrlReg(pub u32);
impl MiscCtrlReg {
    #[doc = "Write Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn write_lock(&self) -> MiscCtrlRegWriteLock {
        let val = (self.0 >> 0usize) & 0x03;
        MiscCtrlRegWriteLock::from_bits(val as u8)
    }
    #[doc = "Write Lock."]
    #[inline(always)]
    pub const fn set_write_lock(&mut self, val: MiscCtrlRegWriteLock) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Enable Secure Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_secure_checking(&self) -> MiscCtrlRegEnableSecureChecking {
        let val = (self.0 >> 2usize) & 0x03;
        MiscCtrlRegEnableSecureChecking::from_bits(val as u8)
    }
    #[doc = "Enable Secure Checking."]
    #[inline(always)]
    pub const fn set_enable_secure_checking(&mut self, val: MiscCtrlRegEnableSecureChecking) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Secure Privilege Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_s_priv_check(&self) -> MiscCtrlRegEnableSPrivCheck {
        let val = (self.0 >> 4usize) & 0x03;
        MiscCtrlRegEnableSPrivCheck::from_bits(val as u8)
    }
    #[doc = "Enable Secure Privilege Checking."]
    #[inline(always)]
    pub const fn set_enable_s_priv_check(&mut self, val: MiscCtrlRegEnableSPrivCheck) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Enable Non-Secure Privilege Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ns_priv_check(&self) -> MiscCtrlRegEnableNsPrivCheck {
        let val = (self.0 >> 6usize) & 0x03;
        MiscCtrlRegEnableNsPrivCheck::from_bits(val as u8)
    }
    #[doc = "Enable Non-Secure Privilege Checking."]
    #[inline(always)]
    pub const fn set_enable_ns_priv_check(&mut self, val: MiscCtrlRegEnableNsPrivCheck) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Disable Violation Abort."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_violation_abort(&self) -> MiscCtrlRegDisableViolationAbort {
        let val = (self.0 >> 8usize) & 0x03;
        MiscCtrlRegDisableViolationAbort::from_bits(val as u8)
    }
    #[doc = "Disable Violation Abort."]
    #[inline(always)]
    pub const fn set_disable_violation_abort(&mut self, val: MiscCtrlRegDisableViolationAbort) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Disable Strict Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_strict_mode(&self) -> MiscCtrlRegDisableStrictMode {
        let val = (self.0 >> 10usize) & 0x03;
        MiscCtrlRegDisableStrictMode::from_bits(val as u8)
    }
    #[doc = "Disable Strict Mode."]
    #[inline(always)]
    pub const fn set_disable_strict_mode(&mut self, val: MiscCtrlRegDisableStrictMode) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "IDAU All Non-Secure."]
    #[must_use]
    #[inline(always)]
    pub const fn idau_all_ns(&self) -> MiscCtrlRegIdauAllNs {
        let val = (self.0 >> 14usize) & 0x03;
        MiscCtrlRegIdauAllNs::from_bits(val as u8)
    }
    #[doc = "IDAU All Non-Secure."]
    #[inline(always)]
    pub const fn set_idau_all_ns(&mut self, val: MiscCtrlRegIdauAllNs) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for MiscCtrlReg {
    #[inline(always)]
    fn default() -> MiscCtrlReg {
        MiscCtrlReg(0)
    }
}
impl core::fmt::Debug for MiscCtrlReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiscCtrlReg")
            .field("write_lock", &self.write_lock())
            .field("enable_secure_checking", &self.enable_secure_checking())
            .field("enable_s_priv_check", &self.enable_s_priv_check())
            .field("enable_ns_priv_check", &self.enable_ns_priv_check())
            .field("disable_violation_abort", &self.disable_violation_abort())
            .field("disable_strict_mode", &self.disable_strict_mode())
            .field("idau_all_ns", &self.idau_all_ns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiscCtrlReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MiscCtrlReg {{ write_lock: {:?}, enable_secure_checking: {:?}, enable_s_priv_check: {:?}, enable_ns_priv_check: {:?}, disable_violation_abort: {:?}, disable_strict_mode: {:?}, idau_all_ns: {:?} }}",
            self.write_lock(),
            self.enable_secure_checking(),
            self.enable_s_priv_check(),
            self.enable_ns_priv_check(),
            self.disable_violation_abort(),
            self.disable_strict_mode(),
            self.idau_all_ns()
        )
    }
}
#[doc = "RAMA Memory Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamaMemRule(pub u32);
impl RamaMemRule {
    #[doc = "Rule."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> RamaMemRuleRule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        RamaMemRuleRule::from_bits(val as u8)
    }
    #[doc = "Rule."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: RamaMemRuleRule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for RamaMemRule {
    #[inline(always)]
    fn default() -> RamaMemRule {
        RamaMemRule(0)
    }
}
impl core::fmt::Debug for RamaMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamaMemRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamaMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamaMemRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "RAMB Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RambMemRule(pub u32);
impl RambMemRule {
    #[doc = "Rule."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> RambMemRuleRule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        RambMemRuleRule::from_bits(val as u8)
    }
    #[doc = "Rule."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: RambMemRuleRule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for RambMemRule {
    #[inline(always)]
    fn default() -> RambMemRule {
        RambMemRule(0)
    }
}
impl core::fmt::Debug for RambMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RambMemRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RambMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RambMemRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "RAMC Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamcMemRule(pub u32);
impl RamcMemRule {
    #[doc = "Rule."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> RamcMemRuleRule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        RamcMemRuleRule::from_bits(val as u8)
    }
    #[doc = "Rule."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: RamcMemRuleRule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for RamcMemRule {
    #[inline(always)]
    fn default() -> RamcMemRule {
        RamcMemRule(0)
    }
}
impl core::fmt::Debug for RamcMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamcMemRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamcMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamcMemRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "RAMD Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamdMemRule(pub u32);
impl RamdMemRule {
    #[doc = "Rule."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> RamdMemRuleRule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        RamdMemRuleRule::from_bits(val as u8)
    }
    #[doc = "Rule."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: RamdMemRuleRule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for RamdMemRule {
    #[inline(always)]
    fn default() -> RamdMemRule {
        RamdMemRule(0)
    }
}
impl core::fmt::Debug for RamdMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamdMemRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamdMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamdMemRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "RAME Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RameMemRule(pub u32);
impl RameMemRule {
    #[doc = "Rule."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> RameMemRuleRule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        RameMemRuleRule::from_bits(val as u8)
    }
    #[doc = "Rule."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: RameMemRuleRule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for RameMemRule {
    #[inline(always)]
    fn default() -> RameMemRule {
        RameMemRule(0)
    }
}
impl core::fmt::Debug for RameMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RameMemRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RameMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RameMemRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "RAMF Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamfMemRule(pub u32);
impl RamfMemRule {
    #[doc = "Rule."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> RamfMemRuleRule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        RamfMemRuleRule::from_bits(val as u8)
    }
    #[doc = "Rule."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: RamfMemRuleRule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for RamfMemRule {
    #[inline(always)]
    fn default() -> RamfMemRule {
        RamfMemRule(0)
    }
}
impl core::fmt::Debug for RamfMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamfMemRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamfMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamfMemRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "RAMG Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamgMemRule(pub u32);
impl RamgMemRule {
    #[doc = "Rule."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> RamgMemRuleRule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        RamgMemRuleRule::from_bits(val as u8)
    }
    #[doc = "Rule."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: RamgMemRuleRule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for RamgMemRule {
    #[inline(always)]
    fn default() -> RamgMemRule {
        RamgMemRule(0)
    }
}
impl core::fmt::Debug for RamgMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamgMemRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamgMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamgMemRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "RAMH Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamhMemRule(pub u32);
impl RamhMemRule {
    #[doc = "Rule."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> RamhMemRuleRule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        RamhMemRuleRule::from_bits(val as u8)
    }
    #[doc = "Rule."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: RamhMemRuleRule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for RamhMemRule {
    #[inline(always)]
    fn default() -> RamhMemRule {
        RamhMemRule(0)
    }
}
impl core::fmt::Debug for RamhMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamhMemRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamhMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamhMemRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "RAMX Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamxMemRule(pub u32);
impl RamxMemRule {
    #[doc = "Rule."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> RamxMemRuleRule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        RamxMemRuleRule::from_bits(val as u8)
    }
    #[doc = "Rule."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: RamxMemRuleRule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for RamxMemRule {
    #[inline(always)]
    fn default() -> RamxMemRule {
        RamxMemRule(0)
    }
}
impl core::fmt::Debug for RamxMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamxMemRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamxMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamxMemRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "ROM Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RomMemRule(pub u32);
impl RomMemRule {
    #[doc = "Rule."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> RomMemRuleRule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        RomMemRuleRule::from_bits(val as u8)
    }
    #[doc = "Rule."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: RomMemRuleRule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for RomMemRule {
    #[inline(always)]
    fn default() -> RomMemRule {
        RomMemRule(0)
    }
}
impl core::fmt::Debug for RomMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RomMemRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RomMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RomMemRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "Secure Interrupt Mask for CPU1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCpu1IntMask(pub u32);
impl SecCpu1IntMask {
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn int_mask(&self, n: usize) -> IntMask {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        IntMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_int_mask(&mut self, n: usize, val: IntMask) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for SecCpu1IntMask {
    #[inline(always)]
    fn default() -> SecCpu1IntMask {
        SecCpu1IntMask(0)
    }
}
impl core::fmt::Debug for SecCpu1IntMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCpu1IntMask")
            .field("int_mask[0]", &self.int_mask(0usize))
            .field("int_mask[1]", &self.int_mask(1usize))
            .field("int_mask[2]", &self.int_mask(2usize))
            .field("int_mask[3]", &self.int_mask(3usize))
            .field("int_mask[4]", &self.int_mask(4usize))
            .field("int_mask[5]", &self.int_mask(5usize))
            .field("int_mask[6]", &self.int_mask(6usize))
            .field("int_mask[7]", &self.int_mask(7usize))
            .field("int_mask[8]", &self.int_mask(8usize))
            .field("int_mask[9]", &self.int_mask(9usize))
            .field("int_mask[10]", &self.int_mask(10usize))
            .field("int_mask[11]", &self.int_mask(11usize))
            .field("int_mask[12]", &self.int_mask(12usize))
            .field("int_mask[13]", &self.int_mask(13usize))
            .field("int_mask[14]", &self.int_mask(14usize))
            .field("int_mask[15]", &self.int_mask(15usize))
            .field("int_mask[16]", &self.int_mask(16usize))
            .field("int_mask[17]", &self.int_mask(17usize))
            .field("int_mask[18]", &self.int_mask(18usize))
            .field("int_mask[19]", &self.int_mask(19usize))
            .field("int_mask[20]", &self.int_mask(20usize))
            .field("int_mask[21]", &self.int_mask(21usize))
            .field("int_mask[22]", &self.int_mask(22usize))
            .field("int_mask[23]", &self.int_mask(23usize))
            .field("int_mask[24]", &self.int_mask(24usize))
            .field("int_mask[25]", &self.int_mask(25usize))
            .field("int_mask[26]", &self.int_mask(26usize))
            .field("int_mask[27]", &self.int_mask(27usize))
            .field("int_mask[28]", &self.int_mask(28usize))
            .field("int_mask[29]", &self.int_mask(29usize))
            .field("int_mask[30]", &self.int_mask(30usize))
            .field("int_mask[31]", &self.int_mask(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCpu1IntMask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecCpu1IntMask {{ int_mask[0]: {:?}, int_mask[1]: {:?}, int_mask[2]: {:?}, int_mask[3]: {:?}, int_mask[4]: {:?}, int_mask[5]: {:?}, int_mask[6]: {:?}, int_mask[7]: {:?}, int_mask[8]: {:?}, int_mask[9]: {:?}, int_mask[10]: {:?}, int_mask[11]: {:?}, int_mask[12]: {:?}, int_mask[13]: {:?}, int_mask[14]: {:?}, int_mask[15]: {:?}, int_mask[16]: {:?}, int_mask[17]: {:?}, int_mask[18]: {:?}, int_mask[19]: {:?}, int_mask[20]: {:?}, int_mask[21]: {:?}, int_mask[22]: {:?}, int_mask[23]: {:?}, int_mask[24]: {:?}, int_mask[25]: {:?}, int_mask[26]: {:?}, int_mask[27]: {:?}, int_mask[28]: {:?}, int_mask[29]: {:?}, int_mask[30]: {:?}, int_mask[31]: {:?} }}",
            self.int_mask(0usize),
            self.int_mask(1usize),
            self.int_mask(2usize),
            self.int_mask(3usize),
            self.int_mask(4usize),
            self.int_mask(5usize),
            self.int_mask(6usize),
            self.int_mask(7usize),
            self.int_mask(8usize),
            self.int_mask(9usize),
            self.int_mask(10usize),
            self.int_mask(11usize),
            self.int_mask(12usize),
            self.int_mask(13usize),
            self.int_mask(14usize),
            self.int_mask(15usize),
            self.int_mask(16usize),
            self.int_mask(17usize),
            self.int_mask(18usize),
            self.int_mask(19usize),
            self.int_mask(20usize),
            self.int_mask(21usize),
            self.int_mask(22usize),
            self.int_mask(23usize),
            self.int_mask(24usize),
            self.int_mask(25usize),
            self.int_mask(26usize),
            self.int_mask(27usize),
            self.int_mask(28usize),
            self.int_mask(29usize),
            self.int_mask(30usize),
            self.int_mask(31usize)
        )
    }
}
#[doc = "Secure Mask Lock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpRegLock(pub u32);
impl SecGpRegLock {
    #[doc = "Secure GPIO _MASK0 Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_gpio_mask0_lock(&self) -> SecGpioMask0Lock {
        let val = (self.0 >> 0usize) & 0x03;
        SecGpioMask0Lock::from_bits(val as u8)
    }
    #[doc = "Secure GPIO _MASK0 Lock."]
    #[inline(always)]
    pub const fn set_sec_gpio_mask0_lock(&mut self, val: SecGpioMask0Lock) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Secure GPIO _MASK1 Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_gpio_mask1_lock(&self) -> SecGpioMask1Lock {
        let val = (self.0 >> 2usize) & 0x03;
        SecGpioMask1Lock::from_bits(val as u8)
    }
    #[doc = "Secure GPIO _MASK1 Lock."]
    #[inline(always)]
    pub const fn set_sec_gpio_mask1_lock(&mut self, val: SecGpioMask1Lock) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "SEC_CPU1_INT_MASK Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_cpu1_int_mask_lock(&self, n: usize) -> SecCpu1IntMaskLock {
        assert!(n < 5usize);
        let offs = 12usize + n * 2usize;
        let val = (self.0 >> offs) & 0x03;
        SecCpu1IntMaskLock::from_bits(val as u8)
    }
    #[doc = "SEC_CPU1_INT_MASK Lock."]
    #[inline(always)]
    pub const fn set_sec_cpu1_int_mask_lock(&mut self, n: usize, val: SecCpu1IntMaskLock) {
        assert!(n < 5usize);
        let offs = 12usize + n * 2usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for SecGpRegLock {
    #[inline(always)]
    fn default() -> SecGpRegLock {
        SecGpRegLock(0)
    }
}
impl core::fmt::Debug for SecGpRegLock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpRegLock")
            .field("sec_gpio_mask0_lock", &self.sec_gpio_mask0_lock())
            .field("sec_gpio_mask1_lock", &self.sec_gpio_mask1_lock())
            .field(
                "sec_cpu1_int_mask_lock[0]",
                &self.sec_cpu1_int_mask_lock(0usize),
            )
            .field(
                "sec_cpu1_int_mask_lock[1]",
                &self.sec_cpu1_int_mask_lock(1usize),
            )
            .field(
                "sec_cpu1_int_mask_lock[2]",
                &self.sec_cpu1_int_mask_lock(2usize),
            )
            .field(
                "sec_cpu1_int_mask_lock[3]",
                &self.sec_cpu1_int_mask_lock(3usize),
            )
            .field(
                "sec_cpu1_int_mask_lock[4]",
                &self.sec_cpu1_int_mask_lock(4usize),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpRegLock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpRegLock {{ sec_gpio_mask0_lock: {:?}, sec_gpio_mask1_lock: {:?}, sec_cpu1_int_mask_lock[0]: {:?}, sec_cpu1_int_mask_lock[1]: {:?}, sec_cpu1_int_mask_lock[2]: {:?}, sec_cpu1_int_mask_lock[3]: {:?}, sec_cpu1_int_mask_lock[4]: {:?} }}",
            self.sec_gpio_mask0_lock(),
            self.sec_gpio_mask1_lock(),
            self.sec_cpu1_int_mask_lock(0usize),
            self.sec_cpu1_int_mask_lock(1usize),
            self.sec_cpu1_int_mask_lock(2usize),
            self.sec_cpu1_int_mask_lock(3usize),
            self.sec_cpu1_int_mask_lock(4usize)
        )
    }
}
#[doc = "GPIO Mask for Port index."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpioMask(pub u32);
impl SecGpioMask {
    #[doc = "Mask bit."]
    #[must_use]
    #[inline(always)]
    pub const fn pio0_pin_sec_mask(&self, n: usize) -> Pio0PinSecMask {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Pio0PinSecMask::from_bits(val as u8)
    }
    #[doc = "Mask bit."]
    #[inline(always)]
    pub const fn set_pio0_pin_sec_mask(&mut self, n: usize, val: Pio0PinSecMask) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for SecGpioMask {
    #[inline(always)]
    fn default() -> SecGpioMask {
        SecGpioMask(0)
    }
}
impl core::fmt::Debug for SecGpioMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpioMask")
            .field("pio0_pin_sec_mask[0]", &self.pio0_pin_sec_mask(0usize))
            .field("pio0_pin_sec_mask[1]", &self.pio0_pin_sec_mask(1usize))
            .field("pio0_pin_sec_mask[2]", &self.pio0_pin_sec_mask(2usize))
            .field("pio0_pin_sec_mask[3]", &self.pio0_pin_sec_mask(3usize))
            .field("pio0_pin_sec_mask[4]", &self.pio0_pin_sec_mask(4usize))
            .field("pio0_pin_sec_mask[5]", &self.pio0_pin_sec_mask(5usize))
            .field("pio0_pin_sec_mask[6]", &self.pio0_pin_sec_mask(6usize))
            .field("pio0_pin_sec_mask[7]", &self.pio0_pin_sec_mask(7usize))
            .field("pio0_pin_sec_mask[8]", &self.pio0_pin_sec_mask(8usize))
            .field("pio0_pin_sec_mask[9]", &self.pio0_pin_sec_mask(9usize))
            .field("pio0_pin_sec_mask[10]", &self.pio0_pin_sec_mask(10usize))
            .field("pio0_pin_sec_mask[11]", &self.pio0_pin_sec_mask(11usize))
            .field("pio0_pin_sec_mask[12]", &self.pio0_pin_sec_mask(12usize))
            .field("pio0_pin_sec_mask[13]", &self.pio0_pin_sec_mask(13usize))
            .field("pio0_pin_sec_mask[14]", &self.pio0_pin_sec_mask(14usize))
            .field("pio0_pin_sec_mask[15]", &self.pio0_pin_sec_mask(15usize))
            .field("pio0_pin_sec_mask[16]", &self.pio0_pin_sec_mask(16usize))
            .field("pio0_pin_sec_mask[17]", &self.pio0_pin_sec_mask(17usize))
            .field("pio0_pin_sec_mask[18]", &self.pio0_pin_sec_mask(18usize))
            .field("pio0_pin_sec_mask[19]", &self.pio0_pin_sec_mask(19usize))
            .field("pio0_pin_sec_mask[20]", &self.pio0_pin_sec_mask(20usize))
            .field("pio0_pin_sec_mask[21]", &self.pio0_pin_sec_mask(21usize))
            .field("pio0_pin_sec_mask[22]", &self.pio0_pin_sec_mask(22usize))
            .field("pio0_pin_sec_mask[23]", &self.pio0_pin_sec_mask(23usize))
            .field("pio0_pin_sec_mask[24]", &self.pio0_pin_sec_mask(24usize))
            .field("pio0_pin_sec_mask[25]", &self.pio0_pin_sec_mask(25usize))
            .field("pio0_pin_sec_mask[26]", &self.pio0_pin_sec_mask(26usize))
            .field("pio0_pin_sec_mask[27]", &self.pio0_pin_sec_mask(27usize))
            .field("pio0_pin_sec_mask[28]", &self.pio0_pin_sec_mask(28usize))
            .field("pio0_pin_sec_mask[29]", &self.pio0_pin_sec_mask(29usize))
            .field("pio0_pin_sec_mask[30]", &self.pio0_pin_sec_mask(30usize))
            .field("pio0_pin_sec_mask[31]", &self.pio0_pin_sec_mask(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpioMask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpioMask {{ pio0_pin_sec_mask[0]: {:?}, pio0_pin_sec_mask[1]: {:?}, pio0_pin_sec_mask[2]: {:?}, pio0_pin_sec_mask[3]: {:?}, pio0_pin_sec_mask[4]: {:?}, pio0_pin_sec_mask[5]: {:?}, pio0_pin_sec_mask[6]: {:?}, pio0_pin_sec_mask[7]: {:?}, pio0_pin_sec_mask[8]: {:?}, pio0_pin_sec_mask[9]: {:?}, pio0_pin_sec_mask[10]: {:?}, pio0_pin_sec_mask[11]: {:?}, pio0_pin_sec_mask[12]: {:?}, pio0_pin_sec_mask[13]: {:?}, pio0_pin_sec_mask[14]: {:?}, pio0_pin_sec_mask[15]: {:?}, pio0_pin_sec_mask[16]: {:?}, pio0_pin_sec_mask[17]: {:?}, pio0_pin_sec_mask[18]: {:?}, pio0_pin_sec_mask[19]: {:?}, pio0_pin_sec_mask[20]: {:?}, pio0_pin_sec_mask[21]: {:?}, pio0_pin_sec_mask[22]: {:?}, pio0_pin_sec_mask[23]: {:?}, pio0_pin_sec_mask[24]: {:?}, pio0_pin_sec_mask[25]: {:?}, pio0_pin_sec_mask[26]: {:?}, pio0_pin_sec_mask[27]: {:?}, pio0_pin_sec_mask[28]: {:?}, pio0_pin_sec_mask[29]: {:?}, pio0_pin_sec_mask[30]: {:?}, pio0_pin_sec_mask[31]: {:?} }}",
            self.pio0_pin_sec_mask(0usize),
            self.pio0_pin_sec_mask(1usize),
            self.pio0_pin_sec_mask(2usize),
            self.pio0_pin_sec_mask(3usize),
            self.pio0_pin_sec_mask(4usize),
            self.pio0_pin_sec_mask(5usize),
            self.pio0_pin_sec_mask(6usize),
            self.pio0_pin_sec_mask(7usize),
            self.pio0_pin_sec_mask(8usize),
            self.pio0_pin_sec_mask(9usize),
            self.pio0_pin_sec_mask(10usize),
            self.pio0_pin_sec_mask(11usize),
            self.pio0_pin_sec_mask(12usize),
            self.pio0_pin_sec_mask(13usize),
            self.pio0_pin_sec_mask(14usize),
            self.pio0_pin_sec_mask(15usize),
            self.pio0_pin_sec_mask(16usize),
            self.pio0_pin_sec_mask(17usize),
            self.pio0_pin_sec_mask(18usize),
            self.pio0_pin_sec_mask(19usize),
            self.pio0_pin_sec_mask(20usize),
            self.pio0_pin_sec_mask(21usize),
            self.pio0_pin_sec_mask(22usize),
            self.pio0_pin_sec_mask(23usize),
            self.pio0_pin_sec_mask(24usize),
            self.pio0_pin_sec_mask(25usize),
            self.pio0_pin_sec_mask(26usize),
            self.pio0_pin_sec_mask(27usize),
            self.pio0_pin_sec_mask(28usize),
            self.pio0_pin_sec_mask(29usize),
            self.pio0_pin_sec_mask(30usize),
            self.pio0_pin_sec_mask(31usize)
        )
    }
}
#[doc = "Security Violation Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecVioAddr(pub u32);
impl SecVioAddr {
    #[doc = "Security violation address for AHB layer a reset value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Security violation address for AHB layer a reset value 0."]
    #[inline(always)]
    pub const fn set_sec_vio_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SecVioAddr {
    #[inline(always)]
    fn default() -> SecVioAddr {
        SecVioAddr(0)
    }
}
impl core::fmt::Debug for SecVioAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecVioAddr")
            .field("sec_vio_addr", &self.sec_vio_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecVioAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecVioAddr {{ sec_vio_addr: {=u32:?} }}",
            self.sec_vio_addr()
        )
    }
}
#[doc = "Security Violation Info Validity for Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecVioInfoValid(pub u32);
impl SecVioInfoValid {
    #[doc = "Violation information valid flag for AHB port index."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid(&self, n: usize) -> bool {
        assert!(n < 19usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port index."]
    #[inline(always)]
    pub const fn set_vio_info_valid(&mut self, n: usize, val: bool) {
        assert!(n < 19usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for SecVioInfoValid {
    #[inline(always)]
    fn default() -> SecVioInfoValid {
        SecVioInfoValid(0)
    }
}
impl core::fmt::Debug for SecVioInfoValid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecVioInfoValid")
            .field("vio_info_valid[0]", &self.vio_info_valid(0usize))
            .field("vio_info_valid[1]", &self.vio_info_valid(1usize))
            .field("vio_info_valid[2]", &self.vio_info_valid(2usize))
            .field("vio_info_valid[3]", &self.vio_info_valid(3usize))
            .field("vio_info_valid[4]", &self.vio_info_valid(4usize))
            .field("vio_info_valid[5]", &self.vio_info_valid(5usize))
            .field("vio_info_valid[6]", &self.vio_info_valid(6usize))
            .field("vio_info_valid[7]", &self.vio_info_valid(7usize))
            .field("vio_info_valid[8]", &self.vio_info_valid(8usize))
            .field("vio_info_valid[9]", &self.vio_info_valid(9usize))
            .field("vio_info_valid[10]", &self.vio_info_valid(10usize))
            .field("vio_info_valid[11]", &self.vio_info_valid(11usize))
            .field("vio_info_valid[12]", &self.vio_info_valid(12usize))
            .field("vio_info_valid[13]", &self.vio_info_valid(13usize))
            .field("vio_info_valid[14]", &self.vio_info_valid(14usize))
            .field("vio_info_valid[15]", &self.vio_info_valid(15usize))
            .field("vio_info_valid[16]", &self.vio_info_valid(16usize))
            .field("vio_info_valid[17]", &self.vio_info_valid(17usize))
            .field("vio_info_valid[18]", &self.vio_info_valid(18usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecVioInfoValid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecVioInfoValid {{ vio_info_valid[0]: {=bool:?}, vio_info_valid[1]: {=bool:?}, vio_info_valid[2]: {=bool:?}, vio_info_valid[3]: {=bool:?}, vio_info_valid[4]: {=bool:?}, vio_info_valid[5]: {=bool:?}, vio_info_valid[6]: {=bool:?}, vio_info_valid[7]: {=bool:?}, vio_info_valid[8]: {=bool:?}, vio_info_valid[9]: {=bool:?}, vio_info_valid[10]: {=bool:?}, vio_info_valid[11]: {=bool:?}, vio_info_valid[12]: {=bool:?}, vio_info_valid[13]: {=bool:?}, vio_info_valid[14]: {=bool:?}, vio_info_valid[15]: {=bool:?}, vio_info_valid[16]: {=bool:?}, vio_info_valid[17]: {=bool:?}, vio_info_valid[18]: {=bool:?} }}",
            self.vio_info_valid(0usize),
            self.vio_info_valid(1usize),
            self.vio_info_valid(2usize),
            self.vio_info_valid(3usize),
            self.vio_info_valid(4usize),
            self.vio_info_valid(5usize),
            self.vio_info_valid(6usize),
            self.vio_info_valid(7usize),
            self.vio_info_valid(8usize),
            self.vio_info_valid(9usize),
            self.vio_info_valid(10usize),
            self.vio_info_valid(11usize),
            self.vio_info_valid(12usize),
            self.vio_info_valid(13usize),
            self.vio_info_valid(14usize),
            self.vio_info_valid(15usize),
            self.vio_info_valid(16usize),
            self.vio_info_valid(17usize),
            self.vio_info_valid(18usize)
        )
    }
}
#[doc = "Security Violation Miscellaneous Information at Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecVioMiscInfo(pub u32);
impl SecVioMiscInfo {
    #[doc = "Security violation access read/write indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_write(&self) -> SecVioInfoWrite {
        let val = (self.0 >> 0usize) & 0x01;
        SecVioInfoWrite::from_bits(val as u8)
    }
    #[doc = "Security violation access read/write indicator."]
    #[inline(always)]
    pub const fn set_sec_vio_info_write(&mut self, val: SecVioInfoWrite) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Security Violation Info Data Access."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_data_access(&self) -> SecVioInfoDataAccess {
        let val = (self.0 >> 1usize) & 0x01;
        SecVioInfoDataAccess::from_bits(val as u8)
    }
    #[doc = "Security Violation Info Data Access."]
    #[inline(always)]
    pub const fn set_sec_vio_info_data_access(&mut self, val: SecVioInfoDataAccess) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Security Violation Info Master Security Level."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_master_sec_level(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Security Violation Info Master Security Level."]
    #[inline(always)]
    pub const fn set_sec_vio_info_master_sec_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Security violation master number."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_master(&self) -> SecVioInfoMaster {
        let val = (self.0 >> 8usize) & 0x1f;
        SecVioInfoMaster::from_bits(val as u8)
    }
    #[doc = "Security violation master number."]
    #[inline(always)]
    pub const fn set_sec_vio_info_master(&mut self, val: SecVioInfoMaster) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
}
impl Default for SecVioMiscInfo {
    #[inline(always)]
    fn default() -> SecVioMiscInfo {
        SecVioMiscInfo(0)
    }
}
impl core::fmt::Debug for SecVioMiscInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecVioMiscInfo")
            .field("sec_vio_info_write", &self.sec_vio_info_write())
            .field("sec_vio_info_data_access", &self.sec_vio_info_data_access())
            .field(
                "sec_vio_info_master_sec_level",
                &self.sec_vio_info_master_sec_level(),
            )
            .field("sec_vio_info_master", &self.sec_vio_info_master())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecVioMiscInfo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecVioMiscInfo {{ sec_vio_info_write: {:?}, sec_vio_info_data_access: {:?}, sec_vio_info_master_sec_level: {=u8:?}, sec_vio_info_master: {:?} }}",
            self.sec_vio_info_write(),
            self.sec_vio_info_data_access(),
            self.sec_vio_info_master_sec_level(),
            self.sec_vio_info_master()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Adc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0 {
    #[inline(always)]
    fn from(val: u8) -> Adc0 {
        Adc0::from_bits(val)
    }
}
impl From<Adc0> for u8 {
    #[inline(always)]
    fn from(val: Adc0) -> u8 {
        Adc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Adc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1 {
    #[inline(always)]
    fn from(val: u8) -> Adc1 {
        Adc1::from_bits(val)
    }
}
impl From<Adc1> for u8 {
    #[inline(always)]
    fn from(val: Adc1) -> u8 {
        Adc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSecureCtrlPeripheralRule0Rule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl AhbSecureCtrlPeripheralRule0Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSecureCtrlPeripheralRule0Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSecureCtrlPeripheralRule0Rule {
    #[inline(always)]
    fn from(val: u8) -> AhbSecureCtrlPeripheralRule0Rule {
        AhbSecureCtrlPeripheralRule0Rule::from_bits(val)
    }
}
impl From<AhbSecureCtrlPeripheralRule0Rule> for u8 {
    #[inline(always)]
    fn from(val: AhbSecureCtrlPeripheralRule0Rule) -> u8 {
        AhbSecureCtrlPeripheralRule0Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApbPeripheralGroup1MemRule1Pkc {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl ApbPeripheralGroup1MemRule1Pkc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApbPeripheralGroup1MemRule1Pkc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApbPeripheralGroup1MemRule1Pkc {
    #[inline(always)]
    fn from(val: u8) -> ApbPeripheralGroup1MemRule1Pkc {
        ApbPeripheralGroup1MemRule1Pkc::from_bits(val)
    }
}
impl From<ApbPeripheralGroup1MemRule1Pkc> for u8 {
    #[inline(always)]
    fn from(val: ApbPeripheralGroup1MemRule1Pkc) -> u8 {
        ApbPeripheralGroup1MemRule1Pkc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApbPeripheralGroup1MemRule2Smartdma {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl ApbPeripheralGroup1MemRule2Smartdma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApbPeripheralGroup1MemRule2Smartdma {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApbPeripheralGroup1MemRule2Smartdma {
    #[inline(always)]
    fn from(val: u8) -> ApbPeripheralGroup1MemRule2Smartdma {
        ApbPeripheralGroup1MemRule2Smartdma::from_bits(val)
    }
}
impl From<ApbPeripheralGroup1MemRule2Smartdma> for u8 {
    #[inline(always)]
    fn from(val: ApbPeripheralGroup1MemRule2Smartdma) -> u8 {
        ApbPeripheralGroup1MemRule2Smartdma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Atx0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Atx0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Atx0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Atx0 {
    #[inline(always)]
    fn from(val: u8) -> Atx0 {
        Atx0::from_bits(val)
    }
}
impl From<Atx0> for u8 {
    #[inline(always)]
    fn from(val: Atx0) -> u8 {
        Atx0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cache64Polsel0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Cache64Polsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cache64Polsel0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cache64Polsel0 {
    #[inline(always)]
    fn from(val: u8) -> Cache64Polsel0 {
        Cache64Polsel0::from_bits(val)
    }
}
impl From<Cache64Polsel0> for u8 {
    #[inline(always)]
    fn from(val: Cache64Polsel0) -> u8 {
        Cache64Polsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can0Rule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Can0Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can0Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can0Rule {
    #[inline(always)]
    fn from(val: u8) -> Can0Rule {
        Can0Rule::from_bits(val)
    }
}
impl From<Can0Rule> for u8 {
    #[inline(always)]
    fn from(val: Can0Rule) -> u8 {
        Can0Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can1Rule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Can1Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can1Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can1Rule {
    #[inline(always)]
    fn from(val: u8) -> Can1Rule {
        Can1Rule::from_bits(val)
    }
}
impl From<Can1Rule> for u8 {
    #[inline(always)]
    fn from(val: Can1Rule) -> u8 {
        Can1Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cdog0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Cdog0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdog0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdog0 {
    #[inline(always)]
    fn from(val: u8) -> Cdog0 {
        Cdog0::from_bits(val)
    }
}
impl From<Cdog0> for u8 {
    #[inline(always)]
    fn from(val: Cdog0) -> u8 {
        Cdog0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cdog1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Cdog1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdog1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdog1 {
    #[inline(always)]
    fn from(val: u8) -> Cdog1 {
        Cdog1::from_bits(val)
    }
}
impl From<Cdog1> for u8 {
    #[inline(always)]
    fn from(val: Cdog1) -> u8 {
        Cdog1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cm33LockRegLock {
    _RESERVED_0 = 0x0,
    #[doc = "CM33_LOCK_REG_LOCK is 1."]
    Cm33LockRegLockEq1 = 0x01,
    #[doc = "CM33_LOCK_REG_LOCK is 0."]
    Cm33LockRegLockEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Cm33LockRegLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cm33LockRegLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cm33LockRegLock {
    #[inline(always)]
    fn from(val: u8) -> Cm33LockRegLock {
        Cm33LockRegLock::from_bits(val)
    }
}
impl From<Cm33LockRegLock> for u8 {
    #[inline(always)]
    fn from(val: Cm33LockRegLock) -> u8 {
        Cm33LockRegLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Cmp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp {
    #[inline(always)]
    fn from(val: u8) -> Cmp {
        Cmp::from_bits(val)
    }
}
impl From<Cmp> for u8 {
    #[inline(always)]
    fn from(val: Cmp) -> u8 {
        Cmp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Coolflux {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Coolflux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Coolflux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Coolflux {
    #[inline(always)]
    fn from(val: u8) -> Coolflux {
        Coolflux::from_bits(val)
    }
}
impl From<Coolflux> for u8 {
    #[inline(always)]
    fn from(val: Coolflux) -> u8 {
        Coolflux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0LockRegLockNsMpu {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_NS_MPU is 1."]
    LockNsMpuEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_NS_MPU is 0."]
    LockNsMpuEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Cpu0LockRegLockNsMpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0LockRegLockNsMpu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0LockRegLockNsMpu {
    #[inline(always)]
    fn from(val: u8) -> Cpu0LockRegLockNsMpu {
        Cpu0LockRegLockNsMpu::from_bits(val)
    }
}
impl From<Cpu0LockRegLockNsMpu> for u8 {
    #[inline(always)]
    fn from(val: Cpu0LockRegLockNsMpu) -> u8 {
        Cpu0LockRegLockNsMpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0LockRegLockNsVtor {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCKNSVTOR is 1."]
    LockNsVtorEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCKNSVTOR is 0."]
    LockNsVtorEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Cpu0LockRegLockNsVtor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0LockRegLockNsVtor {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0LockRegLockNsVtor {
    #[inline(always)]
    fn from(val: u8) -> Cpu0LockRegLockNsVtor {
        Cpu0LockRegLockNsVtor::from_bits(val)
    }
}
impl From<Cpu0LockRegLockNsVtor> for u8 {
    #[inline(always)]
    fn from(val: Cpu0LockRegLockNsVtor) -> u8 {
        Cpu0LockRegLockNsVtor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1LockRegLockNsMpu {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_NS_MPU is 1."]
    LockNsMpuEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_NS_MPU is 0."]
    LockNsMpuEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Cpu1LockRegLockNsMpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1LockRegLockNsMpu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1LockRegLockNsMpu {
    #[inline(always)]
    fn from(val: u8) -> Cpu1LockRegLockNsMpu {
        Cpu1LockRegLockNsMpu::from_bits(val)
    }
}
impl From<Cpu1LockRegLockNsMpu> for u8 {
    #[inline(always)]
    fn from(val: Cpu1LockRegLockNsMpu) -> u8 {
        Cpu1LockRegLockNsMpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1LockRegLockNsVtor {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCKNSVTOR is 1."]
    LockNsVtorEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCKNSVTOR is 0."]
    LockNsVtorEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Cpu1LockRegLockNsVtor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1LockRegLockNsVtor {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1LockRegLockNsVtor {
    #[inline(always)]
    fn from(val: u8) -> Cpu1LockRegLockNsVtor {
        Cpu1LockRegLockNsVtor::from_bits(val)
    }
}
impl From<Cpu1LockRegLockNsVtor> for u8 {
    #[inline(always)]
    fn from(val: Cpu1LockRegLockNsVtor) -> u8 {
        Cpu1LockRegLockNsVtor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crc {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Crc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crc {
    #[inline(always)]
    fn from(val: u8) -> Crc {
        Crc::from_bits(val)
    }
}
impl From<Crc> for u8 {
    #[inline(always)]
    fn from(val: Crc) -> u8 {
        Crc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Ctimer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer {
    #[inline(always)]
    fn from(val: u8) -> Ctimer {
        Ctimer::from_bits(val)
    }
}
impl From<Ctimer> for u8 {
    #[inline(always)]
    fn from(val: Ctimer) -> u8 {
        Ctimer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Ctimer4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer4 {
    #[inline(always)]
    fn from(val: u8) -> Ctimer4 {
        Ctimer4::from_bits(val)
    }
}
impl From<Ctimer4> for u8 {
    #[inline(always)]
    fn from(val: Ctimer4) -> u8 {
        Ctimer4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Dac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac {
    #[inline(always)]
    fn from(val: u8) -> Dac {
        Dac::from_bits(val)
    }
}
impl From<Dac> for u8 {
    #[inline(always)]
    fn from(val: Dac) -> u8 {
        Dac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Dac0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac0 {
    #[inline(always)]
    fn from(val: u8) -> Dac0 {
        Dac0::from_bits(val)
    }
}
impl From<Dac0> for u8 {
    #[inline(always)]
    fn from(val: Dac0) -> u8 {
        Dac0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugMailbox {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl DebugMailbox {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugMailbox {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugMailbox {
    #[inline(always)]
    fn from(val: u8) -> DebugMailbox {
        DebugMailbox::from_bits(val)
    }
}
impl From<DebugMailbox> for u8 {
    #[inline(always)]
    fn from(val: DebugMailbox) -> u8 {
        DebugMailbox::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Digtmp {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Digtmp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Digtmp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Digtmp {
    #[inline(always)]
    fn from(val: u8) -> Digtmp {
        Digtmp::from_bits(val)
    }
}
impl From<Digtmp> for u8 {
    #[inline(always)]
    fn from(val: Digtmp) -> u8 {
        Digtmp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch0 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch0 {
        EDma0Ch0::from_bits(val)
    }
}
impl From<EDma0Ch0> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch0) -> u8 {
        EDma0Ch0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch1 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch1 {
        EDma0Ch1::from_bits(val)
    }
}
impl From<EDma0Ch1> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch1) -> u8 {
        EDma0Ch1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch10 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch10 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch10 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch10 {
        EDma0Ch10::from_bits(val)
    }
}
impl From<EDma0Ch10> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch10) -> u8 {
        EDma0Ch10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch11 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch11 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch11 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch11 {
        EDma0Ch11::from_bits(val)
    }
}
impl From<EDma0Ch11> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch11) -> u8 {
        EDma0Ch11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch12 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch12 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch12 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch12 {
        EDma0Ch12::from_bits(val)
    }
}
impl From<EDma0Ch12> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch12) -> u8 {
        EDma0Ch12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch13 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch13 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch13 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch13 {
        EDma0Ch13::from_bits(val)
    }
}
impl From<EDma0Ch13> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch13) -> u8 {
        EDma0Ch13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch14 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch14 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch14 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch14 {
        EDma0Ch14::from_bits(val)
    }
}
impl From<EDma0Ch14> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch14) -> u8 {
        EDma0Ch14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch15 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch15 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch15 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch15 {
        EDma0Ch15::from_bits(val)
    }
}
impl From<EDma0Ch15> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch15) -> u8 {
        EDma0Ch15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch2 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch2 {
        EDma0Ch2::from_bits(val)
    }
}
impl From<EDma0Ch2> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch2) -> u8 {
        EDma0Ch2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch3 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch3 {
        EDma0Ch3::from_bits(val)
    }
}
impl From<EDma0Ch3> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch3) -> u8 {
        EDma0Ch3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch4 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch4 {
        EDma0Ch4::from_bits(val)
    }
}
impl From<EDma0Ch4> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch4) -> u8 {
        EDma0Ch4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch5 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch5 {
        EDma0Ch5::from_bits(val)
    }
}
impl From<EDma0Ch5> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch5) -> u8 {
        EDma0Ch5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch6 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch6 {
        EDma0Ch6::from_bits(val)
    }
}
impl From<EDma0Ch6> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch6) -> u8 {
        EDma0Ch6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch7 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch7 {
        EDma0Ch7::from_bits(val)
    }
}
impl From<EDma0Ch7> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch7) -> u8 {
        EDma0Ch7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch8 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch8 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch8 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch8 {
        EDma0Ch8::from_bits(val)
    }
}
impl From<EDma0Ch8> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch8) -> u8 {
        EDma0Ch8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Ch9 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Ch9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Ch9 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Ch9 {
    #[inline(always)]
    fn from(val: u8) -> EDma0Ch9 {
        EDma0Ch9::from_bits(val)
    }
}
impl From<EDma0Ch9> for u8 {
    #[inline(always)]
    fn from(val: EDma0Ch9) -> u8 {
        EDma0Ch9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma0Mp {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma0Mp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma0Mp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma0Mp {
    #[inline(always)]
    fn from(val: u8) -> EDma0Mp {
        EDma0Mp::from_bits(val)
    }
}
impl From<EDma0Mp> for u8 {
    #[inline(always)]
    fn from(val: EDma0Mp) -> u8 {
        EDma0Mp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma1Ch {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma1Ch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma1Ch {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma1Ch {
    #[inline(always)]
    fn from(val: u8) -> EDma1Ch {
        EDma1Ch::from_bits(val)
    }
}
impl From<EDma1Ch> for u8 {
    #[inline(always)]
    fn from(val: EDma1Ch) -> u8 {
        EDma1Ch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma1Ch15 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma1Ch15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma1Ch15 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma1Ch15 {
    #[inline(always)]
    fn from(val: u8) -> EDma1Ch15 {
        EDma1Ch15::from_bits(val)
    }
}
impl From<EDma1Ch15> for u8 {
    #[inline(always)]
    fn from(val: EDma1Ch15) -> u8 {
        EDma1Ch15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EDma1Mp {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl EDma1Mp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EDma1Mp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EDma1Mp {
    #[inline(always)]
    fn from(val: u8) -> EDma1Mp {
        EDma1Mp::from_bits(val)
    }
}
impl From<EDma1Mp> for u8 {
    #[inline(always)]
    fn from(val: EDma1Mp) -> u8 {
        EDma1Mp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eim0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Eim0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eim0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eim0 {
    #[inline(always)]
    fn from(val: u8) -> Eim0 {
        Eim0::from_bits(val)
    }
}
impl From<Eim0> for u8 {
    #[inline(always)]
    fn from(val: Eim0) -> u8 {
        Eim0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Els {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Els {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Els {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Els {
    #[inline(always)]
    fn from(val: u8) -> Els {
        Els::from_bits(val)
    }
}
impl From<Els> for u8 {
    #[inline(always)]
    fn from(val: Els) -> u8 {
        Els::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ElsAlias {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl ElsAlias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ElsAlias {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ElsAlias {
    #[inline(always)]
    fn from(val: u8) -> ElsAlias {
        ElsAlias::from_bits(val)
    }
}
impl From<ElsAlias> for u8 {
    #[inline(always)]
    fn from(val: ElsAlias) -> u8 {
        ElsAlias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Emvsim0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim0 {
    #[inline(always)]
    fn from(val: u8) -> Emvsim0 {
        Emvsim0::from_bits(val)
    }
}
impl From<Emvsim0> for u8 {
    #[inline(always)]
    fn from(val: Emvsim0) -> u8 {
        Emvsim0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Emvsim1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim1 {
    #[inline(always)]
    fn from(val: u8) -> Emvsim1 {
        Emvsim1::from_bits(val)
    }
}
impl From<Emvsim1> for u8 {
    #[inline(always)]
    fn from(val: Emvsim1) -> u8 {
        Emvsim1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enc {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Enc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enc {
    #[inline(always)]
    fn from(val: u8) -> Enc {
        Enc::from_bits(val)
    }
}
impl From<Enc> for u8 {
    #[inline(always)]
    fn from(val: Enc) -> u8 {
        Enc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enc1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Enc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enc1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enc1 {
    #[inline(always)]
    fn from(val: u8) -> Enc1 {
        Enc1::from_bits(val)
    }
}
impl From<Enc1> for u8 {
    #[inline(always)]
    fn from(val: Enc1) -> u8 {
        Enc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Enet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet {
    #[inline(always)]
    fn from(val: u8) -> Enet {
        Enet::from_bits(val)
    }
}
impl From<Enet> for u8 {
    #[inline(always)]
    fn from(val: Enet) -> u8 {
        Enet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Erm0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Erm0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erm0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erm0 {
    #[inline(always)]
    fn from(val: u8) -> Erm0 {
        Erm0::from_bits(val)
    }
}
impl From<Erm0> for u8 {
    #[inline(always)]
    fn from(val: Erm0) -> u8 {
        Erm0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Evtg {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Evtg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Evtg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Evtg {
    #[inline(always)]
    fn from(val: u8) -> Evtg {
        Evtg::from_bits(val)
    }
}
impl From<Evtg> for u8 {
    #[inline(always)]
    fn from(val: Evtg) -> u8 {
        Evtg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ewm0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Ewm0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ewm0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ewm0 {
    #[inline(always)]
    fn from(val: u8) -> Ewm0 {
        Ewm0::from_bits(val)
    }
}
impl From<Ewm0> for u8 {
    #[inline(always)]
    fn from(val: Ewm0) -> u8 {
        Ewm0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash00MemRuleRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule {
        Flash00MemRuleRule::from_bits(val)
    }
}
impl From<Flash00MemRuleRule> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule) -> u8 {
        Flash00MemRuleRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash01MemRuleRule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash01MemRuleRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash01MemRuleRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash01MemRuleRule {
    #[inline(always)]
    fn from(val: u8) -> Flash01MemRuleRule {
        Flash01MemRuleRule::from_bits(val)
    }
}
impl From<Flash01MemRuleRule> for u8 {
    #[inline(always)]
    fn from(val: Flash01MemRuleRule) -> u8 {
        Flash01MemRuleRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash02MemRuleRule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash02MemRuleRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash02MemRuleRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash02MemRuleRule {
    #[inline(always)]
    fn from(val: u8) -> Flash02MemRuleRule {
        Flash02MemRuleRule::from_bits(val)
    }
}
impl From<Flash02MemRuleRule> for u8 {
    #[inline(always)]
    fn from(val: Flash02MemRuleRule) -> u8 {
        Flash02MemRuleRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash03MemRuleRule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flash03MemRuleRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash03MemRuleRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash03MemRuleRule {
    #[inline(always)]
    fn from(val: u8) -> Flash03MemRuleRule {
        Flash03MemRuleRule::from_bits(val)
    }
}
impl From<Flash03MemRuleRule> for u8 {
    #[inline(always)]
    fn from(val: Flash03MemRuleRule) -> u8 {
        Flash03MemRuleRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flexcomm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm {
        Flexcomm::from_bits(val)
    }
}
impl From<Flexcomm> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm) -> u8 {
        Flexcomm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flexio {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio {
    #[inline(always)]
    fn from(val: u8) -> Flexio {
        Flexio::from_bits(val)
    }
}
impl From<Flexio> for u8 {
    #[inline(always)]
    fn from(val: Flexio) -> u8 {
        Flexio::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flexspi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi {
    #[inline(always)]
    fn from(val: u8) -> Flexspi {
        Flexspi::from_bits(val)
    }
}
impl From<Flexspi> for u8 {
    #[inline(always)]
    fn from(val: Flexspi) -> u8 {
        Flexspi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0MemRuleRule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flexspi0Region0MemRuleRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0MemRuleRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0MemRuleRule {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0MemRuleRule {
        Flexspi0Region0MemRuleRule::from_bits(val)
    }
}
impl From<Flexspi0Region0MemRuleRule> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0MemRuleRule) -> u8 {
        Flexspi0Region0MemRuleRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region16MemRuleFlexspi0RegionMemRule0Rule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flexspi0Region16MemRuleFlexspi0RegionMemRule0Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region16MemRuleFlexspi0RegionMemRule0Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region16MemRuleFlexspi0RegionMemRule0Rule {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region16MemRuleFlexspi0RegionMemRule0Rule {
        Flexspi0Region16MemRuleFlexspi0RegionMemRule0Rule::from_bits(val)
    }
}
impl From<Flexspi0Region16MemRuleFlexspi0RegionMemRule0Rule> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region16MemRuleFlexspi0RegionMemRule0Rule) -> u8 {
        Flexspi0Region16MemRuleFlexspi0RegionMemRule0Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region7MemRuleRule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flexspi0Region7MemRuleRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region7MemRuleRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region7MemRuleRule {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region7MemRuleRule {
        Flexspi0Region7MemRuleRule::from_bits(val)
    }
}
impl From<Flexspi0Region7MemRuleRule> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region7MemRuleRule) -> u8 {
        Flexspi0Region7MemRuleRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region813MemRuleFlexspi0RegionMemRule0Rule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Flexspi0Region813MemRuleFlexspi0RegionMemRule0Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region813MemRuleFlexspi0RegionMemRule0Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region813MemRuleFlexspi0RegionMemRule0Rule {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region813MemRuleFlexspi0RegionMemRule0Rule {
        Flexspi0Region813MemRuleFlexspi0RegionMemRule0Rule::from_bits(val)
    }
}
impl From<Flexspi0Region813MemRuleFlexspi0RegionMemRule0Rule> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region813MemRuleFlexspi0RegionMemRule0Rule) -> u8 {
        Flexspi0Region813MemRuleFlexspi0RegionMemRule0Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiCmx {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl FlexspiCmx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiCmx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiCmx {
    #[inline(always)]
    fn from(val: u8) -> FlexspiCmx {
        FlexspiCmx::from_bits(val)
    }
}
impl From<FlexspiCmx> for u8 {
    #[inline(always)]
    fn from(val: FlexspiCmx) -> u8 {
        FlexspiCmx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fmu0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Fmu0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fmu0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fmu0 {
    #[inline(always)]
    fn from(val: u8) -> Fmu0 {
        Fmu0::from_bits(val)
    }
}
impl From<Fmu0> for u8 {
    #[inline(always)]
    fn from(val: Fmu0) -> u8 {
        Fmu0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmuTest {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl FmuTest {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmuTest {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmuTest {
    #[inline(always)]
    fn from(val: u8) -> FmuTest {
        FmuTest::from_bits(val)
    }
}
impl From<FmuTest> for u8 {
    #[inline(always)]
    fn from(val: FmuTest) -> u8 {
        FmuTest::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Freqme0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Freqme0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Freqme0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Freqme0 {
    #[inline(always)]
    fn from(val: u8) -> Freqme0 {
        Freqme0::from_bits(val)
    }
}
impl From<Freqme0> for u8 {
    #[inline(always)]
    fn from(val: Freqme0) -> u8 {
        Freqme0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gdet {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gdet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gdet {
    #[inline(always)]
    fn from(val: u8) -> Gdet {
        Gdet::from_bits(val)
    }
}
impl From<Gdet> for u8 {
    #[inline(always)]
    fn from(val: Gdet) -> u8 {
        Gdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio0Alias0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio0Alias0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio0Alias0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio0Alias0 {
    #[inline(always)]
    fn from(val: u8) -> Gpio0Alias0 {
        Gpio0Alias0::from_bits(val)
    }
}
impl From<Gpio0Alias0> for u8 {
    #[inline(always)]
    fn from(val: Gpio0Alias0) -> u8 {
        Gpio0Alias0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio0Alias1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio0Alias1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio0Alias1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio0Alias1 {
    #[inline(always)]
    fn from(val: u8) -> Gpio0Alias1 {
        Gpio0Alias1::from_bits(val)
    }
}
impl From<Gpio0Alias1> for u8 {
    #[inline(always)]
    fn from(val: Gpio0Alias1) -> u8 {
        Gpio0Alias1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio1Alias0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio1Alias0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio1Alias0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio1Alias0 {
    #[inline(always)]
    fn from(val: u8) -> Gpio1Alias0 {
        Gpio1Alias0::from_bits(val)
    }
}
impl From<Gpio1Alias0> for u8 {
    #[inline(always)]
    fn from(val: Gpio1Alias0) -> u8 {
        Gpio1Alias0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio1Alias1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio1Alias1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio1Alias1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio1Alias1 {
    #[inline(always)]
    fn from(val: u8) -> Gpio1Alias1 {
        Gpio1Alias1::from_bits(val)
    }
}
impl From<Gpio1Alias1> for u8 {
    #[inline(always)]
    fn from(val: Gpio1Alias1) -> u8 {
        Gpio1Alias1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio2Alias0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio2Alias0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio2Alias0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio2Alias0 {
    #[inline(always)]
    fn from(val: u8) -> Gpio2Alias0 {
        Gpio2Alias0::from_bits(val)
    }
}
impl From<Gpio2Alias0> for u8 {
    #[inline(always)]
    fn from(val: Gpio2Alias0) -> u8 {
        Gpio2Alias0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio2Alias1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio2Alias1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio2Alias1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio2Alias1 {
    #[inline(always)]
    fn from(val: u8) -> Gpio2Alias1 {
        Gpio2Alias1::from_bits(val)
    }
}
impl From<Gpio2Alias1> for u8 {
    #[inline(always)]
    fn from(val: Gpio2Alias1) -> u8 {
        Gpio2Alias1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio3Alias0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio3Alias0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio3Alias0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio3Alias0 {
    #[inline(always)]
    fn from(val: u8) -> Gpio3Alias0 {
        Gpio3Alias0::from_bits(val)
    }
}
impl From<Gpio3Alias0> for u8 {
    #[inline(always)]
    fn from(val: Gpio3Alias0) -> u8 {
        Gpio3Alias0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio3Alias1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio3Alias1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio3Alias1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio3Alias1 {
    #[inline(always)]
    fn from(val: u8) -> Gpio3Alias1 {
        Gpio3Alias1::from_bits(val)
    }
}
impl From<Gpio3Alias1> for u8 {
    #[inline(always)]
    fn from(val: Gpio3Alias1) -> u8 {
        Gpio3Alias1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio4Alias0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio4Alias0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio4Alias0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio4Alias0 {
    #[inline(always)]
    fn from(val: u8) -> Gpio4Alias0 {
        Gpio4Alias0::from_bits(val)
    }
}
impl From<Gpio4Alias0> for u8 {
    #[inline(always)]
    fn from(val: Gpio4Alias0) -> u8 {
        Gpio4Alias0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio4Alias1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio4Alias1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio4Alias1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio4Alias1 {
    #[inline(always)]
    fn from(val: u8) -> Gpio4Alias1 {
        Gpio4Alias1::from_bits(val)
    }
}
impl From<Gpio4Alias1> for u8 {
    #[inline(always)]
    fn from(val: Gpio4Alias1) -> u8 {
        Gpio4Alias1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio5Alias0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio5Alias0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio5Alias0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio5Alias0 {
    #[inline(always)]
    fn from(val: u8) -> Gpio5Alias0 {
        Gpio5Alias0::from_bits(val)
    }
}
impl From<Gpio5Alias0> for u8 {
    #[inline(always)]
    fn from(val: Gpio5Alias0) -> u8 {
        Gpio5Alias0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio5Alias1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Gpio5Alias1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio5Alias1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio5Alias1 {
    #[inline(always)]
    fn from(val: u8) -> Gpio5Alias1 {
        Gpio5Alias1::from_bits(val)
    }
}
impl From<Gpio5Alias1> for u8 {
    #[inline(always)]
    fn from(val: Gpio5Alias1) -> u8 {
        Gpio5Alias1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hpdac0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Hpdac0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hpdac0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hpdac0 {
    #[inline(always)]
    fn from(val: u8) -> Hpdac0 {
        Hpdac0::from_bits(val)
    }
}
impl From<Hpdac0> for u8 {
    #[inline(always)]
    fn from(val: Hpdac0) -> u8 {
        Hpdac0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl I3c0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0 {
    #[inline(always)]
    fn from(val: u8) -> I3c0 {
        I3c0::from_bits(val)
    }
}
impl From<I3c0> for u8 {
    #[inline(always)]
    fn from(val: I3c0) -> u8 {
        I3c0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl I3c1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1 {
    #[inline(always)]
    fn from(val: u8) -> I3c1 {
        I3c1::from_bits(val)
    }
}
impl From<I3c1> for u8 {
    #[inline(always)]
    fn from(val: I3c1) -> u8 {
        I3c1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inputmux {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Inputmux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inputmux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inputmux {
    #[inline(always)]
    fn from(val: u8) -> Inputmux {
        Inputmux::from_bits(val)
    }
}
impl From<Inputmux> for u8 {
    #[inline(always)]
    fn from(val: Inputmux) -> u8 {
        Inputmux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl IntMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntMask {
    #[inline(always)]
    fn from(val: u8) -> IntMask {
        IntMask::from_bits(val)
    }
}
impl From<IntMask> for u8 {
    #[inline(always)]
    fn from(val: IntMask) -> u8 {
        IntMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intm0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Intm0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intm0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intm0 {
    #[inline(always)]
    fn from(val: u8) -> Intm0 {
        Intm0::from_bits(val)
    }
}
impl From<Intm0> for u8 {
    #[inline(always)]
    fn from(val: Intm0) -> u8 {
        Intm0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Itrc {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Itrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Itrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Itrc {
    #[inline(always)]
    fn from(val: u8) -> Itrc {
        Itrc::from_bits(val)
    }
}
impl From<Itrc> for u8 {
    #[inline(always)]
    fn from(val: Itrc) -> u8 {
        Itrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSMpu {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_S_MPU is 1."]
    LockSMpuEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_S_MPU is 0."]
    LockSMpuEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockSMpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSMpu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSMpu {
    #[inline(always)]
    fn from(val: u8) -> LockSMpu {
        LockSMpu::from_bits(val)
    }
}
impl From<LockSMpu> for u8 {
    #[inline(always)]
    fn from(val: LockSMpu) -> u8 {
        LockSMpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSVtaircr {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_S_VTAIRCR is 1."]
    LockSVtaircrEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_S_VTAIRCR is 0."]
    LockSVtaircrEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockSVtaircr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSVtaircr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSVtaircr {
    #[inline(always)]
    fn from(val: u8) -> LockSVtaircr {
        LockSVtaircr::from_bits(val)
    }
}
impl From<LockSVtaircr> for u8 {
    #[inline(always)]
    fn from(val: LockSVtaircr) -> u8 {
        LockSVtaircr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSau {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_SAU is 1."]
    LockSauEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_SAU is 0."]
    LockSauEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockSau {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSau {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSau {
    #[inline(always)]
    fn from(val: u8) -> LockSau {
        LockSau::from_bits(val)
    }
}
impl From<LockSau> for u8 {
    #[inline(always)]
    fn from(val: LockSau) -> u8 {
        LockSau::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpFlexcomm {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl LpFlexcomm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpFlexcomm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpFlexcomm {
    #[inline(always)]
    fn from(val: u8) -> LpFlexcomm {
        LpFlexcomm::from_bits(val)
    }
}
impl From<LpFlexcomm> for u8 {
    #[inline(always)]
    fn from(val: LpFlexcomm) -> u8 {
        LpFlexcomm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpcac {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Lpcac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpcac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpcac {
    #[inline(always)]
    fn from(val: u8) -> Lpcac {
        Lpcac::from_bits(val)
    }
}
impl From<Lpcac> for u8 {
    #[inline(always)]
    fn from(val: Lpcac) -> u8 {
        Lpcac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lptmr0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Lptmr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lptmr0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lptmr0 {
    #[inline(always)]
    fn from(val: u8) -> Lptmr0 {
        Lptmr0::from_bits(val)
    }
}
impl From<Lptmr0> for u8 {
    #[inline(always)]
    fn from(val: Lptmr0) -> u8 {
        Lptmr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lptmr1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Lptmr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lptmr1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lptmr1 {
    #[inline(always)]
    fn from(val: u8) -> Lptmr1 {
        Lptmr1::from_bits(val)
    }
}
impl From<Lptmr1> for u8 {
    #[inline(always)]
    fn from(val: Lptmr1) -> u8 {
        Lptmr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mailbox {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Mailbox {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mailbox {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mailbox {
    #[inline(always)]
    fn from(val: u8) -> Mailbox {
        Mailbox::from_bits(val)
    }
}
impl From<Mailbox> for u8 {
    #[inline(always)]
    fn from(val: Mailbox) -> u8 {
        Mailbox::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegCoolfluxi {
    #[doc = "Secure and privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Secure and non-privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Non-secure and privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Non-secure and non-privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecAntiPolRegCoolfluxi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegCoolfluxi {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegCoolfluxi {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegCoolfluxi {
        MasterSecAntiPolRegCoolfluxi::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegCoolfluxi> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegCoolfluxi) -> u8 {
        MasterSecAntiPolRegCoolfluxi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegCpu1 {
    #[doc = "Secure and privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Secure and non-privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Non-secure and privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Non-secure and non-privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecAntiPolRegCpu1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegCpu1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegCpu1 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegCpu1 {
        MasterSecAntiPolRegCpu1::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegCpu1> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegCpu1) -> u8 {
        MasterSecAntiPolRegCpu1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegEDma0 {
    #[doc = "Secure and privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Secure and non-privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Non-secure and privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Non-secure and non-privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecAntiPolRegEDma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegEDma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegEDma0 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegEDma0 {
        MasterSecAntiPolRegEDma0::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegEDma0> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegEDma0) -> u8 {
        MasterSecAntiPolRegEDma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegEDma1 {
    #[doc = "Secure and privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Secure and non-privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Non-secure and privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Non-secure and non-privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecAntiPolRegEDma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegEDma1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegEDma1 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegEDma1 {
        MasterSecAntiPolRegEDma1::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegEDma1> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegEDma1) -> u8 {
        MasterSecAntiPolRegEDma1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegEthernet {
    #[doc = "Secure and privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Secure and non-privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Non-secure and privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Non-secure and non-privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecAntiPolRegEthernet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegEthernet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegEthernet {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegEthernet {
        MasterSecAntiPolRegEthernet::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegEthernet> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegEthernet) -> u8 {
        MasterSecAntiPolRegEthernet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegNpuo {
    #[doc = "Secure and privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Secure and non-privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Non-secure and privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Non-secure and non-privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecAntiPolRegNpuo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegNpuo {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegNpuo {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegNpuo {
        MasterSecAntiPolRegNpuo::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegNpuo> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegNpuo) -> u8 {
        MasterSecAntiPolRegNpuo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegPkc {
    #[doc = "Secure and privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Secure and non-privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Non-secure and privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Non-secure and non-privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecAntiPolRegPkc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegPkc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegPkc {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegPkc {
        MasterSecAntiPolRegPkc::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegPkc> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegPkc) -> u8 {
        MasterSecAntiPolRegPkc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegPq {
    #[doc = "Secure and privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Secure and non-privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Non-secure and privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Non-secure and non-privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecAntiPolRegPq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegPq {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegPq {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegPq {
        MasterSecAntiPolRegPq::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegPq> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegPq) -> u8 {
        MasterSecAntiPolRegPq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegSmartdma {
    #[doc = "Secure and privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Secure and non-privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Non-secure and privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Non-secure and non-privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecAntiPolRegSmartdma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegSmartdma {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegSmartdma {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegSmartdma {
        MasterSecAntiPolRegSmartdma::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegSmartdma> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegSmartdma) -> u8 {
        MasterSecAntiPolRegSmartdma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegUsbFs {
    #[doc = "Secure and privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Secure and non-privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Non-secure and privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Non-secure and non-privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecAntiPolRegUsbFs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegUsbFs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegUsbFs {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegUsbFs {
        MasterSecAntiPolRegUsbFs::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegUsbFs> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegUsbFs) -> u8 {
        MasterSecAntiPolRegUsbFs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegUsbHs {
    #[doc = "Secure and privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Secure and non-privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Non-secure and privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Non-secure and non-privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecAntiPolRegUsbHs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegUsbHs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegUsbHs {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegUsbHs {
        MasterSecAntiPolRegUsbHs::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegUsbHs> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegUsbHs) -> u8 {
        MasterSecAntiPolRegUsbHs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegUsdhc {
    #[doc = "Secure and privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Secure and non-privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Non-secure and privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Non-secure and non-privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecAntiPolRegUsdhc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegUsdhc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegUsdhc {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegUsdhc {
        MasterSecAntiPolRegUsdhc::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegUsdhc> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegUsdhc) -> u8 {
        MasterSecAntiPolRegUsdhc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelAntipolLock {
    _RESERVED_0 = 0x0,
    #[doc = "MASTER_SEC_LEVEL_LOCK cannot be written."]
    Enable = 0x01,
    #[doc = "MASTER_SEC_LEVEL_LOCK can be written."]
    Disable = 0x02,
    _RESERVED_3 = 0x03,
}
impl MasterSecLevelAntipolLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelAntipolLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelAntipolLock {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelAntipolLock {
        MasterSecLevelAntipolLock::from_bits(val)
    }
}
impl From<MasterSecLevelAntipolLock> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelAntipolLock) -> u8 {
        MasterSecLevelAntipolLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelCoolfluxi {
    #[doc = "Non-secure and non-privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Secure and privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecLevelCoolfluxi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelCoolfluxi {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelCoolfluxi {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelCoolfluxi {
        MasterSecLevelCoolfluxi::from_bits(val)
    }
}
impl From<MasterSecLevelCoolfluxi> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelCoolfluxi) -> u8 {
        MasterSecLevelCoolfluxi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelCpu1 {
    #[doc = "Non-secure and non-privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Secure and privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecLevelCpu1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelCpu1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelCpu1 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelCpu1 {
        MasterSecLevelCpu1::from_bits(val)
    }
}
impl From<MasterSecLevelCpu1> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelCpu1) -> u8 {
        MasterSecLevelCpu1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelEDma0 {
    #[doc = "Non-secure and non-privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Secure and privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecLevelEDma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelEDma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelEDma0 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelEDma0 {
        MasterSecLevelEDma0::from_bits(val)
    }
}
impl From<MasterSecLevelEDma0> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelEDma0) -> u8 {
        MasterSecLevelEDma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelEDma1 {
    #[doc = "Non-secure and non-privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Secure and privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecLevelEDma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelEDma1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelEDma1 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelEDma1 {
        MasterSecLevelEDma1::from_bits(val)
    }
}
impl From<MasterSecLevelEDma1> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelEDma1) -> u8 {
        MasterSecLevelEDma1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelEthernet {
    #[doc = "Non-secure and non-privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Secure and privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecLevelEthernet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelEthernet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelEthernet {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelEthernet {
        MasterSecLevelEthernet::from_bits(val)
    }
}
impl From<MasterSecLevelEthernet> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelEthernet) -> u8 {
        MasterSecLevelEthernet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelLock {
    _RESERVED_0 = 0x0,
    #[doc = "MASTER_SEC_LEVEL_LOCK cannot be written."]
    Enable = 0x01,
    #[doc = "MASTER_SEC_LEVEL_LOCK can be written."]
    Disable = 0x02,
    _RESERVED_3 = 0x03,
}
impl MasterSecLevelLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelLock {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelLock {
        MasterSecLevelLock::from_bits(val)
    }
}
impl From<MasterSecLevelLock> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelLock) -> u8 {
        MasterSecLevelLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelNpuo {
    #[doc = "Non-secure and non-privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Secure and privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecLevelNpuo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelNpuo {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelNpuo {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelNpuo {
        MasterSecLevelNpuo::from_bits(val)
    }
}
impl From<MasterSecLevelNpuo> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelNpuo) -> u8 {
        MasterSecLevelNpuo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelPkc {
    #[doc = "Non-secure and non-privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Secure and privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecLevelPkc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelPkc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelPkc {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelPkc {
        MasterSecLevelPkc::from_bits(val)
    }
}
impl From<MasterSecLevelPkc> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelPkc) -> u8 {
        MasterSecLevelPkc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelPq {
    #[doc = "Non-secure and non-privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Secure and privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecLevelPq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelPq {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelPq {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelPq {
        MasterSecLevelPq::from_bits(val)
    }
}
impl From<MasterSecLevelPq> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelPq) -> u8 {
        MasterSecLevelPq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelSmartdma {
    #[doc = "Non-secure and non-privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Secure and privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecLevelSmartdma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelSmartdma {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelSmartdma {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelSmartdma {
        MasterSecLevelSmartdma::from_bits(val)
    }
}
impl From<MasterSecLevelSmartdma> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelSmartdma) -> u8 {
        MasterSecLevelSmartdma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelUsbFs {
    #[doc = "Non-secure and non-privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Secure and privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecLevelUsbFs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelUsbFs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelUsbFs {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelUsbFs {
        MasterSecLevelUsbFs::from_bits(val)
    }
}
impl From<MasterSecLevelUsbFs> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelUsbFs) -> u8 {
        MasterSecLevelUsbFs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelUsbHs {
    #[doc = "Non-secure and non-privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Secure and privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecLevelUsbHs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelUsbHs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelUsbHs {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelUsbHs {
        MasterSecLevelUsbHs::from_bits(val)
    }
}
impl From<MasterSecLevelUsbHs> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelUsbHs) -> u8 {
        MasterSecLevelUsbHs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelUsdhc {
    #[doc = "Non-secure and non-privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Secure and privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSecLevelUsdhc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelUsdhc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelUsdhc {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelUsdhc {
        MasterSecLevelUsdhc::from_bits(val)
    }
}
impl From<MasterSecLevelUsdhc> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelUsdhc) -> u8 {
        MasterSecLevelUsdhc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Mbc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc {
    #[inline(always)]
    fn from(val: u8) -> Mbc {
        Mbc::from_bits(val)
    }
}
impl From<Mbc> for u8 {
    #[inline(always)]
    fn from(val: Mbc) -> u8 {
        Mbc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Micd {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Micd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Micd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Micd {
    #[inline(always)]
    fn from(val: u8) -> Micd {
        Micd::from_bits(val)
    }
}
impl From<Micd> for u8 {
    #[inline(always)]
    fn from(val: Micd) -> u8 {
        Micd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegDisableStrictMode {
    _RESERVED_0 = 0x0,
    #[doc = "Master can access memories and peripherals at the same level or below that level."]
    Ahbtm = 0x01,
    #[doc = "Master can access memories and peripherals at same level only."]
    Ahbsm1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegDisableStrictMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegDisableStrictMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegDisableStrictMode {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegDisableStrictMode {
        MiscCtrlDpRegDisableStrictMode::from_bits(val)
    }
}
impl From<MiscCtrlDpRegDisableStrictMode> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegDisableStrictMode) -> u8 {
        MiscCtrlDpRegDisableStrictMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegDisableViolationAbort {
    _RESERVED_0 = 0x0,
    #[doc = "The violation detected by the secure checker will not cause an abort, but a secure_violation_irq (interrupt request) will still be asserted and serviced by ISR."]
    NoAbort = 0x01,
    #[doc = "The violation detected by the secure checker will cause an abort."]
    Abort = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegDisableViolationAbort {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegDisableViolationAbort {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegDisableViolationAbort {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegDisableViolationAbort {
        MiscCtrlDpRegDisableViolationAbort::from_bits(val)
    }
}
impl From<MiscCtrlDpRegDisableViolationAbort> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegDisableViolationAbort) -> u8 {
        MiscCtrlDpRegDisableViolationAbort::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegEnableNsPrivCheck {
    _RESERVED_0 = 0x0,
    #[doc = "Enables the privilege checking of non-secure mode access."]
    Enabled = 0x01,
    #[doc = "Disables the privilege checking of non-secure mode access."]
    Disabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegEnableNsPrivCheck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegEnableNsPrivCheck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegEnableNsPrivCheck {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegEnableNsPrivCheck {
        MiscCtrlDpRegEnableNsPrivCheck::from_bits(val)
    }
}
impl From<MiscCtrlDpRegEnableNsPrivCheck> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegEnableNsPrivCheck) -> u8 {
        MiscCtrlDpRegEnableNsPrivCheck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegEnableSPrivCheck {
    _RESERVED_0 = 0x0,
    #[doc = "Enables the privilege checking of secure mode access."]
    Enabled = 0x01,
    #[doc = "Disables the privilege checking of secure mode access."]
    Disabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegEnableSPrivCheck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegEnableSPrivCheck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegEnableSPrivCheck {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegEnableSPrivCheck {
        MiscCtrlDpRegEnableSPrivCheck::from_bits(val)
    }
}
impl From<MiscCtrlDpRegEnableSPrivCheck> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegEnableSPrivCheck) -> u8 {
        MiscCtrlDpRegEnableSPrivCheck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegEnableSecureChecking {
    _RESERVED_0 = 0x0,
    #[doc = "Enables secure checking. Violation can be detected when the security level of a transaction does not meet the security rule of the slave or memory to be accessed."]
    Enabled = 0x01,
    #[doc = "Disables secure checking. Even if the security level of a transaction does not conform to the security rule of the slave or memory, it will not be detected as a violation."]
    Disabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegEnableSecureChecking {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegEnableSecureChecking {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegEnableSecureChecking {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegEnableSecureChecking {
        MiscCtrlDpRegEnableSecureChecking::from_bits(val)
    }
}
impl From<MiscCtrlDpRegEnableSecureChecking> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegEnableSecureChecking) -> u8 {
        MiscCtrlDpRegEnableSecureChecking::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegIdauAllNs {
    _RESERVED_0 = 0x0,
    #[doc = "IDAU is disabled, which means that all memories are attributed as non-secure memory."]
    Disabled = 0x01,
    #[doc = "IDAU is enabled (restrictive mode)."]
    Enabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegIdauAllNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegIdauAllNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegIdauAllNs {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegIdauAllNs {
        MiscCtrlDpRegIdauAllNs::from_bits(val)
    }
}
impl From<MiscCtrlDpRegIdauAllNs> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegIdauAllNs) -> u8 {
        MiscCtrlDpRegIdauAllNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegWriteLock {
    _RESERVED_0 = 0x0,
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are not allowed."]
    Locked = 0x01,
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are allowed."]
    NotLocked = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegWriteLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegWriteLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegWriteLock {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegWriteLock {
        MiscCtrlDpRegWriteLock::from_bits(val)
    }
}
impl From<MiscCtrlDpRegWriteLock> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegWriteLock) -> u8 {
        MiscCtrlDpRegWriteLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegDisableStrictMode {
    _RESERVED_0 = 0x0,
    #[doc = "Master strict mode is on and can access memories and peripherals at the same level or below that level."]
    Ahbtm = 0x01,
    #[doc = "Master strict mode is disabled and can access memories and peripherals at same level only."]
    Ahbsm1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegDisableStrictMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegDisableStrictMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegDisableStrictMode {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegDisableStrictMode {
        MiscCtrlRegDisableStrictMode::from_bits(val)
    }
}
impl From<MiscCtrlRegDisableStrictMode> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegDisableStrictMode) -> u8 {
        MiscCtrlRegDisableStrictMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegDisableViolationAbort {
    _RESERVED_0 = 0x0,
    #[doc = "The violation detected by the secure checker will not cause an abort, but a secure_violation_irq (interrupt request) will still be asserted and serviced by ISR."]
    NoAbort = 0x01,
    #[doc = "The violation detected by the secure checker will cause an abort."]
    Abort = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegDisableViolationAbort {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegDisableViolationAbort {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegDisableViolationAbort {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegDisableViolationAbort {
        MiscCtrlRegDisableViolationAbort::from_bits(val)
    }
}
impl From<MiscCtrlRegDisableViolationAbort> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegDisableViolationAbort) -> u8 {
        MiscCtrlRegDisableViolationAbort::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegEnableNsPrivCheck {
    _RESERVED_0 = 0x0,
    #[doc = "Enables privilege checking of non-secure mode access."]
    Enabled = 0x01,
    #[doc = "Disables privilege checking of non-secure mode access is disabled."]
    Disabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegEnableNsPrivCheck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegEnableNsPrivCheck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegEnableNsPrivCheck {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegEnableNsPrivCheck {
        MiscCtrlRegEnableNsPrivCheck::from_bits(val)
    }
}
impl From<MiscCtrlRegEnableNsPrivCheck> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegEnableNsPrivCheck) -> u8 {
        MiscCtrlRegEnableNsPrivCheck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegEnableSPrivCheck {
    _RESERVED_0 = 0x0,
    #[doc = "Enables privilege checking of secure mode access."]
    Enabled = 0x01,
    #[doc = "Disables privilege checking of secure mode access."]
    Disabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegEnableSPrivCheck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegEnableSPrivCheck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegEnableSPrivCheck {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegEnableSPrivCheck {
        MiscCtrlRegEnableSPrivCheck::from_bits(val)
    }
}
impl From<MiscCtrlRegEnableSPrivCheck> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegEnableSPrivCheck) -> u8 {
        MiscCtrlRegEnableSPrivCheck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegEnableSecureChecking {
    _RESERVED_0 = 0x0,
    #[doc = "Enables secure checking. Violation can be detected when the security level of a transaction does not meet the security rule of the slave or memory to be accessed."]
    Enabled = 0x01,
    #[doc = "Disables secure checking. Even if the security level of a transaction does not conform to the security rule of the slave or memory, it will not be detected as a violation."]
    Disabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegEnableSecureChecking {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegEnableSecureChecking {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegEnableSecureChecking {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegEnableSecureChecking {
        MiscCtrlRegEnableSecureChecking::from_bits(val)
    }
}
impl From<MiscCtrlRegEnableSecureChecking> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegEnableSecureChecking) -> u8 {
        MiscCtrlRegEnableSecureChecking::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegIdauAllNs {
    _RESERVED_0 = 0x0,
    #[doc = "IDAU is disabled, which means that all memories are attributed as non-secure memory."]
    Disabled = 0x01,
    #[doc = "IDAU is enabled (restrictive mode)."]
    Enabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegIdauAllNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegIdauAllNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegIdauAllNs {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegIdauAllNs {
        MiscCtrlRegIdauAllNs::from_bits(val)
    }
}
impl From<MiscCtrlRegIdauAllNs> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegIdauAllNs) -> u8 {
        MiscCtrlRegIdauAllNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegWriteLock {
    _RESERVED_0 = 0x0,
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are not allowed."]
    Locked = 0x01,
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are allowed."]
    NotLocked = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegWriteLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegWriteLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegWriteLock {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegWriteLock {
        MiscCtrlRegWriteLock::from_bits(val)
    }
}
impl From<MiscCtrlRegWriteLock> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegWriteLock) -> u8 {
        MiscCtrlRegWriteLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mrt0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Mrt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrt0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrt0 {
    #[inline(always)]
    fn from(val: u8) -> Mrt0 {
        Mrt0::from_bits(val)
    }
}
impl From<Mrt0> for u8 {
    #[inline(always)]
    fn from(val: Mrt0) -> u8 {
        Mrt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mtr0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Mtr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mtr0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mtr0 {
    #[inline(always)]
    fn from(val: u8) -> Mtr0 {
        Mtr0::from_bits(val)
    }
}
impl From<Mtr0> for u8 {
    #[inline(always)]
    fn from(val: Mtr0) -> u8 {
        Mtr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npu {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Npu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npu {
    #[inline(always)]
    fn from(val: u8) -> Npu {
        Npu::from_bits(val)
    }
}
impl From<Npu> for u8 {
    #[inline(always)]
    fn from(val: Npu) -> u8 {
        Npu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npx {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Npx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npx {
    #[inline(always)]
    fn from(val: u8) -> Npx {
        Npx::from_bits(val)
    }
}
impl From<Npx> for u8 {
    #[inline(always)]
    fn from(val: Npx) -> u8 {
        Npx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opamp0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Opamp0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opamp0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opamp0 {
    #[inline(always)]
    fn from(val: u8) -> Opamp0 {
        Opamp0::from_bits(val)
    }
}
impl From<Opamp0> for u8 {
    #[inline(always)]
    fn from(val: Opamp0) -> u8 {
        Opamp0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opamp1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Opamp1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opamp1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opamp1 {
    #[inline(always)]
    fn from(val: u8) -> Opamp1 {
        Opamp1::from_bits(val)
    }
}
impl From<Opamp1> for u8 {
    #[inline(always)]
    fn from(val: Opamp1) -> u8 {
        Opamp1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opamp2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Opamp2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opamp2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opamp2 {
    #[inline(always)]
    fn from(val: u8) -> Opamp2 {
        Opamp2::from_bits(val)
    }
}
impl From<Opamp2> for u8 {
    #[inline(always)]
    fn from(val: Opamp2) -> u8 {
        Opamp2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ostimer0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Ostimer0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ostimer0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ostimer0 {
    #[inline(always)]
    fn from(val: u8) -> Ostimer0 {
        Ostimer0::from_bits(val)
    }
}
impl From<Ostimer0> for u8 {
    #[inline(always)]
    fn from(val: Ostimer0) -> u8 {
        Ostimer0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Otpc {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Otpc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Otpc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Otpc {
    #[inline(always)]
    fn from(val: u8) -> Otpc {
        Otpc::from_bits(val)
    }
}
impl From<Otpc> for u8 {
    #[inline(always)]
    fn from(val: Otpc) -> u8 {
        Otpc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pint0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Pint0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pint0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pint0 {
    #[inline(always)]
    fn from(val: u8) -> Pint0 {
        Pint0::from_bits(val)
    }
}
impl From<Pint0> for u8 {
    #[inline(always)]
    fn from(val: Pint0) -> u8 {
        Pint0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio0PinSecMask {
    #[doc = "Masked."]
    Masked = 0x0,
    #[doc = "Not masked."]
    NotMasked = 0x01,
}
impl Pio0PinSecMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio0PinSecMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio0PinSecMask {
    #[inline(always)]
    fn from(val: u8) -> Pio0PinSecMask {
        Pio0PinSecMask::from_bits(val)
    }
}
impl From<Pio0PinSecMask> for u8 {
    #[inline(always)]
    fn from(val: Pio0PinSecMask) -> u8 {
        Pio0PinSecMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkcRam {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl PkcRam {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkcRam {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkcRam {
    #[inline(always)]
    fn from(val: u8) -> PkcRam {
        PkcRam::from_bits(val)
    }
}
impl From<PkcRam> for u8 {
    #[inline(always)]
    fn from(val: PkcRam) -> u8 {
        PkcRam::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Plu {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Plu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Plu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Plu {
    #[inline(always)]
    fn from(val: u8) -> Plu {
        Plu::from_bits(val)
    }
}
impl From<Plu> for u8 {
    #[inline(always)]
    fn from(val: Plu) -> u8 {
        Plu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Port {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port {
    #[inline(always)]
    fn from(val: u8) -> Port {
        Port::from_bits(val)
    }
}
impl From<Port> for u8 {
    #[inline(always)]
    fn from(val: Port) -> u8 {
        Port::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Port0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port0 {
    #[inline(always)]
    fn from(val: u8) -> Port0 {
        Port0::from_bits(val)
    }
}
impl From<Port0> for u8 {
    #[inline(always)]
    fn from(val: Port0) -> u8 {
        Port0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Port1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port1 {
    #[inline(always)]
    fn from(val: u8) -> Port1 {
        Port1::from_bits(val)
    }
}
impl From<Port1> for u8 {
    #[inline(always)]
    fn from(val: Port1) -> u8 {
        Port1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Port5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port5 {
    #[inline(always)]
    fn from(val: u8) -> Port5 {
        Port5::from_bits(val)
    }
}
impl From<Port5> for u8 {
    #[inline(always)]
    fn from(val: Port5) -> u8 {
        Port5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Powerquad {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Powerquad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Powerquad {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Powerquad {
    #[inline(always)]
    fn from(val: u8) -> Powerquad {
        Powerquad::from_bits(val)
    }
}
impl From<Powerquad> for u8 {
    #[inline(always)]
    fn from(val: Powerquad) -> u8 {
        Powerquad::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PufAlias {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl PufAlias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PufAlias {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PufAlias {
    #[inline(always)]
    fn from(val: u8) -> PufAlias {
        PufAlias::from_bits(val)
    }
}
impl From<PufAlias> for u8 {
    #[inline(always)]
    fn from(val: PufAlias) -> u8 {
        PufAlias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwm {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Pwm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwm {
    #[inline(always)]
    fn from(val: u8) -> Pwm {
        Pwm::from_bits(val)
    }
}
impl From<Pwm> for u8 {
    #[inline(always)]
    fn from(val: Pwm) -> u8 {
        Pwm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwm1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Pwm1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwm1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwm1 {
    #[inline(always)]
    fn from(val: u8) -> Pwm1 {
        Pwm1::from_bits(val)
    }
}
impl From<Pwm1> for u8 {
    #[inline(always)]
    fn from(val: Pwm1) -> u8 {
        Pwm1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamaMemRuleRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule {
        RamaMemRuleRule::from_bits(val)
    }
}
impl From<RamaMemRuleRule> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule) -> u8 {
        RamaMemRuleRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RambMemRuleRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule {
        RambMemRuleRule::from_bits(val)
    }
}
impl From<RambMemRuleRule> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule) -> u8 {
        RambMemRuleRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamcMemRuleRule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamcMemRuleRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamcMemRuleRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamcMemRuleRule {
    #[inline(always)]
    fn from(val: u8) -> RamcMemRuleRule {
        RamcMemRuleRule::from_bits(val)
    }
}
impl From<RamcMemRuleRule> for u8 {
    #[inline(always)]
    fn from(val: RamcMemRuleRule) -> u8 {
        RamcMemRuleRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamdMemRuleRule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamdMemRuleRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamdMemRuleRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamdMemRuleRule {
    #[inline(always)]
    fn from(val: u8) -> RamdMemRuleRule {
        RamdMemRuleRule::from_bits(val)
    }
}
impl From<RamdMemRuleRule> for u8 {
    #[inline(always)]
    fn from(val: RamdMemRuleRule) -> u8 {
        RamdMemRuleRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RameMemRuleRule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RameMemRuleRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RameMemRuleRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RameMemRuleRule {
    #[inline(always)]
    fn from(val: u8) -> RameMemRuleRule {
        RameMemRuleRule::from_bits(val)
    }
}
impl From<RameMemRuleRule> for u8 {
    #[inline(always)]
    fn from(val: RameMemRuleRule) -> u8 {
        RameMemRuleRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamfMemRuleRule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamfMemRuleRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamfMemRuleRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamfMemRuleRule {
    #[inline(always)]
    fn from(val: u8) -> RamfMemRuleRule {
        RamfMemRuleRule::from_bits(val)
    }
}
impl From<RamfMemRuleRule> for u8 {
    #[inline(always)]
    fn from(val: RamfMemRuleRule) -> u8 {
        RamfMemRuleRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamgMemRuleRule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamgMemRuleRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamgMemRuleRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamgMemRuleRule {
    #[inline(always)]
    fn from(val: u8) -> RamgMemRuleRule {
        RamgMemRuleRule::from_bits(val)
    }
}
impl From<RamgMemRuleRule> for u8 {
    #[inline(always)]
    fn from(val: RamgMemRuleRule) -> u8 {
        RamgMemRuleRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamhMemRuleRule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamhMemRuleRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamhMemRuleRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamhMemRuleRule {
    #[inline(always)]
    fn from(val: u8) -> RamhMemRuleRule {
        RamhMemRuleRule::from_bits(val)
    }
}
impl From<RamhMemRuleRule> for u8 {
    #[inline(always)]
    fn from(val: RamhMemRuleRule) -> u8 {
        RamhMemRuleRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RamxMemRuleRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule {
        RamxMemRuleRule::from_bits(val)
    }
}
impl From<RamxMemRuleRule> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule) -> u8 {
        RamxMemRuleRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl RomMemRuleRule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule {
        RomMemRuleRule::from_bits(val)
    }
}
impl From<RomMemRuleRule> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule) -> u8 {
        RomMemRuleRule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtc {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Rtc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtc {
    #[inline(always)]
    fn from(val: u8) -> Rtc {
        Rtc::from_bits(val)
    }
}
impl From<Rtc> for u8 {
    #[inline(always)]
    fn from(val: Rtc) -> u8 {
        Rtc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Sai0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai0 {
    #[inline(always)]
    fn from(val: u8) -> Sai0 {
        Sai0::from_bits(val)
    }
}
impl From<Sai0> for u8 {
    #[inline(always)]
    fn from(val: Sai0) -> u8 {
        Sai0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Sai1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1 {
    #[inline(always)]
    fn from(val: u8) -> Sai1 {
        Sai1::from_bits(val)
    }
}
impl From<Sai1> for u8 {
    #[inline(always)]
    fn from(val: Sai1) -> u8 {
        Sai1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scg0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Scg0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scg0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scg0 {
    #[inline(always)]
    fn from(val: u8) -> Scg0 {
        Scg0::from_bits(val)
    }
}
impl From<Scg0> for u8 {
    #[inline(always)]
    fn from(val: Scg0) -> u8 {
        Scg0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sct0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Sct0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sct0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sct0 {
    #[inline(always)]
    fn from(val: u8) -> Sct0 {
        Sct0::from_bits(val)
    }
}
impl From<Sct0> for u8 {
    #[inline(always)]
    fn from(val: Sct0) -> u8 {
        Sct0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecCpu1IntMaskLock {
    _RESERVED_0 = 0x0,
    #[doc = "Cannot be written."]
    CannotBeWritten = 0x01,
    #[doc = "Can be written."]
    CanBeWritten = 0x02,
    _RESERVED_3 = 0x03,
}
impl SecCpu1IntMaskLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecCpu1IntMaskLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecCpu1IntMaskLock {
    #[inline(always)]
    fn from(val: u8) -> SecCpu1IntMaskLock {
        SecCpu1IntMaskLock::from_bits(val)
    }
}
impl From<SecCpu1IntMaskLock> for u8 {
    #[inline(always)]
    fn from(val: SecCpu1IntMaskLock) -> u8 {
        SecCpu1IntMaskLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecGpioMask0Lock {
    _RESERVED_0 = 0x0,
    #[doc = "SEC_GPIO_MASK0 cannot be written."]
    CannotBeWritten = 0x01,
    #[doc = "SEC_GPIO_MASK0 can be written."]
    CanBeWritten = 0x02,
    _RESERVED_3 = 0x03,
}
impl SecGpioMask0Lock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecGpioMask0Lock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecGpioMask0Lock {
    #[inline(always)]
    fn from(val: u8) -> SecGpioMask0Lock {
        SecGpioMask0Lock::from_bits(val)
    }
}
impl From<SecGpioMask0Lock> for u8 {
    #[inline(always)]
    fn from(val: SecGpioMask0Lock) -> u8 {
        SecGpioMask0Lock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecGpioMask1Lock {
    _RESERVED_0 = 0x0,
    #[doc = "SEC_GPIO_MASK1 cannot be written."]
    CannotBeWritten = 0x01,
    #[doc = "SEC_GPIO_MASK1 can be written."]
    CanBeWritten = 0x02,
    _RESERVED_3 = 0x03,
}
impl SecGpioMask1Lock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecGpioMask1Lock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecGpioMask1Lock {
    #[inline(always)]
    fn from(val: u8) -> SecGpioMask1Lock {
        SecGpioMask1Lock::from_bits(val)
    }
}
impl From<SecGpioMask1Lock> for u8 {
    #[inline(always)]
    fn from(val: SecGpioMask1Lock) -> u8 {
        SecGpioMask1Lock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecVioInfoDataAccess {
    #[doc = "Code."]
    Code = 0x0,
    #[doc = "Data."]
    Data = 0x01,
}
impl SecVioInfoDataAccess {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecVioInfoDataAccess {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecVioInfoDataAccess {
    #[inline(always)]
    fn from(val: u8) -> SecVioInfoDataAccess {
        SecVioInfoDataAccess::from_bits(val)
    }
}
impl From<SecVioInfoDataAccess> for u8 {
    #[inline(always)]
    fn from(val: SecVioInfoDataAccess) -> u8 {
        SecVioInfoDataAccess::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecVioInfoMaster {
    #[doc = "M33 Code."]
    Cpu0Code = 0x0,
    #[doc = "M33 System."]
    Cpu0Sys = 0x01,
    #[doc = "CPU1 (Mirco-CM33) Code."]
    Cpu1Code = 0x02,
    #[doc = "SMARTDMA Instruction."]
    SdmaInstr = 0x03,
    #[doc = "CPU1 (Mirco-CM33) system."]
    Cpu1Sys = 0x04,
    #[doc = "SMARTDMA Data."]
    SdmaData = 0x05,
    #[doc = "eDMA0."]
    EDma0 = 0x06,
    #[doc = "eDMA1."]
    EDma1 = 0x07,
    #[doc = "PKC."]
    Pkc = 0x08,
    #[doc = "ELS S50."]
    Cssv2 = 0x09,
    #[doc = "PKC M0."]
    Pq = 0x0a,
    #[doc = "NPU Operands."]
    Npuo = 0x0b,
    #[doc = "DSP Instruction."]
    Dspi = 0x0c,
    #[doc = "DSPX."]
    Dspx = 0x0d,
    #[doc = "DSPY."]
    Dspy = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "NPU Data."]
    Npud = 0x10,
    #[doc = "USB FS."]
    UsbFs = 0x11,
    #[doc = "Ethernet."]
    Ethernet = 0x12,
    #[doc = "USB HS."]
    UsbHs = 0x13,
    #[doc = "uSDHC."]
    Usdhc = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl SecVioInfoMaster {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecVioInfoMaster {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecVioInfoMaster {
    #[inline(always)]
    fn from(val: u8) -> SecVioInfoMaster {
        SecVioInfoMaster::from_bits(val)
    }
}
impl From<SecVioInfoMaster> for u8 {
    #[inline(always)]
    fn from(val: SecVioInfoMaster) -> u8 {
        SecVioInfoMaster::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecVioInfoWrite {
    #[doc = "Read access."]
    Read = 0x0,
    #[doc = "Write access."]
    Write = 0x01,
}
impl SecVioInfoWrite {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecVioInfoWrite {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecVioInfoWrite {
    #[inline(always)]
    fn from(val: u8) -> SecVioInfoWrite {
        SecVioInfoWrite::from_bits(val)
    }
}
impl From<SecVioInfoWrite> for u8 {
    #[inline(always)]
    fn from(val: SecVioInfoWrite) -> u8 {
        SecVioInfoWrite::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sema42 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Sema42 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sema42 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sema42 {
    #[inline(always)]
    fn from(val: u8) -> Sema42 {
        Sema42::from_bits(val)
    }
}
impl From<Sema42> for u8 {
    #[inline(always)]
    fn from(val: Sema42) -> u8 {
        Sema42::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sfa {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Sfa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sfa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sfa {
    #[inline(always)]
    fn from(val: u8) -> Sfa {
        Sfa::from_bits(val)
    }
}
impl From<Sfa> for u8 {
    #[inline(always)]
    fn from(val: Sfa) -> u8 {
        Sfa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sinc0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Sinc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sinc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sinc0 {
    #[inline(always)]
    fn from(val: u8) -> Sinc0 {
        Sinc0::from_bits(val)
    }
}
impl From<Sinc0> for u8 {
    #[inline(always)]
    fn from(val: Sinc0) -> u8 {
        Sinc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Sm3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3 {
    #[inline(always)]
    fn from(val: u8) -> Sm3 {
        Sm3::from_bits(val)
    }
}
impl From<Sm3> for u8 {
    #[inline(always)]
    fn from(val: Sm3) -> u8 {
        Sm3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spc0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Spc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spc0 {
    #[inline(always)]
    fn from(val: u8) -> Spc0 {
        Spc0::from_bits(val)
    }
}
impl From<Spc0> for u8 {
    #[inline(always)]
    fn from(val: Spc0) -> u8 {
        Spc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Syscon {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Syscon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Syscon {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Syscon {
    #[inline(always)]
    fn from(val: u8) -> Syscon {
        Syscon::from_bits(val)
    }
}
impl From<Syscon> for u8 {
    #[inline(always)]
    fn from(val: Syscon) -> u8 {
        Syscon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trng {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Trng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trng {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trng {
    #[inline(always)]
    fn from(val: u8) -> Trng {
        Trng::from_bits(val)
    }
}
impl From<Trng> for u8 {
    #[inline(always)]
    fn from(val: Trng) -> u8 {
        Trng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tro0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Tro0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tro0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tro0 {
    #[inline(always)]
    fn from(val: u8) -> Tro0 {
        Tro0::from_bits(val)
    }
}
impl From<Tro0> for u8 {
    #[inline(always)]
    fn from(val: Tro0) -> u8 {
        Tro0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsi {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Tsi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsi {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsi {
    #[inline(always)]
    fn from(val: u8) -> Tsi {
        Tsi::from_bits(val)
    }
}
impl From<Tsi> for u8 {
    #[inline(always)]
    fn from(val: Tsi) -> u8 {
        Tsi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USdhc0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl USdhc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USdhc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USdhc0 {
    #[inline(always)]
    fn from(val: u8) -> USdhc0 {
        USdhc0::from_bits(val)
    }
}
impl From<USdhc0> for u8 {
    #[inline(always)]
    fn from(val: USdhc0) -> u8 {
        USdhc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbFsOtgRam {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl UsbFsOtgRam {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbFsOtgRam {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbFsOtgRam {
    #[inline(always)]
    fn from(val: u8) -> UsbFsOtgRam {
        UsbFsOtgRam::from_bits(val)
    }
}
impl From<UsbFsOtgRam> for u8 {
    #[inline(always)]
    fn from(val: UsbFsOtgRam) -> u8 {
        UsbFsOtgRam::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbdcd {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Usbdcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbdcd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbdcd {
    #[inline(always)]
    fn from(val: u8) -> Usbdcd {
        Usbdcd::from_bits(val)
    }
}
impl From<Usbdcd> for u8 {
    #[inline(always)]
    fn from(val: Usbdcd) -> u8 {
        Usbdcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbfs {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Usbfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbfs {
    #[inline(always)]
    fn from(val: u8) -> Usbfs {
        Usbfs::from_bits(val)
    }
}
impl From<Usbfs> for u8 {
    #[inline(always)]
    fn from(val: Usbfs) -> u8 {
        Usbfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbhs {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Usbhs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbhs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbhs {
    #[inline(always)]
    fn from(val: u8) -> Usbhs {
        Usbhs::from_bits(val)
    }
}
impl From<Usbhs> for u8 {
    #[inline(always)]
    fn from(val: Usbhs) -> u8 {
        Usbhs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbhsphy {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Usbhsphy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbhsphy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbhsphy {
    #[inline(always)]
    fn from(val: u8) -> Usbhsphy {
        Usbhsphy::from_bits(val)
    }
}
impl From<Usbhsphy> for u8 {
    #[inline(always)]
    fn from(val: Usbhsphy) -> u8 {
        Usbhsphy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Utcik0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Utcik0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Utcik0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Utcik0 {
    #[inline(always)]
    fn from(val: u8) -> Utcik0 {
        Utcik0::from_bits(val)
    }
}
impl From<Utcik0> for u8 {
    #[inline(always)]
    fn from(val: Utcik0) -> u8 {
        Utcik0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbat {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Vbat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbat {
    #[inline(always)]
    fn from(val: u8) -> Vbat {
        Vbat::from_bits(val)
    }
}
impl From<Vbat> for u8 {
    #[inline(always)]
    fn from(val: Vbat) -> u8 {
        Vbat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vref {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Vref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vref {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vref {
    #[inline(always)]
    fn from(val: u8) -> Vref {
        Vref::from_bits(val)
    }
}
impl From<Vref> for u8 {
    #[inline(always)]
    fn from(val: Vref) -> u8 {
        Vref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuu0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Wuu0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuu0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuu0 {
    #[inline(always)]
    fn from(val: u8) -> Wuu0 {
        Wuu0::from_bits(val)
    }
}
impl From<Wuu0> for u8 {
    #[inline(always)]
    fn from(val: Wuu0) -> u8 {
        Wuu0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wwdt0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Wwdt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wwdt0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wwdt0 {
    #[inline(always)]
    fn from(val: u8) -> Wwdt0 {
        Wwdt0::from_bits(val)
    }
}
impl From<Wwdt0> for u8 {
    #[inline(always)]
    fn from(val: Wwdt0) -> u8 {
        Wwdt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wwdt1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Wwdt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wwdt1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wwdt1 {
    #[inline(always)]
    fn from(val: u8) -> Wwdt1 {
        Wwdt1::from_bits(val)
    }
}
impl From<Wwdt1> for u8 {
    #[inline(always)]
    fn from(val: Wwdt1) -> u8 {
        Wwdt1::to_bits(val)
    }
}
