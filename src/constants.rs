#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::all)]
// list of all NIC registers and some structs
// copied and changed from the ixy C driver and DPDK

/*******************************************************************************

Copyright (c) 2001-2020, Intel Corporation
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

 1. Redistributions of source code must retain the above copyright notice,
    this list of conditions and the following disclaimer.

 2. Redistributions in binary form must reproduce the above copyright
    notice, this list of conditions and the following disclaimer in the
    documentation and/or other materials provided with the distribution.

 3. Neither the name of the Intel Corporation nor the names of its
    contributors may be used to endorse or promote products derived from
    this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.

***************************************************************************/

/* Vendor ID */
pub const IXGBE_INTEL_VENDOR_ID: u32                                   = 0x8086;

/* Device IDs */
pub const IXGBE_DEV_ID_82598: u32                                      = 0x10B6;
pub const IXGBE_DEV_ID_82598_BX: u32                                   = 0x1508;
pub const IXGBE_DEV_ID_82598AF_DUAL_PORT: u32                          = 0x10C6;
pub const IXGBE_DEV_ID_82598AF_SINGLE_PORT: u32                        = 0x10C7;
pub const IXGBE_DEV_ID_82598AT: u32                                    = 0x10C8;
pub const IXGBE_DEV_ID_82598AT2: u32                                   = 0x150B;
pub const IXGBE_DEV_ID_82598EB_SFP_LOM: u32                            = 0x10DB;
pub const IXGBE_DEV_ID_82598EB_CX4: u32                                = 0x10DD;
pub const IXGBE_DEV_ID_82598_CX4_DUAL_PORT: u32                        = 0x10EC;
pub const IXGBE_DEV_ID_82598_DA_DUAL_PORT: u32                         = 0x10F1;
pub const IXGBE_DEV_ID_82598_SR_DUAL_PORT_EM: u32                      = 0x10E1;
pub const IXGBE_DEV_ID_82598EB_XF_LR: u32                              = 0x10F4;
pub const IXGBE_DEV_ID_82599_KX4: u32                                  = 0x10F7;
pub const IXGBE_DEV_ID_82599_KX4_MEZZ: u32                             = 0x1514;
pub const IXGBE_DEV_ID_82599_KR: u32                                   = 0x1517;
pub const IXGBE_DEV_ID_82599_COMBO_BACKPLANE: u32                      = 0x10F8;
pub const IXGBE_SUBDEV_ID_82599_KX4_KR_MEZZ: u32                       = 0x000C;
pub const IXGBE_DEV_ID_82599_CX4: u32                                  = 0x10F9;
pub const IXGBE_DEV_ID_82599_SFP: u32                                  = 0x10FB;
pub const IXGBE_SUBDEV_ID_82599_SFP: u32                               = 0x11A9;
pub const IXGBE_SUBDEV_ID_82599_SFP_WOL0: u32                          = 0x1071;
pub const IXGBE_SUBDEV_ID_82599_RNDC: u32                              = 0x1F72;
pub const IXGBE_SUBDEV_ID_82599_560FLR: u32                            = 0x17D0;
pub const IXGBE_SUBDEV_ID_82599_ECNA_DP: u32                           = 0x0470;
pub const IXGBE_SUBDEV_ID_82599_SP_560FLR: u32                         = 0x211B;
pub const IXGBE_SUBDEV_ID_82599_LOM_SNAP6: u32                         = 0x2159;
pub const IXGBE_SUBDEV_ID_82599_SFP_1OCP: u32                          = 0x000D;
pub const IXGBE_SUBDEV_ID_82599_SFP_2OCP: u32                          = 0x0008;
pub const IXGBE_SUBDEV_ID_82599_SFP_LOM_OEM1: u32                      = 0x8976;
pub const IXGBE_SUBDEV_ID_82599_SFP_LOM_OEM2: u32                      = 0x06EE;
pub const IXGBE_DEV_ID_82599_BACKPLANE_FCOE: u32                       = 0x152A;
pub const IXGBE_DEV_ID_82599_SFP_FCOE: u32                             = 0x1529;
pub const IXGBE_DEV_ID_82599_SFP_EM: u32                               = 0x1507;
pub const IXGBE_DEV_ID_82599_SFP_SF2: u32                              = 0x154D;
pub const IXGBE_DEV_ID_82599_SFP_SF_QP: u32                            = 0x154A;
pub const IXGBE_DEV_ID_82599_QSFP_SF_QP: u32                           = 0x1558;
pub const IXGBE_DEV_ID_82599EN_SFP: u32                                = 0x1557;
pub const IXGBE_SUBDEV_ID_82599EN_SFP_OCP1: u32                        = 0x0001;
pub const IXGBE_DEV_ID_82599_XAUI_LOM: u32                             = 0x10FC;
pub const IXGBE_DEV_ID_82599_T3_LOM: u32                               = 0x151C;
pub const IXGBE_DEV_ID_82599_VF: u32                                   = 0x10ED;
pub const IXGBE_DEV_ID_82599_VF_HV: u32                                = 0x152E;
pub const IXGBE_DEV_ID_82599_LS: u32                                   = 0x154F;
pub const IXGBE_DEV_ID_X540T: u32                                      = 0x1528;
pub const IXGBE_DEV_ID_X540_VF: u32                                    = 0x1515;
pub const IXGBE_DEV_ID_X540_VF_HV: u32                                 = 0x1530;
pub const IXGBE_DEV_ID_X540T1: u32                                     = 0x1560;
pub const IXGBE_DEV_ID_X550T: u32                                      = 0x1563;
pub const IXGBE_DEV_ID_X550T1: u32                                     = 0x15D1;
/* Placeholder value, pending official value. */
pub const IXGBE_DEV_ID_X550EM_A_KR: u32                                = 0x15C2;
pub const IXGBE_DEV_ID_X550EM_A_KR_L: u32                              = 0x15C3;
pub const IXGBE_DEV_ID_X550EM_A_SFP_N: u32                             = 0x15C4;
pub const IXGBE_DEV_ID_X550EM_A_SGMII: u32                             = 0x15C6;
pub const IXGBE_DEV_ID_X550EM_A_SGMII_L: u32                           = 0x15C7;
pub const IXGBE_DEV_ID_X550EM_A_10G_T: u32                             = 0x15C8;
pub const IXGBE_DEV_ID_X550EM_A_QSFP: u32                              = 0x15CA;
pub const IXGBE_DEV_ID_X550EM_A_QSFP_N: u32                            = 0x15CC;
pub const IXGBE_DEV_ID_X550EM_A_SFP: u32                               = 0x15CE;
pub const IXGBE_DEV_ID_X550EM_A_1G_T: u32                              = 0x15E4;
pub const IXGBE_DEV_ID_X550EM_A_1G_T_L: u32                            = 0x15E5;
pub const IXGBE_DEV_ID_X550EM_X_KX4: u32                               = 0x15AA;
pub const IXGBE_DEV_ID_X550EM_X_KR: u32                                = 0x15AB;
pub const IXGBE_DEV_ID_X550EM_X_SFP: u32                               = 0x15AC;
pub const IXGBE_DEV_ID_X550EM_X_10G_T: u32                             = 0x15AD;
pub const IXGBE_DEV_ID_X550EM_X_1G_T: u32                              = 0x15AE;
pub const IXGBE_DEV_ID_X550EM_X_XFI: u32                               = 0x15B0;
pub const IXGBE_DEV_ID_X550_VF_HV: u32                                 = 0x1564;
pub const IXGBE_DEV_ID_X550_VF: u32                                    = 0x1565;
pub const IXGBE_DEV_ID_X550EM_A_VF: u32                                = 0x15C5;
pub const IXGBE_DEV_ID_X550EM_A_VF_HV: u32                             = 0x15B4;
pub const IXGBE_DEV_ID_X550EM_X_VF: u32                                = 0x15A8;
pub const IXGBE_DEV_ID_X550EM_X_VF_HV: u32                             = 0x15A9;

// unused/unsupported by ixy
pub fn IXGBE_BY_MAC(_hw: u32, _r: u32) -> u32 { 0 }

/* General Registers */
pub const IXGBE_CTRL: u32                                              = 0x00000;
pub const IXGBE_STATUS: u32                                            = 0x00008;
pub const IXGBE_CTRL_EXT: u32                                          = 0x00018;
pub const IXGBE_ESDP: u32                                              = 0x00020;
pub const IXGBE_EODSDP: u32                                            = 0x00028;
pub const IXGBE_I2CCTL_82599: u32                                      = 0x00028;
pub const IXGBE_I2CCTL: u32                                            = IXGBE_I2CCTL_82599;
pub const IXGBE_I2CCTL_X540: u32                                       = IXGBE_I2CCTL_82599;
pub const IXGBE_I2CCTL_X550: u32                                       = 0x15F5C;
pub const IXGBE_I2CCTL_X550EM_x: u32                                   = IXGBE_I2CCTL_X550;
pub const IXGBE_I2CCTL_X550EM_a: u32                                   = IXGBE_I2CCTL_X550;

pub fn IXGBE_I2CCTL_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_I2CCTL) }

pub const IXGBE_PHY_GPIO: u32                                          = 0x00028;
pub const IXGBE_MAC_GPIO: u32                                          = 0x00030;
pub const IXGBE_PHYINT_STATUS0: u32                                    = 0x00100;
pub const IXGBE_PHYINT_STATUS1: u32                                    = 0x00104;
pub const IXGBE_PHYINT_STATUS2: u32                                    = 0x00108;
pub const IXGBE_LEDCTL: u32                                            = 0x00200;
pub const IXGBE_FRTIMER: u32                                           = 0x00048;
pub const IXGBE_TCPTIMER: u32                                          = 0x0004C;
pub const IXGBE_CORESPARE: u32                                         = 0x00600;
pub const IXGBE_EXVET: u32                                             = 0x05078;

/* NVM Registers */
pub const IXGBE_EEC: u32                                               = 0x10010;
pub const IXGBE_EEC_X540: u32                                          = IXGBE_EEC;
pub const IXGBE_EEC_X550: u32                                          = IXGBE_EEC;
pub const IXGBE_EEC_X550EM_x: u32                                      = IXGBE_EEC;
pub const IXGBE_EEC_X550EM_a: u32                                      = 0x15FF8;

pub fn IXGBE_EEC_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_EEC) }

pub const IXGBE_EERD: u32                                              = 0x10014;
pub const IXGBE_EEWR: u32                                              = 0x10018;

pub const IXGBE_FLA: u32                                               = 0x1001C;
pub const IXGBE_FLA_X540: u32                                          = IXGBE_FLA;
pub const IXGBE_FLA_X550: u32                                          = IXGBE_FLA;
pub const IXGBE_FLA_X550EM_x: u32                                      = IXGBE_FLA;
pub const IXGBE_FLA_X550EM_a: u32                                      = 0x15F68;

pub fn IXGBE_FLA_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_FLA) }

pub const IXGBE_EEMNGCTL: u32                                          = 0x10110;
pub const IXGBE_EEMNGDATA: u32                                         = 0x10114;
pub const IXGBE_FLMNGCTL: u32                                          = 0x10118;
pub const IXGBE_FLMNGDATA: u32                                         = 0x1011C;
pub const IXGBE_FLMNGCNT: u32                                          = 0x10120;
pub const IXGBE_FLOP: u32                                              = 0x1013C;

pub const IXGBE_GRC: u32                                               = 0x10200;
pub const IXGBE_GRC_X540: u32                                          = IXGBE_GRC;
pub const IXGBE_GRC_X550: u32                                          = IXGBE_GRC;
pub const IXGBE_GRC_X550EM_x: u32                                      = IXGBE_GRC;
pub const IXGBE_GRC_X550EM_a: u32                                      = 0x15F64;

pub fn IXGBE_GRC_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_GRC) }

pub const IXGBE_SRAMREL: u32                                           = 0x10210;
pub const IXGBE_SRAMREL_X540: u32                                      = IXGBE_SRAMREL;
pub const IXGBE_SRAMREL_X550: u32                                      = IXGBE_SRAMREL;
pub const IXGBE_SRAMREL_X550EM_x: u32                                  = IXGBE_SRAMREL;
pub const IXGBE_SRAMREL_X550EM_a: u32                                  = 0x15F6C;

pub fn IXGBE_SRAMREL_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_SRAMREL) }

pub const IXGBE_PHYDBG: u32                                            = 0x10218;

/* General Receive Control */
pub const IXGBE_GRC_MNG: u32                                           = 0x00000001; /* Manageability Enable */
pub const IXGBE_GRC_APME: u32                                          = 0x00000002; /* APM enabled in EEPROM */

pub const IXGBE_VPDDIAG0: u32                                          = 0x10204;
pub const IXGBE_VPDDIAG1: u32                                          = 0x10208;

/* I2CCTL Bit Masks */
pub const IXGBE_I2C_CLK_IN: u32                                        = 0x00000001;
pub const IXGBE_I2C_CLK_IN_X540: u32                                   = IXGBE_I2C_CLK_IN;
pub const IXGBE_I2C_CLK_IN_X550: u32                                   = 0x00004000;
pub const IXGBE_I2C_CLK_IN_X550EM_x: u32                               = IXGBE_I2C_CLK_IN_X550;
pub const IXGBE_I2C_CLK_IN_X550EM_a: u32                               = IXGBE_I2C_CLK_IN_X550;

pub fn IXGBE_I2C_CLK_IN_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_I2C_CLK_IN) }

pub const IXGBE_I2C_CLK_OUT: u32                                       = 0x00000002;
pub const IXGBE_I2C_CLK_OUT_X540: u32                                  = IXGBE_I2C_CLK_OUT;
pub const IXGBE_I2C_CLK_OUT_X550: u32                                  = 0x00000200;
pub const IXGBE_I2C_CLK_OUT_X550EM_x: u32                              = IXGBE_I2C_CLK_OUT_X550;
pub const IXGBE_I2C_CLK_OUT_X550EM_a: u32                              = IXGBE_I2C_CLK_OUT_X550;

pub fn IXGBE_I2C_CLK_OUT_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_I2C_CLK_OUT) }

pub const IXGBE_I2C_DATA_IN: u32                                       = 0x00000004;
pub const IXGBE_I2C_DATA_IN_X540: u32                                  = IXGBE_I2C_DATA_IN;
pub const IXGBE_I2C_DATA_IN_X550: u32                                  = 0x00001000;
pub const IXGBE_I2C_DATA_IN_X550EM_x: u32                              = IXGBE_I2C_DATA_IN_X550;
pub const IXGBE_I2C_DATA_IN_X550EM_a: u32                              = IXGBE_I2C_DATA_IN_X550;

pub fn IXGBE_I2C_DATA_IN_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_I2C_DATA_IN) }

pub const IXGBE_I2C_DATA_OUT: u32                                      = 0x00000008;
pub const IXGBE_I2C_DATA_OUT_X540: u32                                 = IXGBE_I2C_DATA_OUT;
pub const IXGBE_I2C_DATA_OUT_X550: u32                                 = 0x00000400;
pub const IXGBE_I2C_DATA_OUT_X550EM_x: u32                             = IXGBE_I2C_DATA_OUT_X550;
pub const IXGBE_I2C_DATA_OUT_X550EM_a: u32                             = IXGBE_I2C_DATA_OUT_X550;

pub fn IXGBE_I2C_DATA_OUT_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_I2C_DATA_OUT) }

pub const IXGBE_I2C_DATA_OE_N_EN: u32                                  = 0;
pub const IXGBE_I2C_DATA_OE_N_EN_X540: u32                             = IXGBE_I2C_DATA_OE_N_EN;
pub const IXGBE_I2C_DATA_OE_N_EN_X550: u32                             = 0x00000800;
pub const IXGBE_I2C_DATA_OE_N_EN_X550EM_x: u32                         = IXGBE_I2C_DATA_OE_N_EN_X550;
pub const IXGBE_I2C_DATA_OE_N_EN_X550EM_a: u32                         = IXGBE_I2C_DATA_OE_N_EN_X550;

pub fn IXGBE_I2C_DATA_OE_N_EN_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_I2C_DATA_OE_N_EN) }

pub const IXGBE_I2C_BB_EN: u32                                         = 0;
pub const IXGBE_I2C_BB_EN_X540: u32                                    = IXGBE_I2C_BB_EN;
pub const IXGBE_I2C_BB_EN_X550: u32                                    = 0x00000100;
pub const IXGBE_I2C_BB_EN_X550EM_x: u32                                = IXGBE_I2C_BB_EN_X550;
pub const IXGBE_I2C_BB_EN_X550EM_a: u32                                = IXGBE_I2C_BB_EN_X550;

pub fn IXGBE_I2C_BB_EN_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_I2C_BB_EN) }

pub const IXGBE_I2C_CLK_OE_N_EN: u32                                   = 0;
pub const IXGBE_I2C_CLK_OE_N_EN_X540: u32                              = IXGBE_I2C_CLK_OE_N_EN;
pub const IXGBE_I2C_CLK_OE_N_EN_X550: u32                              = 0x00002000;
pub const IXGBE_I2C_CLK_OE_N_EN_X550EM_x: u32                          = IXGBE_I2C_CLK_OE_N_EN_X550;
pub const IXGBE_I2C_CLK_OE_N_EN_X550EM_a: u32                          = IXGBE_I2C_CLK_OE_N_EN_X550;

pub fn IXGBE_I2C_CLK_OE_N_EN_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_I2C_CLK_OE_N_EN) }

pub const IXGBE_I2C_CLOCK_STRETCHING_TIMEOUT: u32                      = 500;

pub const IXGBE_I2C_THERMAL_SENSOR_ADDR: u32                           = 0xF8;
pub const IXGBE_EMC_INTERNAL_DATA: u32                                 = 0x00;
pub const IXGBE_EMC_INTERNAL_THERM_LIMIT: u32                          = 0x20;
pub const IXGBE_EMC_DIODE1_DATA: u32                                   = 0x01;
pub const IXGBE_EMC_DIODE1_THERM_LIMIT: u32                            = 0x19;
pub const IXGBE_EMC_DIODE2_DATA: u32                                   = 0x23;
pub const IXGBE_EMC_DIODE2_THERM_LIMIT: u32                            = 0x1A;

pub const IXGBE_MAX_SENSORS: u32                                       = 3;

/* Interrupt Registers */
pub const IXGBE_EICR: u32                                              = 0x00800;
pub const IXGBE_EICS: u32                                              = 0x00808;
pub const IXGBE_EIMS: u32                                              = 0x00880;
pub const IXGBE_EIMC: u32                                              = 0x00888;
pub const IXGBE_EIAC: u32                                              = 0x00810;
pub const IXGBE_EIAM: u32                                              = 0x00890;

pub fn IXGBE_EICS_EX(i: u32) -> u32 { 0x00A90 + i * 4 }

pub fn IXGBE_EIMS_EX(i: u32) -> u32 { 0x00AA0 + i * 4 }

pub fn IXGBE_EIMC_EX(i: u32) -> u32 { 0x00AB0 + i * 4 }

pub fn IXGBE_EIAM_EX(i: u32) -> u32 { 0x00AD0 + i * 4 }
/* 82599 EITR is only 12 bits, with the lower 3 always zero */
/*
 * 82598 EITR is 16 bits but set the limits based on the max
 * supported by all ixgbe hardware
 */
pub const IXGBE_MAX_INT_RATE: u32                                      = 488281;
pub const IXGBE_MIN_INT_RATE: u32                                      = 956;
pub const IXGBE_MAX_EITR: u32                                          = 0x00000FF8;
pub const IXGBE_MIN_EITR: u32                                          = 8;

pub fn IXGBE_EITR(i: u32) -> u32 { if i <= 23 { 0x00820 + i * 4 } else { 0x012300 + ((i - 24) * 4) } }

pub const IXGBE_EITR_ITR_INT_MASK: u32                                 = 0x00000FF8;
pub const IXGBE_EITR_LLI_MOD: u32                                      = 0x00008000;
pub const IXGBE_EITR_CNT_WDIS: u32                                     = 0x80000000;

pub fn IXGBE_IVAR(i: u32) -> u32 { 0x00900 + i * 4 } /* 24 at 0x900-0x960 */
pub const IXGBE_IVAR_MISC: u32                                         = 0x00A00; /* misc MSI-X interrupt causes */
pub const IXGBE_EITRSEL: u32                                           = 0x00894;
pub const IXGBE_MSIXT: u32                                             = 0x00000; /* MSI-X Table. 0x0000 - 0x01C */
pub const IXGBE_MSIXPBA: u32                                           = 0x02000; /* MSI-X Pending bit array */
pub fn IXGBE_PBACL(i: u32) -> u32 { if i == 0 { 0x11068 } else { 0x110C0 + i * 4 } }

pub const IXGBE_GPIE: u32                                              = 0x00898;

/* Flow Control Registers */
pub const IXGBE_FCADBUL: u32                                           = 0x03210;
pub const IXGBE_FCADBUH: u32                                           = 0x03214;
pub const IXGBE_FCAMACL: u32                                           = 0x04328;
pub const IXGBE_FCAMACH: u32                                           = 0x0432C;

pub fn IXGBE_FCRTH_82599(i: u32) -> u32 { 0x03260 + i * 4 } /* 8 of these (0-7) */
pub fn IXGBE_FCRTL_82599(i: u32) -> u32 { 0x03220 + i * 4 } /* 8 of these (0-7) */
pub const IXGBE_PFCTOP: u32                                            = 0x03008;

pub fn IXGBE_FCTTV(i: u32) -> u32 { 0x03200 + i * 4 } /* 4 of these (0-3) */
pub fn IXGBE_FCRTL(i: u32) -> u32 { 0x03220 + i * 8 } /* 8 of these (0-7) */
pub fn IXGBE_FCRTH(i: u32) -> u32 { 0x03260 + i * 8 } /* 8 of these (0-7) */
pub const IXGBE_FCRTV: u32                                             = 0x032A0;
pub const IXGBE_FCCFG: u32                                             = 0x03D00;
pub const IXGBE_TFCS: u32                                              = 0x0CE00;

/* Receive DMA Registers */
pub fn IXGBE_RDBAL(i: u32) -> u32 { if i < 64 { 0x01000 + i * 0x40 } else { 0x0D000 + ((i - 64) * 0x40) } }
pub fn IXGBE_RDBAH(i: u32) -> u32 { if i < 64 { 0x01004 + i * 0x40 } else { 0x0D004 + ((i - 64) * 0x40) } }
pub fn IXGBE_RDLEN(i: u32) -> u32 { if i < 64 { 0x01008 + i * 0x40 } else { 0x0D008 + ((i - 64) * 0x40) } }
pub fn IXGBE_RDH(i: u32) -> u32 { if i < 64 { 0x01010 + i * 0x40 } else { 0x0D010 + ((i - 64) * 0x40) } }
pub fn IXGBE_RDT(i: u32) -> u32 { if i < 64 { 0x01018 + i * 0x40 } else { 0x0D018 + ((i - 64) * 0x40) } }
pub fn IXGBE_RXDCTL(i: u32) -> u32 { if i < 64 { 0x01028 + i * 0x40 } else { 0x0D028 + ((i - 64) * 0x40) } }
pub fn IXGBE_RSCCTL(i: u32) -> u32 { if i < 64 { 0x0102C + i * 0x40 } else { 0x0D02C + ((i - 64) * 0x40) } }

pub const IXGBE_RSCDBU: u32                                            = 0x03028;
pub const IXGBE_RDDCC: u32                                             = 0x02F20;
pub const IXGBE_RXMEMWRAP: u32                                         = 0x03190;
pub const IXGBE_STARCTRL: u32                                          = 0x03024;
/*
 * Split and Replication Receive Control Registers
 * 00-15 : 0x02100 + n*4;
 * 16-64 : 0x01014 + n*0x40;
 * 64-127: 0x0D014 + (n-64)*0x40;
 */
pub fn IXGBE_SRRCTL(i: u32) -> u32 {
    if i <= 15 {
        0x02100 + i * 4
    } else if i < 64 {
        0x01014 + i * 0x40
    } else {
        0x0D014 + ((i - 64) * 0x40)
    }
}
/*
 * Rx DCA Control Register:
 * 00-15 : 0x02200 + n*4;
 * 16-64 : 0x0100C + n*0x40;
 * 64-127: 0x0D00C + (n-64)*0x40;
 */
pub fn IXGBE_DCA_RXCTRL(i: u32) -> u32 {
    if i <= 15 {
        0x02200 + i * 4
    } else if i < 64 {
        0x0100C + i * 0x40
    } else {
        0x0D00C + ((i - 64) * 0x40)
    }
}

pub const IXGBE_RDRXCTL: u32                                           = 0x02F00;
/* 8 of these 0x03C00 - 0x03C1C */
pub fn IXGBE_RXPBSIZE(i: u32) -> u32 { 0x03C00 + i * 4 }

pub const IXGBE_RXCTRL: u32                                            = 0x03000;
pub const IXGBE_DROPEN: u32                                            = 0x03D04;
pub const IXGBE_RXPBSIZE_SHIFT: u32                                    = 10;
pub const IXGBE_RXPBSIZE_MASK: u32                                     = 0x000FFC00;

/* Receive Registers */
pub const IXGBE_RXCSUM: u32                                            = 0x05000;
pub const IXGBE_RFCTL: u32                                             = 0x05008;
pub const IXGBE_DRECCCTL: u32                                          = 0x02F08;
pub const IXGBE_DRECCCTL_DISABLE: u32                                  = 0;
pub const IXGBE_DRECCCTL2: u32                                         = 0x02F8C;

/* Multicast Table Array - 128 entries */
pub fn IXGBE_MTA(i: u32) -> u32 { 0x05200 + i * 4 }

pub fn IXGBE_RAL(i: u32) -> u32 { if i <= 15 { 0x05400 + i * 8 } else { 0x0A200 + i * 8 } }

pub fn IXGBE_RAH(i: u32) -> u32 { if i <= 15 { 0x05404 + i * 8 } else { 0x0A204 + i * 8 } }

pub fn IXGBE_MPSAR_LO(i: u32) -> u32 { 0x0A600 + i * 8 }

pub fn IXGBE_MPSAR_HI(i: u32) -> u32 { 0x0A604 + i * 8 }
/* Packet split receive type */
pub fn IXGBE_PSRTYPE(i: u32) -> u32 { if i <= 15 { 0x05480 + i * 4 } else { 0x0EA00 + i * 4 } }
/* array of 4096 1-bit vlan filters */
pub fn IXGBE_VFTA(i: u32) -> u32 { 0x0A000 + i * 4 }
/*array of 4096 4-bit vlan vmdq indices */
pub fn IXGBE_VFTAVIND(j: u32, i: u32) -> u32 { 0x0A200 + j * 0x200 + i * 4 }

pub const IXGBE_FCTRL: u32                                             = 0x05080;
pub const IXGBE_VLNCTRL: u32                                           = 0x05088;
pub const IXGBE_MCSTCTRL: u32                                          = 0x05090;
pub const IXGBE_MRQC: u32                                              = 0x05818;

pub fn IXGBE_SAQF(i: u32) -> u32 { 0x0E000 + i * 4 } /* Source Address Queue Filter */
pub fn IXGBE_DAQF(i: u32) -> u32 { 0x0E200 + i * 4 } /* Dest. Address Queue Filter */
pub fn IXGBE_SDPQF(i: u32) -> u32 { 0x0E400 + i * 4 } /* Src Dest. Addr Queue Filter */
pub fn IXGBE_FTQF(i: u32) -> u32 { 0x0E600 + i * 4 } /* Five Tuple Queue Filter */
pub fn IXGBE_ETQF(i: u32) -> u32 { 0x05128 + i * 4 } /* EType Queue Filter */
pub fn IXGBE_ETQS(i: u32) -> u32 { 0x0EC00 + i * 4 } /* EType Queue Select */
pub const IXGBE_SYNQF: u32                                             = 0x0EC30; /* SYN Packet Queue Filter */
pub const IXGBE_RQTC: u32                                              = 0x0EC70;
pub const IXGBE_MTQC: u32                                              = 0x08120;

pub fn IXGBE_VLVF(i: u32) -> u32 { 0x0F100 + i * 4 }  /* 64 of these (0-63) */
pub fn IXGBE_VLVFB(i: u32) -> u32 { 0x0F200 + i * 4 }  /* 128 of these (0-127) */
pub fn IXGBE_VMVIR(i: u32) -> u32 { 0x08000 + i * 4 }  /* 64 of these (0-63) */
pub const IXGBE_PFFLPL: u32                                            = 0x050B0;
pub const IXGBE_PFFLPH: u32                                            = 0x050B4;
pub const IXGBE_VT_CTL: u32                                            = 0x051B0;

pub fn IXGBE_PFMAILBOX(i: u32) -> u32 { 0x04B00 + 4 * i } /* 64 total */
/* 64 Mailboxes, 16 DW each */
pub fn IXGBE_PFMBMEM(i: u32) -> u32 { 0x13000 + 64 * i }

pub fn IXGBE_PFMBICR(i: u32) -> u32 { 0x00710 + 4 * i } /* 4 total */
pub fn IXGBE_PFMBIMR(i: u32) -> u32 { 0x00720 + 4 * i } /* 4 total */
pub fn IXGBE_VFRE(i: u32) -> u32 { 0x051E0 + i * 4 }

pub fn IXGBE_VFTE(i: u32) -> u32 { 0x08110 + i * 4 }

pub fn IXGBE_VMECM(i: u32) -> u32 { 0x08790 + i * 4 }

pub const IXGBE_QDE: u32                                               = 0x2F04;

pub fn IXGBE_VMTXSW(i: u32) -> u32 { 0x05180 + i * 4 } /* 2 total */
pub fn IXGBE_VMOLR(i: u32) -> u32 { 0x0F000 + i * 4 } /* 64 total */
pub fn IXGBE_UTA(i: u32) -> u32 { 0x0F400 + i * 4 }

pub fn IXGBE_MRCTL(i: u32) -> u32 { 0x0F600 + i * 4 }

pub fn IXGBE_VMRVLAN(i: u32) -> u32 { 0x0F610 + i * 4 }

pub fn IXGBE_VMRVM(i: u32) -> u32 { 0x0F630 + i * 4 }

pub const IXGBE_LVMMC_RX: u32                                          = 0x2FA8;
pub const IXGBE_LVMMC_TX: u32                                          = 0x8108;
pub const IXGBE_LMVM_RX: u32                                           = 0x2FA4;
pub const IXGBE_LMVM_TX: u32                                           = 0x8124;

pub fn IXGBE_WQBR_RX(i: u32) -> u32 { 0x2FB0 + i * 4 } /* 4 total */
pub fn IXGBE_WQBR_TX(i: u32) -> u32 { 0x8130 + i * 4 } /* 4 total */
pub fn IXGBE_L34T_IMIR(i: u32) -> u32 { 0x0E800 + i * 4 } /*128 of these (0-127) */
pub const IXGBE_RXFECCERR0: u32                                        = 0x051B8;
pub const IXGBE_LLITHRESH: u32                                         = 0x0EC90;

pub fn IXGBE_IMIR(i: u32) -> u32 { 0x05A80 + i * 4 }  /* 8 of these (0-7) */
pub fn IXGBE_IMIREXT(i: u32) -> u32 { 0x05AA0 + i * 4 }  /* 8 of these (0-7) */
pub const IXGBE_IMIRVP: u32                                            = 0x05AC0;
pub const IXGBE_VMD_CTL: u32                                           = 0x0581C;

pub fn IXGBE_RETA(i: u32) -> u32 { 0x05C00 + i * 4 }  /* 32 of these (0-31) */
pub fn IXGBE_ERETA(i: u32) -> u32 { 0x0EE80 + i * 4 }  /* 96 of these (0-95) */
pub fn IXGBE_RSSRK(i: u32) -> u32 { 0x05C80 + i * 4 }  /* 10 of these (0-9) */

/* Registers for setting up RSS on X550 with SRIOV;
 * p - pool number (0..63)
 * i - index (0..10 for PFVFRSSRK, 0..15 for PFVFRETA)
 */
pub fn IXGBE_PFVFMRQC(p: u32) -> u32 { 0x03400 + p * 4 }

pub fn IXGBE_PFVFRSSRK(i: u32, p: u32) -> u32 { 0x018000 + i * 4 + p * 0x40 }

pub fn IXGBE_PFVFRETA(i: u32, p: u32) -> u32 { 0x019000 + i * 4 + p * 0x40 }

/* Flow Director registers */
pub const IXGBE_FDIRCTRL: u32                                          = 0x0EE00;
pub const IXGBE_FDIRHKEY: u32                                          = 0x0EE68;
pub const IXGBE_FDIRSKEY: u32                                          = 0x0EE6C;
pub const IXGBE_FDIRDIP4M: u32                                         = 0x0EE3C;
pub const IXGBE_FDIRSIP4M: u32                                         = 0x0EE40;
pub const IXGBE_FDIRTCPM: u32                                          = 0x0EE44;
pub const IXGBE_FDIRUDPM: u32                                          = 0x0EE48;
pub const IXGBE_FDIRSCTPM: u32                                         = 0x0EE78;
pub const IXGBE_FDIRIP6M: u32                                          = 0x0EE74;
pub const IXGBE_FDIRM: u32                                             = 0x0EE70;

/* Flow Director Stats registers */
pub const IXGBE_FDIRFREE: u32                                          = 0x0EE38;
pub const IXGBE_FDIRLEN: u32                                           = 0x0EE4C;
pub const IXGBE_FDIRUSTAT: u32                                         = 0x0EE50;
pub const IXGBE_FDIRFSTAT: u32                                         = 0x0EE54;
pub const IXGBE_FDIRMATCH: u32                                         = 0x0EE58;
pub const IXGBE_FDIRMISS: u32                                          = 0x0EE5C;

/* Flow Director Programming registers */
pub fn IXGBE_FDIRSIPv6(i: u32) -> u32 { 0x0EE0C + i * 4 } /* 3 of these (0-2) */
pub const IXGBE_FDIRIPSA: u32                                          = 0x0EE18;
pub const IXGBE_FDIRIPDA: u32                                          = 0x0EE1C;
pub const IXGBE_FDIRPORT: u32                                          = 0x0EE20;
pub const IXGBE_FDIRVLAN: u32                                          = 0x0EE24;
pub const IXGBE_FDIRHASH: u32                                          = 0x0EE28;
pub const IXGBE_FDIRCMD: u32                                           = 0x0EE2C;

/* Transmit DMA registers */
pub fn IXGBE_TDBAL(i: u32) -> u32 { 0x06000 + i * 0x40 } /* 32 of them (0-31)*/
pub fn IXGBE_TDBAH(i: u32) -> u32 { 0x06004 + i * 0x40 }

pub fn IXGBE_TDLEN(i: u32) -> u32 { 0x06008 + i * 0x40 }

pub fn IXGBE_TDH(i: u32) -> u32 { 0x06010 + i * 0x40 }

pub fn IXGBE_TDT(i: u32) -> u32 { 0x06018 + i * 0x40 }

pub fn IXGBE_TXDCTL(i: u32) -> u32 { 0x06028 + i * 0x40 }

pub fn IXGBE_TDWBAL(i: u32) -> u32 { 0x06038 + i * 0x40 }

pub fn IXGBE_TDWBAH(i: u32) -> u32 { 0x0603C + i * 0x40 }

pub const IXGBE_DTXCTL: u32                                            = 0x07E00;

pub const IXGBE_DMATXCTL: u32                                          = 0x04A80;

pub fn IXGBE_PFVFSPOOF(i: u32) -> u32 { 0x08200 + i * 4 } /* 8 of these 0 - 7 */
pub const IXGBE_PFDTXGSWC: u32                                         = 0x08220;
pub const IXGBE_DTXMXSZRQ: u32                                         = 0x08100;
pub const IXGBE_DTXTCPFLGL: u32                                        = 0x04A88;
pub const IXGBE_DTXTCPFLGH: u32                                        = 0x04A8C;
pub const IXGBE_LBDRPEN: u32                                           = 0x0CA00;

pub fn IXGBE_TXPBTHRESH(i: u32) -> u32 { 0x04950 + i * 4 } /* 8 of these 0 - 7 */

pub const IXGBE_DMATXCTL_TE: u32                                       = 0x1; /* Transmit Enable */
pub const IXGBE_DMATXCTL_NS: u32                                       = 0x2; /* No Snoop LSO hdr buffer */
pub const IXGBE_DMATXCTL_GDV: u32                                      = 0x8; /* Global Double VLAN */
pub const IXGBE_DMATXCTL_MDP_EN: u32                                   = 0x20; /* Bit 5 */
pub const IXGBE_DMATXCTL_MBINTEN: u32                                  = 0x40; /* Bit 6 */
pub const IXGBE_DMATXCTL_VT_SHIFT: u32                                 = 16;  /* VLAN EtherType */

pub const IXGBE_PFDTXGSWC_VT_LBEN: u32                                 = 0x1; /* Local L2 VT switch enable */

/* Anti-spoofing defines */
pub const IXGBE_SPOOF_MACAS_MASK: u32                                  = 0xFF;
pub const IXGBE_SPOOF_VLANAS_MASK: u32                                 = 0xFF00;
pub const IXGBE_SPOOF_VLANAS_SHIFT: u32                                = 8;
pub const IXGBE_SPOOF_ETHERTYPEAS: u32                                 = 0xFF000000;
pub const IXGBE_SPOOF_ETHERTYPEAS_SHIFT: u32                           = 16;
pub const IXGBE_PFVFSPOOF_REG_COUNT: u32                               = 8;
/* 16 of these (0-15) */
pub fn IXGBE_DCA_TXCTRL(i: u32) -> u32 { 0x07200 + i * 4 }
/* Tx DCA Control register : 128 of these (0-127) */
pub fn IXGBE_DCA_TXCTRL_82599(i: u32) -> u32 { 0x0600C + i * 0x40 }

pub const IXGBE_TIPG: u32                                              = 0x0CB00;

pub fn IXGBE_TXPBSIZE(i: u32) -> u32 { 0x0CC00 + i * 4 } /* 8 of these */
pub const IXGBE_MNGTXMAP: u32                                          = 0x0CD10;
pub const IXGBE_TIPG_FIBER_DEFAULT: u32                                = 3;
pub const IXGBE_TXPBSIZE_SHIFT: u32                                    = 10;

/* Wake up registers */
pub const IXGBE_WUC: u32                                               = 0x05800;
pub const IXGBE_WUFC: u32                                              = 0x05808;
pub const IXGBE_WUS: u32                                               = 0x05810;
pub const IXGBE_IPAV: u32                                              = 0x05838;
pub const IXGBE_IP4AT: u32                                             = 0x05840; /* IPv4 table 0x5840-0x5858 */
pub const IXGBE_IP6AT: u32                                             = 0x05880; /* IPv6 table 0x5880-0x588F */

pub const IXGBE_WUPL: u32                                              = 0x05900;
pub const IXGBE_WUPM: u32                                              = 0x05A00; /* wake up pkt memory 0x5A00-0x5A7C */
pub const IXGBE_PROXYS: u32                                            = 0x05F60; /* Proxying Status Register */
pub const IXGBE_PROXYFC: u32                                           = 0x05F64; /* Proxying Filter Control Register */
pub const IXGBE_VXLANCTRL: u32                                         = 0x0000507C; /* Rx filter VXLAN UDPPORT Register */

/* masks for accessing VXLAN and GENEVE UDP ports */
pub const IXGBE_VXLANCTRL_VXLAN_UDPPORT_MASK: u32                      = 0x0000ffff; /* VXLAN port */
pub const IXGBE_VXLANCTRL_GENEVE_UDPPORT_MASK: u32                     = 0xffff0000; /* GENEVE port */
pub const IXGBE_VXLANCTRL_ALL_UDPPORT_MASK: u32                        = 0xffffffff; /* GENEVE/VXLAN */

pub const IXGBE_VXLANCTRL_GENEVE_UDPPORT_SHIFT: u32                    = 16;

pub fn IXGBE_FHFT(n: u32) -> u32 { 0x09000 + n * 0x100 } /* Flex host filter table */
/* Ext Flexible Host Filter Table */
pub fn IXGBE_FHFT_EXT(n: u32) -> u32 { 0x09800 + n * 0x100 }

pub fn IXGBE_FHFT_EXT_X550(n: u32) -> u32 { 0x09600 + n * 0x100 }

/* Four Flexible Filters are supported */
pub const IXGBE_FLEXIBLE_FILTER_COUNT_MAX: u32                         = 4;

/* Six Flexible Filters are supported */
pub const IXGBE_FLEXIBLE_FILTER_COUNT_MAX_6: u32                       = 6;
/* Eight Flexible Filters are supported */
pub const IXGBE_FLEXIBLE_FILTER_COUNT_MAX_8: u32                       = 8;
pub const IXGBE_EXT_FLEXIBLE_FILTER_COUNT_MAX: u32                     = 2;

/* Each Flexible Filter is at most 128 (0x80) bytes in length */
pub const IXGBE_FLEXIBLE_FILTER_SIZE_MAX: u32                          = 128;
pub const IXGBE_FHFT_LENGTH_OFFSET: u32                                = 0xFC;  /* Length byte in FHFT */
pub const IXGBE_FHFT_LENGTH_MASK: u32                                  = 0x0FF; /* Length in lower byte */

/* Definitions for power management and wakeup registers */
/* Wake Up Control */
pub const IXGBE_WUC_PME_EN: u32                                        = 0x00000002; /* PME Enable */
pub const IXGBE_WUC_PME_STATUS: u32                                    = 0x00000004; /* PME Status */
pub const IXGBE_WUC_WKEN: u32                                          = 0x00000010; /* Enable PE_WAKE_N pin assertion  */

/* Wake Up Filter Control */
pub const IXGBE_WUFC_LNKC: u32                                         = 0x00000001; /* Link Status Change Wakeup Enable */
pub const IXGBE_WUFC_MAG: u32                                          = 0x00000002; /* Magic Packet Wakeup Enable */
pub const IXGBE_WUFC_EX: u32                                           = 0x00000004; /* Directed Exact Wakeup Enable */
pub const IXGBE_WUFC_MC: u32                                           = 0x00000008; /* Directed Multicast Wakeup Enable */
pub const IXGBE_WUFC_BC: u32                                           = 0x00000010; /* Broadcast Wakeup Enable */
pub const IXGBE_WUFC_ARP: u32                                          = 0x00000020; /* ARP Request Packet Wakeup Enable */
pub const IXGBE_WUFC_IPV4: u32                                         = 0x00000040; /* Directed IPv4 Packet Wakeup Enable */
pub const IXGBE_WUFC_IPV6: u32                                         = 0x00000080; /* Directed IPv6 Packet Wakeup Enable */
pub const IXGBE_WUFC_MNG: u32                                          = 0x00000100; /* Directed Mgmt Packet Wakeup Enable */

pub const IXGBE_WUFC_IGNORE_TCO: u32                                   = 0x00008000; /* Ignore WakeOn TCO packets */
pub const IXGBE_WUFC_FLX0: u32                                         = 0x00010000; /* Flexible Filter 0 Enable */
pub const IXGBE_WUFC_FLX1: u32                                         = 0x00020000; /* Flexible Filter 1 Enable */
pub const IXGBE_WUFC_FLX2: u32                                         = 0x00040000; /* Flexible Filter 2 Enable */
pub const IXGBE_WUFC_FLX3: u32                                         = 0x00080000; /* Flexible Filter 3 Enable */
pub const IXGBE_WUFC_FLX4: u32                                         = 0x00100000; /* Flexible Filter 4 Enable */
pub const IXGBE_WUFC_FLX5: u32                                         = 0x00200000; /* Flexible Filter 5 Enable */
pub const IXGBE_WUFC_FLX_FILTERS: u32                                  = 0x000F0000; /* Mask for 4 flex filters */
pub const IXGBE_WUFC_FLX_FILTERS_6: u32                                = 0x003F0000; /* Mask for 6 flex filters */
pub const IXGBE_WUFC_FLX_FILTERS_8: u32                                = 0x00FF0000; /* Mask for 8 flex filters */
pub const IXGBE_WUFC_FW_RST_WK: u32                                    = 0x80000000; /* Ena wake on FW reset assertion */
/* Mask for Ext. flex filters */
pub const IXGBE_WUFC_EXT_FLX_FILTERS: u32                              = 0x00300000;
pub const IXGBE_WUFC_ALL_FILTERS: u32                                  = 0x000F00FF; /* Mask all 4 flex filters */
pub const IXGBE_WUFC_ALL_FILTERS_6: u32                                = 0x003F00FF; /* Mask all 6 flex filters */
pub const IXGBE_WUFC_ALL_FILTERS_8: u32                                = 0x00FF00FF; /* Mask all 8 flex filters */
pub const IXGBE_WUFC_FLX_OFFSET: u32                                   = 16; /* Offset to the Flexible Filters bits */

/* Wake Up Status */
pub const IXGBE_WUS_LNKC: u32                                          = IXGBE_WUFC_LNKC;
pub const IXGBE_WUS_MAG: u32                                           = IXGBE_WUFC_MAG;
pub const IXGBE_WUS_EX: u32                                            = IXGBE_WUFC_EX;
pub const IXGBE_WUS_MC: u32                                            = IXGBE_WUFC_MC;
pub const IXGBE_WUS_BC: u32                                            = IXGBE_WUFC_BC;
pub const IXGBE_WUS_ARP: u32                                           = IXGBE_WUFC_ARP;
pub const IXGBE_WUS_IPV4: u32                                          = IXGBE_WUFC_IPV4;
pub const IXGBE_WUS_IPV6: u32                                          = IXGBE_WUFC_IPV6;
pub const IXGBE_WUS_MNG: u32                                           = IXGBE_WUFC_MNG;
pub const IXGBE_WUS_FLX0: u32                                          = IXGBE_WUFC_FLX0;
pub const IXGBE_WUS_FLX1: u32                                          = IXGBE_WUFC_FLX1;
pub const IXGBE_WUS_FLX2: u32                                          = IXGBE_WUFC_FLX2;
pub const IXGBE_WUS_FLX3: u32                                          = IXGBE_WUFC_FLX3;
pub const IXGBE_WUS_FLX4: u32                                          = IXGBE_WUFC_FLX4;
pub const IXGBE_WUS_FLX5: u32                                          = IXGBE_WUFC_FLX5;
pub const IXGBE_WUS_FLX_FILTERS: u32                                   = IXGBE_WUFC_FLX_FILTERS;
pub const IXGBE_WUS_FW_RST_WK: u32                                     = IXGBE_WUFC_FW_RST_WK;
/* Proxy Status */
pub const IXGBE_PROXYS_EX: u32                                         = 0x00000004; /* Exact packet received */
pub const IXGBE_PROXYS_ARP_DIR: u32                                    = 0x00000020; /* ARP w/filter match received */
pub const IXGBE_PROXYS_NS: u32                                         = 0x00000200; /* IPV6 NS received */
pub const IXGBE_PROXYS_NS_DIR: u32                                     = 0x00000400; /* IPV6 NS w/DA match received */
pub const IXGBE_PROXYS_ARP: u32                                        = 0x00000800; /* ARP request packet received */
pub const IXGBE_PROXYS_MLD: u32                                        = 0x00001000; /* IPv6 MLD packet received */

/* Proxying Filter Control */
pub const IXGBE_PROXYFC_ENABLE: u32                                    = 0x00000001; /* Port Proxying Enable */
pub const IXGBE_PROXYFC_EX: u32                                        = 0x00000004; /* Directed Exact Proxy Enable */
pub const IXGBE_PROXYFC_ARP_DIR: u32                                   = 0x00000020; /* Directed ARP Proxy Enable */
pub const IXGBE_PROXYFC_NS: u32                                        = 0x00000200; /* IPv6 Neighbor Solicitation */
pub const IXGBE_PROXYFC_ARP: u32                                       = 0x00000800; /* ARP Request Proxy Enable */
pub const IXGBE_PROXYFC_MLD: u32                                       = 0x00000800; /* IPv6 MLD Proxy Enable */
pub const IXGBE_PROXYFC_NO_TCO: u32                                    = 0x00008000; /* Ignore TCO packets */

pub const IXGBE_WUPL_LENGTH_MASK: u32                                  = 0xFFFF;

/* DCB registers */
pub const IXGBE_DCB_MAX_TRAFFIC_CLASS: u32                             = 8;
pub const IXGBE_RMCS: u32                                              = 0x03D00;
pub const IXGBE_DPMCS: u32                                             = 0x07F40;
pub const IXGBE_PDPMCS: u32                                            = 0x0CD00;
pub const IXGBE_RUPPBMR: u32                                           = 0x050A0;

pub fn IXGBE_RT2CR(i: u32) -> u32 { 0x03C20 + i * 4 } /* 8 of these (0-7) */
pub fn IXGBE_RT2SR(i: u32) -> u32 { 0x03C40 + i * 4 } /* 8 of these (0-7) */
pub fn IXGBE_TDTQ2TCCR(i: u32) -> u32 { 0x0602C + i * 0x40 } /* 8 of these (0-7) */
pub fn IXGBE_TDTQ2TCSR(i: u32) -> u32 { 0x0622C + i * 0x40 } /* 8 of these (0-7) */
pub fn IXGBE_TDPT2TCCR(i: u32) -> u32 { 0x0CD20 + i * 4 } /* 8 of these (0-7) */
pub fn IXGBE_TDPT2TCSR(i: u32) -> u32 { 0x0CD40 + i * 4 } /* 8 of these (0-7) */

/* Power Management */
/* DMA Coalescing configuration */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbe_dmac_config {
    pub watchdog_timer: u16,
    /* usec units */
    pub fcoe_en: bool,
    pub link_speed: u32,
    pub fcoe_tc: u8,
    pub num_tcs: u8,
}

/*
 * DMA Coalescing threshold Rx PB TC[n] value in Kilobyte by link speed.
 * DMACRXT = 10Gbps = 10,000 bits / usec = 1250 bytes / usec 70 * 1250 == 87500 bytes [85KB]
 */
pub const IXGBE_DMACRXT_10G: u32                                       = 0x55;
pub const IXGBE_DMACRXT_1G: u32                                        = 0x09;
pub const IXGBE_DMACRXT_100M: u32                                      = 0x01;

/* DMA Coalescing registers */
pub const IXGBE_DMCMNGTH: u32                                          = 0x15F20; /* Management Threshold */
pub const IXGBE_DMACR: u32                                             = 0x02400; /* Control register */
pub fn IXGBE_DMCTH(i: u32) -> u32 { 0x03300 + i * 4 } /* 8 of these */
pub const IXGBE_DMCTLX: u32                                            = 0x02404; /* Time to Lx request */
/* DMA Coalescing register fields */
pub const IXGBE_DMCMNGTH_DMCMNGTH_MASK: u32                            = 0x000FFFF0; /* Mng Threshold mask */
pub const IXGBE_DMCMNGTH_DMCMNGTH_SHIFT: u32                           = 4; /* Management Threshold shift */
pub const IXGBE_DMACR_DMACWT_MASK: u32                                 = 0x0000FFFF; /* Watchdog Timer mask */
pub const IXGBE_DMACR_HIGH_PRI_TC_MASK: u32                            = 0x00FF0000;
pub const IXGBE_DMACR_HIGH_PRI_TC_SHIFT: u32                           = 16;
pub const IXGBE_DMACR_EN_MNG_IND: u32                                  = 0x10000000; /* Enable Mng Indications */
pub const IXGBE_DMACR_LX_COAL_IND: u32                                 = 0x40000000; /* Lx Coalescing indicate */
pub const IXGBE_DMACR_DMAC_EN: u32                                     = 0x80000000; /* DMA Coalescing Enable */
pub const IXGBE_DMCTH_DMACRXT_MASK: u32                                = 0x000001FF; /* Receive Threshold mask */
pub const IXGBE_DMCTLX_TTLX_MASK: u32                                  = 0x00000FFF; /* Time to Lx request mask */

/* EEE registers */
pub const IXGBE_EEER: u32                                              = 0x043A0; /* EEE register */
pub const IXGBE_EEE_STAT: u32                                          = 0x04398; /* EEE Status */
pub const IXGBE_EEE_SU: u32                                            = 0x04380; /* EEE Set up */
pub const IXGBE_EEE_SU_TEEE_DLY_SHIFT: u32                             = 26;
pub const IXGBE_TLPIC: u32                                             = 0x041F4; /* EEE Tx LPI count */
pub const IXGBE_RLPIC: u32                                             = 0x041F8; /* EEE Rx LPI count */

/* EEE register fields */
pub const IXGBE_EEER_TX_LPI_EN: u32                                    = 0x00010000; /* Enable EEE LPI TX path */
pub const IXGBE_EEER_RX_LPI_EN: u32                                    = 0x00020000; /* Enable EEE LPI RX path */
pub const IXGBE_EEE_STAT_NEG: u32                                      = 0x20000000; /* EEE support neg on link */
pub const IXGBE_EEE_RX_LPI_STATUS: u32                                 = 0x40000000; /* RX Link in LPI status */
pub const IXGBE_EEE_TX_LPI_STATUS: u32                                 = 0x80000000; /* TX Link in LPI status */


/* Security Control Registers */
pub const IXGBE_SECTXCTRL: u32                                         = 0x08800;
pub const IXGBE_SECTXSTAT: u32                                         = 0x08804;
pub const IXGBE_SECTXBUFFAF: u32                                       = 0x08808;
pub const IXGBE_SECTXMINIFG: u32                                       = 0x08810;
pub const IXGBE_SECRXCTRL: u32                                         = 0x08D00;
pub const IXGBE_SECRXSTAT: u32                                         = 0x08D04;

/* Security Bit Fields and Masks */
pub const IXGBE_SECTXCTRL_SECTX_DIS: u32                               = 0x00000001;
pub const IXGBE_SECTXCTRL_TX_DIS: u32                                  = 0x00000002;
pub const IXGBE_SECTXCTRL_STORE_FORWARD: u32                           = 0x00000004;

pub const IXGBE_SECTXSTAT_SECTX_RDY: u32                               = 0x00000001;
pub const IXGBE_SECTXSTAT_ECC_TXERR: u32                               = 0x00000002;

pub const IXGBE_SECRXCTRL_SECRX_DIS: u32                               = 0x00000001;
pub const IXGBE_SECRXCTRL_RX_DIS: u32                                  = 0x00000002;

pub const IXGBE_SECRXSTAT_SECRX_RDY: u32                               = 0x00000001;
pub const IXGBE_SECRXSTAT_ECC_RXERR: u32                               = 0x00000002;

/* LinkSec (MacSec) Registers */
pub const IXGBE_LSECTXCAP: u32                                         = 0x08A00;
pub const IXGBE_LSECRXCAP: u32                                         = 0x08F00;
pub const IXGBE_LSECTXCTRL: u32                                        = 0x08A04;
pub const IXGBE_LSECTXSCL: u32                                         = 0x08A08; /* SCI Low */
pub const IXGBE_LSECTXSCH: u32                                         = 0x08A0C; /* SCI High */
pub const IXGBE_LSECTXSA: u32                                          = 0x08A10;
pub const IXGBE_LSECTXPN0: u32                                         = 0x08A14;
pub const IXGBE_LSECTXPN1: u32                                         = 0x08A18;

pub fn IXGBE_LSECTXKEY0(n: u32) -> u32 { 0x08A1C + 4 * n } /* 4 of these (0-3) */
pub fn IXGBE_LSECTXKEY1(n: u32) -> u32 { 0x08A2C + 4 * n } /* 4 of these (0-3) */
pub const IXGBE_LSECRXCTRL: u32                                        = 0x08F04;
pub const IXGBE_LSECRXSCL: u32                                         = 0x08F08;
pub const IXGBE_LSECRXSCH: u32                                         = 0x08F0C;

pub fn IXGBE_LSECRXSA(i: u32) -> u32 { 0x08F10 + 4 * i } /* 2 of these (0-1) */
pub fn IXGBE_LSECRXPN(i: u32) -> u32 { 0x08F18 + 4 * i } /* 2 of these (0-1) */
pub fn IXGBE_LSECRXKEY(n: u32, m: u32) -> u32 { 0x08F20 + (0x10 * n + 4 * m) }

pub const IXGBE_LSECTXUT: u32                                          = 0x08A3C; /* OutPktsUntagged */
pub const IXGBE_LSECTXPKTE: u32                                        = 0x08A40; /* OutPktsEncrypted */
pub const IXGBE_LSECTXPKTP: u32                                        = 0x08A44; /* OutPktsProtected */
pub const IXGBE_LSECTXOCTE: u32                                        = 0x08A48; /* OutOctetsEncrypted */
pub const IXGBE_LSECTXOCTP: u32                                        = 0x08A4C; /* OutOctetsProtected */
pub const IXGBE_LSECRXUT: u32                                          = 0x08F40; /* InPktsUntagged/InPktsNoTag */
pub const IXGBE_LSECRXOCTD: u32                                        = 0x08F44; /* InOctetsDecrypted */
pub const IXGBE_LSECRXOCTV: u32                                        = 0x08F48; /* InOctetsValidated */
pub const IXGBE_LSECRXBAD: u32                                         = 0x08F4C; /* InPktsBadTag */
pub const IXGBE_LSECRXNOSCI: u32                                       = 0x08F50; /* InPktsNoSci */
pub const IXGBE_LSECRXUNSCI: u32                                       = 0x08F54; /* InPktsUnknownSci */
pub const IXGBE_LSECRXUNCH: u32                                        = 0x08F58; /* InPktsUnchecked */
pub const IXGBE_LSECRXDELAY: u32                                       = 0x08F5C; /* InPktsDelayed */
pub const IXGBE_LSECRXLATE: u32                                        = 0x08F60; /* InPktsLate */
pub fn IXGBE_LSECRXOK(n: u32) -> u32 { 0x08F64 + 0x04 * n } /* InPktsOk */
pub fn IXGBE_LSECRXINV(n: u32) -> u32 { 0x08F6C + 0x04 * n } /* InPktsInvalid */
pub fn IXGBE_LSECRXNV(n: u32) -> u32 { 0x08F74 + 0x04 * n } /* InPktsNotValid */
pub const IXGBE_LSECRXUNSA: u32                                        = 0x08F7C; /* InPktsUnusedSa */
pub const IXGBE_LSECRXNUSA: u32                                        = 0x08F80; /* InPktsNotUsingSa */

/* LinkSec (MacSec) Bit Fields and Masks */
pub const IXGBE_LSECTXCAP_SUM_MASK: u32                                = 0x00FF0000;
pub const IXGBE_LSECTXCAP_SUM_SHIFT: u32                               = 16;
pub const IXGBE_LSECRXCAP_SUM_MASK: u32                                = 0x00FF0000;
pub const IXGBE_LSECRXCAP_SUM_SHIFT: u32                               = 16;

pub const IXGBE_LSECTXCTRL_EN_MASK: u32                                = 0x00000003;
pub const IXGBE_LSECTXCTRL_DISABLE: u32                                = 0x0;
pub const IXGBE_LSECTXCTRL_AUTH: u32                                   = 0x1;
pub const IXGBE_LSECTXCTRL_AUTH_ENCRYPT: u32                           = 0x2;
pub const IXGBE_LSECTXCTRL_AISCI: u32                                  = 0x00000020;
pub const IXGBE_LSECTXCTRL_PNTHRSH_MASK: u32                           = 0xFFFFFF00;
pub const IXGBE_LSECTXCTRL_RSV_MASK: u32                               = 0x000000D8;

pub const IXGBE_LSECRXCTRL_EN_MASK: u32                                = 0x0000000C;
pub const IXGBE_LSECRXCTRL_EN_SHIFT: u32                               = 2;
pub const IXGBE_LSECRXCTRL_DISABLE: u32                                = 0x0;
pub const IXGBE_LSECRXCTRL_CHECK: u32                                  = 0x1;
pub const IXGBE_LSECRXCTRL_STRICT: u32                                 = 0x2;
pub const IXGBE_LSECRXCTRL_DROP: u32                                   = 0x3;
pub const IXGBE_LSECRXCTRL_PLSH: u32                                   = 0x00000040;
pub const IXGBE_LSECRXCTRL_RP: u32                                     = 0x00000080;
pub const IXGBE_LSECRXCTRL_RSV_MASK: u32                               = 0xFFFFFF33;

/* IpSec Registers */
pub const IXGBE_IPSTXIDX: u32                                          = 0x08900;
pub const IXGBE_IPSTXSALT: u32                                         = 0x08904;

pub fn IXGBE_IPSTXKEY(i: u32) -> u32 { 0x08908 + 4 * i } /* 4 of these (0-3) */
pub const IXGBE_IPSRXIDX: u32                                          = 0x08E00;

pub fn IXGBE_IPSRXIPADDR(i: u32) -> u32 { 0x08E04 + 4 * i } /* 4 of these (0-3) */
pub const IXGBE_IPSRXSPI: u32                                          = 0x08E14;
pub const IXGBE_IPSRXIPIDX: u32                                        = 0x08E18;

pub fn IXGBE_IPSRXKEY(i: u32) -> u32 { 0x08E1C + 4 * i } /* 4 of these (0-3) */
pub const IXGBE_IPSRXSALT: u32                                         = 0x08E2C;
pub const IXGBE_IPSRXMOD: u32                                          = 0x08E30;

pub const IXGBE_SECTXCTRL_STORE_FORWARD_ENABLE: u32                    = 0x4;

/* DCB registers */
pub const IXGBE_RTRPCS: u32                                            = 0x02430;
pub const IXGBE_RTTDCS: u32                                            = 0x04900;
pub const IXGBE_RTTDCS_ARBDIS: u32                                     = 0x00000040; /* DCB arbiter disable */
pub const IXGBE_RTTPCS: u32                                            = 0x0CD00;
pub const IXGBE_RTRUP2TC: u32                                          = 0x03020;
pub const IXGBE_RTTUP2TC: u32                                          = 0x0C800;

pub fn IXGBE_RTRPT4C(i: u32) -> u32 { 0x02140 + i * 4 } /* 8 of these (0-7) */
pub fn IXGBE_TXLLQ(i: u32) -> u32 { 0x082E0 + i * 4 } /* 4 of these (0-3) */
pub fn IXGBE_RTRPT4S(i: u32) -> u32 { 0x02160 + i * 4 } /* 8 of these (0-7) */
pub fn IXGBE_RTTDT2C(i: u32) -> u32 { 0x04910 + i * 4 } /* 8 of these (0-7) */
pub fn IXGBE_RTTDT2S(i: u32) -> u32 { 0x04930 + i * 4 } /* 8 of these (0-7) */
pub fn IXGBE_RTTPT2C(i: u32) -> u32 { 0x0CD20 + i * 4 } /* 8 of these (0-7) */
pub fn IXGBE_RTTPT2S(i: u32) -> u32 { 0x0CD40 + i * 4 } /* 8 of these (0-7) */
pub const IXGBE_RTTDQSEL: u32                                          = 0x04904;
pub const IXGBE_RTTDT1C: u32                                           = 0x04908;
pub const IXGBE_RTTDT1S: u32                                           = 0x0490C;
pub const IXGBE_RTTDTECC: u32                                          = 0x04990;
pub const IXGBE_RTTDTECC_NO_BCN: u32                                   = 0x00000100;

pub const IXGBE_RTTBCNRC: u32                                          = 0x04984;
pub const IXGBE_RTTBCNRC_RS_ENA: u32                                   = 0x80000000;
pub const IXGBE_RTTBCNRC_RF_DEC_MASK: u32                              = 0x00003FFF;
pub const IXGBE_RTTBCNRC_RF_INT_SHIFT: u32                             = 14;
pub const IXGBE_RTTBCNRC_RF_INT_MASK: u32                              = IXGBE_RTTBCNRC_RF_DEC_MASK << IXGBE_RTTBCNRC_RF_INT_SHIFT;
pub const IXGBE_RTTBCNRM: u32                                          = 0x04980;

/* BCN (for DCB) Registers */
pub const IXGBE_RTTBCNRS: u32                                          = 0x04988;
pub const IXGBE_RTTBCNCR: u32                                          = 0x08B00;
pub const IXGBE_RTTBCNACH: u32                                         = 0x08B04;
pub const IXGBE_RTTBCNACL: u32                                         = 0x08B08;
pub const IXGBE_RTTBCNTG: u32                                          = 0x04A90;
pub const IXGBE_RTTBCNIDX: u32                                         = 0x08B0C;
pub const IXGBE_RTTBCNCP: u32                                          = 0x08B10;
pub const IXGBE_RTFRTIMER: u32                                         = 0x08B14;
pub const IXGBE_RTTBCNRTT: u32                                         = 0x05150;
pub const IXGBE_RTTBCNRD: u32                                          = 0x0498C;


/* FCoE DMA Context Registers */
/* FCoE Direct DMA Context */
pub fn IXGBE_FCDDC(i: u32, j: u32) -> u32 { 0x20000 + i * 0x4 + j * 0x10 }

pub const IXGBE_FCPTRL: u32                                            = 0x02410; /* FC User Desc. PTR Low */
pub const IXGBE_FCPTRH: u32                                            = 0x02414; /* FC USer Desc. PTR High */
pub const IXGBE_FCBUFF: u32                                            = 0x02418; /* FC Buffer Control */
pub const IXGBE_FCDMARW: u32                                           = 0x02420; /* FC Receive DMA RW */
pub const IXGBE_FCBUFF_VALID: u32                                      = 1 << 0;   /* DMA Context Valid */
pub const IXGBE_FCBUFF_BUFFSIZE: u32                                   = 3 << 3;   /* User Buffer Size */
pub const IXGBE_FCBUFF_WRCONTX: u32                                    = 1 << 7;   /* 0: Initiator, 1: Target */
pub const IXGBE_FCBUFF_BUFFCNT: u32                                    = 0x0000ff00; /* Number of User Buffers */
pub const IXGBE_FCBUFF_OFFSET: u32                                     = 0xffff0000; /* User Buffer Offset */
pub const IXGBE_FCBUFF_BUFFSIZE_SHIFT: u32                             = 3;
pub const IXGBE_FCBUFF_BUFFCNT_SHIFT: u32                              = 8;
pub const IXGBE_FCBUFF_OFFSET_SHIFT: u32                               = 16;
pub const IXGBE_FCDMARW_WE: u32                                        = 1 << 14;   /* Write enable */
pub const IXGBE_FCDMARW_RE: u32                                        = 1 << 15;   /* Read enable */
pub const IXGBE_FCDMARW_FCOESEL: u32                                   = 0x000001ff;  /* FC X_ID: 11 bits */
pub const IXGBE_FCDMARW_LASTSIZE: u32                                  = 0xffff0000;  /* Last User Buffer Size */
pub const IXGBE_FCDMARW_LASTSIZE_SHIFT: u32                            = 16;
/* FCoE SOF/EOF */
pub const IXGBE_TEOFF: u32                                             = 0x04A94; /* Tx FC EOF */
pub const IXGBE_TSOFF: u32                                             = 0x04A98; /* Tx FC SOF */
pub const IXGBE_REOFF: u32                                             = 0x05158; /* Rx FC EOF */
pub const IXGBE_RSOFF: u32                                             = 0x051F8; /* Rx FC SOF */
/* FCoE Filter Context Registers */
pub const IXGBE_FCD_ID: u32                                            = 0x05114; /* FCoE D_ID */
pub const IXGBE_FCSMAC: u32                                            = 0x0510C; /* FCoE Source MAC */
pub const IXGBE_FCFLTRW_SMAC_HIGH_SHIFT: u32                           = 16;
/* FCoE Direct Filter Context */
pub fn IXGBE_FCDFC(i: u32, j: u32) -> u32 { 0x28000 + i * 0x4 + j * 0x10 }

pub fn IXGBE_FCDFCD(i: u32) -> u32 { 0x30000 + i * 0x4 }

pub const IXGBE_FCFLT: u32                                             = 0x05108; /* FC FLT Context */
pub const IXGBE_FCFLTRW: u32                                           = 0x05110; /* FC Filter RW Control */
pub const IXGBE_FCPARAM: u32                                           = 0x051d8; /* FC Offset Parameter */
pub const IXGBE_FCFLT_VALID: u32                                       = 1 << 0;   /* Filter Context Valid */
pub const IXGBE_FCFLT_FIRST: u32                                       = 1 << 1;   /* Filter First */
pub const IXGBE_FCFLT_SEQID: u32                                       = 0x00ff0000; /* Sequence ID */
pub const IXGBE_FCFLT_SEQCNT: u32                                      = 0xff000000; /* Sequence Count */
pub const IXGBE_FCFLTRW_RVALDT: u32                                    = 1 << 13;  /* Fast Re-Validation */
pub const IXGBE_FCFLTRW_WE: u32                                        = 1 << 14;  /* Write Enable */
pub const IXGBE_FCFLTRW_RE: u32                                        = 1 << 15;  /* Read Enable */
/* FCoE Receive Control */
pub const IXGBE_FCRXCTRL: u32                                          = 0x05100; /* FC Receive Control */
pub const IXGBE_FCRXCTRL_FCOELLI: u32                                  = 1 << 0;   /* Low latency interrupt */
pub const IXGBE_FCRXCTRL_SAVBAD: u32                                   = 1 << 1;   /* Save Bad Frames */
pub const IXGBE_FCRXCTRL_FRSTRDH: u32                                  = 1 << 2;   /* EN 1st Read Header */
pub const IXGBE_FCRXCTRL_LASTSEQH: u32                                 = 1 << 3;   /* EN Last Header in Seq */
pub const IXGBE_FCRXCTRL_ALLH: u32                                     = 1 << 4;   /* EN All Headers */
pub const IXGBE_FCRXCTRL_FRSTSEQH: u32                                 = 1 << 5;   /* EN 1st Seq. Header */
pub const IXGBE_FCRXCTRL_ICRC: u32                                     = 1 << 6;   /* Ignore Bad FC CRC */
pub const IXGBE_FCRXCTRL_FCCRCBO: u32                                  = 1 << 7;   /* FC CRC Byte Ordering */
pub const IXGBE_FCRXCTRL_FCOEVER: u32                                  = 0x00000f00; /* FCoE Version: 4 bits */
pub const IXGBE_FCRXCTRL_FCOEVER_SHIFT: u32                            = 8;
/* FCoE Redirection */
pub const IXGBE_FCRECTL: u32                                           = 0x0ED00; /* FC Redirection Control */
pub const IXGBE_FCRETA0: u32                                           = 0x0ED10; /* FC Redirection Table 0 */
pub fn IXGBE_FCRETA(i: u32) -> u32 { IXGBE_FCRETA0 + i * 4 } /* FCoE Redir */
pub const IXGBE_FCRECTL_ENA: u32                                       = 0x1; /* FCoE Redir Table Enable */
pub const IXGBE_FCRETASEL_ENA: u32                                     = 0x2; /* FCoE FCRETASEL bit */
pub const IXGBE_FCRETA_SIZE: u32                                       = 8; /* Max entries in FCRETA */
pub const IXGBE_FCRETA_ENTRY_MASK: u32                                 = 0x0000007f; /* 7 bits for the queue index */
pub const IXGBE_FCRETA_SIZE_X550: u32                                  = 32; /* Max entries in FCRETA */
/* Higher 7 bits for the queue index */
pub const IXGBE_FCRETA_ENTRY_HIGH_MASK: u32                            = 0x007F0000;
pub const IXGBE_FCRETA_ENTRY_HIGH_SHIFT: u32                           = 16;

/* Stats registers */
pub const IXGBE_CRCERRS: u32                                           = 0x04000;
pub const IXGBE_ILLERRC: u32                                           = 0x04004;
pub const IXGBE_ERRBC: u32                                             = 0x04008;
pub const IXGBE_MSPDC: u32                                             = 0x04010;

pub fn IXGBE_MPC(i: u32) -> u32 { 0x03FA0 + i * 4 } /* 8 of these 3FA0-3FBC*/
pub const IXGBE_MLFC: u32                                              = 0x04034;
pub const IXGBE_MRFC: u32                                              = 0x04038;
pub const IXGBE_RLEC: u32                                              = 0x04040;
pub const IXGBE_LXONTXC: u32                                           = 0x03F60;
pub const IXGBE_LXONRXC: u32                                           = 0x0CF60;
pub const IXGBE_LXOFFTXC: u32                                          = 0x03F68;
pub const IXGBE_LXOFFRXC: u32                                          = 0x0CF68;
pub const IXGBE_LXONRXCNT: u32                                         = 0x041A4;
pub const IXGBE_LXOFFRXCNT: u32                                        = 0x041A8;

pub fn IXGBE_PXONRXCNT(i: u32) -> u32 { 0x04140 + i * 4 } /* 8 of these */
pub fn IXGBE_PXOFFRXCNT(i: u32) -> u32 { 0x04160 + i * 4 } /* 8 of these */
pub fn IXGBE_PXON2OFFCNT(i: u32) -> u32 { 0x03240 + i * 4 } /* 8 of these */
pub fn IXGBE_PXONTXC(i: u32) -> u32 { 0x03F00 + i * 4 } /* 8 of these 3F00-3F1C*/
pub fn IXGBE_PXONRXC(i: u32) -> u32 { 0x0CF00 + i * 4 } /* 8 of these CF00-CF1C*/
pub fn IXGBE_PXOFFTXC(i: u32) -> u32 { 0x03F20 + i * 4 } /* 8 of these 3F20-3F3C*/
pub fn IXGBE_PXOFFRXC(i: u32) -> u32 { 0x0CF20 + i * 4 } /* 8 of these CF20-CF3C*/
pub const IXGBE_PRC64: u32                                             = 0x0405C;
pub const IXGBE_PRC127: u32                                            = 0x04060;
pub const IXGBE_PRC255: u32                                            = 0x04064;
pub const IXGBE_PRC511: u32                                            = 0x04068;
pub const IXGBE_PRC1023: u32                                           = 0x0406C;
pub const IXGBE_PRC1522: u32                                           = 0x04070;
pub const IXGBE_GPRC: u32                                              = 0x04074;
pub const IXGBE_BPRC: u32                                              = 0x04078;
pub const IXGBE_MPRC: u32                                              = 0x0407C;
pub const IXGBE_GPTC: u32                                              = 0x04080;
pub const IXGBE_GORCL: u32                                             = 0x04088;
pub const IXGBE_GORCH: u32                                             = 0x0408C;
pub const IXGBE_GOTCL: u32                                             = 0x04090;
pub const IXGBE_GOTCH: u32                                             = 0x04094;

pub fn IXGBE_RNBC(i: u32) -> u32 { 0x03FC0 + i * 4 } /* 8 of these 3FC0-3FDC*/
pub const IXGBE_RUC: u32                                               = 0x040A4;
pub const IXGBE_RFC: u32                                               = 0x040A8;
pub const IXGBE_ROC: u32                                               = 0x040AC;
pub const IXGBE_RJC: u32                                               = 0x040B0;
pub const IXGBE_MNGPRC: u32                                            = 0x040B4;
pub const IXGBE_MNGPDC: u32                                            = 0x040B8;
pub const IXGBE_MNGPTC: u32                                            = 0x0CF90;
pub const IXGBE_TORL: u32                                              = 0x040C0;
pub const IXGBE_TORH: u32                                              = 0x040C4;
pub const IXGBE_TPR: u32                                               = 0x040D0;
pub const IXGBE_TPT: u32                                               = 0x040D4;
pub const IXGBE_PTC64: u32                                             = 0x040D8;
pub const IXGBE_PTC127: u32                                            = 0x040DC;
pub const IXGBE_PTC255: u32                                            = 0x040E0;
pub const IXGBE_PTC511: u32                                            = 0x040E4;
pub const IXGBE_PTC1023: u32                                           = 0x040E8;
pub const IXGBE_PTC1522: u32                                           = 0x040EC;
pub const IXGBE_MPTC: u32                                              = 0x040F0;
pub const IXGBE_BPTC: u32                                              = 0x040F4;
pub const IXGBE_XEC: u32                                               = 0x04120;
pub const IXGBE_SSVPC: u32                                             = 0x08780;

pub fn IXGBE_RQSMR(i: u32) -> u32 { 0x02300 + i * 4 }

pub fn IXGBE_TQSMR(i: u32) -> u32 { if i <= 7 { 0x07300 + i * 4 } else { 0x08600 + i * 4 } }

pub fn IXGBE_TQSM(i: u32) -> u32 { 0x08600 + i * 4 }

pub fn IXGBE_QPRC(i: u32) -> u32 { 0x01030 + i * 0x40 } /* 16 of these */
pub fn IXGBE_QPTC(i: u32) -> u32 { 0x06030 + i * 0x40 } /* 16 of these */
pub fn IXGBE_QBRC(i: u32) -> u32 { 0x01034 + i * 0x40 } /* 16 of these */
pub fn IXGBE_QBTC(i: u32) -> u32 { 0x06034 + i * 0x40 } /* 16 of these */
pub fn IXGBE_QBRC_L(i: u32) -> u32 { 0x01034 + i * 0x40 } /* 16 of these */
pub fn IXGBE_QBRC_H(i: u32) -> u32 { 0x01038 + i * 0x40 } /* 16 of these */
pub fn IXGBE_QPRDC(i: u32) -> u32 { 0x01430 + i * 0x40 } /* 16 of these */
pub fn IXGBE_QBTC_L(i: u32) -> u32 { 0x08700 + i * 0x8 } /* 16 of these */
pub fn IXGBE_QBTC_H(i: u32) -> u32 { 0x08704 + i * 0x8 } /* 16 of these */
pub const IXGBE_FCCRC: u32                                             = 0x05118; /* Num of Good Eth CRC w/ Bad FC CRC */
pub const IXGBE_FCOERPDC: u32                                          = 0x0241C; /* FCoE Rx Packets Dropped Count */
pub const IXGBE_FCLAST: u32                                            = 0x02424; /* FCoE Last Error Count */
pub const IXGBE_FCOEPRC: u32                                           = 0x02428; /* Number of FCoE Packets Received */
pub const IXGBE_FCOEDWRC: u32                                          = 0x0242C; /* Number of FCoE DWords Received */
pub const IXGBE_FCOEPTC: u32                                           = 0x08784; /* Number of FCoE Packets Transmitted */
pub const IXGBE_FCOEDWTC: u32                                          = 0x08788; /* Number of FCoE DWords Transmitted */
pub const IXGBE_FCCRC_CNT_MASK: u32                                    = 0x0000FFFF; /* CRC_CNT: bit 0 - 15 */
pub const IXGBE_FCLAST_CNT_MASK: u32                                   = 0x0000FFFF; /* Last_CNT: bit 0 - 15 */
pub const IXGBE_O2BGPTC: u32                                           = 0x041C4;
pub const IXGBE_O2BSPC: u32                                            = 0x087B0;
pub const IXGBE_B2OSPC: u32                                            = 0x041C0;
pub const IXGBE_B2OGPRC: u32                                           = 0x02F90;
pub const IXGBE_BUPRC: u32                                             = 0x04180;
pub const IXGBE_BMPRC: u32                                             = 0x04184;
pub const IXGBE_BBPRC: u32                                             = 0x04188;
pub const IXGBE_BUPTC: u32                                             = 0x0418C;
pub const IXGBE_BMPTC: u32                                             = 0x04190;
pub const IXGBE_BBPTC: u32                                             = 0x04194;
pub const IXGBE_BCRCERRS: u32                                          = 0x04198;
pub const IXGBE_BXONRXC: u32                                           = 0x0419C;
pub const IXGBE_BXOFFRXC: u32                                          = 0x041E0;
pub const IXGBE_BXONTXC: u32                                           = 0x041E4;
pub const IXGBE_BXOFFTXC: u32                                          = 0x041E8;

/* Management */
pub fn IXGBE_MAVTV(i: u32) -> u32 { 0x05010 + i * 4 } /* 8 of these (0-7) */
pub fn IXGBE_MFUTP(i: u32) -> u32 { 0x05030 + i * 4 } /* 8 of these (0-7) */
pub const IXGBE_MANC: u32                                              = 0x05820;
pub const IXGBE_MFVAL: u32                                             = 0x05824;
pub const IXGBE_MANC2H: u32                                            = 0x05860;

pub fn IXGBE_MDEF(i: u32) -> u32 { 0x05890 + i * 4 } /* 8 of these (0-7) */
pub const IXGBE_MIPAF: u32                                             = 0x058B0;

pub fn IXGBE_MMAL(i: u32) -> u32 { 0x05910 + i * 8 } /* 4 of these (0-3) */
pub fn IXGBE_MMAH(i: u32) -> u32 { 0x05914 + i * 8 } /* 4 of these (0-3) */
pub const IXGBE_FTFT: u32                                              = 0x09400; /* 0x9400-0x97FC */
pub fn IXGBE_METF(i: u32) -> u32 { 0x05190 + i * 4 } /* 4 of these (0-3) */
pub fn IXGBE_MDEF_EXT(i: u32) -> u32 { 0x05160 + i * 4 } /* 8 of these (0-7) */
pub const IXGBE_LSWFW: u32                                             = 0x15F14;

pub fn IXGBE_BMCIP(i: u32) -> u32 { 0x05050 + i * 4 } /* 0x5050-0x505C */
pub const IXGBE_BMCIPVAL: u32                                          = 0x05060;
pub const IXGBE_BMCIP_IPADDR_TYPE: u32                                 = 0x00000001;
pub const IXGBE_BMCIP_IPADDR_VALID: u32                                = 0x00000002;

/* Management Bit Fields and Masks */
pub const IXGBE_MANC_MPROXYE: u32                                      = 0x40000000; /* Management Proxy Enable */
pub const IXGBE_MANC_RCV_TCO_EN: u32                                   = 0x00020000; /* Rcv TCO packet enable */
pub const IXGBE_MANC_EN_BMC2OS: u32                                    = 0x10000000; /* Ena BMC2OS and OS2BMC traffic */
pub const IXGBE_MANC_EN_BMC2OS_SHIFT: u32                              = 28;

/* Firmware Semaphore Register */
pub const IXGBE_FWSM_MODE_MASK: u32                                    = 0xE;
pub const IXGBE_FWSM_TS_ENABLED: u32                                   = 0x1;
pub const IXGBE_FWSM_FW_MODE_PT: u32                                   = 0x4;

/* ARC Subsystem registers */
pub const IXGBE_HICR: u32                                              = 0x15F00;
pub const IXGBE_FWSTS: u32                                             = 0x15F0C;
pub const IXGBE_HSMC0R: u32                                            = 0x15F04;
pub const IXGBE_HSMC1R: u32                                            = 0x15F08;
pub const IXGBE_SWSR: u32                                              = 0x15F10;
pub const IXGBE_HFDR: u32                                              = 0x15FE8;
pub const IXGBE_FLEX_MNG: u32                                          = 0x15800; /* 0x15800 - 0x15EFC */

pub const IXGBE_HICR_EN: u32                                           = 0x01;  /* Enable bit - RO */
/* Driver sets this bit when done to put command in RAM */
pub const IXGBE_HICR_C: u32                                            = 0x02;
pub const IXGBE_HICR_SV: u32                                           = 0x04;  /* Status Validity */
pub const IXGBE_HICR_FW_RESET_ENABLE: u32                              = 0x40;
pub const IXGBE_HICR_FW_RESET: u32                                     = 0x80;

/* PCI-E registers */
pub const IXGBE_GCR: u32                                               = 0x11000;
pub const IXGBE_GTV: u32                                               = 0x11004;
pub const IXGBE_FUNCTAG: u32                                           = 0x11008;
pub const IXGBE_GLT: u32                                               = 0x1100C;
pub const IXGBE_PCIEPIPEADR: u32                                       = 0x11004;
pub const IXGBE_PCIEPIPEDAT: u32                                       = 0x11008;
pub const IXGBE_GSCL_1: u32                                            = 0x11010;
pub const IXGBE_GSCL_2: u32                                            = 0x11014;
pub const IXGBE_GSCL_1_X540: u32                                       = IXGBE_GSCL_1;
pub const IXGBE_GSCL_2_X540: u32                                       = IXGBE_GSCL_2;
pub const IXGBE_GSCL_3: u32                                            = 0x11018;
pub const IXGBE_GSCL_4: u32                                            = 0x1101C;
pub const IXGBE_GSCN_0: u32                                            = 0x11020;
pub const IXGBE_GSCN_1: u32                                            = 0x11024;
pub const IXGBE_GSCN_2: u32                                            = 0x11028;
pub const IXGBE_GSCN_3: u32                                            = 0x1102C;
pub const IXGBE_GSCN_0_X540: u32                                       = IXGBE_GSCN_0;
pub const IXGBE_GSCN_1_X540: u32                                       = IXGBE_GSCN_1;
pub const IXGBE_GSCN_2_X540: u32                                       = IXGBE_GSCN_2;
pub const IXGBE_GSCN_3_X540: u32                                       = IXGBE_GSCN_3;
pub const IXGBE_FACTPS: u32                                            = 0x10150;
pub const IXGBE_FACTPS_X540: u32                                       = IXGBE_FACTPS;
pub const IXGBE_GSCL_1_X550: u32                                       = 0x11800;
pub const IXGBE_GSCL_2_X550: u32                                       = 0x11804;
pub const IXGBE_GSCL_1_X550EM_x: u32                                   = IXGBE_GSCL_1_X550;
pub const IXGBE_GSCL_2_X550EM_x: u32                                   = IXGBE_GSCL_2_X550;
pub const IXGBE_GSCN_0_X550: u32                                       = 0x11820;
pub const IXGBE_GSCN_1_X550: u32                                       = 0x11824;
pub const IXGBE_GSCN_2_X550: u32                                       = 0x11828;
pub const IXGBE_GSCN_3_X550: u32                                       = 0x1182C;
pub const IXGBE_GSCN_0_X550EM_x: u32                                   = IXGBE_GSCN_0_X550;
pub const IXGBE_GSCN_1_X550EM_x: u32                                   = IXGBE_GSCN_1_X550;
pub const IXGBE_GSCN_2_X550EM_x: u32                                   = IXGBE_GSCN_2_X550;
pub const IXGBE_GSCN_3_X550EM_x: u32                                   = IXGBE_GSCN_3_X550;
pub const IXGBE_FACTPS_X550: u32                                       = IXGBE_FACTPS;
pub const IXGBE_FACTPS_X550EM_x: u32                                   = IXGBE_FACTPS;
pub const IXGBE_GSCL_1_X550EM_a: u32                                   = IXGBE_GSCL_1_X550;
pub const IXGBE_GSCL_2_X550EM_a: u32                                   = IXGBE_GSCL_2_X550;
pub const IXGBE_GSCN_0_X550EM_a: u32                                   = IXGBE_GSCN_0_X550;
pub const IXGBE_GSCN_1_X550EM_a: u32                                   = IXGBE_GSCN_1_X550;
pub const IXGBE_GSCN_2_X550EM_a: u32                                   = IXGBE_GSCN_2_X550;
pub const IXGBE_GSCN_3_X550EM_a: u32                                   = IXGBE_GSCN_3_X550;
pub const IXGBE_FACTPS_X550EM_a: u32                                   = 0x15FEC;

pub fn IXGBE_FACTPS_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_FACTPS) }

pub const IXGBE_PCIEANACTL: u32                                        = 0x11040;
pub const IXGBE_SWSM: u32                                              = 0x10140;
pub const IXGBE_SWSM_X540: u32                                         = IXGBE_SWSM;
pub const IXGBE_SWSM_X550: u32                                         = IXGBE_SWSM;
pub const IXGBE_SWSM_X550EM_x: u32                                     = IXGBE_SWSM;
pub const IXGBE_SWSM_X550EM_a: u32                                     = 0x15F70;

pub fn IXGBE_SWSM_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_SWSM) }

pub const IXGBE_FWSM: u32                                              = 0x10148;
pub const IXGBE_FWSM_X540: u32                                         = IXGBE_FWSM;
pub const IXGBE_FWSM_X550: u32                                         = IXGBE_FWSM;
pub const IXGBE_FWSM_X550EM_x: u32                                     = IXGBE_FWSM;
pub const IXGBE_FWSM_X550EM_a: u32                                     = 0x15F74;

pub fn IXGBE_FWSM_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_FWSM) }

pub const IXGBE_SWFW_SYNC: u32                                         = IXGBE_GSSR;
pub const IXGBE_SWFW_SYNC_X540: u32                                    = IXGBE_SWFW_SYNC;
pub const IXGBE_SWFW_SYNC_X550: u32                                    = IXGBE_SWFW_SYNC;
pub const IXGBE_SWFW_SYNC_X550EM_x: u32                                = IXGBE_SWFW_SYNC;
pub const IXGBE_SWFW_SYNC_X550EM_a: u32                                = 0x15F78;

pub fn IXGBE_SWFW_SYNC_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_SWFW_SYNC) }

pub const IXGBE_GSSR: u32                                              = 0x10160;
pub const IXGBE_MREVID: u32                                            = 0x11064;
pub const IXGBE_DCA_ID: u32                                            = 0x11070;
pub const IXGBE_DCA_CTRL: u32                                          = 0x11074;

/* PCI-E registers 82599-Specific */
pub const IXGBE_GCR_EXT: u32                                           = 0x11050;
pub const IXGBE_GSCL_5_82599: u32                                      = 0x11030;
pub const IXGBE_GSCL_6_82599: u32                                      = 0x11034;
pub const IXGBE_GSCL_7_82599: u32                                      = 0x11038;
pub const IXGBE_GSCL_8_82599: u32                                      = 0x1103C;
pub const IXGBE_GSCL_5_X540: u32                                       = IXGBE_GSCL_5_82599;
pub const IXGBE_GSCL_6_X540: u32                                       = IXGBE_GSCL_6_82599;
pub const IXGBE_GSCL_7_X540: u32                                       = IXGBE_GSCL_7_82599;
pub const IXGBE_GSCL_8_X540: u32                                       = IXGBE_GSCL_8_82599;
pub const IXGBE_PHYADR_82599: u32                                      = 0x11040;
pub const IXGBE_PHYDAT_82599: u32                                      = 0x11044;
pub const IXGBE_PHYCTL_82599: u32                                      = 0x11048;
pub const IXGBE_PBACLR_82599: u32                                      = 0x11068;
pub const IXGBE_CIAA: u32                                              = 0x11088;
pub const IXGBE_CIAD: u32                                              = 0x1108C;
pub const IXGBE_CIAA_82599: u32                                        = IXGBE_CIAA;
pub const IXGBE_CIAD_82599: u32                                        = IXGBE_CIAD;
pub const IXGBE_CIAA_X540: u32                                         = IXGBE_CIAA;
pub const IXGBE_CIAD_X540: u32                                         = IXGBE_CIAD;
pub const IXGBE_GSCL_5_X550: u32                                       = 0x11810;
pub const IXGBE_GSCL_6_X550: u32                                       = 0x11814;
pub const IXGBE_GSCL_7_X550: u32                                       = 0x11818;
pub const IXGBE_GSCL_8_X550: u32                                       = 0x1181C;
pub const IXGBE_GSCL_5_X550EM_x: u32                                   = IXGBE_GSCL_5_X550;
pub const IXGBE_GSCL_6_X550EM_x: u32                                   = IXGBE_GSCL_6_X550;
pub const IXGBE_GSCL_7_X550EM_x: u32                                   = IXGBE_GSCL_7_X550;
pub const IXGBE_GSCL_8_X550EM_x: u32                                   = IXGBE_GSCL_8_X550;
pub const IXGBE_CIAA_X550: u32                                         = 0x11508;
pub const IXGBE_CIAD_X550: u32                                         = 0x11510;
pub const IXGBE_CIAA_X550EM_x: u32                                     = IXGBE_CIAA_X550;
pub const IXGBE_CIAD_X550EM_x: u32                                     = IXGBE_CIAD_X550;
pub const IXGBE_GSCL_5_X550EM_a: u32                                   = IXGBE_GSCL_5_X550;
pub const IXGBE_GSCL_6_X550EM_a: u32                                   = IXGBE_GSCL_6_X550;
pub const IXGBE_GSCL_7_X550EM_a: u32                                   = IXGBE_GSCL_7_X550;
pub const IXGBE_GSCL_8_X550EM_a: u32                                   = IXGBE_GSCL_8_X550;
pub const IXGBE_CIAA_X550EM_a: u32                                     = IXGBE_CIAA_X550;
pub const IXGBE_CIAD_X550EM_a: u32                                     = IXGBE_CIAD_X550;

pub fn IXGBE_CIAA_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_CIAA) }

pub fn IXGBE_CIAD_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_CIAD) }

pub const IXGBE_PICAUSE: u32                                           = 0x110B0;
pub const IXGBE_PIENA: u32                                             = 0x110B8;
pub const IXGBE_CDQ_MBR_82599: u32                                     = 0x110B4;
pub const IXGBE_PCIESPARE: u32                                         = 0x110BC;
pub const IXGBE_MISC_REG_82599: u32                                    = 0x110F0;
pub const IXGBE_ECC_CTRL_0_82599: u32                                  = 0x11100;
pub const IXGBE_ECC_CTRL_1_82599: u32                                  = 0x11104;
pub const IXGBE_ECC_STATUS_82599: u32                                  = 0x110E0;
pub const IXGBE_BAR_CTRL_82599: u32                                    = 0x110F4;

/* PCI Express Control */
pub const IXGBE_GCR_CMPL_TMOUT_MASK: u32                               = 0x0000F000;
pub const IXGBE_GCR_CMPL_TMOUT_10ms: u32                               = 0x00001000;
pub const IXGBE_GCR_CMPL_TMOUT_RESEND: u32                             = 0x00010000;
pub const IXGBE_GCR_CAP_VER2: u32                                      = 0x00040000;

pub const IXGBE_GCR_EXT_MSIX_EN: u32                                   = 0x80000000;
pub const IXGBE_GCR_EXT_BUFFERS_CLEAR: u32                             = 0x40000000;
pub const IXGBE_GCR_EXT_VT_MODE_16: u32                                = 0x00000001;
pub const IXGBE_GCR_EXT_VT_MODE_32: u32                                = 0x00000002;
pub const IXGBE_GCR_EXT_VT_MODE_64: u32                                = 0x00000003;
pub const IXGBE_GCR_EXT_SRIOV: u32                                     = IXGBE_GCR_EXT_MSIX_EN | IXGBE_GCR_EXT_VT_MODE_64;
pub const IXGBE_GCR_EXT_VT_MODE_MASK: u32                              = 0x00000003;
/* Time Sync Registers */
pub const IXGBE_TSYNCRXCTL: u32                                        = 0x05188; /* Rx Time Sync Control register - RW */
pub const IXGBE_TSYNCTXCTL: u32                                        = 0x08C00; /* Tx Time Sync Control register - RW */
pub const IXGBE_RXSTMPL: u32                                           = 0x051E8; /* Rx timestamp Low - RO */
pub const IXGBE_RXSTMPH: u32                                           = 0x051A4; /* Rx timestamp High - RO */
pub const IXGBE_RXSATRL: u32                                           = 0x051A0; /* Rx timestamp attribute low - RO */
pub const IXGBE_RXSATRH: u32                                           = 0x051A8; /* Rx timestamp attribute high - RO */
pub const IXGBE_RXMTRL: u32                                            = 0x05120; /* RX message type register low - RW */
pub const IXGBE_TXSTMPL: u32                                           = 0x08C04; /* Tx timestamp value Low - RO */
pub const IXGBE_TXSTMPH: u32                                           = 0x08C08; /* Tx timestamp value High - RO */
pub const IXGBE_SYSTIML: u32                                           = 0x08C0C; /* System time register Low - RO */
pub const IXGBE_SYSTIMH: u32                                           = 0x08C10; /* System time register High - RO */
pub const IXGBE_SYSTIMR: u32                                           = 0x08C58; /* System time register Residue - RO */
pub const IXGBE_TIMINCA: u32                                           = 0x08C14; /* Increment attributes register - RW */
pub const IXGBE_TIMADJL: u32                                           = 0x08C18; /* Time Adjustment Offset register Low - RW */
pub const IXGBE_TIMADJH: u32                                           = 0x08C1C; /* Time Adjustment Offset register High - RW */
pub const IXGBE_TSAUXC: u32                                            = 0x08C20; /* TimeSync Auxiliary Control register - RW */
pub const IXGBE_TRGTTIML0: u32                                         = 0x08C24; /* Target Time Register 0 Low - RW */
pub const IXGBE_TRGTTIMH0: u32                                         = 0x08C28; /* Target Time Register 0 High - RW */
pub const IXGBE_TRGTTIML1: u32                                         = 0x08C2C; /* Target Time Register 1 Low - RW */
pub const IXGBE_TRGTTIMH1: u32                                         = 0x08C30; /* Target Time Register 1 High - RW */
pub const IXGBE_CLKTIML: u32                                           = 0x08C34; /* Clock Out Time Register Low - RW */
pub const IXGBE_CLKTIMH: u32                                           = 0x08C38; /* Clock Out Time Register High - RW */
pub const IXGBE_FREQOUT0: u32                                          = 0x08C34; /* Frequency Out 0 Control register - RW */
pub const IXGBE_FREQOUT1: u32                                          = 0x08C38; /* Frequency Out 1 Control register - RW */
pub const IXGBE_AUXSTMPL0: u32                                         = 0x08C3C; /* Auxiliary Time Stamp 0 register Low - RO */
pub const IXGBE_AUXSTMPH0: u32                                         = 0x08C40; /* Auxiliary Time Stamp 0 register High - RO */
pub const IXGBE_AUXSTMPL1: u32                                         = 0x08C44; /* Auxiliary Time Stamp 1 register Low - RO */
pub const IXGBE_AUXSTMPH1: u32                                         = 0x08C48; /* Auxiliary Time Stamp 1 register High - RO */
pub const IXGBE_TSIM: u32                                              = 0x08C68; /* TimeSync Interrupt Mask Register - RW */
pub const IXGBE_TSICR: u32                                             = 0x08C60; /* TimeSync Interrupt Cause Register - WO */
pub const IXGBE_TSSDP: u32                                             = 0x0003C; /* TimeSync SDP Configuration Register - RW */

/* Diagnostic Registers */
pub const IXGBE_RDSTATCTL: u32                                         = 0x02C20;

pub fn IXGBE_RDSTAT(i: u32) -> u32 { 0x02C00 + i * 4 } /* 0x02C00-0x02C1C */
pub const IXGBE_RDHMPN: u32                                            = 0x02F08;

pub fn IXGBE_RIC_DW(i: u32) -> u32 { 0x02F10 + i * 4 }

pub const IXGBE_RDPROBE: u32                                           = 0x02F20;
pub const IXGBE_RDMAM: u32                                             = 0x02F30;
pub const IXGBE_RDMAD: u32                                             = 0x02F34;
pub const IXGBE_TDHMPN: u32                                            = 0x07F08;
pub const IXGBE_TDHMPN2: u32                                           = 0x082FC;
pub const IXGBE_TXDESCIC: u32                                          = 0x082CC;

pub fn IXGBE_TIC_DW(i: u32) -> u32 { 0x07F10 + i * 4 }

pub fn IXGBE_TIC_DW2(i: u32) -> u32 { 0x082B0 + i * 4 }

pub const IXGBE_TDPROBE: u32                                           = 0x07F20;
pub const IXGBE_TXBUFCTRL: u32                                         = 0x0C600;
pub const IXGBE_TXBUFDATA0: u32                                        = 0x0C610;
pub const IXGBE_TXBUFDATA1: u32                                        = 0x0C614;
pub const IXGBE_TXBUFDATA2: u32                                        = 0x0C618;
pub const IXGBE_TXBUFDATA3: u32                                        = 0x0C61C;
pub const IXGBE_RXBUFCTRL: u32                                         = 0x03600;
pub const IXGBE_RXBUFDATA0: u32                                        = 0x03610;
pub const IXGBE_RXBUFDATA1: u32                                        = 0x03614;
pub const IXGBE_RXBUFDATA2: u32                                        = 0x03618;
pub const IXGBE_RXBUFDATA3: u32                                        = 0x0361C;

pub fn IXGBE_PCIE_DIAG(i: u32) -> u32 { 0x11090 + i * 4 } /* 8 of these */
pub const IXGBE_RFVAL: u32                                             = 0x050A4;
pub const IXGBE_MDFTC1: u32                                            = 0x042B8;
pub const IXGBE_MDFTC2: u32                                            = 0x042C0;
pub const IXGBE_MDFTFIFO1: u32                                         = 0x042C4;
pub const IXGBE_MDFTFIFO2: u32                                         = 0x042C8;
pub const IXGBE_MDFTS: u32                                             = 0x042CC;

pub fn IXGBE_RXDATAWRPTR(i: u32) -> u32 { 0x03700 + i * 4 } /* 8 of these 3700-370C */
pub fn IXGBE_RXDESCWRPTR(i: u32) -> u32 { 0x03710 + i * 4 } /* 8 of these 3710-371C */
pub fn IXGBE_RXDATARDPTR(i: u32) -> u32 { 0x03720 + i * 4 } /* 8 of these 3720-372C */
pub fn IXGBE_RXDESCRDPTR(i: u32) -> u32 { 0x03730 + i * 4 } /* 8 of these 3730-373C */
pub fn IXGBE_TXDATAWRPTR(i: u32) -> u32 { 0x0C700 + i * 4 } /* 8 of these C700-C70C */
pub fn IXGBE_TXDESCWRPTR(i: u32) -> u32 { 0x0C710 + i * 4 } /* 8 of these C710-C71C */
pub fn IXGBE_TXDATARDPTR(i: u32) -> u32 { 0x0C720 + i * 4 } /* 8 of these C720-C72C */
pub fn IXGBE_TXDESCRDPTR(i: u32) -> u32 { 0x0C730 + i * 4 } /* 8 of these C730-C73C */
pub const IXGBE_PCIEECCCTL: u32                                        = 0x1106C;

pub fn IXGBE_RXWRPTR(i: u32) -> u32 { 0x03100 + i * 4 } /* 8 of these 3100-310C */
pub fn IXGBE_RXUSED(i: u32) -> u32 { 0x03120 + i * 4 } /* 8 of these 3120-312C */
pub fn IXGBE_RXRDPTR(i: u32) -> u32 { 0x03140 + i * 4 } /* 8 of these 3140-314C */
pub fn IXGBE_RXRDWRPTR(i: u32) -> u32 { 0x03160 + i * 4 } /* 8 of these 3160-310C */
pub fn IXGBE_TXWRPTR(i: u32) -> u32 { 0x0C100 + i * 4 } /* 8 of these C100-C10C */
pub fn IXGBE_TXUSED(i: u32) -> u32 { 0x0C120 + i * 4 } /* 8 of these C120-C12C */
pub fn IXGBE_TXRDPTR(i: u32) -> u32 { 0x0C140 + i * 4 } /* 8 of these C140-C14C */
pub fn IXGBE_TXRDWRPTR(i: u32) -> u32 { 0x0C160 + i * 4 } /* 8 of these C160-C10C */
pub const IXGBE_PCIEECCCTL0: u32                                       = 0x11100;
pub const IXGBE_PCIEECCCTL1: u32                                       = 0x11104;
pub const IXGBE_RXDBUECC: u32                                          = 0x03F70;
pub const IXGBE_TXDBUECC: u32                                          = 0x0CF70;
pub const IXGBE_RXDBUEST: u32                                          = 0x03F74;
pub const IXGBE_TXDBUEST: u32                                          = 0x0CF74;
pub const IXGBE_PBTXECC: u32                                           = 0x0C300;
pub const IXGBE_PBRXECC: u32                                           = 0x03300;
pub const IXGBE_GHECCR: u32                                            = 0x110B0;

/* MAC Registers */
pub const IXGBE_PCS1GCFIG: u32                                         = 0x04200;
pub const IXGBE_PCS1GLCTL: u32                                         = 0x04208;
pub const IXGBE_PCS1GLSTA: u32                                         = 0x0420C;
pub const IXGBE_PCS1GDBG0: u32                                         = 0x04210;
pub const IXGBE_PCS1GDBG1: u32                                         = 0x04214;
pub const IXGBE_PCS1GANA: u32                                          = 0x04218;
pub const IXGBE_PCS1GANLP: u32                                         = 0x0421C;
pub const IXGBE_PCS1GANNP: u32                                         = 0x04220;
pub const IXGBE_PCS1GANLPNP: u32                                       = 0x04224;
pub const IXGBE_HLREG0: u32                                            = 0x04240;
pub const IXGBE_HLREG1: u32                                            = 0x04244;
pub const IXGBE_PAP: u32                                               = 0x04248;
pub const IXGBE_MACA: u32                                              = 0x0424C;
pub const IXGBE_APAE: u32                                              = 0x04250;
pub const IXGBE_ARD: u32                                               = 0x04254;
pub const IXGBE_AIS: u32                                               = 0x04258;
pub const IXGBE_MSCA: u32                                              = 0x0425C;
pub const IXGBE_MSRWD: u32                                             = 0x04260;
pub const IXGBE_MLADD: u32                                             = 0x04264;
pub const IXGBE_MHADD: u32                                             = 0x04268;
pub const IXGBE_MAXFRS: u32                                            = 0x04268;
pub const IXGBE_TREG: u32                                              = 0x0426C;
pub const IXGBE_PCSS1: u32                                             = 0x04288;
pub const IXGBE_PCSS2: u32                                             = 0x0428C;
pub const IXGBE_XPCSS: u32                                             = 0x04290;
pub const IXGBE_MFLCN: u32                                             = 0x04294;
pub const IXGBE_SERDESC: u32                                           = 0x04298;
pub const IXGBE_MAC_SGMII_BUSY: u32                                    = 0x04298;
pub const IXGBE_MACS: u32                                              = 0x0429C;
pub const IXGBE_AUTOC: u32                                             = 0x042A0;
pub const IXGBE_LINKS: u32                                             = 0x042A4;
pub const IXGBE_LINKS2: u32                                            = 0x04324;
pub const IXGBE_AUTOC2: u32                                            = 0x042A8;
pub const IXGBE_AUTOC3: u32                                            = 0x042AC;
pub const IXGBE_ANLP1: u32                                             = 0x042B0;
pub const IXGBE_ANLP2: u32                                             = 0x042B4;
pub const IXGBE_MACC: u32                                              = 0x04330;
pub const IXGBE_ATLASCTL: u32                                          = 0x04800;
pub const IXGBE_MMNGC: u32                                             = 0x042D0;
pub const IXGBE_ANLPNP1: u32                                           = 0x042D4;
pub const IXGBE_ANLPNP2: u32                                           = 0x042D8;
pub const IXGBE_KRPCSFC: u32                                           = 0x042E0;
pub const IXGBE_KRPCSS: u32                                            = 0x042E4;
pub const IXGBE_FECS1: u32                                             = 0x042E8;
pub const IXGBE_FECS2: u32                                             = 0x042EC;
pub const IXGBE_SMADARCTL: u32                                         = 0x14F10;
pub const IXGBE_MPVC: u32                                              = 0x04318;
pub const IXGBE_SGMIIC: u32                                            = 0x04314;

/* Statistics Registers */
pub const IXGBE_RXNFGPC: u32                                           = 0x041B0;
pub const IXGBE_RXNFGBCL: u32                                          = 0x041B4;
pub const IXGBE_RXNFGBCH: u32                                          = 0x041B8;
pub const IXGBE_RXDGPC: u32                                            = 0x02F50;
pub const IXGBE_RXDGBCL: u32                                           = 0x02F54;
pub const IXGBE_RXDGBCH: u32                                           = 0x02F58;
pub const IXGBE_RXDDGPC: u32                                           = 0x02F5C;
pub const IXGBE_RXDDGBCL: u32                                          = 0x02F60;
pub const IXGBE_RXDDGBCH: u32                                          = 0x02F64;
pub const IXGBE_RXLPBKGPC: u32                                         = 0x02F68;
pub const IXGBE_RXLPBKGBCL: u32                                        = 0x02F6C;
pub const IXGBE_RXLPBKGBCH: u32                                        = 0x02F70;
pub const IXGBE_RXDLPBKGPC: u32                                        = 0x02F74;
pub const IXGBE_RXDLPBKGBCL: u32                                       = 0x02F78;
pub const IXGBE_RXDLPBKGBCH: u32                                       = 0x02F7C;
pub const IXGBE_TXDGPC: u32                                            = 0x087A0;
pub const IXGBE_TXDGBCL: u32                                           = 0x087A4;
pub const IXGBE_TXDGBCH: u32                                           = 0x087A8;

pub const IXGBE_RXDSTATCTRL: u32                                       = 0x02F40;

/* Copper Pond 2 link timeout */
pub const IXGBE_VALIDATE_LINK_READY_TIMEOUT: u32                       = 50;

/* Omer CORECTL */
pub const IXGBE_CORECTL: u32                                           = 0x014F00;
/* BARCTRL */
pub const IXGBE_BARCTRL: u32                                           = 0x110F4;
pub const IXGBE_BARCTRL_FLSIZE: u32                                    = 0x0700;
pub const IXGBE_BARCTRL_FLSIZE_SHIFT: u32                              = 8;
pub const IXGBE_BARCTRL_CSRSIZE: u32                                   = 0x2000;

/* RSCCTL Bit Masks */
pub const IXGBE_RSCCTL_RSCEN: u32                                      = 0x01;
pub const IXGBE_RSCCTL_MAXDESC_1: u32                                  = 0x00;
pub const IXGBE_RSCCTL_MAXDESC_4: u32                                  = 0x04;
pub const IXGBE_RSCCTL_MAXDESC_8: u32                                  = 0x08;
pub const IXGBE_RSCCTL_MAXDESC_16: u32                                 = 0x0C;
pub const IXGBE_RSCCTL_TS_DIS: u32                                     = 0x02;

/* RSCDBU Bit Masks */
pub const IXGBE_RSCDBU_RSCSMALDIS_MASK: u32                            = 0x0000007F;
pub const IXGBE_RSCDBU_RSCACKDIS: u32                                  = 0x00000080;

/* RDRXCTL Bit Masks */
pub const IXGBE_RDRXCTL_RDMTS_1_2: u32                                 = 0x00000000; /* Rx Desc Min THLD Size */
pub const IXGBE_RDRXCTL_CRCSTRIP: u32                                  = 0x00000002; /* CRC Strip */
pub const IXGBE_RDRXCTL_PSP: u32                                       = 0x00000004; /* Pad Small Packet */
pub const IXGBE_RDRXCTL_MVMEN: u32                                     = 0x00000020;
pub const IXGBE_RDRXCTL_RSC_PUSH_DIS: u32                              = 0x00000020;
pub const IXGBE_RDRXCTL_DMAIDONE: u32                                  = 0x00000008; /* DMA init cycle done */
pub const IXGBE_RDRXCTL_RSC_PUSH: u32                                  = 0x00000080;
pub const IXGBE_RDRXCTL_AGGDIS: u32                                    = 0x00010000; /* Aggregation disable */
pub const IXGBE_RDRXCTL_RSCFRSTSIZE: u32                               = 0x003E0000; /* RSC First packet size */
pub const IXGBE_RDRXCTL_RSCLLIDIS: u32                                 = 0x00800000; /* Disable RSC compl on LLI*/
pub const IXGBE_RDRXCTL_RSCACKC: u32                                   = 0x02000000; /* must set 1 when RSC ena */
pub const IXGBE_RDRXCTL_FCOE_WRFIX: u32                                = 0x04000000; /* must set 1 when RSC ena */
pub const IXGBE_RDRXCTL_MBINTEN: u32                                   = 0x10000000;
pub const IXGBE_RDRXCTL_MDP_EN: u32                                    = 0x20000000;

/* RQTC Bit Masks and Shifts */
pub fn IXGBE_RQTC_SHIFT_TC(i: u32) -> u32 { i * 4 }

pub const IXGBE_RQTC_TC0_MASK: u32                                     = 0x7 << 0;
pub const IXGBE_RQTC_TC1_MASK: u32                                     = 0x7 << 4;
pub const IXGBE_RQTC_TC2_MASK: u32                                     = 0x7 << 8;
pub const IXGBE_RQTC_TC3_MASK: u32                                     = 0x7 << 12;
pub const IXGBE_RQTC_TC4_MASK: u32                                     = 0x7 << 16;
pub const IXGBE_RQTC_TC5_MASK: u32                                     = 0x7 << 20;
pub const IXGBE_RQTC_TC6_MASK: u32                                     = 0x7 << 24;
pub const IXGBE_RQTC_TC7_MASK: u32                                     = 0x7 << 28;

/* PSRTYPE.RQPL Bit masks and shift */
pub const IXGBE_PSRTYPE_RQPL_MASK: u32                                 = 0x7;
pub const IXGBE_PSRTYPE_RQPL_SHIFT: u32                                = 29;

/* CTRL Bit Masks */
pub const IXGBE_CTRL_GIO_DIS: u32                                      = 0x00000004; /* Global IO Master Disable bit */
pub const IXGBE_CTRL_LNK_RST: u32                                      = 0x00000008; /* Link Reset. Resets everything. */
pub const IXGBE_CTRL_RST: u32                                          = 0x04000000; /* Reset (SW) */
pub const IXGBE_CTRL_RST_MASK: u32                                     = IXGBE_CTRL_LNK_RST | IXGBE_CTRL_RST;

/* FACTPS */
pub const IXGBE_FACTPS_MNGCG: u32                                      = 0x20000000; /* Manageblility Clock Gated */
pub const IXGBE_FACTPS_LFS: u32                                        = 0x40000000; /* LAN Function Select */

/* MHADD Bit Masks */
pub const IXGBE_MHADD_MFS_MASK: u32                                    = 0xFFFF0000;
pub const IXGBE_MHADD_MFS_SHIFT: u32                                   = 16;

/* Extended Device Control */
pub const IXGBE_CTRL_EXT_PFRSTD: u32                                   = 0x00004000; /* Physical Function Reset Done */
pub const IXGBE_CTRL_EXT_NS_DIS: u32                                   = 0x00010000; /* No Snoop disable */
pub const IXGBE_CTRL_EXT_RO_DIS: u32                                   = 0x00020000; /* Relaxed Ordering disable */
pub const IXGBE_CTRL_EXT_DRV_LOAD: u32                                 = 0x10000000; /* Driver loaded bit for FW */

/* Direct Cache Access (DCA) definitions */
pub const IXGBE_DCA_CTRL_DCA_ENABLE: u32                               = 0x00000000; /* DCA Enable */
pub const IXGBE_DCA_CTRL_DCA_DISABLE: u32                              = 0x00000001; /* DCA Disable */

pub const IXGBE_DCA_CTRL_DCA_MODE_CB1: u32                             = 0x00; /* DCA Mode CB1 */
pub const IXGBE_DCA_CTRL_DCA_MODE_CB2: u32                             = 0x02; /* DCA Mode CB2 */

pub const IXGBE_DCA_RXCTRL_CPUID_MASK: u32                             = 0x0000001F; /* Rx CPUID Mask */
pub const IXGBE_DCA_RXCTRL_CPUID_MASK_82599: u32                       = 0xFF000000; /* Rx CPUID Mask */
pub const IXGBE_DCA_RXCTRL_CPUID_SHIFT_82599: u32                      = 24; /* Rx CPUID Shift */
pub const IXGBE_DCA_RXCTRL_DESC_DCA_EN: u32                            = 1 << 5; /* Rx Desc enable */
pub const IXGBE_DCA_RXCTRL_HEAD_DCA_EN: u32                            = 1 << 6; /* Rx Desc header ena */
pub const IXGBE_DCA_RXCTRL_DATA_DCA_EN: u32                            = 1 << 7; /* Rx Desc payload ena */
pub const IXGBE_DCA_RXCTRL_DESC_RRO_EN: u32                            = 1 << 9; /* Rx rd Desc Relax Order */
pub const IXGBE_DCA_RXCTRL_DATA_WRO_EN: u32                            = 1 << 13; /* Rx wr data Relax Order */
pub const IXGBE_DCA_RXCTRL_HEAD_WRO_EN: u32                            = 1 << 15; /* Rx wr header RO */

pub const IXGBE_DCA_TXCTRL_CPUID_MASK: u32                             = 0x0000001F; /* Tx CPUID Mask */
pub const IXGBE_DCA_TXCTRL_CPUID_MASK_82599: u32                       = 0xFF000000; /* Tx CPUID Mask */
pub const IXGBE_DCA_TXCTRL_CPUID_SHIFT_82599: u32                      = 24; /* Tx CPUID Shift */
pub const IXGBE_DCA_TXCTRL_DESC_DCA_EN: u32                            = 1 << 5; /* DCA Tx Desc enable */
pub const IXGBE_DCA_TXCTRL_DESC_RRO_EN: u32                            = 1 << 9; /* Tx rd Desc Relax Order */
pub const IXGBE_DCA_TXCTRL_DESC_WRO_EN: u32                            = 1 << 11; /* Tx Desc writeback RO bit */
pub const IXGBE_DCA_TXCTRL_DATA_RRO_EN: u32                            = 1 << 13; /* Tx rd data Relax Order */
pub const IXGBE_DCA_MAX_QUEUES_82598: u32                              = 16; /* DCA regs only on 16 queues */

/* MSCA Bit Masks */
pub const IXGBE_MSCA_NP_ADDR_MASK: u32                                 = 0x0000FFFF; /* MDI Addr (new prot) */
pub const IXGBE_MSCA_NP_ADDR_SHIFT: u32                                = 0;
pub const IXGBE_MSCA_DEV_TYPE_MASK: u32                                = 0x001F0000; /* Dev Type (new prot) */
pub const IXGBE_MSCA_DEV_TYPE_SHIFT: u32                               = 16; /* Register Address (old prot */
pub const IXGBE_MSCA_PHY_ADDR_MASK: u32                                = 0x03E00000; /* PHY Address mask */
pub const IXGBE_MSCA_PHY_ADDR_SHIFT: u32                               = 21; /* PHY Address shift*/
pub const IXGBE_MSCA_OP_CODE_MASK: u32                                 = 0x0C000000; /* OP CODE mask */
pub const IXGBE_MSCA_OP_CODE_SHIFT: u32                                = 26; /* OP CODE shift */
pub const IXGBE_MSCA_ADDR_CYCLE: u32                                   = 0x00000000; /* OP CODE 00 (addr cycle) */
pub const IXGBE_MSCA_WRITE: u32                                        = 0x04000000; /* OP CODE 01 (wr) */
pub const IXGBE_MSCA_READ: u32                                         = 0x0C000000; /* OP CODE 11 (rd) */
pub const IXGBE_MSCA_READ_AUTOINC: u32                                 = 0x08000000; /* OP CODE 10 (rd auto inc)*/
pub const IXGBE_MSCA_ST_CODE_MASK: u32                                 = 0x30000000; /* ST Code mask */
pub const IXGBE_MSCA_ST_CODE_SHIFT: u32                                = 28; /* ST Code shift */
pub const IXGBE_MSCA_NEW_PROTOCOL: u32                                 = 0x00000000; /* ST CODE 00 (new prot) */
pub const IXGBE_MSCA_OLD_PROTOCOL: u32                                 = 0x10000000; /* ST CODE 01 (old prot) */
pub const IXGBE_MSCA_MDI_COMMAND: u32                                  = 0x40000000; /* Initiate MDI command */
pub const IXGBE_MSCA_MDI_IN_PROG_EN: u32                               = 0x80000000; /* MDI in progress ena */

/* MSRWD bit masks */
pub const IXGBE_MSRWD_WRITE_DATA_MASK: u32                             = 0x0000FFFF;
pub const IXGBE_MSRWD_WRITE_DATA_SHIFT: u32                            = 0;
pub const IXGBE_MSRWD_READ_DATA_MASK: u32                              = 0xFFFF0000;
pub const IXGBE_MSRWD_READ_DATA_SHIFT: u32                             = 16;

/* Atlas registers */
pub const IXGBE_ATLAS_PDN_LPBK: u32                                    = 0x24;
pub const IXGBE_ATLAS_PDN_10G: u32                                     = 0xB;
pub const IXGBE_ATLAS_PDN_1G: u32                                      = 0xC;
pub const IXGBE_ATLAS_PDN_AN: u32                                      = 0xD;

/* Atlas bit masks */
pub const IXGBE_ATLASCTL_WRITE_CMD: u32                                = 0x00010000;
pub const IXGBE_ATLAS_PDN_TX_REG_EN: u32                               = 0x10;
pub const IXGBE_ATLAS_PDN_TX_10G_QL_ALL: u32                           = 0xF0;
pub const IXGBE_ATLAS_PDN_TX_1G_QL_ALL: u32                            = 0xF0;
pub const IXGBE_ATLAS_PDN_TX_AN_QL_ALL: u32                            = 0xF0;

/* Omer bit masks */
pub const IXGBE_CORECTL_WRITE_CMD: u32                                 = 0x00010000;

/* Device Type definitions for new protocol MDIO commands */
pub const IXGBE_MDIO_ZERO_DEV_TYPE: u32                                = 0x0;
pub const IXGBE_MDIO_PMA_PMD_DEV_TYPE: u32                             = 0x1;
pub const IXGBE_MDIO_PCS_DEV_TYPE: u32                                 = 0x3;
pub const IXGBE_MDIO_PHY_XS_DEV_TYPE: u32                              = 0x4;
pub const IXGBE_MDIO_AUTO_NEG_DEV_TYPE: u32                            = 0x7;
pub const IXGBE_MDIO_VENDOR_SPECIFIC_1_DEV_TYPE: u32                   = 0x1E;   /* Device 30 */
pub const IXGBE_TWINAX_DEV: u32                                        = 1;

pub const IXGBE_MDIO_COMMAND_TIMEOUT: u32                              = 100; /* PHY Timeout for 1 GB mode */

pub const IXGBE_MDIO_VENDOR_SPECIFIC_1_CONTROL: u32                    = 0x0; /* VS1 Ctrl Reg */
pub const IXGBE_MDIO_VENDOR_SPECIFIC_1_STATUS: u32                     = 0x1; /* VS1 Status Reg */
pub const IXGBE_MDIO_VENDOR_SPECIFIC_1_LINK_STATUS: u32 = 0x0008; /* 1 = Link Up */
pub const IXGBE_MDIO_VENDOR_SPECIFIC_1_SPEED_STATUS: u32               = 0x0010; /* 0-10G, 1-1G */
pub const IXGBE_MDIO_VENDOR_SPECIFIC_1_10G_SPEED: u32                  = 0x0018;
pub const IXGBE_MDIO_VENDOR_SPECIFIC_1_1G_SPEED: u32                   = 0x0010;

pub const IXGBE_MDIO_AUTO_NEG_CONTROL: u32                             = 0x0; /* AUTO_NEG Control Reg */
pub const IXGBE_MDIO_AUTO_NEG_STATUS: u32                              = 0x1; /* AUTO_NEG Status Reg */
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STAT: u32                         = 0xC800; /* AUTO_NEG Vendor Status Reg */
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_TX_ALARM: u32                     = 0xCC00; /* AUTO_NEG Vendor TX Reg */
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_TX_ALARM2: u32                    = 0xCC01; /* AUTO_NEG Vendor Tx Reg */
pub const IXGBE_MDIO_AUTO_NEG_VEN_LSC: u32                             = 0x1; /* AUTO_NEG Vendor Tx LSC */
pub const IXGBE_MDIO_AUTO_NEG_ADVT: u32                                = 0x10; /* AUTO_NEG Advt Reg */
pub const IXGBE_MDIO_AUTO_NEG_LP: u32                                  = 0x13; /* AUTO_NEG LP Status Reg */
pub const IXGBE_MDIO_AUTO_NEG_EEE_ADVT: u32                            = 0x3C; /* AUTO_NEG EEE Advt Reg */
pub const IXGBE_AUTO_NEG_10GBASE_EEE_ADVT: u32                         = 0x8;  /* AUTO NEG EEE 10GBaseT Advt */
pub const IXGBE_AUTO_NEG_1000BASE_EEE_ADVT: u32                        = 0x4;  /* AUTO NEG EEE 1000BaseT Advt */
pub const IXGBE_AUTO_NEG_100BASE_EEE_ADVT: u32                         = 0x2;  /* AUTO NEG EEE 100BaseT Advt */
pub const IXGBE_MDIO_PHY_XS_CONTROL: u32                               = 0x0; /* PHY_XS Control Reg */
pub const IXGBE_MDIO_PHY_XS_RESET: u32                                 = 0x8000; /* PHY_XS Reset */
pub const IXGBE_MDIO_PHY_ID_HIGH: u32                                  = 0x2; /* PHY ID High Reg*/
pub const IXGBE_MDIO_PHY_ID_LOW: u32                                   = 0x3; /* PHY ID Low Reg*/
pub const IXGBE_MDIO_PHY_SPEED_ABILITY: u32                            = 0x4; /* Speed Ability Reg */
pub const IXGBE_MDIO_PHY_SPEED_10G: u32                                = 0x0001; /* 10G capable */
pub const IXGBE_MDIO_PHY_SPEED_1G: u32                                 = 0x0010; /* 1G capable */
pub const IXGBE_MDIO_PHY_SPEED_100M: u32                               = 0x0020; /* 100M capable */
pub const IXGBE_MDIO_PHY_EXT_ABILITY: u32                              = 0xB; /* Ext Ability Reg */
pub const IXGBE_MDIO_PHY_10GBASET_ABILITY: u32                         = 0x0004; /* 10GBaseT capable */
pub const IXGBE_MDIO_PHY_1000BASET_ABILITY: u32                        = 0x0020; /* 1000BaseT capable */
pub const IXGBE_MDIO_PHY_100BASETX_ABILITY: u32                        = 0x0080; /* 100BaseTX capable */
pub const IXGBE_MDIO_PHY_SET_LOW_POWER_MODE: u32                       = 0x0800; /* Set low power mode */
pub const IXGBE_AUTO_NEG_LP_STATUS: u32                                = 0xE820; /* AUTO NEG Rx LP Status Reg */
pub const IXGBE_AUTO_NEG_LP_1000BASE_CAP: u32                          = 0x8000; /* AUTO NEG Rx LP 1000BaseT Cap */
pub const IXGBE_AUTO_NEG_LP_10GBASE_CAP: u32                           = 0x0800; /* AUTO NEG Rx LP 10GBaseT Cap */
pub const IXGBE_AUTO_NEG_10GBASET_STAT: u32                            = 0x0021; /* AUTO NEG 10G BaseT Stat */

pub const IXGBE_MDIO_TX_VENDOR_ALARMS_3: u32                           = 0xCC02; /* Vendor Alarms 3 Reg */
pub const IXGBE_MDIO_TX_VENDOR_ALARMS_3_RST_MASK: u32                  = 0x3; /* PHY Reset Complete Mask */
pub const IXGBE_MDIO_GLOBAL_RES_PR_10: u32                             = 0xC479; /* Global Resv Provisioning 10 Reg */
pub const IXGBE_MDIO_POWER_UP_STALL: u32                               = 0x8000; /* Power Up Stall */
pub const IXGBE_MDIO_GLOBAL_INT_CHIP_STD_MASK: u32                     = 0xFF00; /* int std mask */
pub const IXGBE_MDIO_GLOBAL_CHIP_STD_INT_FLAG: u32                     = 0xFC00; /* chip std int flag */
pub const IXGBE_MDIO_GLOBAL_INT_CHIP_VEN_MASK: u32                     = 0xFF01; /* int chip-wide mask */
pub const IXGBE_MDIO_GLOBAL_INT_CHIP_VEN_FLAG: u32                     = 0xFC01; /* int chip-wide mask */
pub const IXGBE_MDIO_GLOBAL_ALARM_1: u32                               = 0xCC00; /* Global alarm 1 */
pub const IXGBE_MDIO_GLOBAL_ALM_1_DEV_FAULT: u32                       = 0x0010; /* device fault */
pub const IXGBE_MDIO_GLOBAL_ALM_1_HI_TMP_FAIL: u32                     = 0x4000; /* high temp failure */
pub const IXGBE_MDIO_GLOBAL_FAULT_MSG: u32                             = 0xC850; /* Global Fault Message */
pub const IXGBE_MDIO_GLOBAL_FAULT_MSG_HI_TMP: u32                      = 0x8007; /* high temp failure */
pub const IXGBE_MDIO_GLOBAL_INT_MASK: u32                              = 0xD400; /* Global int mask */
pub const IXGBE_MDIO_GLOBAL_AN_VEN_ALM_INT_EN: u32                     = 0x1000; /* autoneg vendor alarm int enable */
pub const IXGBE_MDIO_GLOBAL_ALARM_1_INT: u32                           = 0x4; /* int in Global alarm 1 */
pub const IXGBE_MDIO_GLOBAL_VEN_ALM_INT_EN: u32                        = 0x1; /* vendor alarm int enable */
pub const IXGBE_MDIO_GLOBAL_STD_ALM2_INT: u32                          = 0x200; /* vendor alarm2 int mask */
pub const IXGBE_MDIO_GLOBAL_INT_HI_TEMP_EN: u32                        = 0x4000; /* int high temp enable */
pub const IXGBE_MDIO_GLOBAL_INT_DEV_FAULT_EN: u32                      = 0x0010; /* int dev fault enable */
pub const IXGBE_MDIO_PMA_PMD_CONTROL_ADDR: u32                         = 0x0000; /* PMA/PMD Control Reg */
pub const IXGBE_MDIO_PMA_PMD_SDA_SCL_ADDR: u32                         = 0xC30A; /* PHY_XS SDA/SCL Addr Reg */
pub const IXGBE_MDIO_PMA_PMD_SDA_SCL_DATA: u32                         = 0xC30B; /* PHY_XS SDA/SCL Data Reg */
pub const IXGBE_MDIO_PMA_PMD_SDA_SCL_STAT: u32                         = 0xC30C; /* PHY_XS SDA/SCL Status Reg */
pub const IXGBE_MDIO_PMA_TX_VEN_LASI_INT_MASK: u32                     = 0xD401; /* PHY TX Vendor LASI */
pub const IXGBE_MDIO_PMA_TX_VEN_LASI_INT_EN: u32                       = 0x1; /* PHY TX Vendor LASI enable */
pub const IXGBE_MDIO_PMD_STD_TX_DISABLE_CNTR: u32                      = 0x9; /* Standard Transmit Dis Reg */
pub const IXGBE_MDIO_PMD_GLOBAL_TX_DISABLE: u32                        = 0x0001; /* PMD Global Transmit Dis */

pub const IXGBE_PCRC8ECL: u32                                          = 0x0E810; /* PCR CRC-8 Error Count Lo */
pub const IXGBE_PCRC8ECH: u32                                          = 0x0E811; /* PCR CRC-8 Error Count Hi */
pub const IXGBE_PCRC8ECH_MASK: u32                                     = 0x1F;
pub const IXGBE_LDPCECL: u32                                           = 0x0E820; /* PCR Uncorrected Error Count Lo */
pub const IXGBE_LDPCECH: u32                                           = 0x0E821; /* PCR Uncorrected Error Count Hi */

/* MII clause 22/28 definitions */
pub const IXGBE_MDIO_PHY_LOW_POWER_MODE: u32                           = 0x0800;

pub const IXGBE_MDIO_XENPAK_LASI_STATUS: u32                           = 0x9005; /* XENPAK LASI Status register*/
pub const IXGBE_XENPAK_LASI_LINK_STATUS_ALARM: u32                     = 0x1; /* Link Status Alarm change */

pub const IXGBE_MDIO_AUTO_NEG_LINK_STATUS: u32                         = 0x4; /* Indicates if link is up */

pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_MASK: u32                  = 0x7; /* Speed/Duplex Mask */
pub const IXGBE_MDIO_AUTO_NEG_VEN_STAT_SPEED_MASK: u32                 = 0x6; /* Speed Mask */
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_10M_HALF: u32              = 0x0; /* 10Mb/s Half Duplex */
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_10M_FULL: u32              = 0x1; /* 10Mb/s Full Duplex */
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_100M_HALF: u32             = 0x2; /* 100Mb/s Half Duplex */
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_100M_FULL: u32             = 0x3; /* 100Mb/s Full Duplex */
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_1GB_HALF: u32              = 0x4; /* 1Gb/s Half Duplex */
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_1GB_FULL: u32              = 0x5; /* 1Gb/s Full Duplex */
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_10GB_HALF: u32             = 0x6; /* 10Gb/s Half Duplex */
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_10GB_FULL: u32             = 0x7; /* 10Gb/s Full Duplex */
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_1GB: u32                   = 0x4; /* 1Gb/s */
pub const IXGBE_MDIO_AUTO_NEG_VENDOR_STATUS_10GB: u32                  = 0x6; /* 10Gb/s */

pub const IXGBE_MII_10GBASE_T_AUTONEG_CTRL_REG: u32                    = 0x20;   /* 10G Control Reg */
pub const IXGBE_MII_AUTONEG_VENDOR_PROVISION_1_REG: u32                = 0xC400; /* 1G Provisioning 1 */
pub const IXGBE_MII_AUTONEG_XNP_TX_REG: u32                            = 0x17;   /* 1G XNP Transmit */
pub const IXGBE_MII_AUTONEG_ADVERTISE_REG: u32                         = 0x10;   /* 100M Advertisement */
pub const IXGBE_MII_10GBASE_T_ADVERTISE: u32                           = 0x1000; /* full duplex, bit:12*/
pub const IXGBE_MII_1GBASE_T_ADVERTISE_XNP_TX: u32                     = 0x4000; /* full duplex, bit:14*/
pub const IXGBE_MII_1GBASE_T_ADVERTISE: u32                            = 0x8000; /* full duplex, bit:15*/
pub const IXGBE_MII_2_5GBASE_T_ADVERTISE: u32                          = 0x0400;
pub const IXGBE_MII_5GBASE_T_ADVERTISE: u32                            = 0x0800;
pub const IXGBE_MII_100BASE_T_ADVERTISE: u32                           = 0x0100; /* full duplex, bit:8 */
pub const IXGBE_MII_100BASE_T_ADVERTISE_HALF: u32                      = 0x0080; /* half duplex, bit:7 */
pub const IXGBE_MII_RESTART: u32                                       = 0x200;
pub const IXGBE_MII_AUTONEG_COMPLETE: u32                              = 0x20;
pub const IXGBE_MII_AUTONEG_LINK_UP: u32                               = 0x04;
pub const IXGBE_MII_AUTONEG_REG: u32                                   = 0x0;

pub const IXGBE_PHY_REVISION_MASK: u32                                 = 0xFFFFFFF0;
pub const IXGBE_MAX_PHY_ADDR: u32                                      = 32;

/* PHY IDs*/
pub const TN1010_PHY_ID: u32                                           = 0x00A19410;
pub const TNX_FW_REV: u32                                              = 0xB;
pub const X540_PHY_ID: u32                                             = 0x01540200;
pub const X550_PHY_ID2: u32                                            = 0x01540223;
pub const X550_PHY_ID3: u32                                            = 0x01540221;
pub const X557_PHY_ID: u32                                             = 0x01540240;
pub const X557_PHY_ID2: u32                                            = 0x01540250;
pub const AQ_FW_REV: u32                                               = 0x20;
pub const QT2022_PHY_ID: u32                                           = 0x0043A400;
pub const ATH_PHY_ID: u32                                              = 0x03429050;

/* PHY Types */
pub const IXGBE_M88E1500_E_PHY_ID: u32                                 = 0x01410DD0;
pub const IXGBE_M88E1543_E_PHY_ID: u32                                 = 0x01410EA0;

/* Special PHY Init Routine */
pub const IXGBE_PHY_INIT_OFFSET_NL: u32                                = 0x002B;
pub const IXGBE_PHY_INIT_END_NL: u32                                   = 0xFFFF;
pub const IXGBE_CONTROL_MASK_NL: u32                                   = 0xF000;
pub const IXGBE_DATA_MASK_NL: u32                                      = 0x0FFF;
pub const IXGBE_CONTROL_SHIFT_NL: u32                                  = 12;
pub const IXGBE_DELAY_NL: u32                                          = 0;
pub const IXGBE_DATA_NL: u32                                           = 1;
pub const IXGBE_CONTROL_NL: u32                                        = 0x000F;
pub const IXGBE_CONTROL_EOL_NL: u32                                    = 0x0FFF;
pub const IXGBE_CONTROL_SOL_NL: u32                                    = 0x0000;

/* General purpose Interrupt Enable */
pub const IXGBE_SDP0_GPIEN: u32                                        = 0x00000001; /* SDP0 */
pub const IXGBE_SDP1_GPIEN: u32                                        = 0x00000002; /* SDP1 */
pub const IXGBE_SDP2_GPIEN: u32                                        = 0x00000004; /* SDP2 */
pub const IXGBE_SDP0_GPIEN_X540: u32                                   = 0x00000002; /* SDP0 on X540 and X550 */
pub const IXGBE_SDP1_GPIEN_X540: u32                                   = 0x00000004; /* SDP1 on X540 and X550 */
pub const IXGBE_SDP2_GPIEN_X540: u32                                   = 0x00000008; /* SDP2 on X540 and X550 */
pub const IXGBE_SDP0_GPIEN_X550: u32                                   = IXGBE_SDP0_GPIEN_X540;
pub const IXGBE_SDP1_GPIEN_X550: u32                                   = IXGBE_SDP1_GPIEN_X540;
pub const IXGBE_SDP2_GPIEN_X550: u32                                   = IXGBE_SDP2_GPIEN_X540;
pub const IXGBE_SDP0_GPIEN_X550EM_x: u32                               = IXGBE_SDP0_GPIEN_X540;
pub const IXGBE_SDP1_GPIEN_X550EM_x: u32                               = IXGBE_SDP1_GPIEN_X540;
pub const IXGBE_SDP2_GPIEN_X550EM_x: u32                               = IXGBE_SDP2_GPIEN_X540;
pub const IXGBE_SDP0_GPIEN_X550EM_a: u32                               = IXGBE_SDP0_GPIEN_X540;
pub const IXGBE_SDP1_GPIEN_X550EM_a: u32                               = IXGBE_SDP1_GPIEN_X540;
pub const IXGBE_SDP2_GPIEN_X550EM_a: u32                               = IXGBE_SDP2_GPIEN_X540;

pub fn IXGBE_SDP0_GPIEN_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_SDP0_GPIEN) }

pub fn IXGBE_SDP1_GPIEN_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_SDP1_GPIEN) }

pub fn IXGBE_SDP2_GPIEN_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_SDP2_GPIEN) }

pub const IXGBE_GPIE_MSIX_MODE: u32                                    = 0x00000010; /* MSI-X mode */
pub const IXGBE_GPIE_OCD: u32                                          = 0x00000020; /* Other Clear Disable */
pub const IXGBE_GPIE_EIMEN: u32                                        = 0x00000040; /* Immediate Interrupt Enable */
pub const IXGBE_GPIE_EIAME: u32                                        = 0x40000000;
pub const IXGBE_GPIE_PBA_SUPPORT: u32                                  = 0x80000000;
pub const IXGBE_GPIE_RSC_DELAY_SHIFT: u32                              = 11;
pub const IXGBE_GPIE_VTMODE_MASK: u32                                  = 0x0000C000; /* VT Mode Mask */
pub const IXGBE_GPIE_VTMODE_16: u32                                    = 0x00004000; /* 16 VFs 8 queues per VF */
pub const IXGBE_GPIE_VTMODE_32: u32                                    = 0x00008000; /* 32 VFs 4 queues per VF */
pub const IXGBE_GPIE_VTMODE_64: u32                                    = 0x0000C000; /* 64 VFs 2 queues per VF */

/* Packet Buffer Initialization */
pub const IXGBE_MAX_PACKET_BUFFERS: u32                                = 8;

pub const IXGBE_TXPBSIZE_20KB: u32                                     = 0x00005000; /* 20KB Packet Buffer */
pub const IXGBE_TXPBSIZE_40KB: u32                                     = 0x0000A000; /* 40KB Packet Buffer */
pub const IXGBE_RXPBSIZE_48KB: u32                                     = 0x0000C000; /* 48KB Packet Buffer */
pub const IXGBE_RXPBSIZE_64KB: u32                                     = 0x00010000; /* 64KB Packet Buffer */
pub const IXGBE_RXPBSIZE_80KB: u32                                     = 0x00014000; /* 80KB Packet Buffer */
pub const IXGBE_RXPBSIZE_128KB: u32                                    = 0x00020000; /* 128KB Packet Buffer */
pub const IXGBE_RXPBSIZE_MAX: u32                                      = 0x00080000; /* 512KB Packet Buffer */
pub const IXGBE_TXPBSIZE_MAX: u32                                      = 0x00028000; /* 160KB Packet Buffer */

pub const IXGBE_TXPKT_SIZE_MAX: u32                                    = 0xA; /* Max Tx Packet size */
pub const IXGBE_MAX_PB: u32                                            = 8;

/* Packet buffer allocation strategies */

pub const PBA_STRATEGY_EQUAL: u32                                      = 0; /* Distribute PB space equally */
pub const PBA_STRATEGY_WEIGHTED: u32                                   = 1; /* Weight front half of TCs */

/* Transmit Flow Control status */
pub const IXGBE_TFCS_TXOFF: u32                                        = 0x00000001;
pub const IXGBE_TFCS_TXOFF0: u32                                       = 0x00000100;
pub const IXGBE_TFCS_TXOFF1: u32                                       = 0x00000200;
pub const IXGBE_TFCS_TXOFF2: u32                                       = 0x00000400;
pub const IXGBE_TFCS_TXOFF3: u32                                       = 0x00000800;
pub const IXGBE_TFCS_TXOFF4: u32                                       = 0x00001000;
pub const IXGBE_TFCS_TXOFF5: u32                                       = 0x00002000;
pub const IXGBE_TFCS_TXOFF6: u32                                       = 0x00004000;
pub const IXGBE_TFCS_TXOFF7: u32                                       = 0x00008000;

/* TCP Timer */
pub const IXGBE_TCPTIMER_KS: u32                                       = 0x00000100;
pub const IXGBE_TCPTIMER_COUNT_ENABLE: u32                             = 0x00000200;
pub const IXGBE_TCPTIMER_COUNT_FINISH: u32                             = 0x00000400;
pub const IXGBE_TCPTIMER_LOOP: u32                                     = 0x00000800;
pub const IXGBE_TCPTIMER_DURATION_MASK: u32                            = 0x000000FF;

/* HLREG0 Bit Masks */
pub const IXGBE_HLREG0_TXCRCEN: u32                                    = 0x00000001; /* bit  0 */
pub const IXGBE_HLREG0_RXCRCSTRP: u32                                  = 0x00000002; /* bit  1 */
pub const IXGBE_HLREG0_JUMBOEN: u32                                    = 0x00000004; /* bit  2 */
pub const IXGBE_HLREG0_TXPADEN: u32                                    = 0x00000400; /* bit 10 */
pub const IXGBE_HLREG0_TXPAUSEEN: u32                                  = 0x00001000; /* bit 12 */
pub const IXGBE_HLREG0_RXPAUSEEN: u32                                  = 0x00004000; /* bit 14 */
pub const IXGBE_HLREG0_LPBK: u32                                       = 0x00008000; /* bit 15 */
pub const IXGBE_HLREG0_MDCSPD: u32                                     = 0x00010000; /* bit 16 */
pub const IXGBE_HLREG0_CONTMDC: u32                                    = 0x00020000; /* bit 17 */
pub const IXGBE_HLREG0_CTRLFLTR: u32                                   = 0x00040000; /* bit 18 */
pub const IXGBE_HLREG0_PREPEND: u32                                    = 0x00F00000; /* bits 20-23 */
pub const IXGBE_HLREG0_PRIPAUSEEN: u32                                 = 0x01000000; /* bit 24 */
pub const IXGBE_HLREG0_RXPAUSERECDA: u32                               = 0x06000000; /* bits 25-26 */
pub const IXGBE_HLREG0_RXLNGTHERREN: u32                               = 0x08000000; /* bit 27 */
pub const IXGBE_HLREG0_RXPADSTRIPEN: u32                               = 0x10000000; /* bit 28 */

/* VMD_CTL bitmasks */
pub const IXGBE_VMD_CTL_VMDQ_EN: u32                                   = 0x00000001;
pub const IXGBE_VMD_CTL_VMDQ_FILTER: u32                               = 0x00000002;

/* VT_CTL bitmasks */
pub const IXGBE_VT_CTL_DIS_DEFPL: u32                                  = 0x20000000; /* disable default pool */
pub const IXGBE_VT_CTL_REPLEN: u32                                     = 0x40000000; /* replication enabled */
pub const IXGBE_VT_CTL_VT_ENABLE: u32                                  = 0x00000001;  /* Enable VT Mode */
pub const IXGBE_VT_CTL_POOL_SHIFT: u32                                 = 7;
pub const IXGBE_VT_CTL_POOL_MASK: u32                                  = 0x3F << IXGBE_VT_CTL_POOL_SHIFT;

/* VMOLR bitmasks */
pub const IXGBE_VMOLR_UPE: u32                                         = 0x00400000; /* unicast promiscuous */
pub const IXGBE_VMOLR_VPE: u32                                         = 0x00800000; /* VLAN promiscuous */
pub const IXGBE_VMOLR_AUPE: u32                                        = 0x01000000; /* accept untagged packets */
pub const IXGBE_VMOLR_ROMPE: u32                                       = 0x02000000; /* accept packets in MTA tbl */
pub const IXGBE_VMOLR_ROPE: u32                                        = 0x04000000; /* accept packets in UC tbl */
pub const IXGBE_VMOLR_BAM: u32                                         = 0x08000000; /* accept broadcast packets */
pub const IXGBE_VMOLR_MPE: u32                                         = 0x10000000; /* multicast promiscuous */

/* VFRE bitmask */
pub const IXGBE_VFRE_ENABLE_ALL: u32                                   = 0xFFFFFFFF;

pub const IXGBE_VF_INIT_TIMEOUT: u32                                   = 200; /* Number of retries to clear RSTI */

/* RDHMPN and TDHMPN bitmasks */
pub const IXGBE_RDHMPN_RDICADDR: u32                                   = 0x007FF800;
pub const IXGBE_RDHMPN_RDICRDREQ: u32                                  = 0x00800000;
pub const IXGBE_RDHMPN_RDICADDR_SHIFT: u32                             = 11;
pub const IXGBE_TDHMPN_TDICADDR: u32                                   = 0x003FF800;
pub const IXGBE_TDHMPN_TDICRDREQ: u32                                  = 0x00800000;
pub const IXGBE_TDHMPN_TDICADDR_SHIFT: u32                             = 11;

pub const IXGBE_RDMAM_MEM_SEL_SHIFT: u32                               = 13;
pub const IXGBE_RDMAM_DWORD_SHIFT: u32                                 = 9;
pub const IXGBE_RDMAM_DESC_COMP_FIFO: u32                              = 1;
pub const IXGBE_RDMAM_DFC_CMD_FIFO: u32                                = 2;
pub const IXGBE_RDMAM_RSC_HEADER_ADDR: u32                             = 3;
pub const IXGBE_RDMAM_TCN_STATUS_RAM: u32                              = 4;
pub const IXGBE_RDMAM_WB_COLL_FIFO: u32                                = 5;
pub const IXGBE_RDMAM_QSC_CNT_RAM: u32                                 = 6;
pub const IXGBE_RDMAM_QSC_FCOE_RAM: u32                                = 7;
pub const IXGBE_RDMAM_QSC_QUEUE_CNT: u32                               = 8;
pub const IXGBE_RDMAM_QSC_QUEUE_RAM: u32                               = 0xA;
pub const IXGBE_RDMAM_QSC_RSC_RAM: u32                                 = 0xB;
pub const IXGBE_RDMAM_DESC_COM_FIFO_RANGE: u32                         = 135;
pub const IXGBE_RDMAM_DESC_COM_FIFO_COUNT: u32                         = 4;
pub const IXGBE_RDMAM_DFC_CMD_FIFO_RANGE: u32                          = 48;
pub const IXGBE_RDMAM_DFC_CMD_FIFO_COUNT: u32                          = 7;
pub const IXGBE_RDMAM_RSC_HEADER_ADDR_RANGE: u32                       = 32;
pub const IXGBE_RDMAM_RSC_HEADER_ADDR_COUNT: u32                       = 4;
pub const IXGBE_RDMAM_TCN_STATUS_RAM_RANGE: u32                        = 256;
pub const IXGBE_RDMAM_TCN_STATUS_RAM_COUNT: u32                        = 9;
pub const IXGBE_RDMAM_WB_COLL_FIFO_RANGE: u32                          = 8;
pub const IXGBE_RDMAM_WB_COLL_FIFO_COUNT: u32                          = 4;
pub const IXGBE_RDMAM_QSC_CNT_RAM_RANGE: u32                           = 64;
pub const IXGBE_RDMAM_QSC_CNT_RAM_COUNT: u32                           = 4;
pub const IXGBE_RDMAM_QSC_FCOE_RAM_RANGE: u32                          = 512;
pub const IXGBE_RDMAM_QSC_FCOE_RAM_COUNT: u32                          = 5;
pub const IXGBE_RDMAM_QSC_QUEUE_CNT_RANGE: u32                         = 32;
pub const IXGBE_RDMAM_QSC_QUEUE_CNT_COUNT: u32                         = 4;
pub const IXGBE_RDMAM_QSC_QUEUE_RAM_RANGE: u32                         = 128;
pub const IXGBE_RDMAM_QSC_QUEUE_RAM_COUNT: u32                         = 8;
pub const IXGBE_RDMAM_QSC_RSC_RAM_RANGE: u32                           = 32;
pub const IXGBE_RDMAM_QSC_RSC_RAM_COUNT: u32                           = 8;

pub const IXGBE_TXDESCIC_READY: u32                                    = 0x80000000;

/* Receive Checksum Control */
pub const IXGBE_RXCSUM_IPPCSE: u32                                     = 0x00001000; /* IP payload checksum enable */
pub const IXGBE_RXCSUM_PCSD: u32                                       = 0x00002000; /* packet checksum disabled */

/* FCRTL Bit Masks */
pub const IXGBE_FCRTL_XONE: u32                                        = 0x80000000; /* XON enable */
pub const IXGBE_FCRTH_FCEN: u32                                        = 0x80000000; /* Packet buffer fc enable */

/* PAP bit masks*/
pub const IXGBE_PAP_TXPAUSECNT_MASK: u32                               = 0x0000FFFF; /* Pause counter mask */

/* RMCS Bit Masks */
pub const IXGBE_RMCS_RRM: u32                                          = 0x00000002; /* Rx Recycle Mode enable */
/* Receive Arbitration Control: 0 Round Robin, 1 DFP */
pub const IXGBE_RMCS_RAC: u32                                          = 0x00000004;
/* Deficit Fixed Prio ena */
pub const IXGBE_RMCS_DFP: u32                                          = IXGBE_RMCS_RAC;
pub const IXGBE_RMCS_TFCE_802_3X: u32                                  = 0x00000008; /* Tx Priority FC ena */
pub const IXGBE_RMCS_TFCE_PRIORITY: u32                                = 0x00000010; /* Tx Priority FC ena */
pub const IXGBE_RMCS_ARBDIS: u32                                       = 0x00000040; /* Arbitration disable bit */

/* FCCFG Bit Masks */
pub const IXGBE_FCCFG_TFCE_802_3X: u32                                 = 0x00000008; /* Tx link FC enable */
pub const IXGBE_FCCFG_TFCE_PRIORITY: u32                               = 0x00000010; /* Tx priority FC enable */

/* Interrupt register bitmasks */

/* Extended Interrupt Cause Read */
pub const IXGBE_EICR_RTX_QUEUE: u32                                    = 0x0000FFFF; /* RTx Queue Interrupt */
pub const IXGBE_EICR_FLOW_DIR: u32                                     = 0x00010000; /* FDir Exception */
pub const IXGBE_EICR_RX_MISS: u32                                      = 0x00020000; /* Packet Buffer Overrun */
pub const IXGBE_EICR_PCI: u32                                          = 0x00040000; /* PCI Exception */
pub const IXGBE_EICR_MAILBOX: u32                                      = 0x00080000; /* VF to PF Mailbox Interrupt */
pub const IXGBE_EICR_LSC: u32                                          = 0x00100000; /* Link Status Change */
pub const IXGBE_EICR_LINKSEC: u32                                      = 0x00200000; /* PN Threshold */
pub const IXGBE_EICR_MNG: u32                                          = 0x00400000; /* Manageability Event Interrupt */
pub const IXGBE_EICR_TS: u32                                           = 0x00800000; /* Thermal Sensor Event */
pub const IXGBE_EICR_TIMESYNC: u32                                     = 0x01000000; /* Timesync Event */
pub const IXGBE_EICR_GPI_SDP0: u32                                     = 0x01000000; /* Gen Purpose Interrupt on SDP0 */
pub const IXGBE_EICR_GPI_SDP1: u32                                     = 0x02000000; /* Gen Purpose Interrupt on SDP1 */
pub const IXGBE_EICR_GPI_SDP2: u32                                     = 0x04000000; /* Gen Purpose Interrupt on SDP2 */
pub const IXGBE_EICR_ECC: u32                                          = 0x10000000; /* ECC Error */
pub const IXGBE_EICR_GPI_SDP0_X540: u32                                = 0x02000000; /* Gen Purpose Interrupt on SDP0 */
pub const IXGBE_EICR_GPI_SDP1_X540: u32                                = 0x04000000; /* Gen Purpose Interrupt on SDP1 */
pub const IXGBE_EICR_GPI_SDP2_X540: u32                                = 0x08000000; /* Gen Purpose Interrupt on SDP2 */
pub const IXGBE_EICR_GPI_SDP0_X550: u32                                = IXGBE_EICR_GPI_SDP0_X540;
pub const IXGBE_EICR_GPI_SDP1_X550: u32                                = IXGBE_EICR_GPI_SDP1_X540;
pub const IXGBE_EICR_GPI_SDP2_X550: u32                                = IXGBE_EICR_GPI_SDP2_X540;
pub const IXGBE_EICR_GPI_SDP0_X550EM_x: u32                            = IXGBE_EICR_GPI_SDP0_X540;
pub const IXGBE_EICR_GPI_SDP1_X550EM_x: u32                            = IXGBE_EICR_GPI_SDP1_X540;
pub const IXGBE_EICR_GPI_SDP2_X550EM_x: u32                            = IXGBE_EICR_GPI_SDP2_X540;
pub const IXGBE_EICR_GPI_SDP0_X550EM_a: u32                            = IXGBE_EICR_GPI_SDP0_X540;
pub const IXGBE_EICR_GPI_SDP1_X550EM_a: u32                            = IXGBE_EICR_GPI_SDP1_X540;
pub const IXGBE_EICR_GPI_SDP2_X550EM_a: u32                            = IXGBE_EICR_GPI_SDP2_X540;

pub fn IXGBE_EICR_GPI_SDP0_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_EICR_GPI_SDP0) }

pub fn IXGBE_EICR_GPI_SDP1_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_EICR_GPI_SDP1) }

pub fn IXGBE_EICR_GPI_SDP2_BY_MAC(hw: u32) -> u32 { IXGBE_BY_MAC(hw, IXGBE_EICR_GPI_SDP2) }

pub const IXGBE_EICR_PBUR: u32                                         = 0x10000000; /* Packet Buffer Handler Error */
pub const IXGBE_EICR_DHER: u32                                         = 0x20000000; /* Descriptor Handler Error */
pub const IXGBE_EICR_TCP_TIMER: u32                                    = 0x40000000; /* TCP Timer */
pub const IXGBE_EICR_OTHER: u32                                        = 0x80000000; /* Interrupt Cause Active */

/* Extended Interrupt Cause Set */
pub const IXGBE_EICS_RTX_QUEUE: u32                                    = IXGBE_EICR_RTX_QUEUE; /* RTx Queue Interrupt */
pub const IXGBE_EICS_FLOW_DIR: u32                                     = IXGBE_EICR_FLOW_DIR;  /* FDir Exception */
pub const IXGBE_EICS_RX_MISS: u32                                      = IXGBE_EICR_RX_MISS;   /* Pkt Buffer Overrun */
pub const IXGBE_EICS_PCI: u32                                          = IXGBE_EICR_PCI; /* PCI Exception */
pub const IXGBE_EICS_MAILBOX: u32                                      = IXGBE_EICR_MAILBOX;   /* VF to PF Mailbox Int */
pub const IXGBE_EICS_LSC: u32                                          = IXGBE_EICR_LSC; /* Link Status Change */
pub const IXGBE_EICS_MNG: u32                                          = IXGBE_EICR_MNG; /* MNG Event Interrupt */
pub const IXGBE_EICS_TIMESYNC: u32                                     = IXGBE_EICR_TIMESYNC; /* Timesync Event */
pub const IXGBE_EICS_GPI_SDP0: u32                                     = IXGBE_EICR_GPI_SDP0; /* SDP0 Gen Purpose Int */
pub const IXGBE_EICS_GPI_SDP1: u32                                     = IXGBE_EICR_GPI_SDP1; /* SDP1 Gen Purpose Int */
pub const IXGBE_EICS_GPI_SDP2: u32                                     = IXGBE_EICR_GPI_SDP2; /* SDP2 Gen Purpose Int */
pub const IXGBE_EICS_ECC: u32                                          = IXGBE_EICR_ECC; /* ECC Error */
pub fn IXGBE_EICS_GPI_SDP0_BY_MAC(hw: u32) -> u32 { IXGBE_EICR_GPI_SDP0_BY_MAC(hw) }

pub fn IXGBE_EICS_GPI_SDP1_BY_MAC(hw: u32) -> u32 { IXGBE_EICR_GPI_SDP1_BY_MAC(hw) }

pub fn IXGBE_EICS_GPI_SDP2_BY_MAC(hw: u32) -> u32 { IXGBE_EICR_GPI_SDP2_BY_MAC(hw) }

pub const IXGBE_EICS_PBUR: u32                                         = IXGBE_EICR_PBUR; /* Pkt Buf Handler Err */
pub const IXGBE_EICS_DHER: u32                                         = IXGBE_EICR_DHER; /* Desc Handler Error */
pub const IXGBE_EICS_TCP_TIMER: u32                                    = IXGBE_EICR_TCP_TIMER; /* TCP Timer */
pub const IXGBE_EICS_OTHER: u32                                        = IXGBE_EICR_OTHER; /* INT Cause Active */

/* Extended Interrupt Mask Set */
pub const IXGBE_EIMS_RTX_QUEUE: u32                                    = IXGBE_EICR_RTX_QUEUE; /* RTx Queue Interrupt */
pub const IXGBE_EIMS_FLOW_DIR: u32                                     = IXGBE_EICR_FLOW_DIR; /* FDir Exception */
pub const IXGBE_EIMS_RX_MISS: u32                                      = IXGBE_EICR_RX_MISS; /* Packet Buffer Overrun */
pub const IXGBE_EIMS_PCI: u32                                          = IXGBE_EICR_PCI; /* PCI Exception */
pub const IXGBE_EIMS_MAILBOX: u32                                      = IXGBE_EICR_MAILBOX;   /* VF to PF Mailbox Int */
pub const IXGBE_EIMS_LSC: u32                                          = IXGBE_EICR_LSC; /* Link Status Change */
pub const IXGBE_EIMS_MNG: u32                                          = IXGBE_EICR_MNG; /* MNG Event Interrupt */
pub const IXGBE_EIMS_TS: u32                                           = IXGBE_EICR_TS; /* Thermal Sensor Event */
pub const IXGBE_EIMS_TIMESYNC: u32                                     = IXGBE_EICR_TIMESYNC; /* Timesync Event */
pub const IXGBE_EIMS_GPI_SDP0: u32                                     = IXGBE_EICR_GPI_SDP0; /* SDP0 Gen Purpose Int */
pub const IXGBE_EIMS_GPI_SDP1: u32                                     = IXGBE_EICR_GPI_SDP1; /* SDP1 Gen Purpose Int */
pub const IXGBE_EIMS_GPI_SDP2: u32                                     = IXGBE_EICR_GPI_SDP2; /* SDP2 Gen Purpose Int */
pub const IXGBE_EIMS_ECC: u32                                          = IXGBE_EICR_ECC; /* ECC Error */
pub fn IXGBE_EIMS_GPI_SDP0_BY_MAC(hw: u32) -> u32 { IXGBE_EICR_GPI_SDP0_BY_MAC(hw) }

pub fn IXGBE_EIMS_GPI_SDP1_BY_MAC(hw: u32) -> u32 { IXGBE_EICR_GPI_SDP1_BY_MAC(hw) }

pub fn IXGBE_EIMS_GPI_SDP2_BY_MAC(hw: u32) -> u32 { IXGBE_EICR_GPI_SDP2_BY_MAC(hw) }

pub const IXGBE_EIMS_PBUR: u32                                         = IXGBE_EICR_PBUR; /* Pkt Buf Handler Err */
pub const IXGBE_EIMS_DHER: u32                                         = IXGBE_EICR_DHER; /* Descr Handler Error */
pub const IXGBE_EIMS_TCP_TIMER: u32                                    = IXGBE_EICR_TCP_TIMER; /* TCP Timer */
pub const IXGBE_EIMS_OTHER: u32                                        = IXGBE_EICR_OTHER; /* INT Cause Active */

/* Extended Interrupt Mask Clear */
pub const IXGBE_EIMC_RTX_QUEUE: u32                                    = IXGBE_EICR_RTX_QUEUE; /* RTx Queue Interrupt */
pub const IXGBE_EIMC_FLOW_DIR: u32                                     = IXGBE_EICR_FLOW_DIR; /* FDir Exception */
pub const IXGBE_EIMC_RX_MISS: u32                                      = IXGBE_EICR_RX_MISS; /* Packet Buffer Overrun */
pub const IXGBE_EIMC_PCI: u32                                          = IXGBE_EICR_PCI; /* PCI Exception */
pub const IXGBE_EIMC_MAILBOX: u32                                      = IXGBE_EICR_MAILBOX; /* VF to PF Mailbox Int */
pub const IXGBE_EIMC_LSC: u32                                          = IXGBE_EICR_LSC; /* Link Status Change */
pub const IXGBE_EIMC_MNG: u32                                          = IXGBE_EICR_MNG; /* MNG Event Interrupt */
pub const IXGBE_EIMC_TIMESYNC: u32                                     = IXGBE_EICR_TIMESYNC; /* Timesync Event */
pub const IXGBE_EIMC_GPI_SDP0: u32                                     = IXGBE_EICR_GPI_SDP0; /* SDP0 Gen Purpose Int */
pub const IXGBE_EIMC_GPI_SDP1: u32                                     = IXGBE_EICR_GPI_SDP1; /* SDP1 Gen Purpose Int */
pub const IXGBE_EIMC_GPI_SDP2: u32                                     = IXGBE_EICR_GPI_SDP2;  /* SDP2 Gen Purpose Int */
pub const IXGBE_EIMC_ECC: u32                                          = IXGBE_EICR_ECC; /* ECC Error */
pub fn IXGBE_EIMC_GPI_SDP0_BY_MAC(hw: u32) -> u32 { IXGBE_EICR_GPI_SDP0_BY_MAC(hw) }

pub fn IXGBE_EIMC_GPI_SDP1_BY_MAC(hw: u32) -> u32 { IXGBE_EICR_GPI_SDP1_BY_MAC(hw) }

pub fn IXGBE_EIMC_GPI_SDP2_BY_MAC(hw: u32) -> u32 { IXGBE_EICR_GPI_SDP2_BY_MAC(hw) }

pub const IXGBE_EIMC_PBUR: u32                                         = IXGBE_EICR_PBUR; /* Pkt Buf Handler Err */
pub const IXGBE_EIMC_DHER: u32                                         = IXGBE_EICR_DHER; /* Desc Handler Err */
pub const IXGBE_EIMC_TCP_TIMER: u32                                    = IXGBE_EICR_TCP_TIMER; /* TCP Timer */
pub const IXGBE_EIMC_OTHER: u32                                        = IXGBE_EICR_OTHER; /* INT Cause Active */

pub const IXGBE_EIMS_ENABLE_MASK: u32                                  = IXGBE_EIMS_RTX_QUEUE | IXGBE_EIMS_LSC | IXGBE_EIMS_TCP_TIMER | IXGBE_EIMS_OTHER;

/* Immediate Interrupt Rx (A.K.A. Low Latency Interrupt) */
pub const IXGBE_IMIR_PORT_IM_EN: u32                                   = 0x00010000;  /* TCP port enable */
pub const IXGBE_IMIR_PORT_BP: u32                                      = 0x00020000;  /* TCP port check bypass */
pub const IXGBE_IMIREXT_SIZE_BP: u32                                   = 0x00001000;  /* Packet size bypass */
pub const IXGBE_IMIREXT_CTRL_URG: u32                                  = 0x00002000;  /* Check URG bit in header */
pub const IXGBE_IMIREXT_CTRL_ACK: u32                                  = 0x00004000;  /* Check ACK bit in header */
pub const IXGBE_IMIREXT_CTRL_PSH: u32                                  = 0x00008000;  /* Check PSH bit in header */
pub const IXGBE_IMIREXT_CTRL_RST: u32                                  = 0x00010000;  /* Check RST bit in header */
pub const IXGBE_IMIREXT_CTRL_SYN: u32                                  = 0x00020000;  /* Check SYN bit in header */
pub const IXGBE_IMIREXT_CTRL_FIN: u32                                  = 0x00040000;  /* Check FIN bit in header */
pub const IXGBE_IMIREXT_CTRL_BP: u32                                   = 0x00080000;  /* Bypass check of control bits */
pub const IXGBE_IMIR_SIZE_BP_82599: u32                                = 0x00001000; /* Packet size bypass */
pub const IXGBE_IMIR_CTRL_URG_82599: u32                               = 0x00002000; /* Check URG bit in header */
pub const IXGBE_IMIR_CTRL_ACK_82599: u32                               = 0x00004000; /* Check ACK bit in header */
pub const IXGBE_IMIR_CTRL_PSH_82599: u32                               = 0x00008000; /* Check PSH bit in header */
pub const IXGBE_IMIR_CTRL_RST_82599: u32                               = 0x00010000; /* Check RST bit in header */
pub const IXGBE_IMIR_CTRL_SYN_82599: u32                               = 0x00020000; /* Check SYN bit in header */
pub const IXGBE_IMIR_CTRL_FIN_82599: u32                               = 0x00040000; /* Check FIN bit in header */
pub const IXGBE_IMIR_CTRL_BP_82599: u32                                = 0x00080000; /* Bypass chk of ctrl bits */
pub const IXGBE_IMIR_LLI_EN_82599: u32                                 = 0x00100000; /* Enables low latency Int */
pub const IXGBE_IMIR_RX_QUEUE_MASK_82599: u32                          = 0x0000007F; /* Rx Queue Mask */
pub const IXGBE_IMIR_RX_QUEUE_SHIFT_82599: u32                         = 21; /* Rx Queue Shift */
pub const IXGBE_IMIRVP_PRIORITY_MASK: u32                              = 0x00000007; /* VLAN priority mask */
pub const IXGBE_IMIRVP_PRIORITY_EN: u32                                = 0x00000008; /* VLAN priority enable */

pub const IXGBE_MAX_FTQF_FILTERS: u32                                  = 128;
pub const IXGBE_FTQF_PROTOCOL_MASK: u32                                = 0x00000003;
pub const IXGBE_FTQF_PROTOCOL_TCP: u32                                 = 0x00000000;
pub const IXGBE_FTQF_PROTOCOL_UDP: u32                                 = 0x00000001;
pub const IXGBE_FTQF_PROTOCOL_SCTP: u32                                = 2;
pub const IXGBE_FTQF_PRIORITY_MASK: u32                                = 0x00000007;
pub const IXGBE_FTQF_PRIORITY_SHIFT: u32                               = 2;
pub const IXGBE_FTQF_POOL_MASK: u32                                    = 0x0000003F;
pub const IXGBE_FTQF_POOL_SHIFT: u32                                   = 8;
pub const IXGBE_FTQF_5TUPLE_MASK_MASK: u32                             = 0x0000001F;
pub const IXGBE_FTQF_5TUPLE_MASK_SHIFT: u32                            = 25;
pub const IXGBE_FTQF_SOURCE_ADDR_MASK: u32                             = 0x1E;
pub const IXGBE_FTQF_DEST_ADDR_MASK: u32                               = 0x1D;
pub const IXGBE_FTQF_SOURCE_PORT_MASK: u32                             = 0x1B;
pub const IXGBE_FTQF_DEST_PORT_MASK: u32                               = 0x17;
pub const IXGBE_FTQF_PROTOCOL_COMP_MASK: u32                           = 0x0F;
pub const IXGBE_FTQF_POOL_MASK_EN: u32                                 = 0x40000000;
pub const IXGBE_FTQF_QUEUE_ENABLE: u32                                 = 0x80000000;

/* Interrupt clear mask */
pub const IXGBE_IRQ_CLEAR_MASK: u32                                    = 0xFFFFFFFF;

/* Interrupt Vector Allocation Registers */
pub const IXGBE_IVAR_REG_NUM: u32                                      = 25;
pub const IXGBE_IVAR_REG_NUM_82599: u32                                = 64;
pub const IXGBE_IVAR_TXRX_ENTRY: u32                                   = 96;
pub const IXGBE_IVAR_RX_ENTRY: u32                                     = 64;

pub fn IXGBE_IVAR_RX_QUEUE(i: u32) -> u32 { 0 + i }

pub fn IXGBE_IVAR_TX_QUEUE(i: u32) -> u32 { 64 + i }

pub const IXGBE_IVAR_TX_ENTRY: u32                                     = 32;

pub const IXGBE_IVAR_TCP_TIMER_INDEX: u32                              = 96; /* 0 based index */
pub const IXGBE_IVAR_OTHER_CAUSES_INDEX: u32                           = 97; /* 0 based index */

pub fn IXGBE_MSIX_VECTOR(i: u32) -> u32 { 0 + i }

pub const IXGBE_IVAR_ALLOC_VAL: u32                                    = 0x80; /* Interrupt Allocation valid */

/* ETYPE Queue Filter/Select Bit Masks */
pub const IXGBE_MAX_ETQF_FILTERS: u32                                  = 8;
pub const IXGBE_ETQF_FCOE: u32                                         = 0x08000000; /* bit 27 */
pub const IXGBE_ETQF_BCN: u32                                          = 0x10000000; /* bit 28 */
pub const IXGBE_ETQF_TX_ANTISPOOF: u32                                 = 0x20000000; /* bit 29 */
pub const IXGBE_ETQF_1588: u32                                         = 0x40000000; /* bit 30 */
pub const IXGBE_ETQF_FILTER_EN: u32                                    = 0x80000000; /* bit 31 */
pub const IXGBE_ETQF_POOL_ENABLE: u32                                  = 1 << 26; /* bit 26 */
pub const IXGBE_ETQF_POOL_SHIFT: u32                                   = 20;

pub const IXGBE_ETQS_RX_QUEUE: u32                                     = 0x007F0000; /* bits 22:16 */
pub const IXGBE_ETQS_RX_QUEUE_SHIFT: u32                               = 16;
pub const IXGBE_ETQS_LLI: u32                                          = 0x20000000; /* bit 29 */
pub const IXGBE_ETQS_QUEUE_EN: u32                                     = 0x80000000; /* bit 31 */

/*
 * ETQF filter list: one static filter per filter consumer. This is
 *		   to avoid filter collisions later. Add new filters
 *		   here!!
 *
 * Current filters:
 *	EAPOL 802.1x (0x888e): Filter 0;
 *	FCoE (0x8906):	 Filter 2;
 *: u32	= 1588 (0x88f7):	 Filter 3;
 *	FIP  (0x8914):	 Filter 4;
 *	LLDP (0x88CC):	 Filter 5;
 *	LACP (0x8809):	 Filter 6;
 *	FC   (0x8808):	 Filter 7;
 */
pub const IXGBE_ETQF_FILTER_EAPOL: u32                                 = 0;
pub const IXGBE_ETQF_FILTER_FCOE: u32                                  = 2;
pub const IXGBE_ETQF_FILTER_1588: u32                                  = 3;
pub const IXGBE_ETQF_FILTER_FIP: u32                                   = 4;
pub const IXGBE_ETQF_FILTER_LLDP: u32                                  = 5;
pub const IXGBE_ETQF_FILTER_LACP: u32                                  = 6;
pub const IXGBE_ETQF_FILTER_FC: u32                                    = 7;
/* VLAN Control Bit Masks */
pub const IXGBE_VLNCTRL_VET: u32                                       = 0x0000FFFF;  /* bits 0-15 */
pub const IXGBE_VLNCTRL_CFI: u32                                       = 0x10000000;  /* bit 28 */
pub const IXGBE_VLNCTRL_CFIEN: u32                                     = 0x20000000;  /* bit 29 */
pub const IXGBE_VLNCTRL_VFE: u32                                       = 0x40000000;  /* bit 30 */
pub const IXGBE_VLNCTRL_VME: u32                                       = 0x80000000;  /* bit 31 */

/* VLAN pool filtering masks */
pub const IXGBE_VLVF_VIEN: u32                                         = 0x80000000;  /* filter is valid */
pub const IXGBE_VLVF_ENTRIES: u32                                      = 64;
pub const IXGBE_VLVF_VLANID_MASK: u32                                  = 0x00000FFF;
/* Per VF Port VLAN insertion rules */
pub const IXGBE_VMVIR_VLANA_DEFAULT: u32                               = 0x40000000; /* Always use default VLAN */
pub const IXGBE_VMVIR_VLANA_NEVER: u32                                 = 0x80000000; /* Never insert VLAN tag */

pub const IXGBE_ETHERNET_IEEE_VLAN_TYPE: u32                           = 0x8100;  /* 802.1q protocol */

/* STATUS Bit Masks */
pub const IXGBE_STATUS_LAN_ID: u32                                     = 0x0000000C; /* LAN ID */
pub const IXGBE_STATUS_LAN_ID_SHIFT: u32                               = 2; /* LAN ID Shift*/
pub const IXGBE_STATUS_GIO: u32                                        = 0x00080000; /* GIO Master Ena Status */

pub const IXGBE_STATUS_LAN_ID_0: u32                                   = 0x00000000; /* LAN ID 0 */
pub const IXGBE_STATUS_LAN_ID_1: u32                                   = 0x00000004; /* LAN ID 1 */

/* ESDP Bit Masks */
pub const IXGBE_ESDP_SDP0: u32                                         = 0x00000001; /* SDP0 Data Value */
pub const IXGBE_ESDP_SDP1: u32                                         = 0x00000002; /* SDP1 Data Value */
pub const IXGBE_ESDP_SDP2: u32                                         = 0x00000004; /* SDP2 Data Value */
pub const IXGBE_ESDP_SDP3: u32                                         = 0x00000008; /* SDP3 Data Value */
pub const IXGBE_ESDP_SDP4: u32                                         = 0x00000010; /* SDP4 Data Value */
pub const IXGBE_ESDP_SDP5: u32                                         = 0x00000020; /* SDP5 Data Value */
pub const IXGBE_ESDP_SDP6: u32                                         = 0x00000040; /* SDP6 Data Value */
pub const IXGBE_ESDP_SDP7: u32                                         = 0x00000080; /* SDP7 Data Value */
pub const IXGBE_ESDP_SDP0_DIR: u32                                     = 0x00000100; /* SDP0 IO direction */
pub const IXGBE_ESDP_SDP1_DIR: u32                                     = 0x00000200; /* SDP1 IO direction */
pub const IXGBE_ESDP_SDP2_DIR: u32                                     = 0x00000400; /* SDP1 IO direction */
pub const IXGBE_ESDP_SDP3_DIR: u32                                     = 0x00000800; /* SDP3 IO direction */
pub const IXGBE_ESDP_SDP4_DIR: u32                                     = 0x00001000; /* SDP4 IO direction */
pub const IXGBE_ESDP_SDP5_DIR: u32                                     = 0x00002000; /* SDP5 IO direction */
pub const IXGBE_ESDP_SDP6_DIR: u32                                     = 0x00004000; /* SDP6 IO direction */
pub const IXGBE_ESDP_SDP7_DIR: u32                                     = 0x00008000; /* SDP7 IO direction */
pub const IXGBE_ESDP_SDP0_NATIVE: u32                                  = 0x00010000; /* SDP0 IO mode */
pub const IXGBE_ESDP_SDP1_NATIVE: u32                                  = 0x00020000; /* SDP1 IO mode */


/* LEDCTL Bit Masks */
pub const IXGBE_LED_IVRT_BASE: u32                                     = 0x00000040;
pub const IXGBE_LED_BLINK_BASE: u32                                    = 0x00000080;
pub const IXGBE_LED_MODE_MASK_BASE: u32                                = 0x0000000F;

pub fn IXGBE_LED_OFFSET(base: u32, i: u32) -> u32 { base << (8 * i) }

pub fn IXGBE_LED_MODE_SHIFT(i: u32) -> u32 { 8 * i }

pub fn IXGBE_LED_IVRT(i: u32) -> u32 { IXGBE_LED_OFFSET(IXGBE_LED_IVRT_BASE, i) }

pub fn IXGBE_LED_BLINK(i: u32) -> u32 { IXGBE_LED_OFFSET(IXGBE_LED_BLINK_BASE, i) }

pub fn IXGBE_LED_MODE_MASK(i: u32) -> u32 { IXGBE_LED_OFFSET(IXGBE_LED_MODE_MASK_BASE, i) }

pub const IXGBE_X557_LED_MANUAL_SET_MASK: u32                          = 1 << 8;
pub const IXGBE_X557_MAX_LED_INDEX: u32                                = 3;
pub const IXGBE_X557_LED_PROVISIONING: u32                             = 0xC430;

/* LED modes */
pub const IXGBE_LED_LINK_UP: u32                                       = 0x0;
pub const IXGBE_LED_LINK_10G: u32                                      = 0x1;
pub const IXGBE_LED_MAC: u32                                           = 0x2;
pub const IXGBE_LED_FILTER: u32                                        = 0x3;
pub const IXGBE_LED_LINK_ACTIVE: u32                                   = 0x4;
pub const IXGBE_LED_LINK_1G: u32                                       = 0x5;
pub const IXGBE_LED_ON: u32                                            = 0xE;
pub const IXGBE_LED_OFF: u32                                           = 0xF;

/* AUTOC Bit Masks */
pub const IXGBE_AUTOC_KX4_KX_SUPP_MASK: u32                            = 0xC0000000;
pub const IXGBE_AUTOC_KX4_SUPP: u32                                    = 0x80000000;
pub const IXGBE_AUTOC_KX_SUPP: u32                                     = 0x40000000;
pub const IXGBE_AUTOC_PAUSE: u32                                       = 0x30000000;
pub const IXGBE_AUTOC_ASM_PAUSE: u32                                   = 0x20000000;
pub const IXGBE_AUTOC_SYM_PAUSE: u32                                   = 0x10000000;
pub const IXGBE_AUTOC_RF: u32                                          = 0x08000000;
pub const IXGBE_AUTOC_PD_TMR: u32                                      = 0x06000000;
pub const IXGBE_AUTOC_AN_RX_LOOSE: u32                                 = 0x01000000;
pub const IXGBE_AUTOC_AN_RX_DRIFT: u32                                 = 0x00800000;
pub const IXGBE_AUTOC_AN_RX_ALIGN: u32                                 = 0x007C0000;
pub const IXGBE_AUTOC_FECA: u32                                        = 0x00040000;
pub const IXGBE_AUTOC_FECR: u32                                        = 0x00020000;
pub const IXGBE_AUTOC_KR_SUPP: u32                                     = 0x00010000;
pub const IXGBE_AUTOC_AN_RESTART: u32                                  = 0x00001000;
pub const IXGBE_AUTOC_FLU: u32                                         = 0x00000001;
pub const IXGBE_AUTOC_LMS_SHIFT: u32                                   = 13;
pub const IXGBE_AUTOC_LMS_10G_SERIAL: u32                              = 0x3 << IXGBE_AUTOC_LMS_SHIFT;
pub const IXGBE_AUTOC_LMS_KX4_KX_KR: u32                               = 0x4 << IXGBE_AUTOC_LMS_SHIFT;
pub const IXGBE_AUTOC_LMS_SGMII_1G_100M: u32                           = 0x5 << IXGBE_AUTOC_LMS_SHIFT;
pub const IXGBE_AUTOC_LMS_KX4_KX_KR_1G_AN: u32                         = 0x6 << IXGBE_AUTOC_LMS_SHIFT;
pub const IXGBE_AUTOC_LMS_KX4_KX_KR_SGMII: u32                         = 0x7 << IXGBE_AUTOC_LMS_SHIFT;
pub const IXGBE_AUTOC_LMS_MASK: u32                                    = 0x7 << IXGBE_AUTOC_LMS_SHIFT;
pub const IXGBE_AUTOC_LMS_1G_LINK_NO_AN: u32                           = 0x0 << IXGBE_AUTOC_LMS_SHIFT;
pub const IXGBE_AUTOC_LMS_10G_LINK_NO_AN: u32                          = 0x1 << IXGBE_AUTOC_LMS_SHIFT;
pub const IXGBE_AUTOC_LMS_1G_AN: u32                                   = 0x2 << IXGBE_AUTOC_LMS_SHIFT;
pub const IXGBE_AUTOC_LMS_KX4_AN: u32                                  = 0x4 << IXGBE_AUTOC_LMS_SHIFT;
pub const IXGBE_AUTOC_LMS_KX4_AN_1G_AN: u32                            = 0x6 << IXGBE_AUTOC_LMS_SHIFT;
pub const IXGBE_AUTOC_LMS_ATTACH_TYPE: u32                             = 0x7 << IXGBE_AUTOC_10G_PMA_PMD_SHIFT;

pub const IXGBE_AUTOC_1G_PMA_PMD_MASK: u32                             = 0x00000200;
pub const IXGBE_AUTOC_1G_PMA_PMD_SHIFT: u32                            = 9;
pub const IXGBE_AUTOC_10G_PMA_PMD_MASK: u32                            = 0x00000180;
pub const IXGBE_AUTOC_10G_PMA_PMD_SHIFT: u32                           = 7;
pub const IXGBE_AUTOC_10G_XAUI: u32                                    = 0x0 << IXGBE_AUTOC_10G_PMA_PMD_SHIFT;
pub const IXGBE_AUTOC_10G_KX4: u32                                     = 0x1 << IXGBE_AUTOC_10G_PMA_PMD_SHIFT;
pub const IXGBE_AUTOC_10G_CX4: u32                                     = 0x2 << IXGBE_AUTOC_10G_PMA_PMD_SHIFT;
pub const IXGBE_AUTOC_1G_BX: u32                                       = 0x0 << IXGBE_AUTOC_1G_PMA_PMD_SHIFT;
pub const IXGBE_AUTOC_1G_KX: u32                                       = 0x1 << IXGBE_AUTOC_1G_PMA_PMD_SHIFT;
pub const IXGBE_AUTOC_1G_SFI: u32                                      = 0x0 << IXGBE_AUTOC_1G_PMA_PMD_SHIFT;
pub const IXGBE_AUTOC_1G_KX_BX: u32                                    = 0x1 << IXGBE_AUTOC_1G_PMA_PMD_SHIFT;

pub const IXGBE_AUTOC2_UPPER_MASK: u32                                 = 0xFFFF0000;
pub const IXGBE_AUTOC2_10G_SERIAL_PMA_PMD_MASK: u32                    = 0x00030000;
pub const IXGBE_AUTOC2_10G_SERIAL_PMA_PMD_SHIFT: u32                   = 16;
pub const IXGBE_AUTOC2_10G_KR: u32                                     = 0x0 << IXGBE_AUTOC2_10G_SERIAL_PMA_PMD_SHIFT;
pub const IXGBE_AUTOC2_10G_XFI: u32                                    = 0x1 << IXGBE_AUTOC2_10G_SERIAL_PMA_PMD_SHIFT;
pub const IXGBE_AUTOC2_10G_SFI: u32                                    = 0x2 << IXGBE_AUTOC2_10G_SERIAL_PMA_PMD_SHIFT;
pub const IXGBE_AUTOC2_LINK_DISABLE_ON_D3_MASK: u32                    = 0x50000000;
pub const IXGBE_AUTOC2_LINK_DISABLE_MASK: u32                          = 0x70000000;

pub const IXGBE_MACC_FLU: u32                                          = 0x00000001;
pub const IXGBE_MACC_FSV_10G: u32                                      = 0x00030000;
pub const IXGBE_MACC_FS: u32                                           = 0x00040000;
pub const IXGBE_MAC_RX2TX_LPBK: u32                                    = 0x00000002;

/* Veto Bit definiton */
pub const IXGBE_MMNGC_MNG_VETO: u32                                    = 0x00000001;

/* LINKS Bit Masks */
pub const IXGBE_LINKS_KX_AN_COMP: u32                                  = 0x80000000;
pub const IXGBE_LINKS_UP: u32                                          = 0x40000000;
pub const IXGBE_LINKS_SPEED: u32                                       = 0x20000000;
pub const IXGBE_LINKS_MODE: u32                                        = 0x18000000;
pub const IXGBE_LINKS_RX_MODE: u32                                     = 0x06000000;
pub const IXGBE_LINKS_TX_MODE: u32                                     = 0x01800000;
pub const IXGBE_LINKS_XGXS_EN: u32                                     = 0x00400000;
pub const IXGBE_LINKS_SGMII_EN: u32                                    = 0x02000000;
pub const IXGBE_LINKS_PCS_1G_EN: u32                                   = 0x00200000;
pub const IXGBE_LINKS_1G_AN_EN: u32                                    = 0x00100000;
pub const IXGBE_LINKS_KX_AN_IDLE: u32                                  = 0x00080000;
pub const IXGBE_LINKS_1G_SYNC: u32                                     = 0x00040000;
pub const IXGBE_LINKS_10G_ALIGN: u32                                   = 0x00020000;
pub const IXGBE_LINKS_10G_LANE_SYNC: u32                               = 0x00017000;
pub const IXGBE_LINKS_TL_FAULT: u32                                    = 0x00001000;
pub const IXGBE_LINKS_SIGNAL: u32                                      = 0x00000F00;

pub const IXGBE_LINKS_SPEED_NON_STD: u32                               = 0x08000000;
pub const IXGBE_LINKS_SPEED_82599: u32                                 = 0x30000000;
pub const IXGBE_LINKS_SPEED_10G_82599: u32                             = 0x30000000;
pub const IXGBE_LINKS_SPEED_1G_82599: u32                              = 0x20000000;
pub const IXGBE_LINKS_SPEED_100_82599: u32                             = 0x10000000;
pub const IXGBE_LINKS_SPEED_10_X550EM_A: u32                           = 0x00000000;
pub const IXGBE_LINK_UP_TIME: u32                                      = 90; /* 9.0 Seconds */
pub const IXGBE_AUTO_NEG_TIME: u32                                     = 45; /* 4.5 Seconds */

pub const IXGBE_LINKS2_AN_SUPPORTED: u32                               = 0x00000040;

/* PCS1GLSTA Bit Masks */
pub const IXGBE_PCS1GLSTA_LINK_OK: u32                                 = 1;
pub const IXGBE_PCS1GLSTA_SYNK_OK: u32                                 = 0x10;
pub const IXGBE_PCS1GLSTA_AN_COMPLETE: u32                             = 0x10000;
pub const IXGBE_PCS1GLSTA_AN_PAGE_RX: u32                              = 0x20000;
pub const IXGBE_PCS1GLSTA_AN_TIMED_OUT: u32                            = 0x40000;
pub const IXGBE_PCS1GLSTA_AN_REMOTE_FAULT: u32                         = 0x80000;
pub const IXGBE_PCS1GLSTA_AN_ERROR_RWS: u32                            = 0x100000;

pub const IXGBE_PCS1GANA_SYM_PAUSE: u32                                = 0x80;
pub const IXGBE_PCS1GANA_ASM_PAUSE: u32                                = 0x100;

/* PCS1GLCTL Bit Masks */
pub const IXGBE_PCS1GLCTL_AN_1G_TIMEOUT_EN: u32                        = 0x00040000; /* PCS 1G autoneg to en */
pub const IXGBE_PCS1GLCTL_FLV_LINK_UP: u32                             = 1;
pub const IXGBE_PCS1GLCTL_FORCE_LINK: u32                              = 0x20;
pub const IXGBE_PCS1GLCTL_LOW_LINK_LATCH: u32                          = 0x40;
pub const IXGBE_PCS1GLCTL_AN_ENABLE: u32                               = 0x10000;
pub const IXGBE_PCS1GLCTL_AN_RESTART: u32                              = 0x20000;

/* ANLP1 Bit Masks */
pub const IXGBE_ANLP1_PAUSE: u32                                       = 0x0C00;
pub const IXGBE_ANLP1_SYM_PAUSE: u32                                   = 0x0400;
pub const IXGBE_ANLP1_ASM_PAUSE: u32                                   = 0x0800;
pub const IXGBE_ANLP1_AN_STATE_MASK: u32                               = 0x000f0000;

/* SW Semaphore Register bitmasks */
pub const IXGBE_SWSM_SMBI: u32                                         = 0x00000001; /* Driver Semaphore bit */
pub const IXGBE_SWSM_SWESMBI: u32                                      = 0x00000002; /* FW Semaphore bit */
pub const IXGBE_SWSM_WMNG: u32                                         = 0x00000004; /* Wake MNG Clock */
pub const IXGBE_SWFW_REGSMP: u32                                       = 0x80000000; /* Register Semaphore bit 31 */

/* SW_FW_SYNC/GSSR definitions */
pub const IXGBE_GSSR_EEP_SM: u32                                       = 0x0001;
pub const IXGBE_GSSR_PHY0_SM: u32                                      = 0x0002;
pub const IXGBE_GSSR_PHY1_SM: u32                                      = 0x0004;
pub const IXGBE_GSSR_MAC_CSR_SM: u32                                   = 0x0008;
pub const IXGBE_GSSR_FLASH_SM: u32                                     = 0x0010;
pub const IXGBE_GSSR_NVM_UPDATE_SM: u32                                = 0x0200;
pub const IXGBE_GSSR_SW_MNG_SM: u32                                    = 0x0400;
pub const IXGBE_GSSR_TOKEN_SM: u32                                     = 0x40000000; /* SW bit for shared access */
pub const IXGBE_GSSR_SHARED_I2C_SM: u32                                = 0x1806; /* Wait for both phys and both I2Cs */
pub const IXGBE_GSSR_I2C_MASK: u32                                     = 0x1800;
pub const IXGBE_GSSR_NVM_PHY_MASK: u32                                 = 0xF;

/* FW Status register bitmask */
pub const IXGBE_FWSTS_FWRI: u32                                        = 0x00000200; /* Firmware Reset Indication */

/* EEC Register */
pub const IXGBE_EEC_SK: u32                                            = 0x00000001; /* EEPROM Clock */
pub const IXGBE_EEC_CS: u32                                            = 0x00000002; /* EEPROM Chip Select */
pub const IXGBE_EEC_DI: u32                                            = 0x00000004; /* EEPROM Data In */
pub const IXGBE_EEC_DO: u32                                            = 0x00000008; /* EEPROM Data Out */
pub const IXGBE_EEC_FWE_MASK: u32                                      = 0x00000030; /* FLASH Write Enable */
pub const IXGBE_EEC_FWE_DIS: u32                                       = 0x00000010; /* Disable FLASH writes */
pub const IXGBE_EEC_FWE_EN: u32                                        = 0x00000020; /* Enable FLASH writes */
pub const IXGBE_EEC_FWE_SHIFT: u32                                     = 4;
pub const IXGBE_EEC_REQ: u32                                           = 0x00000040; /* EEPROM Access Request */
pub const IXGBE_EEC_GNT: u32                                           = 0x00000080; /* EEPROM Access Grant */
pub const IXGBE_EEC_PRES: u32                                          = 0x00000100; /* EEPROM Present */
pub const IXGBE_EEC_ARD: u32                                           = 0x00000200; /* EEPROM Auto Read Done */
pub const IXGBE_EEC_FLUP: u32                                          = 0x00800000; /* Flash update command */
pub const IXGBE_EEC_SEC1VAL: u32                                       = 0x02000000; /* Sector 1 Valid */
pub const IXGBE_EEC_FLUDONE: u32                                       = 0x04000000; /* Flash update done */
/* EEPROM Addressing bits based on type (0-small, 1-large) */
pub const IXGBE_EEC_ADDR_SIZE: u32                                     = 0x00000400;
pub const IXGBE_EEC_SIZE: u32                                          = 0x00007800; /* EEPROM Size */
pub const IXGBE_EERD_MAX_ADDR: u32                                     = 0x00003FFF; /* EERD alows 14 bits for addr. */

pub const IXGBE_EEC_SIZE_SHIFT: u32                                    = 11;
pub const IXGBE_EEPROM_WORD_SIZE_SHIFT: u32                            = 6;
pub const IXGBE_EEPROM_OPCODE_BITS: u32                                = 8;

/* FLA Register */
pub const IXGBE_FLA_LOCKED: u32                                        = 0x00000040;

/* Part Number String Length */
pub const IXGBE_PBANUM_LENGTH: u32                                     = 11;

/* Checksum and EEPROM pointers */
pub const IXGBE_PBANUM_PTR_GUARD: u32                                  = 0xFAFA;
pub const IXGBE_EEPROM_CHECKSUM: u32                                   = 0x3F;
pub const IXGBE_EEPROM_SUM: u32                                        = 0xBABA;
pub const IXGBE_EEPROM_CTRL_4: u32                                     = 0x45;
pub const IXGBE_EE_CTRL_4_INST_ID: u32                                 = 0x10;
pub const IXGBE_EE_CTRL_4_INST_ID_SHIFT: u32                           = 4;
pub const IXGBE_PCIE_ANALOG_PTR: u32                                   = 0x03;
pub const IXGBE_ATLAS0_CONFIG_PTR: u32                                 = 0x04;
pub const IXGBE_PHY_PTR: u32                                           = 0x04;
pub const IXGBE_ATLAS1_CONFIG_PTR: u32                                 = 0x05;
pub const IXGBE_OPTION_ROM_PTR: u32                                    = 0x05;
pub const IXGBE_PCIE_GENERAL_PTR: u32                                  = 0x06;
pub const IXGBE_PCIE_CONFIG0_PTR: u32                                  = 0x07;
pub const IXGBE_PCIE_CONFIG1_PTR: u32                                  = 0x08;
pub const IXGBE_CORE0_PTR: u32                                         = 0x09;
pub const IXGBE_CORE1_PTR: u32                                         = 0x0A;
pub const IXGBE_MAC0_PTR: u32                                          = 0x0B;
pub const IXGBE_MAC1_PTR: u32                                          = 0x0C;
pub const IXGBE_CSR0_CONFIG_PTR: u32                                   = 0x0D;
pub const IXGBE_CSR1_CONFIG_PTR: u32                                   = 0x0E;
pub const IXGBE_PCIE_ANALOG_PTR_X550: u32                              = 0x02;
pub const IXGBE_SHADOW_RAM_SIZE_X550: u32                              = 0x4000;
pub const IXGBE_IXGBE_PCIE_GENERAL_SIZE: u32                           = 0x24;
pub const IXGBE_PCIE_CONFIG_SIZE: u32                                  = 0x08;
pub const IXGBE_EEPROM_LAST_WORD: u32                                  = 0x41;
pub const IXGBE_FW_PTR: u32                                            = 0x0F;
pub const IXGBE_PBANUM0_PTR: u32                                       = 0x15;
pub const IXGBE_PBANUM1_PTR: u32                                       = 0x16;
pub const IXGBE_ALT_MAC_ADDR_PTR: u32                                  = 0x37;
pub const IXGBE_FREE_SPACE_PTR: u32                                    = 0x3E;

/* External Thermal Sensor Config */
pub const IXGBE_ETS_CFG: u32                                           = 0x26;
pub const IXGBE_ETS_LTHRES_DELTA_MASK: u32                             = 0x07C0;
pub const IXGBE_ETS_LTHRES_DELTA_SHIFT: u32                            = 6;
pub const IXGBE_ETS_TYPE_MASK: u32                                     = 0x0038;
pub const IXGBE_ETS_TYPE_SHIFT: u32                                    = 3;
pub const IXGBE_ETS_TYPE_EMC: u32                                      = 0x000;
pub const IXGBE_ETS_NUM_SENSORS_MASK: u32                              = 0x0007;
pub const IXGBE_ETS_DATA_LOC_MASK: u32                                 = 0x3C00;
pub const IXGBE_ETS_DATA_LOC_SHIFT: u32                                = 10;
pub const IXGBE_ETS_DATA_INDEX_MASK: u32                               = 0x0300;
pub const IXGBE_ETS_DATA_INDEX_SHIFT: u32                              = 8;
pub const IXGBE_ETS_DATA_HTHRESH_MASK: u32                             = 0x00FF;

pub const IXGBE_SAN_MAC_ADDR_PTR: u32                                  = 0x28;
pub const IXGBE_DEVICE_CAPS: u32                                       = 0x2C;
pub const IXGBE_82599_SERIAL_NUMBER_MAC_ADDR: u32                      = 0x11;
pub const IXGBE_X550_SERIAL_NUMBER_MAC_ADDR: u32                       = 0x04;

pub const IXGBE_PCIE_MSIX_82599_CAPS: u32                              = 0x72;
pub const IXGBE_MAX_MSIX_VECTORS_82599: u32                            = 0x40;
pub const IXGBE_PCIE_MSIX_82598_CAPS: u32                              = 0x62;
pub const IXGBE_MAX_MSIX_VECTORS_82598: u32                            = 0x13;

/* MSI-X capability fields masks */
pub const IXGBE_PCIE_MSIX_TBL_SZ_MASK: u32                             = 0x7FF;

/* Legacy EEPROM word offsets */
pub const IXGBE_ISCSI_BOOT_CAPS: u32                                   = 0x0033;
pub const IXGBE_ISCSI_SETUP_PORT_0: u32                                = 0x0030;
pub const IXGBE_ISCSI_SETUP_PORT_1: u32                                = 0x0034;

/* EEPROM Commands - SPI */
pub const IXGBE_EEPROM_MAX_RETRY_SPI: u32                              = 5000; /* Max wait 5ms for RDY signal */
pub const IXGBE_EEPROM_STATUS_RDY_SPI: u32                             = 0x01;
pub const IXGBE_EEPROM_READ_OPCODE_SPI: u32                            = 0x03;  /* EEPROM read opcode */
pub const IXGBE_EEPROM_WRITE_OPCODE_SPI: u32                           = 0x02;  /* EEPROM write opcode */
pub const IXGBE_EEPROM_A8_OPCODE_SPI: u32                              = 0x08;  /* opcode bit-3 = addr bit-8 */
pub const IXGBE_EEPROM_WREN_OPCODE_SPI: u32                            = 0x06;  /* EEPROM set Write Ena latch */
/* EEPROM reset Write Enable latch */
pub const IXGBE_EEPROM_WRDI_OPCODE_SPI: u32                            = 0x04;
pub const IXGBE_EEPROM_RDSR_OPCODE_SPI: u32                            = 0x05;  /* EEPROM read Status reg */
pub const IXGBE_EEPROM_WRSR_OPCODE_SPI: u32                            = 0x01;  /* EEPROM write Status reg */
pub const IXGBE_EEPROM_ERASE4K_OPCODE_SPI: u32                         = 0x20;  /* EEPROM ERASE 4KB */
pub const IXGBE_EEPROM_ERASE64K_OPCODE_SPI: u32                        = 0xD8;  /* EEPROM ERASE 64KB */
pub const IXGBE_EEPROM_ERASE256_OPCODE_SPI: u32                        = 0xDB;  /* EEPROM ERASE 256B */

/* EEPROM Read Register */
pub const IXGBE_EEPROM_RW_REG_DATA: u32                                = 16; /* data offset in EEPROM read reg */
pub const IXGBE_EEPROM_RW_REG_DONE: u32                                = 2; /* Offset to READ done bit */
pub const IXGBE_EEPROM_RW_REG_START: u32                               = 1; /* First bit to start operation */
pub const IXGBE_EEPROM_RW_ADDR_SHIFT: u32                              = 2; /* Shift to the address bits */
pub const IXGBE_NVM_POLL_WRITE: u32                                    = 1; /* Flag for polling for wr complete */
pub const IXGBE_NVM_POLL_READ: u32                                     = 0; /* Flag for polling for rd complete */

pub const NVM_INIT_CTRL_3: u32                                         = 0x38;
pub const NVM_INIT_CTRL_3_LPLU: u32                                    = 0x8;
pub const NVM_INIT_CTRL_3_D10GMP_PORT0: u32                            = 0x40;
pub const NVM_INIT_CTRL_3_D10GMP_PORT1: u32                            = 0x100;

pub const IXGBE_ETH_LENGTH_OF_ADDRESS: u32                             = 6;

pub const IXGBE_EEPROM_PAGE_SIZE_MAX: u32                              = 128;
pub const IXGBE_EEPROM_RD_BUFFER_MAX_COUNT: u32                        = 256; /* words rd in burst */
pub const IXGBE_EEPROM_WR_BUFFER_MAX_COUNT: u32                        = 256; /* words wr in burst */
pub const IXGBE_EEPROM_CTRL_2: u32                                     = 1; /* EEPROM CTRL word 2 */
pub const IXGBE_EEPROM_CCD_BIT: u32                                    = 2;


pub const IXGBE_EEPROM_GRANT_ATTEMPTS: u32                             = 1000; /* EEPROM attempts to gain grant */

/* Number of 5 microseconds we wait for EERD read and;
 * EERW write to complete */
pub const IXGBE_EERD_EEWR_ATTEMPTS: u32                                = 100000;

/* # attempts we wait for flush update to complete */
pub const IXGBE_FLUDONE_ATTEMPTS: u32                                  = 20000;

pub const IXGBE_PCIE_CTRL2: u32                                        = 0x5;   /* PCIe Control 2 Offset */
pub const IXGBE_PCIE_CTRL2_DUMMY_ENABLE: u32                           = 0x8;   /* Dummy Function Enable */
pub const IXGBE_PCIE_CTRL2_LAN_DISABLE: u32                            = 0x2;   /* LAN PCI Disable */
pub const IXGBE_PCIE_CTRL2_DISABLE_SELECT: u32                         = 0x1;   /* LAN Disable Select */

pub const IXGBE_SAN_MAC_ADDR_PORT0_OFFSET: u32                         = 0x0;
pub const IXGBE_SAN_MAC_ADDR_PORT1_OFFSET: u32                         = 0x3;
pub const IXGBE_DEVICE_CAPS_ALLOW_ANY_SFP: u32                         = 0x1;
pub const IXGBE_DEVICE_CAPS_FCOE_OFFLOADS: u32                         = 0x2;
pub const IXGBE_DEVICE_CAPS_NO_CROSSTALK_WR: u32                       = 1 << 7;
pub const IXGBE_FW_LESM_PARAMETERS_PTR: u32                            = 0x2;
pub const IXGBE_FW_LESM_STATE_1: u32                                   = 0x1;
pub const IXGBE_FW_LESM_STATE_ENABLED: u32                             = 0x8000; /* LESM Enable bit */
pub const IXGBE_FW_PASSTHROUGH_PATCH_CONFIG_PTR: u32                   = 0x4;
pub const IXGBE_FW_PATCH_VERSION_4: u32                                = 0x7;
pub const IXGBE_FCOE_IBA_CAPS_BLK_PTR: u32                             = 0x33; /* iSCSI/FCOE block */
pub const IXGBE_FCOE_IBA_CAPS_FCOE: u32                                = 0x20; /* FCOE flags */
pub const IXGBE_ISCSI_FCOE_BLK_PTR: u32                                = 0x17; /* iSCSI/FCOE block */
pub const IXGBE_ISCSI_FCOE_FLAGS_OFFSET: u32                           = 0x0; /* FCOE flags */
pub const IXGBE_ISCSI_FCOE_FLAGS_ENABLE: u32                           = 0x1; /* FCOE flags enable bit */
pub const IXGBE_ALT_SAN_MAC_ADDR_BLK_PTR: u32                          = 0x27; /* Alt. SAN MAC block */
pub const IXGBE_ALT_SAN_MAC_ADDR_CAPS_OFFSET: u32                      = 0x0; /* Alt SAN MAC capability */
pub const IXGBE_ALT_SAN_MAC_ADDR_PORT0_OFFSET: u32                     = 0x1; /* Alt SAN MAC 0 offset */
pub const IXGBE_ALT_SAN_MAC_ADDR_PORT1_OFFSET: u32                     = 0x4; /* Alt SAN MAC 1 offset */
pub const IXGBE_ALT_SAN_MAC_ADDR_WWNN_OFFSET: u32                      = 0x7; /* Alt WWNN prefix offset */
pub const IXGBE_ALT_SAN_MAC_ADDR_WWPN_OFFSET: u32                      = 0x8; /* Alt WWPN prefix offset */
pub const IXGBE_ALT_SAN_MAC_ADDR_CAPS_SANMAC: u32                      = 0x0; /* Alt SAN MAC exists */
pub const IXGBE_ALT_SAN_MAC_ADDR_CAPS_ALTWWN: u32                      = 0x1; /* Alt WWN base exists */

/* FW header offset */
pub const IXGBE_X540_FW_PASSTHROUGH_PATCH_CONFIG_PTR: u32              = 0x4;
pub const IXGBE_X540_FW_MODULE_MASK: u32                               = 0x7FFF;
/* 4KB multiplier */
pub const IXGBE_X540_FW_MODULE_LENGTH: u32                             = 0x1000;
/* version word 2 (month & day) */
pub const IXGBE_X540_FW_PATCH_VERSION_2: u32                           = 0x5;
/* version word 3 (silicon compatibility & year) */
pub const IXGBE_X540_FW_PATCH_VERSION_3: u32                           = 0x6;
/* version word 4 (major & minor numbers) */
pub const IXGBE_X540_FW_PATCH_VERSION_4: u32                           = 0x7;

pub const IXGBE_DEVICE_CAPS_WOL_PORT0_1: u32                           = 0x4; /* WoL supported on ports 0 & 1 */
pub const IXGBE_DEVICE_CAPS_WOL_PORT0: u32                             = 0x8; /* WoL supported on port 0 */
pub const IXGBE_DEVICE_CAPS_WOL_MASK: u32                              = 0xC; /* Mask for WoL capabilities */

/* PCI Bus Info */
pub const IXGBE_PCI_DEVICE_STATUS: u32                                 = 0xAA;
pub const IXGBE_PCI_DEVICE_STATUS_TRANSACTION_PENDING: u32             = 0x0020;
pub const IXGBE_PCI_LINK_STATUS: u32                                   = 0xB2;
pub const IXGBE_PCI_DEVICE_CONTROL2: u32                               = 0xC8;
pub const IXGBE_PCI_LINK_WIDTH: u32                                    = 0x3F0;
pub const IXGBE_PCI_LINK_WIDTH_1: u32                                  = 0x10;
pub const IXGBE_PCI_LINK_WIDTH_2: u32                                  = 0x20;
pub const IXGBE_PCI_LINK_WIDTH_4: u32                                  = 0x40;
pub const IXGBE_PCI_LINK_WIDTH_8: u32                                  = 0x80;
pub const IXGBE_PCI_LINK_SPEED: u32                                    = 0xF;
pub const IXGBE_PCI_LINK_SPEED_2500: u32                               = 0x1;
pub const IXGBE_PCI_LINK_SPEED_5000: u32                               = 0x2;
pub const IXGBE_PCI_LINK_SPEED_8000: u32                               = 0x3;
pub const IXGBE_PCI_HEADER_TYPE_REGISTER: u32                          = 0x0E;
pub const IXGBE_PCI_HEADER_TYPE_MULTIFUNC: u32                         = 0x80;
pub const IXGBE_PCI_DEVICE_CONTROL2_16ms: u32                          = 0x0005;

pub const IXGBE_PCIDEVCTRL2_TIMEO_MASK: u32                            = 0xf;
pub const IXGBE_PCIDEVCTRL2_16_32ms_def: u32                           = 0x0;
pub const IXGBE_PCIDEVCTRL2_50_100us: u32                              = 0x1;
pub const IXGBE_PCIDEVCTRL2_1_2ms: u32                                 = 0x2;
pub const IXGBE_PCIDEVCTRL2_16_32ms: u32                               = 0x5;
pub const IXGBE_PCIDEVCTRL2_65_130ms: u32                              = 0x6;
pub const IXGBE_PCIDEVCTRL2_260_520ms: u32                             = 0x9;
pub const IXGBE_PCIDEVCTRL2_1_2s: u32                                  = 0xa;
pub const IXGBE_PCIDEVCTRL2_4_8s: u32                                  = 0xd;
pub const IXGBE_PCIDEVCTRL2_17_34s: u32                                = 0xe;

/* Number of 100 microseconds we wait for PCI Express master disable */
pub const IXGBE_PCI_MASTER_DISABLE_TIMEOUT: u32                        = 800;

/* Check whether address is multicast. This is little-endian specific check.*/
pub unsafe fn IXGBE_IS_MULTICAST(Address: *const u8) -> bool { (*Address & 0x01) != 0 }

/* Check whether an address is broadcast. */
pub unsafe fn IXGBE_IS_BROADCAST(Address: *const u8) -> bool { *Address == 0xff && *Address.offset(1) == 0xff }

/* RAH */
pub const IXGBE_RAH_VIND_MASK: u32                                     = 0x003C0000;
pub const IXGBE_RAH_VIND_SHIFT: u32                                    = 18;
pub const IXGBE_RAH_AV: u32                                            = 0x80000000;
pub const IXGBE_CLEAR_VMDQ_ALL: u32                                    = 0xFFFFFFFF;

/* Header split receive */
pub const IXGBE_RFCTL_ISCSI_DIS: u32                                   = 0x00000001;
pub const IXGBE_RFCTL_ISCSI_DWC_MASK: u32                              = 0x0000003E;
pub const IXGBE_RFCTL_ISCSI_DWC_SHIFT: u32                             = 1;
pub const IXGBE_RFCTL_RSC_DIS: u32                                     = 0x00000020;
pub const IXGBE_RFCTL_NFSW_DIS: u32                                    = 0x00000040;
pub const IXGBE_RFCTL_NFSR_DIS: u32                                    = 0x00000080;
pub const IXGBE_RFCTL_NFS_VER_MASK: u32                                = 0x00000300;
pub const IXGBE_RFCTL_NFS_VER_SHIFT: u32                               = 8;
pub const IXGBE_RFCTL_NFS_VER_2: u32                                   = 0;
pub const IXGBE_RFCTL_NFS_VER_3: u32                                   = 1;
pub const IXGBE_RFCTL_NFS_VER_4: u32                                   = 2;
pub const IXGBE_RFCTL_IPV6_DIS: u32                                    = 0x00000400;
pub const IXGBE_RFCTL_IPV6_XSUM_DIS: u32                               = 0x00000800;
pub const IXGBE_RFCTL_IPFRSP_DIS: u32                                  = 0x00004000;
pub const IXGBE_RFCTL_IPV6_EX_DIS: u32                                 = 0x00010000;
pub const IXGBE_RFCTL_NEW_IPV6_EXT_DIS: u32                            = 0x00020000;

/* Transmit Config masks */
pub const IXGBE_TXDCTL_ENABLE: u32                                     = 0x02000000; /* Ena specific Tx Queue */
pub const IXGBE_TXDCTL_SWFLSH: u32                                     = 0x04000000; /* Tx Desc. wr-bk flushing */
pub const IXGBE_TXDCTL_WTHRESH_SHIFT: u32                              = 16; /* shift to WTHRESH bits */
/* Enable short packet padding to 64 bytes */
pub const IXGBE_TX_PAD_ENABLE: u32                                     = 0x00000400;
pub const IXGBE_JUMBO_FRAME_ENABLE: u32                                = 0x00000004;  /* Allow jumbo frames */
/* This allows for 16K packets + 4k for vlan */
pub const IXGBE_MAX_FRAME_SZ: u32                                      = 0x40040000;

pub const IXGBE_TDWBAL_HEAD_WB_ENABLE: u32                             = 0x1; /* Tx head write-back enable */
pub const IXGBE_TDWBAL_SEQNUM_WB_ENABLE: u32                           = 0x2; /* Tx seq# write-back enable */

/* Receive Config masks */
pub const IXGBE_RXCTRL_RXEN: u32                                       = 0x00000001; /* Enable Receiver */
pub const IXGBE_RXCTRL_DMBYPS: u32                                     = 0x00000002; /* Desc Monitor Bypass */
pub const IXGBE_RXDCTL_ENABLE: u32                                     = 0x02000000; /* Ena specific Rx Queue */
pub const IXGBE_RXDCTL_SWFLSH: u32                                     = 0x04000000; /* Rx Desc wr-bk flushing */
pub const IXGBE_RXDCTL_RLPMLMASK: u32                                  = 0x00003FFF; /* X540 supported only */
pub const IXGBE_RXDCTL_RLPML_EN: u32                                   = 0x00008000;
pub const IXGBE_RXDCTL_VME: u32                                        = 0x40000000; /* VLAN mode enable */

pub const IXGBE_TSAUXC_EN_CLK: u32                                     = 0x00000004;
pub const IXGBE_TSAUXC_SYNCLK: u32                                     = 0x00000008;
pub const IXGBE_TSAUXC_SDP0_INT: u32                                   = 0x00000040;
pub const IXGBE_TSAUXC_EN_TT0: u32                                     = 0x00000001;
pub const IXGBE_TSAUXC_EN_TT1: u32                                     = 0x00000002;
pub const IXGBE_TSAUXC_ST0: u32                                        = 0x00000010;
pub const IXGBE_TSAUXC_DISABLE_SYSTIME: u32                            = 0x80000000;

pub const IXGBE_TSSDP_TS_SDP0_SEL_MASK: u32                            = 0x000000C0;
pub const IXGBE_TSSDP_TS_SDP0_CLK0: u32                                = 0x00000080;
pub const IXGBE_TSSDP_TS_SDP0_EN: u32                                  = 0x00000100;

pub const IXGBE_TSYNCTXCTL_VALID: u32                                  = 0x00000001; /* Tx timestamp valid */
pub const IXGBE_TSYNCTXCTL_ENABLED: u32                                = 0x00000010; /* Tx timestamping enabled */

pub const IXGBE_TSYNCRXCTL_VALID: u32                                  = 0x00000001; /* Rx timestamp valid */
pub const IXGBE_TSYNCRXCTL_TYPE_MASK: u32                              = 0x0000000E; /* Rx type mask */
pub const IXGBE_TSYNCRXCTL_TYPE_L2_V2: u32                             = 0x00;
pub const IXGBE_TSYNCRXCTL_TYPE_L4_V1: u32                             = 0x02;
pub const IXGBE_TSYNCRXCTL_TYPE_L2_L4_V2: u32                          = 0x04;
pub const IXGBE_TSYNCRXCTL_TYPE_ALL: u32                               = 0x08;
pub const IXGBE_TSYNCRXCTL_TYPE_EVENT_V2: u32                          = 0x0A;
pub const IXGBE_TSYNCRXCTL_ENABLED: u32                                = 0x00000010; /* Rx Timestamping enabled */
pub const IXGBE_TSYNCRXCTL_TSIP_UT_EN: u32                             = 0x00800000; /* Rx Timestamp in Packet */
pub const IXGBE_TSYNCRXCTL_TSIP_UP_MASK: u32                           = 0xFF000000; /* Rx Timestamp UP Mask */

pub const IXGBE_TSIM_SYS_WRAP: u32                                     = 0x00000001;
pub const IXGBE_TSIM_TXTS: u32                                         = 0x00000002;
pub const IXGBE_TSIM_TADJ: u32                                         = 0x00000080;

pub const IXGBE_TSICR_SYS_WRAP: u32                                    = IXGBE_TSIM_SYS_WRAP;
pub const IXGBE_TSICR_TXTS: u32                                        = IXGBE_TSIM_TXTS;
pub const IXGBE_TSICR_TADJ: u32                                        = IXGBE_TSIM_TADJ;

pub const IXGBE_RXMTRL_V1_CTRLT_MASK: u32                              = 0x000000FF;
pub const IXGBE_RXMTRL_V1_SYNC_MSG: u32                                = 0x00;
pub const IXGBE_RXMTRL_V1_DELAY_REQ_MSG: u32                           = 0x01;
pub const IXGBE_RXMTRL_V1_FOLLOWUP_MSG: u32                            = 0x02;
pub const IXGBE_RXMTRL_V1_DELAY_RESP_MSG: u32                          = 0x03;
pub const IXGBE_RXMTRL_V1_MGMT_MSG: u32                                = 0x04;

pub const IXGBE_RXMTRL_V2_MSGID_MASK: u32                              = 0x0000FF00;
pub const IXGBE_RXMTRL_V2_SYNC_MSG: u32                                = 0x0000;
pub const IXGBE_RXMTRL_V2_DELAY_REQ_MSG: u32                           = 0x0100;
pub const IXGBE_RXMTRL_V2_PDELAY_REQ_MSG: u32                          = 0x0200;
pub const IXGBE_RXMTRL_V2_PDELAY_RESP_MSG: u32                         = 0x0300;
pub const IXGBE_RXMTRL_V2_FOLLOWUP_MSG: u32                            = 0x0800;
pub const IXGBE_RXMTRL_V2_DELAY_RESP_MSG: u32                          = 0x0900;
pub const IXGBE_RXMTRL_V2_PDELAY_FOLLOWUP_MSG: u32                     = 0x0A00;
pub const IXGBE_RXMTRL_V2_ANNOUNCE_MSG: u32                            = 0x0B00;
pub const IXGBE_RXMTRL_V2_SIGNALLING_MSG: u32                          = 0x0C00;
pub const IXGBE_RXMTRL_V2_MGMT_MSG: u32                                = 0x0D00;

pub const IXGBE_FCTRL_SBP: u32                                         = 0x00000002; /* Store Bad Packet */
pub const IXGBE_FCTRL_MPE: u32                                         = 0x00000100; /* Multicast Promiscuous Ena*/
pub const IXGBE_FCTRL_UPE: u32                                         = 0x00000200; /* Unicast Promiscuous Ena */
pub const IXGBE_FCTRL_BAM: u32                                         = 0x00000400; /* Broadcast Accept Mode */
pub const IXGBE_FCTRL_PMCF: u32                                        = 0x00001000; /* Pass MAC Control Frames */
pub const IXGBE_FCTRL_DPF: u32                                         = 0x00002000; /* Discard Pause Frame */
/* Receive Priority Flow Control Enable */
pub const IXGBE_FCTRL_RPFCE: u32                                       = 0x00004000;
pub const IXGBE_FCTRL_RFCE: u32                                        = 0x00008000; /* Receive Flow Control Ena */
pub const IXGBE_MFLCN_PMCF: u32                                        = 0x00000001; /* Pass MAC Control Frames */
pub const IXGBE_MFLCN_DPF: u32                                         = 0x00000002; /* Discard Pause Frame */
pub const IXGBE_MFLCN_RPFCE: u32                                       = 0x00000004; /* Receive Priority FC Enable */
pub const IXGBE_MFLCN_RFCE: u32                                        = 0x00000008; /* Receive FC Enable */
pub const IXGBE_MFLCN_RPFCE_MASK: u32                                  = 0x00000FF4; /* Rx Priority FC bitmap mask */
pub const IXGBE_MFLCN_RPFCE_SHIFT: u32                                 = 4; /* Rx Priority FC bitmap shift */

/* Multiple Receive Queue Control */
pub const IXGBE_MRQC_RSSEN: u32                                        = 0x00000001;  /* RSS Enable */
pub const IXGBE_MRQC_MRQE_MASK: u32                                    = 0xF; /* Bits 3:0 */
pub const IXGBE_MRQC_RT8TCEN: u32                                      = 0x00000002; /* 8 TC no RSS */
pub const IXGBE_MRQC_RT4TCEN: u32                                      = 0x00000003; /* 4 TC no RSS */
pub const IXGBE_MRQC_RTRSS8TCEN: u32                                   = 0x00000004; /* 8 TC w/ RSS */
pub const IXGBE_MRQC_RTRSS4TCEN: u32                                   = 0x00000005; /* 4 TC w/ RSS */
pub const IXGBE_MRQC_VMDQEN: u32                                       = 0x00000008; /* VMDq2 64 pools no RSS */
pub const IXGBE_MRQC_VMDQRSS32EN: u32                                  = 0x0000000A; /* VMDq2 32 pools w/ RSS */
pub const IXGBE_MRQC_VMDQRSS64EN: u32                                  = 0x0000000B; /* VMDq2 64 pools w/ RSS */
pub const IXGBE_MRQC_VMDQRT8TCEN: u32                                  = 0x0000000C; /* VMDq2/RT 16 pool 8 TC */
pub const IXGBE_MRQC_VMDQRT4TCEN: u32                                  = 0x0000000D; /* VMDq2/RT 32 pool 4 TC */
pub const IXGBE_MRQC_L3L4TXSWEN: u32                                   = 0x00008000; /* Enable L3/L4 Tx switch */
pub const IXGBE_MRQC_RSS_FIELD_MASK: u32                               = 0xFFFF0000;
pub const IXGBE_MRQC_RSS_FIELD_IPV4_TCP: u32                           = 0x00010000;
pub const IXGBE_MRQC_RSS_FIELD_IPV4: u32                               = 0x00020000;
pub const IXGBE_MRQC_RSS_FIELD_IPV6_EX_TCP: u32                        = 0x00040000;
pub const IXGBE_MRQC_RSS_FIELD_IPV6_EX: u32                            = 0x00080000;
pub const IXGBE_MRQC_RSS_FIELD_IPV6: u32                               = 0x00100000;
pub const IXGBE_MRQC_RSS_FIELD_IPV6_TCP: u32                           = 0x00200000;
pub const IXGBE_MRQC_RSS_FIELD_IPV4_UDP: u32                           = 0x00400000;
pub const IXGBE_MRQC_RSS_FIELD_IPV6_UDP: u32                           = 0x00800000;
pub const IXGBE_MRQC_RSS_FIELD_IPV6_EX_UDP: u32                        = 0x01000000;
pub const IXGBE_MRQC_MULTIPLE_RSS: u32                                 = 0x00002000;

/* Queue Drop Enable */
pub const IXGBE_QDE_ENABLE: u32                                        = 0x00000001;
pub const IXGBE_QDE_HIDE_VLAN: u32                                     = 0x00000002;
pub const IXGBE_QDE_IDX_MASK: u32                                      = 0x00007F00;
pub const IXGBE_QDE_IDX_SHIFT: u32                                     = 8;
pub const IXGBE_QDE_WRITE: u32                                         = 0x00010000;
pub const IXGBE_QDE_READ: u32                                          = 0x00020000;

pub const IXGBE_TXD_POPTS_IXSM: u32                                    = 0x01; /* Insert IP checksum */
pub const IXGBE_TXD_POPTS_TXSM: u32                                    = 0x02; /* Insert TCP/UDP checksum */
pub const IXGBE_TXD_CMD_EOP: u32                                       = 0x01000000; /* End of Packet */
pub const IXGBE_TXD_CMD_IFCS: u32                                      = 0x02000000; /* Insert FCS (Ethernet CRC) */
pub const IXGBE_TXD_CMD_IC: u32                                        = 0x04000000; /* Insert Checksum */
pub const IXGBE_TXD_CMD_RS: u32                                        = 0x08000000; /* Report Status */
pub const IXGBE_TXD_CMD_DEXT: u32                                      = 0x20000000; /* Desc extension (0   = legacy) */
pub const IXGBE_TXD_CMD_VLE: u32                                       = 0x40000000; /* Add VLAN tag */
pub const IXGBE_TXD_STAT_DD: u32                                       = 0x00000001; /* Descriptor Done */

pub const IXGBE_RXDADV_IPSEC_STATUS_SECP: u32                          = 0x00020000;
pub const IXGBE_RXDADV_IPSEC_ERROR_INVALID_PROTOCOL: u32               = 0x08000000;
pub const IXGBE_RXDADV_IPSEC_ERROR_INVALID_LENGTH: u32                 = 0x10000000;
pub const IXGBE_RXDADV_IPSEC_ERROR_AUTH_FAILED: u32                    = 0x18000000;
pub const IXGBE_RXDADV_IPSEC_ERROR_BIT_MASK: u32                       = 0x18000000;
/* Multiple Transmit Queue Command Register */
pub const IXGBE_MTQC_RT_ENA: u32                                       = 0x1; /* DCB Enable */
pub const IXGBE_MTQC_VT_ENA: u32                                       = 0x2; /* VMDQ2 Enable */
pub const IXGBE_MTQC_64Q_1PB: u32                                      = 0x0; /* 64 queues 1 pack buffer */
pub const IXGBE_MTQC_32VF: u32                                         = 0x8; /* 4 TX Queues per pool w/32VF's */
pub const IXGBE_MTQC_64VF: u32                                         = 0x4; /* 2 TX Queues per pool w/64VF's */
pub const IXGBE_MTQC_4TC_4TQ: u32                                      = 0x8; /* 4 TC if RT_ENA and VT_ENA */
pub const IXGBE_MTQC_8TC_8TQ: u32                                      = 0xC; /* 8 TC if RT_ENA or 8 TQ if VT_ENA */

/* Receive Descriptor bit definitions */
pub const IXGBE_RXD_STAT_DD: u32                                       = 0x01; /* Descriptor Done */
pub const IXGBE_RXD_STAT_EOP: u32                                      = 0x02; /* End of Packet */
pub const IXGBE_RXD_STAT_FLM: u32                                      = 0x04; /* FDir Match */
pub const IXGBE_RXD_STAT_VP: u32                                       = 0x08; /* IEEE VLAN Packet */
pub const IXGBE_RXDADV_NEXTP_MASK: u32                                 = 0x000FFFF0; /* Next Descriptor Index */
pub const IXGBE_RXDADV_NEXTP_SHIFT: u32                                = 0x00000004;
pub const IXGBE_RXD_STAT_UDPCS: u32                                    = 0x10; /* UDP xsum calculated */
pub const IXGBE_RXD_STAT_L4CS: u32                                     = 0x20; /* L4 xsum calculated */
pub const IXGBE_RXD_STAT_IPCS: u32                                     = 0x40; /* IP xsum calculated */
pub const IXGBE_RXD_STAT_PIF: u32                                      = 0x80; /* passed in-exact filter */
pub const IXGBE_RXD_STAT_CRCV: u32                                     = 0x100; /* Speculative CRC Valid */
pub const IXGBE_RXD_STAT_OUTERIPCS: u32                                = 0x100; /* Cloud IP xsum calculated */
pub const IXGBE_RXD_STAT_VEXT: u32                                     = 0x200; /* 1st VLAN found */
pub const IXGBE_RXD_STAT_UDPV: u32                                     = 0x400; /* Valid UDP checksum */
pub const IXGBE_RXD_STAT_DYNINT: u32                                   = 0x800; /* Pkt caused INT via DYNINT */
pub const IXGBE_RXD_STAT_LLINT: u32                                    = 0x800; /* Pkt caused Low Latency Interrupt */
pub const IXGBE_RXD_STAT_TSIP: u32                                     = 0x08000; /* Time Stamp in packet buffer */
pub const IXGBE_RXD_STAT_TS: u32                                       = 0x10000; /* Time Stamp */
pub const IXGBE_RXD_STAT_SECP: u32                                     = 0x20000; /* Security Processing */
pub const IXGBE_RXD_STAT_LB: u32                                       = 0x40000; /* Loopback Status */
pub const IXGBE_RXD_STAT_ACK: u32                                      = 0x8000; /* ACK Packet indication */
pub const IXGBE_RXD_ERR_CE: u32                                        = 0x01; /* CRC Error */
pub const IXGBE_RXD_ERR_LE: u32                                        = 0x02; /* Length Error */
pub const IXGBE_RXD_ERR_PE: u32                                        = 0x08; /* Packet Error */
pub const IXGBE_RXD_ERR_OSE: u32                                       = 0x10; /* Oversize Error */
pub const IXGBE_RXD_ERR_USE: u32                                       = 0x20; /* Undersize Error */
pub const IXGBE_RXD_ERR_TCPE: u32                                      = 0x40; /* TCP/UDP Checksum Error */
pub const IXGBE_RXD_ERR_IPE: u32                                       = 0x80; /* IP Checksum Error */
pub const IXGBE_RXDADV_ERR_MASK: u32                                   = 0xfff00000; /* RDESC.ERRORS mask */
pub const IXGBE_RXDADV_ERR_SHIFT: u32                                  = 20; /* RDESC.ERRORS shift */
pub const IXGBE_RXDADV_ERR_OUTERIPER: u32                              = 0x04000000; /* CRC IP Header error */
pub const IXGBE_RXDADV_ERR_RXE: u32                                    = 0x20000000; /* Any MAC Error */
pub const IXGBE_RXDADV_ERR_FCEOFE: u32                                 = 0x80000000; /* FCEOFe/IPE */
pub const IXGBE_RXDADV_ERR_FCERR: u32                                  = 0x00700000; /* FCERR/FDIRERR */
pub const IXGBE_RXDADV_ERR_FDIR_LEN: u32                               = 0x00100000; /* FDIR Length error */
pub const IXGBE_RXDADV_ERR_FDIR_DROP: u32                              = 0x00200000; /* FDIR Drop error */
pub const IXGBE_RXDADV_ERR_FDIR_COLL: u32                              = 0x00400000; /* FDIR Collision error */
pub const IXGBE_RXDADV_ERR_HBO: u32                                    = 0x00800000; /*Header Buffer Overflow */
pub const IXGBE_RXDADV_ERR_CE: u32                                     = 0x01000000; /* CRC Error */
pub const IXGBE_RXDADV_ERR_LE: u32                                     = 0x02000000; /* Length Error */
pub const IXGBE_RXDADV_ERR_PE: u32                                     = 0x08000000; /* Packet Error */
pub const IXGBE_RXDADV_ERR_OSE: u32                                    = 0x10000000; /* Oversize Error */
pub const IXGBE_RXDADV_ERR_USE: u32                                    = 0x20000000; /* Undersize Error */
pub const IXGBE_RXDADV_ERR_TCPE: u32                                   = 0x40000000; /* TCP/UDP Checksum Error */
pub const IXGBE_RXDADV_ERR_IPE: u32                                    = 0x80000000; /* IP Checksum Error */
pub const IXGBE_RXD_VLAN_ID_MASK: u32                                  = 0x0FFF;  /* VLAN ID is in lower 12 bits */
pub const IXGBE_RXD_PRI_MASK: u32                                      = 0xE000;  /* Priority is in upper 3 bits */
pub const IXGBE_RXD_PRI_SHIFT: u32                                     = 13;
pub const IXGBE_RXD_CFI_MASK: u32                                      = 0x1000;  /* CFI is bit 12 */
pub const IXGBE_RXD_CFI_SHIFT: u32                                     = 12;

pub const IXGBE_RXDADV_STAT_DD: u32                                    = IXGBE_RXD_STAT_DD;  /* Done */
pub const IXGBE_RXDADV_STAT_EOP: u32                                   = IXGBE_RXD_STAT_EOP; /* End of Packet */
pub const IXGBE_RXDADV_STAT_FLM: u32                                   = IXGBE_RXD_STAT_FLM; /* FDir Match */
pub const IXGBE_RXDADV_STAT_VP: u32                                    = IXGBE_RXD_STAT_VP;  /* IEEE VLAN Pkt */
pub const IXGBE_RXDADV_STAT_MASK: u32                                  = 0x000fffff; /* Stat/NEXTP: bit 0-19 */
pub const IXGBE_RXDADV_STAT_FCEOFS: u32                                = 0x00000040; /* FCoE EOF/SOF Stat */
pub const IXGBE_RXDADV_STAT_FCSTAT: u32                                = 0x00000030; /* FCoE Pkt Stat */
pub const IXGBE_RXDADV_STAT_FCSTAT_NOMTCH: u32                         = 0x00000000; /* 00: No Ctxt Match */
pub const IXGBE_RXDADV_STAT_FCSTAT_NODDP: u32                          = 0x00000010; /* 01: Ctxt w/o DDP */
pub const IXGBE_RXDADV_STAT_FCSTAT_FCPRSP: u32                         = 0x00000020; /* 10: Recv. FCP_RSP */
pub const IXGBE_RXDADV_STAT_FCSTAT_DDP: u32                            = 0x00000030; /* 11: Ctxt w/ DDP */
pub const IXGBE_RXDADV_STAT_TS: u32                                    = 0x00010000; /* IEEE1588 Time Stamp */
pub const IXGBE_RXDADV_STAT_TSIP: u32                                  = 0x00008000; /* Time Stamp in packet buffer */

/* PSRTYPE bit definitions */
pub const IXGBE_PSRTYPE_TCPHDR: u32                                    = 0x00000010;
pub const IXGBE_PSRTYPE_UDPHDR: u32                                    = 0x00000020;
pub const IXGBE_PSRTYPE_IPV4HDR: u32                                   = 0x00000100;
pub const IXGBE_PSRTYPE_IPV6HDR: u32                                   = 0x00000200;
pub const IXGBE_PSRTYPE_L2HDR: u32                                     = 0x00001000;

/* SRRCTL bit definitions */
pub const IXGBE_SRRCTL_BSIZEPKT_SHIFT: u32                             = 10; /* so many KBs */
pub const IXGBE_SRRCTL_BSIZEHDRSIZE_SHIFT: u32                         = 2; /* 64byte resolution (>> 6)
					                                                         * + at bit 8 offset (<< 8)
					                                                         * = (<< 2)
					                                                         */
pub const IXGBE_SRRCTL_RDMTS_SHIFT: u32                                = 22;
pub const IXGBE_SRRCTL_RDMTS_MASK: u32                                 = 0x01C00000;
pub const IXGBE_SRRCTL_DROP_EN: u32                                    = 0x10000000;
pub const IXGBE_SRRCTL_BSIZEPKT_MASK: u32                              = 0x0000007F;
pub const IXGBE_SRRCTL_BSIZEHDR_MASK: u32                              = 0x00003F00;
pub const IXGBE_SRRCTL_DESCTYPE_LEGACY: u32                            = 0x00000000;
pub const IXGBE_SRRCTL_DESCTYPE_ADV_ONEBUF: u32                        = 0x02000000;
pub const IXGBE_SRRCTL_DESCTYPE_HDR_SPLIT: u32                         = 0x04000000;
pub const IXGBE_SRRCTL_DESCTYPE_HDR_REPLICATION_LARGE_PKT: u32         = 0x08000000;
pub const IXGBE_SRRCTL_DESCTYPE_HDR_SPLIT_ALWAYS: u32                  = 0x0A000000;
pub const IXGBE_SRRCTL_DESCTYPE_MASK: u32                              = 0x0E000000;

pub const IXGBE_RXDPS_HDRSTAT_HDRSP: u32                               = 0x00008000;
pub const IXGBE_RXDPS_HDRSTAT_HDRLEN_MASK: u32                         = 0x000003FF;

pub const IXGBE_RXDADV_RSSTYPE_MASK: u32                               = 0x0000000F;
pub const IXGBE_RXDADV_PKTTYPE_MASK: u32                               = 0x0000FFF0;
pub const IXGBE_RXDADV_PKTTYPE_MASK_EX: u32                            = 0x0001FFF0;
pub const IXGBE_RXDADV_HDRBUFLEN_MASK: u32                             = 0x00007FE0;
pub const IXGBE_RXDADV_RSCCNT_MASK: u32                                = 0x001E0000;
pub const IXGBE_RXDADV_RSCCNT_SHIFT: u32                               = 17;
pub const IXGBE_RXDADV_HDRBUFLEN_SHIFT: u32                            = 5;
pub const IXGBE_RXDADV_SPLITHEADER_EN: u32                             = 0x00001000;
pub const IXGBE_RXDADV_SPH: u32                                        = 0x8000;

/* RSS Hash results */
pub const IXGBE_RXDADV_RSSTYPE_NONE: u32                               = 0x00000000;
pub const IXGBE_RXDADV_RSSTYPE_IPV4_TCP: u32                           = 0x00000001;
pub const IXGBE_RXDADV_RSSTYPE_IPV4: u32                               = 0x00000002;
pub const IXGBE_RXDADV_RSSTYPE_IPV6_TCP: u32                           = 0x00000003;
pub const IXGBE_RXDADV_RSSTYPE_IPV6_EX: u32                            = 0x00000004;
pub const IXGBE_RXDADV_RSSTYPE_IPV6: u32                               = 0x00000005;
pub const IXGBE_RXDADV_RSSTYPE_IPV6_TCP_EX: u32                        = 0x00000006;
pub const IXGBE_RXDADV_RSSTYPE_IPV4_UDP: u32                           = 0x00000007;
pub const IXGBE_RXDADV_RSSTYPE_IPV6_UDP: u32                           = 0x00000008;
pub const IXGBE_RXDADV_RSSTYPE_IPV6_UDP_EX: u32                        = 0x00000009;

/* RSS Packet Types as indicated in the receive descriptor. */
pub const IXGBE_RXDADV_PKTTYPE_NONE: u32                               = 0x00000000;
pub const IXGBE_RXDADV_PKTTYPE_IPV4: u32                               = 0x00000010; /* IPv4 hdr present */
pub const IXGBE_RXDADV_PKTTYPE_IPV4_EX: u32                            = 0x00000020; /* IPv4 hdr + extensions */
pub const IXGBE_RXDADV_PKTTYPE_IPV6: u32                               = 0x00000040; /* IPv6 hdr present */
pub const IXGBE_RXDADV_PKTTYPE_IPV6_EX: u32                            = 0x00000080; /* IPv6 hdr + extensions */
pub const IXGBE_RXDADV_PKTTYPE_TCP: u32                                = 0x00000100; /* TCP hdr present */
pub const IXGBE_RXDADV_PKTTYPE_UDP: u32                                = 0x00000200; /* UDP hdr present */
pub const IXGBE_RXDADV_PKTTYPE_SCTP: u32                               = 0x00000400; /* SCTP hdr present */
pub const IXGBE_RXDADV_PKTTYPE_NFS: u32                                = 0x00000800; /* NFS hdr present */
pub const IXGBE_RXDADV_PKTTYPE_GENEVE: u32                             = 0x00000800; /* GENEVE hdr present */
pub const IXGBE_RXDADV_PKTTYPE_VXLAN: u32                              = 0x00000800; /* VXLAN hdr present */
pub const IXGBE_RXDADV_PKTTYPE_TUNNEL: u32                             = 0x00010000; /* Tunnel type */
pub const IXGBE_RXDADV_PKTTYPE_IPSEC_ESP: u32                          = 0x00001000; /* IPSec ESP */
pub const IXGBE_RXDADV_PKTTYPE_IPSEC_AH: u32                           = 0x00002000; /* IPSec AH */
pub const IXGBE_RXDADV_PKTTYPE_LINKSEC: u32                            = 0x00004000; /* LinkSec Encap */
pub const IXGBE_RXDADV_PKTTYPE_ETQF: u32                               = 0x00008000; /* PKTTYPE is ETQF index */
pub const IXGBE_RXDADV_PKTTYPE_ETQF_MASK: u32                          = 0x00000070; /* ETQF has 8 indices */
pub const IXGBE_RXDADV_PKTTYPE_ETQF_SHIFT: u32                         = 4; /* Right-shift 4 bits */

/* Security Processing bit Indication */
pub const IXGBE_RXDADV_LNKSEC_STATUS_SECP: u32                         = 0x00020000;
pub const IXGBE_RXDADV_LNKSEC_ERROR_NO_SA_MATCH: u32                   = 0x08000000;
pub const IXGBE_RXDADV_LNKSEC_ERROR_REPLAY_ERROR: u32                  = 0x10000000;
pub const IXGBE_RXDADV_LNKSEC_ERROR_BIT_MASK: u32                      = 0x18000000;
pub const IXGBE_RXDADV_LNKSEC_ERROR_BAD_SIG: u32                       = 0x18000000;

/* Masks to determine if packets should be dropped due to frame errors */
pub const IXGBE_RXD_ERR_FRAME_ERR_MASK: u32                            = IXGBE_RXD_ERR_CE | IXGBE_RXD_ERR_LE | IXGBE_RXD_ERR_PE | IXGBE_RXD_ERR_OSE | IXGBE_RXD_ERR_USE;

pub const IXGBE_RXDADV_ERR_FRAME_ERR_MASK: u32                         = IXGBE_RXDADV_ERR_CE | IXGBE_RXDADV_ERR_LE | IXGBE_RXDADV_ERR_PE | IXGBE_RXDADV_ERR_OSE | IXGBE_RXDADV_ERR_USE;

pub const IXGBE_RXDADV_ERR_FRAME_ERR_MASK_82599: u32                   = IXGBE_RXDADV_ERR_RXE;

/* Multicast bit mask */
pub const IXGBE_MCSTCTRL_MFE: u32                                      = 0x4;

/* Number of Transmit and Receive Descriptors must be a multiple of 8 */
pub const IXGBE_REQ_TX_DESCRIPTOR_MULTIPLE: u32                        = 8;
pub const IXGBE_REQ_RX_DESCRIPTOR_MULTIPLE: u32                        = 8;
pub const IXGBE_REQ_TX_BUFFER_GRANULARITY: u32                         = 1024;

/* Vlan-specific macros */
pub const IXGBE_RX_DESC_SPECIAL_VLAN_MASK: u32                         = 0x0FFF; /* VLAN ID in lower 12 bits */
pub const IXGBE_RX_DESC_SPECIAL_PRI_MASK: u32                          = 0xE000; /* Priority in upper 3 bits */
pub const IXGBE_RX_DESC_SPECIAL_PRI_SHIFT: u32                         = 0x000D; /* Priority in upper 3 of 16 */
pub const IXGBE_TX_DESC_SPECIAL_PRI_SHIFT: u32                         = IXGBE_RX_DESC_SPECIAL_PRI_SHIFT;

/* SR-IOV specific macros */
pub fn IXGBE_MBVFICR_INDEX(vf_number: u32) -> u32 { vf_number >> 4 }

pub fn IXGBE_MBVFICR(i: u32) -> u32 { 0x00710 + i * 4 }

pub fn IXGBE_VFLRE(i: u32) -> u32 { if (i & 1) != 0 { 0x001C0 } else { 0x00600 } }

pub fn IXGBE_VFLREC(i: u32) -> u32 { 0x00700 + i * 4 }
/* Translated register consts */
pub fn IXGBE_PVFCTRL(P: u32) -> u32 { 0x00300 + 4 * P }

pub fn IXGBE_PVFSTATUS(P: u32) -> u32 { 0x00008 + 0 * P }

pub fn IXGBE_PVFLINKS(P: u32) -> u32 { 0x042A4 + 0 * P }

pub fn IXGBE_PVFRTIMER(P: u32) -> u32 { 0x00048 + 0 * P }

pub fn IXGBE_PVFMAILBOX(P: u32) -> u32 { 0x04C00 + 4 * P }

pub fn IXGBE_PVFRXMEMWRAP(P: u32) -> u32 { 0x03190 + 0 * P }

pub fn IXGBE_PVTEICR(P: u32) -> u32 { 0x00B00 + 4 * P }

pub fn IXGBE_PVTEICS(P: u32) -> u32 { 0x00C00 + 4 * P }

pub fn IXGBE_PVTEIMS(P: u32) -> u32 { 0x00D00 + 4 * P }

pub fn IXGBE_PVTEIMC(P: u32) -> u32 { 0x00E00 + 4 * P }

pub fn IXGBE_PVTEIAC(P: u32) -> u32 { 0x00F00 + 4 * P }

pub fn IXGBE_PVTEIAM(P: u32) -> u32 { 0x04D00 + 4 * P }

pub fn IXGBE_PVTEITR(P: u32) -> u32 { if P < 24 { 0x00820 + P * 4 } else { 0x012300 + ((P - 24) * 4) } }

pub fn IXGBE_PVTIVAR(P: u32) -> u32 { 0x12500 + 4 * P }

pub fn IXGBE_PVTIVAR_MISC(P: u32) -> u32 { 0x04E00 + 4 * P }

pub fn IXGBE_PVTRSCINT(P: u32) -> u32 { 0x12000 + 4 * P }

pub fn IXGBE_VFPBACL(P: u32) -> u32 { 0x110C8 + 4 * P }

pub fn IXGBE_PVFRDBAL(P: u32) -> u32 { if P < 64 { 0x01000 + 0x40 * P } else { 0x0D000 + (0x40 * (P - 64)) } }

pub fn IXGBE_PVFRDBAH(P: u32) -> u32 { if P < 64 { 0x01004 + 0x40 * P } else { 0x0D004 + (0x40 * (P - 64)) } }

pub fn IXGBE_PVFRDLEN(P: u32) -> u32 { if P < 64 { 0x01008 + 0x40 * P } else { 0x0D008 + (0x40 * (P - 64)) } }

pub fn IXGBE_PVFRDH(P: u32) -> u32 { if P < 64 { 0x01010 + 0x40 * P } else { 0x0D010 + (0x40 * (P - 64)) } }

pub fn IXGBE_PVFRDT(P: u32) -> u32 { if P < 64 { 0x01018 + 0x40 * P } else { 0x0D018 + (0x40 * (P - 64)) } }

pub fn IXGBE_PVFRXDCTL(P: u32) -> u32 { if P < 64 { 0x01028 + 0x40 * P } else { 0x0D028 + (0x40 * (P - 64)) } }

pub fn IXGBE_PVFSRRCTL(P: u32) -> u32 { if P < 64 { 0x01014 + 0x40 * P } else { 0x0D014 + (0x40 * (P - 64)) } }

pub fn IXGBE_PVFPSRTYPE(P: u32) -> u32 { 0x0EA00 + 4 * P }

pub fn IXGBE_PVFTDBAL(P: u32) -> u32 { 0x06000 + 0x40 * P }

pub fn IXGBE_PVFTDBAH(P: u32) -> u32 { 0x06004 + 0x40 * P }

pub fn IXGBE_PVFTDLEN(P: u32) -> u32 { 0x06008 + 0x40 * P }

pub fn IXGBE_PVFTDH(P: u32) -> u32 { 0x06010 + 0x40 * P }

pub fn IXGBE_PVFTDT(P: u32) -> u32 { 0x06018 + 0x40 * P }

pub fn IXGBE_PVFTXDCTL(P: u32) -> u32 { 0x06028 + 0x40 * P }

pub fn IXGBE_PVFTDWBAL(P: u32) -> u32 { 0x06038 + 0x40 * P }

pub fn IXGBE_PVFTDWBAH(P: u32) -> u32 { 0x0603C + 0x40 * P }

pub fn IXGBE_PVFDCA_RXCTRL(P: u32) -> u32 { if P < 64 { 0x0100C + 0x40 * P } else { 0x0D00C + 0x40 * (P - 64) } }
pub fn IXGBE_PVFDCA_TXCTRL(P: u32) -> u32 { 0x0600C + 0x40 * P }

pub fn IXGBE_PVFGPRC(x: u32) -> u32 { 0x0101C + 0x40 * x }

pub fn IXGBE_PVFGPTC(x: u32) -> u32 { 0x08300 + 0x04 * x }

pub fn IXGBE_PVFGORC_LSB(x: u32) -> u32 { 0x01020 + 0x40 * x }

pub fn IXGBE_PVFGORC_MSB(x: u32) -> u32 { 0x0D020 + 0x40 * x }

pub fn IXGBE_PVFGOTC_LSB(x: u32) -> u32 { 0x08400 + 0x08 * x }

pub fn IXGBE_PVFGOTC_MSB(x: u32) -> u32 { 0x08404 + 0x08 * x }

pub fn IXGBE_PVFMPRC(x: u32) -> u32 { 0x0D01C + 0x40 * x }

pub fn IXGBE_PVFTDWBALn(q_per_pool: u32, vf_number: u32, vf_q_index: u32) -> u32 { IXGBE_PVFTDWBAL((q_per_pool) * (vf_number) + (vf_q_index)) }

pub fn IXGBE_PVFTDWBAHn(q_per_pool: u32, vf_number: u32, vf_q_index: u32) -> u32 { IXGBE_PVFTDWBAH((q_per_pool) * (vf_number) + (vf_q_index)) }

pub fn IXGBE_PVFTDHn(q_per_pool: u32, vf_number: u32, vf_q_index: u32) -> u32 { IXGBE_PVFTDH((q_per_pool) * (vf_number) + (vf_q_index)) }

pub fn IXGBE_PVFTDTn(q_per_pool: u32, vf_number: u32, vf_q_index: u32) -> u32 { IXGBE_PVFTDT((q_per_pool) * (vf_number) + (vf_q_index)) }

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ixgbe_fdir_pballoc_type {
    IXGBE_FDIR_PBALLOC_NONE                                            = 0,
    IXGBE_FDIR_PBALLOC_64K                                             = 1,
    IXGBE_FDIR_PBALLOC_128K                                            = 2,
    IXGBE_FDIR_PBALLOC_256K                                            = 3,
}

/* Flow Director register values */
pub const IXGBE_FDIRCTRL_PBALLOC_64K: u32                              = 0x00000001;
pub const IXGBE_FDIRCTRL_PBALLOC_128K: u32                             = 0x00000002;
pub const IXGBE_FDIRCTRL_PBALLOC_256K: u32                             = 0x00000003;
pub const IXGBE_FDIRCTRL_INIT_DONE: u32                                = 0x00000008;
pub const IXGBE_FDIRCTRL_PERFECT_MATCH: u32                            = 0x00000010;
pub const IXGBE_FDIRCTRL_REPORT_STATUS: u32                            = 0x00000020;
pub const IXGBE_FDIRCTRL_REPORT_STATUS_ALWAYS: u32                     = 0x00000080;
pub const IXGBE_FDIRCTRL_DROP_Q_SHIFT: u32                             = 8;
pub const IXGBE_FDIRCTRL_DROP_Q_MASK: u32                              = 0x00007F00;
pub const IXGBE_FDIRCTRL_FLEX_SHIFT: u32                               = 16;
pub const IXGBE_FDIRCTRL_DROP_NO_MATCH: u32                            = 0x00008000;
pub const IXGBE_FDIRCTRL_FILTERMODE_SHIFT: u32                         = 21;
pub const IXGBE_FDIRCTRL_FILTERMODE_MACVLAN: u32                       = 0x0001; /* bit 23:21, 001b */
pub const IXGBE_FDIRCTRL_FILTERMODE_CLOUD: u32                         = 0x0002; /* bit 23:21, 010b */
pub const IXGBE_FDIRCTRL_SEARCHLIM: u32                                = 0x00800000;
pub const IXGBE_FDIRCTRL_FILTERMODE_MASK: u32                          = 0x00E00000;
pub const IXGBE_FDIRCTRL_MAX_LENGTH_SHIFT: u32                         = 24;
pub const IXGBE_FDIRCTRL_FULL_THRESH_MASK: u32                         = 0xF0000000;
pub const IXGBE_FDIRCTRL_FULL_THRESH_SHIFT: u32                        = 28;

pub const IXGBE_FDIRTCPM_DPORTM_SHIFT: u32                             = 16;
pub const IXGBE_FDIRUDPM_DPORTM_SHIFT: u32                             = 16;
pub const IXGBE_FDIRIP6M_DIPM_SHIFT: u32                               = 16;
pub const IXGBE_FDIRM_VLANID: u32                                      = 0x00000001;
pub const IXGBE_FDIRM_VLANP: u32                                       = 0x00000002;
pub const IXGBE_FDIRM_POOL: u32                                        = 0x00000004;
pub const IXGBE_FDIRM_L4P: u32                                         = 0x00000008;
pub const IXGBE_FDIRM_FLEX: u32                                        = 0x00000010;
pub const IXGBE_FDIRM_DIPv6: u32                                       = 0x00000020;
pub const IXGBE_FDIRM_L3P: u32                                         = 0x00000040;

pub const IXGBE_FDIRIP6M_INNER_MAC: u32                                = 0x03F0; /* bit 9:4 */
pub const IXGBE_FDIRIP6M_TUNNEL_TYPE: u32                              = 0x0800; /* bit 11 */
pub const IXGBE_FDIRIP6M_TNI_VNI: u32                                  = 0xF000; /* bit 15:12 */
pub const IXGBE_FDIRIP6M_TNI_VNI_24: u32                               = 0x1000; /* bit 12 */
pub const IXGBE_FDIRIP6M_ALWAYS_MASK: u32                              = 0x040F; /* bit 10, 3:0 */

pub const IXGBE_FDIRFREE_FREE_MASK: u32                                = 0xFFFF;
pub const IXGBE_FDIRFREE_FREE_SHIFT: u32                               = 0;
pub const IXGBE_FDIRFREE_COLL_MASK: u32                                = 0x7FFF0000;
pub const IXGBE_FDIRFREE_COLL_SHIFT: u32                               = 16;
pub const IXGBE_FDIRLEN_MAXLEN_MASK: u32                               = 0x3F;
pub const IXGBE_FDIRLEN_MAXLEN_SHIFT: u32                              = 0;
pub const IXGBE_FDIRLEN_MAXHASH_MASK: u32                              = 0x7FFF0000;
pub const IXGBE_FDIRLEN_MAXHASH_SHIFT: u32                             = 16;
pub const IXGBE_FDIRUSTAT_ADD_MASK: u32                                = 0xFFFF;
pub const IXGBE_FDIRUSTAT_ADD_SHIFT: u32                               = 0;
pub const IXGBE_FDIRUSTAT_REMOVE_MASK: u32                             = 0xFFFF0000;
pub const IXGBE_FDIRUSTAT_REMOVE_SHIFT: u32                            = 16;
pub const IXGBE_FDIRFSTAT_FADD_MASK: u32                               = 0x00FF;
pub const IXGBE_FDIRFSTAT_FADD_SHIFT: u32                              = 0;
pub const IXGBE_FDIRFSTAT_FREMOVE_MASK: u32                            = 0xFF00;
pub const IXGBE_FDIRFSTAT_FREMOVE_SHIFT: u32                           = 8;
pub const IXGBE_FDIRPORT_DESTINATION_SHIFT: u32                        = 16;
pub const IXGBE_FDIRVLAN_FLEX_SHIFT: u32                               = 16;
pub const IXGBE_FDIRHASH_BUCKET_VALID_SHIFT: u32                       = 15;
pub const IXGBE_FDIRHASH_SIG_SW_INDEX_SHIFT: u32                       = 16;

pub const IXGBE_FDIRCMD_CMD_MASK: u32                                  = 0x00000003;
pub const IXGBE_FDIRCMD_CMD_ADD_FLOW: u32                              = 0x00000001;
pub const IXGBE_FDIRCMD_CMD_REMOVE_FLOW: u32                           = 0x00000002;
pub const IXGBE_FDIRCMD_CMD_QUERY_REM_FILT: u32                        = 0x00000003;
pub const IXGBE_FDIRCMD_FILTER_VALID: u32                              = 0x00000004;
pub const IXGBE_FDIRCMD_FILTER_UPDATE: u32                             = 0x00000008;
pub const IXGBE_FDIRCMD_IPv6DMATCH: u32                                = 0x00000010;
pub const IXGBE_FDIRCMD_L4TYPE_UDP: u32                                = 0x00000020;
pub const IXGBE_FDIRCMD_L4TYPE_TCP: u32                                = 0x00000040;
pub const IXGBE_FDIRCMD_L4TYPE_SCTP: u32                               = 0x00000060;
pub const IXGBE_FDIRCMD_IPV6: u32                                      = 0x00000080;
pub const IXGBE_FDIRCMD_CLEARHT: u32                                   = 0x00000100;
pub const IXGBE_FDIRCMD_DROP: u32                                      = 0x00000200;
pub const IXGBE_FDIRCMD_INT: u32                                       = 0x00000400;
pub const IXGBE_FDIRCMD_LAST: u32                                      = 0x00000800;
pub const IXGBE_FDIRCMD_COLLISION: u32                                 = 0x00001000;
pub const IXGBE_FDIRCMD_QUEUE_EN: u32                                  = 0x00008000;
pub const IXGBE_FDIRCMD_FLOW_TYPE_SHIFT: u32                           = 5;
pub const IXGBE_FDIRCMD_RX_QUEUE_SHIFT: u32                            = 16;
pub const IXGBE_FDIRCMD_TUNNEL_FILTER_SHIFT: u32                       = 23;
pub const IXGBE_FDIRCMD_VT_POOL_SHIFT: u32                             = 24;
pub const IXGBE_FDIR_INIT_DONE_POLL: u32                               = 10;
pub const IXGBE_FDIRCMD_CMD_POLL: u32                                  = 10;
pub const IXGBE_FDIRCMD_TUNNEL_FILTER: u32                             = 0x00800000;
pub const IXGBE_FDIR_DROP_QUEUE: u32                                   = 127;


/* Manageablility Host Interface defines */
pub const IXGBE_HI_MAX_BLOCK_BYTE_LENGTH: u32                          = 1792; /* Num of bytes in range */
pub const IXGBE_HI_MAX_BLOCK_DWORD_LENGTH: u32                         = 448; /* Num of dwords in range */
pub const IXGBE_HI_COMMAND_TIMEOUT: u32                                = 500; /* Process HI command limit */
pub const IXGBE_HI_FLASH_ERASE_TIMEOUT: u32                            = 1000; /* Process Erase command limit */
pub const IXGBE_HI_FLASH_UPDATE_TIMEOUT: u32                           = 5000; /* Process Update command limit */
pub const IXGBE_HI_FLASH_APPLY_TIMEOUT: u32                            = 0; /* Process Apply command limit */
pub const IXGBE_HI_PHY_MGMT_REQ_TIMEOUT: u32                           = 2000; /* Wait up to 2 seconds */

/* CEM Support */
pub const FW_CEM_HDR_LEN: u32                                          = 0x4;
pub const FW_CEM_CMD_DRIVER_INFO: u32                                  = 0xDD;
pub const FW_CEM_CMD_DRIVER_INFO_LEN: u32                              = 0x5;
pub const FW_CEM_CMD_RESERVED: u32                                     = 0x0;
pub const FW_CEM_UNUSED_VER: u32                                       = 0x0;
pub const FW_CEM_MAX_RETRIES: u32                                      = 3;
pub const FW_CEM_RESP_STATUS_SUCCESS: u32                              = 0x1;
pub const FW_CEM_DRIVER_VERSION_SIZE: u32                              = 39; /* +9 would send 48 bytes to fw */
pub const FW_READ_SHADOW_RAM_CMD: u32                                  = 0x31;
pub const FW_READ_SHADOW_RAM_LEN: u32                                  = 0x6;
pub const FW_WRITE_SHADOW_RAM_CMD: u32                                 = 0x33;
pub const FW_WRITE_SHADOW_RAM_LEN: u32                                 = 0xA; /* 8 plus 1 WORD to write */
pub const FW_SHADOW_RAM_DUMP_CMD: u32                                  = 0x36;
pub const FW_SHADOW_RAM_DUMP_LEN: u32                                  = 0;
pub const FW_DEFAULT_CHECKSUM: u32                                     = 0xFF; /* checksum always 0xFF */
pub const FW_NVM_DATA_OFFSET: u32                                      = 3;
pub const FW_MAX_READ_BUFFER_SIZE: u32                                 = 1024;
pub const FW_DISABLE_RXEN_CMD: u32                                     = 0xDE;
pub const FW_DISABLE_RXEN_LEN: u32                                     = 0x1;
pub const FW_PHY_MGMT_REQ_CMD: u32                                     = 0x20;
pub const FW_PHY_TOKEN_REQ_CMD: u32                                    = 0xA;
pub const FW_PHY_TOKEN_REQ_LEN: u32                                    = 2;
pub const FW_PHY_TOKEN_REQ: u32                                        = 0;
pub const FW_PHY_TOKEN_REL: u32                                        = 1;
pub const FW_PHY_TOKEN_OK: u32                                         = 1;
pub const FW_PHY_TOKEN_RETRY: u32                                      = 0x80;
pub const FW_PHY_TOKEN_DELAY: u32                                      = 5;    /* milliseconds */
pub const FW_PHY_TOKEN_WAIT: u32                                       = 5;    /* seconds */
pub fn FW_PHY_TOKEN_RETRIES() -> u32 { FW_PHY_TOKEN_WAIT * 1000 / FW_PHY_TOKEN_DELAY }

pub const FW_INT_PHY_REQ_CMD: u32                                      = 0xB;
pub const FW_INT_PHY_REQ_LEN: u32                                      = 10;
pub const FW_INT_PHY_REQ_READ: u32                                     = 0;
pub const FW_INT_PHY_REQ_WRITE: u32                                    = 1;
pub const FW_PHY_ACT_REQ_CMD: u32                                      = 5;
pub const FW_PHY_ACT_DATA_COUNT: u32                                   = 4;
pub const FW_PHY_ACT_REQ_LEN: u32                                      = 4 + 4 * FW_PHY_ACT_DATA_COUNT;
pub const FW_PHY_ACT_INIT_PHY: u32                                     = 1;
pub const FW_PHY_ACT_SETUP_LINK: u32                                   = 2;
pub const FW_PHY_ACT_LINK_SPEED_10: u32                                = 1 << 0;
pub const FW_PHY_ACT_LINK_SPEED_100: u32                               = 1 << 1;
pub const FW_PHY_ACT_LINK_SPEED_1G: u32                                = 1 << 2;
pub const FW_PHY_ACT_LINK_SPEED_2_5G: u32                              = 1 << 3;
pub const FW_PHY_ACT_LINK_SPEED_5G: u32                                = 1 << 4;
pub const FW_PHY_ACT_LINK_SPEED_10G: u32                               = 1 << 5;
pub const FW_PHY_ACT_LINK_SPEED_20G: u32                               = 1 << 6;
pub const FW_PHY_ACT_LINK_SPEED_25G: u32                               = 1 << 7;
pub const FW_PHY_ACT_LINK_SPEED_40G: u32                               = 1 << 8;
pub const FW_PHY_ACT_LINK_SPEED_50G: u32                               = 1 << 9;
pub const FW_PHY_ACT_LINK_SPEED_100G: u32                              = 1 << 10;
pub const FW_PHY_ACT_SETUP_LINK_PAUSE_SHIFT: u32                       = 16;

pub fn FW_PHY_ACT_SETUP_LINK_PAUSE_MASK() -> u32 { 3 << FW_PHY_ACT_SETUP_LINK_PAUSE_SHIFT }

pub const FW_PHY_ACT_SETUP_LINK_PAUSE_NONE: u32                        = 0;
pub const FW_PHY_ACT_SETUP_LINK_PAUSE_TX: u32                          = 1;
pub const FW_PHY_ACT_SETUP_LINK_PAUSE_RX: u32                          = 2;
pub const FW_PHY_ACT_SETUP_LINK_PAUSE_RXTX: u32                        = 3;
pub const FW_PHY_ACT_SETUP_LINK_LP: u32                                = 1 << 18;
pub const FW_PHY_ACT_SETUP_LINK_HP: u32                                = 1 << 19;
pub const FW_PHY_ACT_SETUP_LINK_EEE: u32                               = 1 << 20;
pub const FW_PHY_ACT_SETUP_LINK_AN: u32                                = 1 << 22;
pub const FW_PHY_ACT_SETUP_LINK_RSP_DOWN: u32                          = 1 << 0;
pub const FW_PHY_ACT_GET_LINK_INFO: u32                                = 3;
pub const FW_PHY_ACT_GET_LINK_INFO_EEE: u32                            = 1 << 19;
pub const FW_PHY_ACT_GET_LINK_INFO_FC_TX: u32                          = 1 << 20;
pub const FW_PHY_ACT_GET_LINK_INFO_FC_RX: u32                          = 1 << 21;
pub const FW_PHY_ACT_GET_LINK_INFO_POWER: u32                          = 1 << 22;
pub const FW_PHY_ACT_GET_LINK_INFO_AN_COMPLETE: u32                    = 1 << 24;
pub const FW_PHY_ACT_GET_LINK_INFO_TEMP: u32                           = 1 << 25;
pub const FW_PHY_ACT_GET_LINK_INFO_LP_FC_TX: u32                       = 1 << 28;
pub const FW_PHY_ACT_GET_LINK_INFO_LP_FC_RX: u32                       = 1 << 29;
pub const FW_PHY_ACT_FORCE_LINK_DOWN: u32                              = 4;
pub const FW_PHY_ACT_FORCE_LINK_DOWN_OFF: u32                          = 1 << 0;
pub const FW_PHY_ACT_PHY_SW_RESET: u32                                 = 5;
pub const FW_PHY_ACT_PHY_HW_RESET: u32                                 = 6;
pub const FW_PHY_ACT_GET_PHY_INFO: u32                                 = 7;
pub const FW_PHY_ACT_UD_2: u32                                         = 0x1002;
pub const FW_PHY_ACT_UD_2_10G_KR_EEE: u32                              = 1 << 6;
pub const FW_PHY_ACT_UD_2_10G_KX4_EEE: u32                             = 1 << 5;
pub const FW_PHY_ACT_UD_2_1G_KX_EEE: u32                               = 1 << 4;
pub const FW_PHY_ACT_UD_2_10G_T_EEE: u32                               = 1 << 3;
pub const FW_PHY_ACT_UD_2_1G_T_EEE: u32                                = 1 << 2;
pub const FW_PHY_ACT_UD_2_100M_TX_EEE: u32                             = 1 << 1;
pub const FW_PHY_ACT_RETRIES: u32                                      = 50;
pub const FW_PHY_INFO_SPEED_MASK: u32                                  = 0xFFF;
pub const FW_PHY_INFO_ID_HI_MASK: u32                                  = 0xFFFF0000;
pub const FW_PHY_INFO_ID_LO_MASK: u32                                  = 0x0000FFFF;

/* Host Interface Command Structures */

#[repr(C)]
#[derive(Copy, Clone)]
pub union ixgbe_hic_hdr_cmd_or_resp {
    pub cmd_resv: u8,
    pub ret_status: u8,
    _union_align: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_hdr {
    pub cmd: u8,
    pub buf_len: u8,
    pub cmd_or_resp: ixgbe_hic_hdr_cmd_or_resp,
    pub checksum: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbe_hic_hdr2_req {
    pub cmd: u8,
    pub buf_lenh: u8,
    pub buf_lenl: u8,
    pub checksum: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbe_hic_hdr2_rsp {
    pub cmd: u8,
    pub buf_lenl: u8,
    pub buf_lenh_status: u8,
    /* 7-5: high bits of buf_len, 4-0: status */
    pub checksum: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ixgbe_hic_hdr2 {
    pub req: ixgbe_hic_hdr2_req,
    pub rsp: ixgbe_hic_hdr2_rsp,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_drv_info {
    pub hdr: ixgbe_hic_hdr,
    pub port_num: u8,
    pub ver_sub: u8,
    pub ver_build: u8,
    pub ver_min: u8,
    pub ver_maj: u8,
    pub pad: u8,
    /* end spacing to ensure length is mult. of dword */
    pub pad2: u16,
    /* end spacing to ensure length is mult. of dword2 */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_drv_info2 {
    pub hdr: ixgbe_hic_hdr,
    pub port_num: u8,
    pub ver_sub: u8,
    pub ver_build: u8,
    pub ver_min: u8,
    pub ver_maj: u8,
    pub driver_string: [char; FW_CEM_DRIVER_VERSION_SIZE as usize],
}

/* These need to be dword aligned */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_read_shadow_ram {
    pub hdr: ixgbe_hic_hdr2,
    pub address: u32,
    pub length: u16,
    pub pad2: u16,
    pub data: u16,
    pub pad3: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_write_shadow_ram {
    pub hdr: ixgbe_hic_hdr2,
    pub address: u32,
    pub length: u16,
    pub pad2: u16,
    pub data: u16,
    pub pad3: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_disable_rxen {
    pub hdr: ixgbe_hic_hdr,
    pub port_number: u8,
    pub pad2: u8,
    pub pad3: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_phy_token_req {
    pub hdr: ixgbe_hic_hdr,
    pub port_number: u8,
    pub command_type: u8,
    pub pad: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_internal_phy_req {
    pub hdr: ixgbe_hic_hdr,
    pub port_number: u8,
    pub command_type: u8,
    pub address: u16,
    pub rsv1: u16,
    pub write_data: u32,
    pub pad: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_internal_phy_resp {
    pub hdr: ixgbe_hic_hdr,
    pub read_data: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_phy_activity_req {
    pub hdr: ixgbe_hic_hdr,
    pub port_number: u8,
    pub pad: u8,
    pub activity_id: u16,
    pub data: [u32; FW_PHY_ACT_DATA_COUNT as usize],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ixgbe_hic_phy_activity_resp {
    pub hdr: ixgbe_hic_hdr,
    pub data: [u32; FW_PHY_ACT_DATA_COUNT as usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbe_legacy_tx_desc_lower_flags {
    pub length: u16,
    /* Data buffer length */
    pub cso: u8,
    /* Checksum offset */
    pub cmd: u8,
    /* Descriptor control */
}

/* Transmit Descriptor - Legacy */
#[repr(C)]
#[derive(Copy, Clone)]
pub union ixgbe_legacy_tx_desc_lower {
    pub data: u32,
    pub flags: ixgbe_legacy_tx_desc_lower_flags,
    _union_align: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbe_legacy_tx_desc_upper_fields {
    pub status: u8,
    /* Descriptor status */
    pub css: u8,
    /* Checksum start */
    pub vlan: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ixgbe_legacy_tx_desc_upper {
    pub data: u32,
    pub fields: ixgbe_legacy_tx_desc_upper_fields,
    _union_align: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_legacy_tx_desc {
    pub buffer_addr: u64,
    /* Address of the descriptor's data buffer */
    pub lower: ixgbe_legacy_tx_desc_lower,
    pub upper: ixgbe_legacy_tx_desc_upper,
}

/* Transmit Descriptor - Advanced */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbe_adv_tx_desc_read {
    pub buffer_addr: u64,
    /* Address of descriptor's data buf */
    pub cmd_type_len: u32,
    pub olinfo_status: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbe_adv_tx_desc_wb {
    pub rsvd: u64,
    /* Reserved */
    pub nxtseq_seed: u32,
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ixgbe_adv_tx_desc {
    pub read: ixgbe_adv_tx_desc_read,
    pub wb: ixgbe_adv_tx_desc_wb,
    _union_align: [u64; 2],
}

/* Receive Descriptor - Legacy */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbe_legacy_rx_desc {
    pub buffer_addr: u64,
    /* Address of the descriptor's data buffer */
    pub length: u16,
    /* Length of data DMAed into data buffer */
    pub csum: u16,
    /* Packet checksum */
    pub status: u8,
    /* Descriptor status */
    pub errors: u8,
    /* Descriptor Errors */
    pub vlan: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbe_adv_rx_desc_read {
    pub pkt_addr: u64,
    /* Packet buffer address */
    pub hdr_addr: u64,
    /* Header buffer address */
}

/* Receive Descriptor - Advanced */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbe_adv_rx_desc_wb_lower_lo_dword_hs_rss {
    pub pkt_info: u16,
    /* RSS, Pkt type */
    pub hdr_info: u16,
    /* Splithdr, hdrlen */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ixgbe_adv_rx_desc_wb_lower_lo_dword {
    pub data: u32,
    pub hs_rss: ixgbe_adv_rx_desc_wb_lower_lo_dword_hs_rss,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbe_adv_rx_desc_wb_lower_hi_dword_csum_ip {
    pub ip_id: u16,
    /* IP id */
    pub csum: u16,
    /* Packet Checksum */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ixgbe_adv_rx_desc_wb_lower_hi_dword {
    pub rss: u32,
    /* RSS Hash */
    pub csum_ip: ixgbe_adv_rx_desc_wb_lower_hi_dword_csum_ip,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_adv_rx_desc_wb_lower {
    pub lo_dword: ixgbe_adv_rx_desc_wb_lower_lo_dword,
    pub hi_dword: ixgbe_adv_rx_desc_wb_lower_hi_dword,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbe_adv_rx_desc_wb_upper {
    pub status_error: u32,
    /* ext status/error */
    pub length: u16,
    /* Packet length */
    pub vlan: u16,
    /* VLAN tag */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_adv_rx_desc_wb {
    pub lower: ixgbe_adv_rx_desc_wb_lower,
    pub upper: ixgbe_adv_rx_desc_wb_upper,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ixgbe_adv_rx_desc {
    pub read: ixgbe_adv_rx_desc_read,
    pub wb: ixgbe_adv_rx_desc_wb, /* writeback */
    _union_align: [u64; 2],
}

/* Context descriptors */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbe_adv_tx_context_desc {
    pub vlan_macip_lens: u32,
    pub seqnum_seed: u32,
    pub type_tucmd_mlhl: u32,
    pub mss_l4len_idx: u32,
}

/* Adv Transmit Descriptor Config Masks */
pub const IXGBE_ADVTXD_DTALEN_MASK: u32                                = 0x0000FFFF; /* Data buf length(bytes) */
pub const IXGBE_ADVTXD_MAC_LINKSEC: u32                                = 0x00040000; /* Insert LinkSec */
pub const IXGBE_ADVTXD_MAC_TSTAMP: u32                                 = 0x00080000; /* IEEE1588 time stamp */
pub const IXGBE_ADVTXD_IPSEC_SA_INDEX_MASK: u32                        = 0x000003FF; /* IPSec SA index */
pub const IXGBE_ADVTXD_IPSEC_ESP_LEN_MASK: u32                         = 0x000001FF; /* IPSec ESP length */
pub const IXGBE_ADVTXD_DTYP_MASK: u32                                  = 0x00F00000; /* DTYP mask */
pub const IXGBE_ADVTXD_DTYP_CTXT: u32                                  = 0x00200000; /* Adv Context Desc */
pub const IXGBE_ADVTXD_DTYP_DATA: u32                                  = 0x00300000; /* Adv Data Descriptor */
pub const IXGBE_ADVTXD_DCMD_EOP: u32                                   = IXGBE_TXD_CMD_EOP;  /* End of Packet */
pub const IXGBE_ADVTXD_DCMD_IFCS: u32                                  = IXGBE_TXD_CMD_IFCS; /* Insert FCS */
pub const IXGBE_ADVTXD_DCMD_RS: u32                                    = IXGBE_TXD_CMD_RS; /* Report Status */
pub const IXGBE_ADVTXD_DCMD_DDTYP_ISCSI: u32                           = 0x10000000; /* DDP hdr type or iSCSI */
pub const IXGBE_ADVTXD_DCMD_DEXT: u32                                  = IXGBE_TXD_CMD_DEXT; /* Desc ext 1=Adv */
pub const IXGBE_ADVTXD_DCMD_VLE: u32                                   = IXGBE_TXD_CMD_VLE;  /* VLAN pkt enable */
pub const IXGBE_ADVTXD_DCMD_TSE: u32                                   = 0x80000000; /* TCP Seg enable */
pub const IXGBE_ADVTXD_STAT_DD: u32                                    = IXGBE_TXD_STAT_DD;  /* Descriptor Done */
pub const IXGBE_ADVTXD_STAT_SN_CRC: u32                                = 0x00000002; /* NXTSEQ/SEED pres in WB */
pub const IXGBE_ADVTXD_STAT_RSV: u32                                   = 0x0000000C; /* STA Reserved */
pub const IXGBE_ADVTXD_IDX_SHIFT: u32                                  = 4; /* Adv desc Index shift */
pub const IXGBE_ADVTXD_CC: u32                                         = 0x00000080; /* Check Context */
pub const IXGBE_ADVTXD_POPTS_SHIFT: u32                                = 8;  /* Adv desc POPTS shift */
pub const IXGBE_ADVTXD_POPTS_IXSM: u32                                 = IXGBE_TXD_POPTS_IXSM << IXGBE_ADVTXD_POPTS_SHIFT;
pub const IXGBE_ADVTXD_POPTS_TXSM: u32                                 = IXGBE_TXD_POPTS_TXSM << IXGBE_ADVTXD_POPTS_SHIFT;
pub const IXGBE_ADVTXD_POPTS_ISCO_1ST: u32                             = 0x00000000; /* 1st TSO of iSCSI PDU */
pub const IXGBE_ADVTXD_POPTS_ISCO_MDL: u32                             = 0x00000800; /* Middle TSO of iSCSI PDU */
pub const IXGBE_ADVTXD_POPTS_ISCO_LAST: u32                            = 0x00001000; /* Last TSO of iSCSI PDU */
/* 1st&Last TSO-full iSCSI PDU */
pub const IXGBE_ADVTXD_POPTS_ISCO_FULL: u32                            = 0x00001800;
pub const IXGBE_ADVTXD_POPTS_RSV: u32                                  = 0x00002000; /* POPTS Reserved */
pub const IXGBE_ADVTXD_PAYLEN_SHIFT: u32                               = 14; /* Adv desc PAYLEN shift */
pub const IXGBE_ADVTXD_MACLEN_SHIFT: u32                               = 9;  /* Adv ctxt desc mac len shift */
pub const IXGBE_ADVTXD_VLAN_SHIFT: u32                                 = 16;  /* Adv ctxt vlan tag shift */
pub const IXGBE_ADVTXD_TUCMD_IPV4: u32                                 = 0x00000400; /* IP Packet Type: 1=IPv4 */
pub const IXGBE_ADVTXD_TUCMD_IPV6: u32                                 = 0x00000000; /* IP Packet Type: 0=IPv6 */
pub const IXGBE_ADVTXD_TUCMD_L4T_UDP: u32                              = 0x00000000; /* L4 Packet TYPE of UDP */
pub const IXGBE_ADVTXD_TUCMD_L4T_TCP: u32                              = 0x00000800; /* L4 Packet TYPE of TCP */
pub const IXGBE_ADVTXD_TUCMD_L4T_SCTP: u32                             = 0x00001000; /* L4 Packet TYPE of SCTP */
pub const IXGBE_ADVTXD_TUCMD_L4T_RSV: u32                              = 0x00001800; /* RSV L4 Packet TYPE */
pub const IXGBE_ADVTXD_TUCMD_MKRREQ: u32                               = 0x00002000; /* req Markers and CRC */
pub const IXGBE_ADVTXD_POPTS_IPSEC: u32                                = 0x00000400; /* IPSec offload request */
pub const IXGBE_ADVTXD_TUCMD_IPSEC_TYPE_ESP: u32                       = 0x00002000; /* IPSec Type ESP */
pub const IXGBE_ADVTXD_TUCMD_IPSEC_ENCRYPT_EN: u32                     = 0x00004000; /* ESP Encrypt Enable */
pub const IXGBE_ADVTXT_TUCMD_FCOE: u32                                 = 0x00008000; /* FCoE Frame Type */
pub const IXGBE_ADVTXD_FCOEF_EOF_MASK: u32                             = 0x3 << 10; /* FC EOF index */
pub const IXGBE_ADVTXD_FCOEF_SOF: u32                                  = 1 << 2 << 10; /* FC SOF index */
pub const IXGBE_ADVTXD_FCOEF_PARINC: u32                               = 1 << 3 << 10; /* Rel_Off in F_CTL */
pub const IXGBE_ADVTXD_FCOEF_ORIE: u32                                 = 1 << 4 << 10; /* Orientation End */
pub const IXGBE_ADVTXD_FCOEF_ORIS: u32                                 = 1 << 5 << 10; /* Orientation Start */
pub const IXGBE_ADVTXD_FCOEF_EOF_N: u32                                = 0x0 << 10; /* 00: EOFn */
pub const IXGBE_ADVTXD_FCOEF_EOF_T: u32                                = 0x1 << 10; /* 01: EOFt */
pub const IXGBE_ADVTXD_FCOEF_EOF_NI: u32                               = 0x2 << 10; /* 10: EOFni */
pub const IXGBE_ADVTXD_FCOEF_EOF_A: u32                                = 0x3 << 10; /* 11: EOFa */
pub const IXGBE_ADVTXD_L4LEN_SHIFT: u32                                = 8;  /* Adv ctxt L4LEN shift */
pub const IXGBE_ADVTXD_MSS_SHIFT: u32                                  = 16;  /* Adv ctxt MSS shift */

pub const IXGBE_ADVTXD_OUTER_IPLEN: u32                                = 16; /* Adv ctxt OUTERIPLEN shift */
pub const IXGBE_ADVTXD_TUNNEL_LEN: u32                                 = 24; /* Adv ctxt TUNNELLEN shift */
pub const IXGBE_ADVTXD_TUNNEL_TYPE_SHIFT: u32                          = 16; /* Adv Tx Desc Tunnel Type shift */
pub const IXGBE_ADVTXD_OUTERIPCS_SHIFT: u32                            = 17; /* Adv Tx Desc OUTERIPCS Shift */
pub const IXGBE_ADVTXD_TUNNEL_TYPE_NVGRE: u32                          = 1;  /* Adv Tx Desc Tunnel Type NVGRE */
/* Adv Tx Desc OUTERIPCS Shift for X550EM_a */
pub const IXGBE_ADVTXD_OUTERIPCS_SHIFT_X550EM_a: u32                   = 26;
pub const IXGBE_LINK_SPEED_UNKNOWN: u32                                = 0;
pub const IXGBE_LINK_SPEED_10_FULL: u32                                = 0x0002;
pub const IXGBE_LINK_SPEED_100_FULL: u32                               = 0x0008;
pub const IXGBE_LINK_SPEED_1GB_FULL: u32                               = 0x0020;
pub const IXGBE_LINK_SPEED_2_5GB_FULL: u32                             = 0x0400;
pub const IXGBE_LINK_SPEED_5GB_FULL: u32                               = 0x0800;
pub const IXGBE_LINK_SPEED_10GB_FULL: u32                              = 0x0080;
pub const IXGBE_LINK_SPEED_82598_AUTONEG: u32                          = IXGBE_LINK_SPEED_1GB_FULL | IXGBE_LINK_SPEED_10GB_FULL;
pub const IXGBE_LINK_SPEED_82599_AUTONEG: u32                          = IXGBE_LINK_SPEED_100_FULL | IXGBE_LINK_SPEED_1GB_FULL | IXGBE_LINK_SPEED_10GB_FULL;

/* Physical layer type */
pub const IXGBE_PHYSICAL_LAYER_UNKNOWN: u32                            = 0;
pub const IXGBE_PHYSICAL_LAYER_10GBASE_T: u32                          = 0x00001;
pub const IXGBE_PHYSICAL_LAYER_1000BASE_T: u32                         = 0x00002;
pub const IXGBE_PHYSICAL_LAYER_100BASE_TX: u32                         = 0x00004;
pub const IXGBE_PHYSICAL_LAYER_SFP_PLUS_CU: u32                        = 0x00008;
pub const IXGBE_PHYSICAL_LAYER_10GBASE_LR: u32                         = 0x00010;
pub const IXGBE_PHYSICAL_LAYER_10GBASE_LRM: u32                        = 0x00020;
pub const IXGBE_PHYSICAL_LAYER_10GBASE_SR: u32                         = 0x00040;
pub const IXGBE_PHYSICAL_LAYER_10GBASE_KX4: u32                        = 0x00080;
pub const IXGBE_PHYSICAL_LAYER_10GBASE_CX4: u32                        = 0x00100;
pub const IXGBE_PHYSICAL_LAYER_1000BASE_KX: u32                        = 0x00200;
pub const IXGBE_PHYSICAL_LAYER_1000BASE_BX: u32                        = 0x00400;
pub const IXGBE_PHYSICAL_LAYER_10GBASE_KR: u32                         = 0x00800;
pub const IXGBE_PHYSICAL_LAYER_10GBASE_XAUI: u32                       = 0x01000;
pub const IXGBE_PHYSICAL_LAYER_SFP_ACTIVE_DA: u32                      = 0x02000;
pub const IXGBE_PHYSICAL_LAYER_1000BASE_SX: u32                        = 0x04000;
pub const IXGBE_PHYSICAL_LAYER_10BASE_T: u32                           = 0x08000;
pub const IXGBE_PHYSICAL_LAYER_2500BASE_KX: u32                        = 0x10000;

/* Flow Control Data Sheet defined values
 * Calculation and defines taken from 802.1bb Annex O;
 */

/* BitTimes (BT) conversion */
pub fn IXGBE_BT2KB(BT: u32) -> u32 { (BT + (8 * 1024 - 1)) / 8 * 1024 }

pub fn IXGBE_B2BT(BT: u32) -> u32 { BT * 8 }

/* Calculate Delay to respond to PFC */
pub const IXGBE_PFC_D: u32                                             = 672;

/* Calculate Cable Delay */
pub const IXGBE_CABLE_DC: u32                                          = 5556; /* Delay Copper */
pub const IXGBE_CABLE_DO: u32                                          = 5000; /* Delay Optical */

/* Calculate Interface Delay X540 */
pub const IXGBE_PHY_DC: u32                                            = 25600; /* Delay 10G BASET */
pub const IXGBE_MAC_DC: u32                                            = 8192;  /* Delay Copper XAUI interface */
pub const IXGBE_XAUI_DC: u32                                           = 2 * 2048; /* Delay Copper Phy */

pub const IXGBE_ID_X540: u32                                           = IXGBE_MAC_DC + IXGBE_XAUI_DC + IXGBE_PHY_DC;

/* Calculate Interface Delay 82598, 82599 */
pub const IXGBE_PHY_D: u32                                             = 12800;
pub const IXGBE_MAC_D: u32                                             = 4096;
pub const IXGBE_XAUI_D: u32                                            = 2 * 1024;

pub const IXGBE_ID: u32                                                = IXGBE_MAC_D + IXGBE_XAUI_D + IXGBE_PHY_D;

/* Calculate Delay incurred from higher layer */
pub const IXGBE_HD: u32                                                = 6144;

/* Calculate PCI Bus delay for low thresholds */
pub const IXGBE_PCI_DELAY: u32                                         = 10000;

/* Calculate X540 delay value in bit times */
pub fn IXGBE_DV_X540(max_frame_link: u32, max_frame_tc: u32) -> u32 { (36 * (IXGBE_B2BT(max_frame_link) + IXGBE_PFC_D + 2 * IXGBE_CABLE_DC + 2 * IXGBE_ID_X540 + IXGBE_HD) / 25 + 1) + 2 * IXGBE_B2BT(max_frame_tc) }

/* Calculate 82599, 82598 delay value in bit times */
pub fn IXGBE_DV(max_frame_link: u32, max_frame_tc: u32) -> u32 { (36 * (IXGBE_B2BT(max_frame_link) + IXGBE_PFC_D + 2 * IXGBE_CABLE_DC + 2 * IXGBE_ID + IXGBE_HD) / 25 + 1) + 2 * IXGBE_B2BT(max_frame_tc) }

/* Calculate low threshold delay values */
pub fn IXGBE_LOW_DV_X540(max_frame_tc: u32) -> u32 { 2 * IXGBE_B2BT(max_frame_tc) + (36 * IXGBE_PCI_DELAY / 25) + 1 }

pub fn IXGBE_LOW_DV(max_frame_tc: u32) -> u32 { 2 * IXGBE_LOW_DV_X540(max_frame_tc) }

/* Software ATR hash keys */
pub const IXGBE_ATR_BUCKET_HASH_KEY: u32                               = 0x3DAD14E2;
pub const IXGBE_ATR_SIGNATURE_HASH_KEY: u32                            = 0x174D3614;

/* Software ATR input stream values and masks */
pub const IXGBE_ATR_HASH_MASK: u32                                     = 0x7fff;
pub const IXGBE_ATR_L4TYPE_MASK: u32                                   = 0x3;
pub const IXGBE_ATR_L4TYPE_UDP: u32                                    = 0x1;
pub const IXGBE_ATR_L4TYPE_TCP: u32                                    = 0x2;
pub const IXGBE_ATR_L4TYPE_SCTP: u32                                   = 0x3;
pub const IXGBE_ATR_L4TYPE_IPV6_MASK: u32                              = 0x4;
pub const IXGBE_ATR_L4TYPE_TUNNEL_MASK: u32                            = 0x10;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ixgbe_atr_flow_type {
    IXGBE_ATR_FLOW_TYPE_IPV4                                           = 0x0,
    IXGBE_ATR_FLOW_TYPE_UDPV4                                          = 0x1,
    IXGBE_ATR_FLOW_TYPE_TCPV4                                          = 0x2,
    IXGBE_ATR_FLOW_TYPE_SCTPV4                                         = 0x3,
    IXGBE_ATR_FLOW_TYPE_IPV6                                           = 0x4,
    IXGBE_ATR_FLOW_TYPE_UDPV6                                          = 0x5,
    IXGBE_ATR_FLOW_TYPE_TCPV6                                          = 0x6,
    IXGBE_ATR_FLOW_TYPE_SCTPV6                                         = 0x7,
    IXGBE_ATR_FLOW_TYPE_TUNNELED_IPV4                                  = 0x10,
    IXGBE_ATR_FLOW_TYPE_TUNNELED_UDPV4                                 = 0x11,
    IXGBE_ATR_FLOW_TYPE_TUNNELED_TCPV4                                 = 0x12,
    IXGBE_ATR_FLOW_TYPE_TUNNELED_SCTPV4                                = 0x13,
    IXGBE_ATR_FLOW_TYPE_TUNNELED_IPV6                                  = 0x14,
    IXGBE_ATR_FLOW_TYPE_TUNNELED_UDPV6                                 = 0x15,
    IXGBE_ATR_FLOW_TYPE_TUNNELED_TCPV6                                 = 0x16,
    IXGBE_ATR_FLOW_TYPE_TUNNELED_SCTPV6                                = 0x17,
}

/* Flow Director ATR input struct.
 * Byte layout in order, all values with MSB first:
 *
 * vm_pool	- 1 byte;
 * flow_type	- 1 byte;
 * vlan_id	- 2 bytes
 * src_ip	- 16 bytes
 * inner_mac	- 6 bytes
 * cloud_mode	- 2 bytes
 * tni_vni	- 4 bytes
 * dst_ip	- 16 bytes
 * src_port	- 2 bytes
 * dst_port	- 2 bytes
 * flex_bytes	- 2 bytes
 * bkt_hash	- 2 bytes
 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbe_atr_input_formatted {
    pub vm_pool: u8,
    pub flow_type: u8,
    pub vlan_id: u16,
    pub dst_ip: [u32; 4],
    pub src_ip: [u32; 4],
    pub inner_mac: [u8; 6],
    pub tunnel_type: u16,
    pub tni_vni: u32,
    pub src_port: u16,
    pub dst_port: u16,
    pub flex_bytes: u16,
    pub bkt_hash: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ixgbe_atr_input {
    pub formatted: ixgbe_atr_input_formatted,
    pub dword_stream: [u32; 14],
    _union_align: [u32; 14],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbe_atr_hash_dword_formatted {
    pub vm_pool: u8,
    pub flow_type: u8,
    pub vlan_id: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbe_atr_hash_dword_port {
    pub src: u16,
    pub dst: u16,
}

/* Flow Director compressed ATR hash input struct */
#[repr(C)]
#[derive(Copy, Clone)]
union ixgbe_atr_hash_dword {
    pub formatted: ixgbe_atr_hash_dword_formatted,
    pub ip: u32,
    pub port: ixgbe_atr_hash_dword_port,
    pub flex_bytes: u16,
    pub dword: u32,
    _union_align: u32,
}

/*
 * Unavailable: The FCoE Boot Option ROM is not present in the flash.
 * Disabled: Present; boot order is not set for any targets on the port.
 * Enabled: Present; boot order is set for at least one target on the port.
 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ixgbe_fcoe_boot_status {
    IXGbe_fcoe_bootstatus_disabled    = 0,
    IXGbe_fcoe_bootstatus_enabled     = 1,
    IXGbe_fcoe_bootstatus_unavailable = 0xFFFF,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ixgbe_eeprom_type {
    IXGbe_eeprom_uninitialized        = 0,
    IXGbe_eeprom_spi,
    IXGbe_flash,
    IXGbe_eeprom_none,
    /* No NVM support */
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ixgbe_mac_type {
    IXGbe_mac_unknown                 = 0,
    IXGbe_mac_82598EB,
    IXGbe_mac_82599EB,
    IXGbe_mac_82599_vf,
    IXGbe_mac_X540,
    IXGbe_mac_X540_vf,
    IXGbe_mac_X550,
    IXGbe_mac_X550EM_x,
    IXGbe_mac_X550EM_a,
    IXGbe_mac_X550_vf,
    IXGbe_mac_X550EM_x_vf,
    IXGbe_mac_X550EM_a_vf,
    IXGbe_num_macs,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ixgbe_phy_type {
    IXGbe_phy_unknown                 = 0,
    IXGbe_phy_none,
    IXGbe_phy_tn,
    IXGbe_phy_aq,
    IXGbe_phy_x550em_kr,
    IXGbe_phy_x550em_kx4,
    IXGbe_phy_x550em_xfi,
    IXGbe_phy_x550em_ext_t,
    IXGbe_phy_ext_1g_t,
    IXGbe_phy_cu_unknown,
    IXGbe_phy_qt,
    IXGbe_phy_xaui,
    IXGbe_phy_nl,
    IXGbe_phy_sfp_passive_tyco,
    IXGbe_phy_sfp_passive_unknown,
    IXGbe_phy_sfp_active_unknown,
    IXGbe_phy_sfp_avago,
    IXGbe_phy_sfp_ftl,
    IXGbe_phy_sfp_ftl_active,
    IXGbe_phy_sfp_unknown,
    IXGbe_phy_sfp_intel,
    IXGbe_phy_qsfp_passive_unknown,
    IXGbe_phy_qsfp_active_unknown,
    IXGbe_phy_qsfp_intel,
    IXGbe_phy_qsfp_unknown,
    IXGbe_phy_sfp_unsupported,
    /*Enforce bit set with unsupported module*/
    IXGbe_phy_sgmii,
    IXGbe_phy_fw,
    IXGbe_phy_generic,
}

/*
 * SFP+ module type IDs:
 *
 * ID	Module Type;
 * =============
 * 0	SFP_DA_CU;
 * 1	SFP_SR;
 * 2	SFP_LR;
 * 3	SFP_DA_CU_CORE0 - 82599-specific;
 * 4	SFP_DA_CU_CORE1 - 82599-specific;
 * 5	SFP_SR/LR_CORE0 - 82599-specific;
 * 6	SFP_SR/LR_CORE1 - 82599-specific;
 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ixgbe_sfp_type {
    IXGbe_sfp_type_da_cu              =  0,
    IXGbe_sfp_type_sr                 =  1,
    IXGbe_sfp_type_lr                 =  2,
    IXGbe_sfp_type_da_cu_core0        =  3,
    IXGbe_sfp_type_da_cu_core1        =  4,
    IXGbe_sfp_type_srlr_core0         =  5,
    IXGbe_sfp_type_srlr_core1         =  6,
    IXGbe_sfp_type_da_act_lmt_core0   =  7,
    IXGbe_sfp_type_da_act_lmt_core1   =  8,
    IXGbe_sfp_type_1g_cu_core0        =  9,
    IXGbe_sfp_type_1g_cu_core1        = 10,
    IXGbe_sfp_type_1g_sx_core0        = 11,
    IXGbe_sfp_type_1g_sx_core1        = 12,
    IXGbe_sfp_type_1g_lx_core0        = 13,
    IXGbe_sfp_type_1g_lx_core1        = 14,
    IXGbe_sfp_type_not_present        = 0xFFFE,
    IXGbe_sfp_type_unknown            = 0xFFFF,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ixgbe_media_type {
    IXGbe_media_type_unknown          = 0,
    IXGbe_media_type_fiber,
    IXGbe_media_type_fiber_qsfp,
    IXGbe_media_type_fiber_lco,
    IXGbe_media_type_copper,
    IXGbe_media_type_backplane,
    IXGbe_media_type_cx4,
    IXGbe_media_type_virtual,
}

/* Flow Control Settings */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ixgbe_fc_mode {
    IXGbe_fc_none                     = 0,
    IXGbe_fc_rx_pause,
    IXGbe_fc_tx_pause,
    IXGbe_fc_full,
    IXGbe_fc_default,
}

/* Smart Speed Settings */
pub const IXGBE_SMARTSPEED_MAX_RETRIES: u32                            = 3;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ixgbe_smart_speed {
    IXGbe_smart_speed_auto            = 0,
    IXGbe_smart_speed_on,
    IXGbe_smart_speed_off,
}

/* PCI bus types */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ixgbe_bus_type {
    IXGbe_bus_type_unknown            = 0,
    IXGbe_bus_type_pci,
    IXGbe_bus_type_pcix,
    IXGbe_bus_type_pci_express,
    IXGbe_bus_type_internal,
    IXGbe_bus_type_reserved,
}

/* PCI bus speeds */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ixgbe_bus_speed {
    IXGbe_bus_speed_unknown           = 0,
    IXGbe_bus_speed_33                = 33,
    IXGbe_bus_speed_66                = 66,
    IXGbe_bus_speed_100               = 100,
    IXGbe_bus_speed_120               = 120,
    IXGbe_bus_speed_133               = 133,
    IXGbe_bus_speed_2500              = 2500,
    IXGbe_bus_speed_5000              = 5000,
    IXGbe_bus_speed_8000              = 8000,
    IXGbe_bus_speed_reserved,
}

/* PCI bus widths */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ixgbe_bus_width {
    IXGbe_bus_width_unknown           = 0,
    IXGbe_bus_width_pcie_x1           = 1,
    IXGbe_bus_width_pcie_x2           = 2,
    IXGbe_bus_width_pcie_x4           = 4,
    IXGbe_bus_width_pcie_x8           = 8,
    IXGbe_bus_width_32                = 32,
    IXGbe_bus_width_64                = 64,
    IXGbe_bus_width_reserved,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbe_addr_filter_info {
    pub num_mc_addrs: u32,
    pub rar_used_count: u32,
    pub mta_in_use: u32,
    pub overflow_promisc: u32,
    pub user_set_promisc: bool,
}

/* Bus parameters */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbe_bus_info {
    pub speed: ixgbe_bus_speed,
    pub width: ixgbe_bus_width,
    pub btype: ixgbe_bus_type,
    pub func: u16,
    pub lan_id: u8,
    pub instance_id: u16,
}

/* Flow control parameters */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbe_fc_info {
    pub high_water: [u32; IXGBE_DCB_MAX_TRAFFIC_CLASS as usize],
    /* Flow Ctrl High-water */
    pub low_water: [u32; IXGBE_DCB_MAX_TRAFFIC_CLASS as usize],
    /* Flow Ctrl Low-water */
    pub pause_time: u16,
    /* Flow Control Pause timer */
    pub send_xon: bool,
    /* Flow control send XON */
    pub strict_ieee: bool,
    /* Strict IEEE mode */
    pub disable_fc_autoneg: bool,
    /* Do not autonegotiate FC */
    pub fc_was_autonegged: bool,
    /* Is current_mode the result of autonegging? */
    pub current_mode: ixgbe_fc_mode,
    /* FC mode in effect */
    pub requested_mode: ixgbe_fc_mode,
    /* FC mode requested by caller */
}

/* Statistics counters collected by the MAC */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbe_hw_stats {
    pub crcerrs: u64,
    pub illerrc: u64,
    pub errbc: u64,
    pub mspdc: u64,
    pub mpctotal: u64,
    pub mpc: [u64; 8],
    pub mlfc: u64,
    pub mrfc: u64,
    pub rlec: u64,
    pub lxontxc: u64,
    pub lxonrxc: u64,
    pub lxofftxc: u64,
    pub lxoffrxc: u64,
    pub pxontxc: [u64; 8],
    pub pxonrxc: [u64; 8],
    pub pxofftxc: [u64; 8],
    pub pxoffrxc: [u64; 8],
    pub prc64: u64,
    pub prc127: u64,
    pub prc255: u64,
    pub prc511: u64,
    pub prc1023: u64,
    pub prc1522: u64,
    pub gprc: u64,
    pub bprc: u64,
    pub mprc: u64,
    pub gptc: u64,
    pub gorc: u64,
    pub gotc: u64,
    pub rnbc: [u64; 8],
    pub ruc: u64,
    pub rfc: u64,
    pub roc: u64,
    pub rjc: u64,
    pub mngprc: u64,
    pub mngpdc: u64,
    pub mngptc: u64,
    pub tor: u64,
    pub tpr: u64,
    pub tpt: u64,
    pub ptc64: u64,
    pub ptc127: u64,
    pub ptc255: u64,
    pub ptc511: u64,
    pub ptc1023: u64,
    pub ptc1522: u64,
    pub mptc: u64,
    pub bptc: u64,
    pub xec: u64,
    pub qprc: [u64; 16],
    pub qptc: [u64; 16],
    pub qbrc: [u64; 16],
    pub qbtc: [u64; 16],
    pub qprdc: [u64; 16],
    pub pxon2offc: [u64; 8],
    pub fdirustat_add: u64,
    pub fdirustat_remove: u64,
    pub fdirfstat_fadd: u64,
    pub fdirfstat_fremove: u64,
    pub fdirmatch: u64,
    pub fdirmiss: u64,
    pub fccrc: u64,
    pub fclast: u64,
    pub fcoerpdc: u64,
    pub fcoeprc: u64,
    pub fcoeptc: u64,
    pub fcoedwrc: u64,
    pub fcoedwtc: u64,
    pub fcoe_noddp: u64,
    pub fcoe_noddp_ext_buff: u64,
    pub ldpcec: u64,
    pub pcrc8ec: u64,
    pub b2ospc: u64,
    pub b2ogprc: u64,
    pub o2bgptc: u64,
    pub o2bspc: u64,
}

// functions removed


pub fn IXGBE_FUSES0_GROUP(i: u32) -> u32 { 0x11158 + i * 4 }

pub const IXGBE_FUSES0_300MHZ: u32                                     = 1 << 5;
pub const IXGBE_FUSES0_REV_MASK: u32                                   = 3 << 6;

pub fn IXGBE_KRM_PORT_CAR_GEN_CTRL(P: u32) -> u32 { if P != 0 { 0x8010 } else { 0x4010 } }
pub fn IXGBE_KRM_LINK_S1(P: u32) -> u32 { if P != 0 { 0x8200 } else { 0x4200 } }
pub fn IXGBE_KRM_LINK_CTRL_1(P: u32) -> u32 { if P != 0 { 0x820C } else { 0x420C } }
pub fn IXGBE_KRM_AN_CNTL_1(P: u32) -> u32 { if P != 0 { 0x822C } else { 0x422C } }
pub fn IXGBE_KRM_AN_CNTL_4(P: u32) -> u32 { if P != 0 { 0x8238 } else { 0x4238 } }
pub fn IXGBE_KRM_AN_CNTL_8(P: u32) -> u32 { if P != 0 { 0x8248 } else { 0x4248 } }
pub fn IXGBE_KRM_PCS_KX_AN(P: u32) -> u32 { if P != 0 { 0x9918 } else { 0x5918 } }
pub fn IXGBE_KRM_PCS_KX_AN_LP(P: u32) -> u32 { if P != 0 { 0x991C } else { 0x591C } }
pub fn IXGBE_KRM_SGMII_CTRL(P: u32) -> u32 { if P != 0 { 0x82A0 } else { 0x42A0 } }
pub fn IXGBE_KRM_LP_BASE_PAGE_HIGH(P: u32) -> u32 { if P != 0 { 0x836C } else { 0x436C } }
pub fn IXGBE_KRM_DSP_TXFFE_STATE_4(P: u32) -> u32 { if P != 0 { 0x8634 } else { 0x4634 } }
pub fn IXGBE_KRM_DSP_TXFFE_STATE_5(P: u32) -> u32 { if P != 0 { 0x8638 } else { 0x4638 } }
pub fn IXGBE_KRM_RX_TRN_LINKUP_CTRL(P: u32) -> u32 { if P != 0 { 0x8B00 } else { 0x4B00 } }
pub fn IXGBE_KRM_PMD_DFX_BURNIN(P: u32) -> u32 { if P != 0 { 0x8E00 } else { 0x4E00 } }
pub fn IXGBE_KRM_PMD_FLX_MASK_ST20(P: u32) -> u32 { if P != 0 { 0x9054 } else { 0x5054 } }
pub fn IXGBE_KRM_TX_COEFF_CTRL_1(P: u32) -> u32 { if P != 0 { 0x9520 } else { 0x5520 } }
pub fn IXGBE_KRM_RX_ANA_CTL(P: u32) -> u32 { if P != 0 { 0x9A00 } else { 0x5A00 } }

pub const IXGBE_KRM_PMD_FLX_MASK_ST20_SFI_10G_DA: u32                  = !0x3 << 20;
pub const IXGBE_KRM_PMD_FLX_MASK_ST20_SFI_10G_SR: u32                  = 1 << 20;
pub const IXGBE_KRM_PMD_FLX_MASK_ST20_SFI_10G_LR: u32                  = 0x2 << 20;
pub const IXGBE_KRM_PMD_FLX_MASK_ST20_SGMII_EN: u32                    = 1 << 25;
pub const IXGBE_KRM_PMD_FLX_MASK_ST20_AN37_EN: u32                     = 1 << 26;
pub const IXGBE_KRM_PMD_FLX_MASK_ST20_AN_EN: u32                       = 1 << 27;
pub const IXGBE_KRM_PMD_FLX_MASK_ST20_SPEED_10M: u32                   = !0x7 << 28;
pub const IXGBE_KRM_PMD_FLX_MASK_ST20_SPEED_100M: u32                  = 1 << 28;
pub const IXGBE_KRM_PMD_FLX_MASK_ST20_SPEED_1G: u32                    = 0x2 << 28;
pub const IXGBE_KRM_PMD_FLX_MASK_ST20_SPEED_10G: u32                   = 0x3 << 28;
pub const IXGBE_KRM_PMD_FLX_MASK_ST20_SPEED_AN: u32                    = 0x4 << 28;
pub const IXGBE_KRM_PMD_FLX_MASK_ST20_SPEED_2_5G: u32                  = 0x7 << 28;
pub const IXGBE_KRM_PMD_FLX_MASK_ST20_SPEED_MASK: u32                  = 0x7 << 28;
pub const IXGBE_KRM_PMD_FLX_MASK_ST20_FW_AN_RESTART: u32               = 1 << 31;

pub const IXGBE_KRM_PORT_CAR_GEN_CTRL_NELB_32B: u32                    = 1 << 9;
pub const IXGBE_KRM_PORT_CAR_GEN_CTRL_NELB_KRPCS: u32                  = 1 << 11;

pub const IXGBE_KRM_LINK_CTRL_1_TETH_FORCE_SPEED_MASK: u32             = 0x7 << 8;
pub const IXGBE_KRM_LINK_CTRL_1_TETH_FORCE_SPEED_1G: u32               = 2 << 8;
pub const IXGBE_KRM_LINK_CTRL_1_TETH_FORCE_SPEED_10G: u32              = 4 << 8;
pub const IXGBE_KRM_LINK_CTRL_1_TETH_AN_SGMII_EN: u32                  = 1 << 12;
pub const IXGBE_KRM_LINK_CTRL_1_TETH_AN_CLAUSE_37_EN: u32              = 1 << 13;
pub const IXGBE_KRM_LINK_CTRL_1_TETH_AN_FEC_REQ: u32                   = 1 << 14;
pub const IXGBE_KRM_LINK_CTRL_1_TETH_AN_CAP_FEC: u32                   = 1 << 15;
pub const IXGBE_KRM_LINK_CTRL_1_TETH_AN_CAP_KX: u32                    = 1 << 16;
pub const IXGBE_KRM_LINK_CTRL_1_TETH_AN_CAP_KR: u32                    = 1 << 18;
pub const IXGBE_KRM_LINK_CTRL_1_TETH_EEE_CAP_KX: u32                   = 1 << 24;
pub const IXGBE_KRM_LINK_CTRL_1_TETH_EEE_CAP_KR: u32                   = 1 << 26;
pub const IXGBE_KRM_LINK_S1_MAC_AN_COMPLETE: u32                       = 1 << 28;
pub const IXGBE_KRM_LINK_CTRL_1_TETH_AN_ENABLE: u32                    = 1 << 29;
pub const IXGBE_KRM_LINK_CTRL_1_TETH_AN_RESTART: u32                   = 1 << 31;

pub const IXGBE_KRM_AN_CNTL_1_SYM_PAUSE: u32                           = 1 << 28;
pub const IXGBE_KRM_AN_CNTL_1_ASM_PAUSE: u32                           = 1 << 29;
pub const IXGBE_KRM_PCS_KX_AN_SYM_PAUSE: u32                           = 1 << 1;
pub const IXGBE_KRM_PCS_KX_AN_ASM_PAUSE: u32                           = 1 << 2;
pub const IXGBE_KRM_PCS_KX_AN_LP_SYM_PAUSE: u32                        = 1 << 2;
pub const IXGBE_KRM_PCS_KX_AN_LP_ASM_PAUSE: u32                        = 1 << 3;
pub const IXGBE_KRM_AN_CNTL_4_ECSR_AN37_OVER_73: u32                   = 1 << 29;
pub const IXGBE_KRM_AN_CNTL_8_LINEAR: u32                              = 1 << 0;
pub const IXGBE_KRM_AN_CNTL_8_LIMITING: u32                            = 1 << 1;

pub const IXGBE_KRM_LP_BASE_PAGE_HIGH_SYM_PAUSE: u32                   = 1 << 10;
pub const IXGBE_KRM_LP_BASE_PAGE_HIGH_ASM_PAUSE: u32                   = 1 << 11;

pub const IXGBE_KRM_SGMII_CTRL_MAC_TAR_FORCE_100_D: u32                = 1 << 12;
pub const IXGBE_KRM_SGMII_CTRL_MAC_TAR_FORCE_10_D: u32                 = 1 << 19;

pub const IXGBE_KRM_DSP_TXFFE_STATE_C0_EN: u32                         = 1 << 6;
pub const IXGBE_KRM_DSP_TXFFE_STATE_CP1_CN1_EN: u32                    = 1 << 15;
pub const IXGBE_KRM_DSP_TXFFE_STATE_CO_ADAPT_EN: u32                   = 1 << 16;

pub const IXGBE_KRM_RX_TRN_LINKUP_CTRL_CONV_WO_PROTOCOL: u32           = 1 << 4;
pub const IXGBE_KRM_RX_TRN_LINKUP_CTRL_PROTOCOL_BYPASS: u32            = 1 << 2;

pub const IXGBE_KRM_PMD_DFX_BURNIN_TX_RX_KR_LB_MASK: u32               = 0x3 << 16;

pub const IXGBE_KRM_TX_COEFF_CTRL_1_CMINUS1_OVRRD_EN: u32              = 1 << 1;
pub const IXGBE_KRM_TX_COEFF_CTRL_1_CPLUS1_OVRRD_EN: u32               = 1 << 2;
pub const IXGBE_KRM_TX_COEFF_CTRL_1_CZERO_EN: u32                      = 1 << 3;
pub const IXGBE_KRM_TX_COEFF_CTRL_1_OVRRD_EN: u32                      = 1 << 31;

pub const IXGBE_SB_IOSF_INDIRECT_CTRL: u32                             = 0x00011144;
pub const IXGBE_SB_IOSF_INDIRECT_DATA: u32                             = 0x00011148;

pub const IXGBE_SB_IOSF_CTRL_ADDR_SHIFT: u32                           = 0;
pub const IXGBE_SB_IOSF_CTRL_ADDR_MASK: u32                            = 0xFF;
pub const IXGBE_SB_IOSF_CTRL_RESP_STAT_SHIFT: u32                      = 18;
pub const IXGBE_SB_IOSF_CTRL_RESP_STAT_MASK: u32                       = 0x3 << IXGBE_SB_IOSF_CTRL_RESP_STAT_SHIFT;
pub const IXGBE_SB_IOSF_CTRL_CMPL_ERR_SHIFT: u32                       = 20;
pub const IXGBE_SB_IOSF_CTRL_CMPL_ERR_MASK: u32                        = 0xFF << IXGBE_SB_IOSF_CTRL_CMPL_ERR_SHIFT;
pub const IXGBE_SB_IOSF_CTRL_TARGET_SELECT_SHIFT: u32                  = 28;
pub const IXGBE_SB_IOSF_CTRL_TARGET_SELECT_MASK: u32                   = 0x7;
pub const IXGBE_SB_IOSF_CTRL_BUSY_SHIFT: u32                           = 31;
pub const IXGBE_SB_IOSF_CTRL_BUSY: u32                                 = 1 << IXGBE_SB_IOSF_CTRL_BUSY_SHIFT;
pub const IXGBE_SB_IOSF_TARGET_KR_PHY: u32                             = 0;

pub const IXGBE_NW_MNG_IF_SEL: u32                                     = 0x00011178;
pub const IXGBE_NW_MNG_IF_SEL_MDIO_ACT: u32                            = 1 << 1;
pub const IXGBE_NW_MNG_IF_SEL_MDIO_IF_MODE: u32                        = 1 << 2;
pub const IXGBE_NW_MNG_IF_SEL_EN_SHARED_MDIO: u32                      = 1 << 13;
pub const IXGBE_NW_MNG_IF_SEL_PHY_SPEED_10M: u32                       = 1 << 17;
pub const IXGBE_NW_MNG_IF_SEL_PHY_SPEED_100M: u32                      = 1 << 18;
pub const IXGBE_NW_MNG_IF_SEL_PHY_SPEED_1G: u32                        = 1 << 19;
pub const IXGBE_NW_MNG_IF_SEL_PHY_SPEED_2_5G: u32                      = 1 << 20;
pub const IXGBE_NW_MNG_IF_SEL_PHY_SPEED_10G: u32                       = 1 << 21;
pub const IXGBE_NW_MNG_IF_SEL_SGMII_ENABLE: u32                        = 1 << 25;
pub const IXGBE_NW_MNG_IF_SEL_INT_PHY_MODE: u32                        = 1 << 24; /* X552 reg field only */
pub const IXGBE_NW_MNG_IF_SEL_MDIO_PHY_ADD_SHIFT: u32                  = 3;
pub const IXGBE_NW_MNG_IF_SEL_MDIO_PHY_ADD: u32                        = 0x1F << IXGBE_NW_MNG_IF_SEL_MDIO_PHY_ADD_SHIFT;

// Constants and functions for ixgbevf

pub const IXGBE_VF_IRQ_CLEAR_MASK: u32                                 = 7;
pub const IXGBE_VF_MAX_TX_QUEUES: u32                                  = 8;
pub const IXGBE_VF_MAX_RX_QUEUES: u32                                  = 8;

/* DCB define */
pub const IXGBE_VF_MAX_TRAFFIC_CLASS: u32                              = 8;

pub const IXGBE_VFCTRL: u32                                            = 0x00000;
pub const IXGBE_VFSTATUS: u32                                          = 0x00008;
pub const IXGBE_VFLINKS: u32                                           = 0x00010;
pub const IXGBE_VFFRTIMER: u32                                         = 0x00048;
pub const IXGBE_VFRXMEMWRAP: u32                                       = 0x03190;
pub const IXGBE_VTEICR: u32                                            = 0x00100;
pub const IXGBE_VTEICS: u32                                            = 0x00104;
pub const IXGBE_VTEIMS: u32                                            = 0x00108;
pub const IXGBE_VTEIMC: u32                                            = 0x0010C;
pub const IXGBE_VTEIAC: u32                                            = 0x00110;
pub const IXGBE_VTEIAM: u32                                            = 0x00114;

pub fn IXGBE_VTEITR(x: u32) -> u32 { 0x00820 + 4 * x }
pub fn IXGBE_VTIVAR(x: u32) -> u32 { 0x00120 + 4 * x }
pub const IXGBE_VTIVAR_MISC: u32                                       = 0x00140;
pub fn IXGBE_VTRSCINT(x: u32) -> u32 { 0x00180 + 4 * x }
/* define IXGBE_VFPBACL  still says TBD in EAS */
pub fn IXGBE_VFRDBAL(x: u32) -> u32 { 0x01000 + 0x40 * x }
pub fn IXGBE_VFRDBAH(x: u32) -> u32 { 0x01004 + 0x40 * x }
pub fn IXGBE_VFRDLEN(x: u32) -> u32 { 0x01008 + 0x40 * x }
pub fn IXGBE_VFRDH(x: u32) -> u32 { 0x01010 + 0x40 * x }
pub fn IXGBE_VFRDT(x: u32) -> u32 { 0x01018 + 0x40 * x }
pub fn IXGBE_VFRXDCTL(x: u32) -> u32 { 0x01028 + 0x40 * x }
pub fn IXGBE_VFSRRCTL(x: u32) -> u32 { 0x01014 + 0x40 * x }
pub fn IXGBE_VFRSCCTL(x: u32) -> u32 { 0x0102C + 0x40 * x }
pub const IXGBE_VFPSRTYPE: u32                                         = 0x00300;
pub fn IXGBE_VFTDBAL(x: u32) -> u32 { 0x02000 + 0x40 * x }
pub fn IXGBE_VFTDBAH(x: u32) -> u32 { 0x02004 + 0x40 * x }
pub fn IXGBE_VFTDLEN(x: u32) -> u32 { 0x02008 + 0x40 * x }
pub fn IXGBE_VFTDH(x: u32) -> u32 { 0x02010 + 0x40 * x }
pub fn IXGBE_VFTDT(x: u32) -> u32 { 0x02018 + 0x40 * x }
pub fn IXGBE_VFTXDCTL(x: u32) -> u32 { 0x02028 + 0x40 * x }
pub fn IXGBE_VFTDWBAL(x: u32) -> u32 { 0x02038 + 0x40 * x }
pub fn IXGBE_VFTDWBAH(x: u32) -> u32 { 0x0203C + 0x40 * x }
pub fn IXGBE_VFDCA_RXCTRL(x: u32) -> u32 { 0x0100C + 0x40 * x }
pub fn IXGBE_VFDCA_TXCTRL(x: u32) -> u32 { 0x0200c + 0x40 * x }
pub const IXGBE_VFGPRC: u32                                            = 0x0101C;
pub const IXGBE_VFGPTC: u32                                            = 0x0201C;
pub const IXGBE_VFGORC_LSB: u32                                        = 0x01020;
pub const IXGBE_VFGORC_MSB: u32                                        = 0x01024;
pub const IXGBE_VFGOTC_LSB: u32                                        = 0x02020;
pub const IXGBE_VFGOTC_MSB: u32                                        = 0x02024;
pub const IXGBE_VFMPRC: u32                                            = 0x01034;
pub const IXGBE_VFMRQC: u32                                            = 0x3000;
pub fn IXGBE_VFRSSRK(x: u32) -> u32 { 0x3100 + x * 4 }
pub fn IXGBE_VFRETA(x: u32) -> u32 { 0x3200 + x * 4 }

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ixgbevf_hw_stats {
    pub base_vfgprc: u64,
    pub base_vfgptc: u64,
    pub base_vfgorc: u64,
    pub base_vfgotc: u64,
    pub base_vfmprc: u64,

    pub last_vfgprc: u64,
    pub last_vfgptc: u64,
    pub last_vfgorc: u64,
    pub last_vfgotc: u64,
    pub last_vfmprc: u64,

    pub vfgprc: u64,
    pub vfgptc: u64,
    pub vfgorc: u64,
    pub vfgotc: u64,
    pub vfmprc: u64,

    pub saved_reset_vfgprc: u64,
    pub saved_reset_vfgptc: u64,
    pub saved_reset_vfgorc: u64,
    pub saved_reset_vfgotc: u64,
    pub saved_reset_vfmprc: u64,
}

// Constants and functions for ixgbevf mailbox communication

pub const IXGBE_VFMAILBOX_SIZE: u16                     = 16; /* 16 32 bit words - 64 bytes */
pub const IXGBE_ERR_MBX: i32                            = -100;

pub const IXGBE_VFMAILBOX: u32                          = 0x002FC;
pub const IXGBE_VFMBMEM: u32                            = 0x00200;

/* Define mailbox register bits */
pub const IXGBE_VFMAILBOX_REQ: u32                      = 0x00000001; /* Request for PF Ready bit */
pub const IXGBE_VFMAILBOX_ACK: u32                      = 0x00000002; /* Ack PF message received */
pub const IXGBE_VFMAILBOX_VFU: u32                      = 0x00000004; /* VF owns the mailbox buffer */
pub const IXGBE_VFMAILBOX_PFU: u32                      = 0x00000008; /* PF owns the mailbox buffer */
pub const IXGBE_VFMAILBOX_PFSTS: u32                    = 0x00000010; /* PF wrote a message in the MB */
pub const IXGBE_VFMAILBOX_PFACK: u32                    = 0x00000020; /* PF ack the previous VF msg */
pub const IXGBE_VFMAILBOX_RSTI: u32                     = 0x00000040; /* PF has reset indication */
pub const IXGBE_VFMAILBOX_RSTD: u32                     = 0x00000080; /* PF has indicated reset done */
pub const IXGBE_VFMAILBOX_R2C_BITS: u32                 = 0x000000B0; /* All read to clear bits */

pub const IXGBE_PFMAILBOX_STS: u32                      = 0x00000001; /* Initiate message send to VF */
pub const IXGBE_PFMAILBOX_ACK: u32                      = 0x00000002; /* Ack message recv'd from VF */
pub const IXGBE_PFMAILBOX_VFU: u32                      = 0x00000004; /* VF owns the mailbox buffer */
pub const IXGBE_PFMAILBOX_PFU: u32                      = 0x00000008; /* PF owns the mailbox buffer */
pub const IXGBE_PFMAILBOX_RVFU: u32                     = 0x00000010; /* Reset VFU - used when VF stuck */

pub const IXGBE_MBVFICR_VFREQ_MASK: u32                 = 0x0000FFFF; /* bits for VF messages */
pub const IXGBE_MBVFICR_VFREQ_VF1: u32                  = 0x00000001; /* bit for VF 1 message */
pub const IXGBE_MBVFICR_VFACK_MASK: u32                 = 0xFFFF0000; /* bits for VF acks */
pub const IXGBE_MBVFICR_VFACK_VF1: u32                  = 0x00010000; /* bit for VF 1 ack */


/* If it's a IXGBE_VF_* msg then it originates in the VF and is sent to the
 * PF.  The reverse is true if it is IXGBE_PF_*.
 * Message ACK's are the value or'd with 0xF0000000
 */
pub const IXGBE_VT_MSGTYPE_ACK: u32                     = 0x80000000; /* Messages below or'd with this are the ACK */
pub const IXGBE_VT_MSGTYPE_NACK: u32                    = 0x40000000; /* Messages below or'd with this are the NACK */
pub const IXGBE_VT_MSGTYPE_CTS: u32                     = 0x20000000; /* Indicates that VF is still clear to send requests */
pub const IXGBE_VT_MSGINFO_SHIFT: u32                   = 16;
/* bits 23:16 are used for extra info for certain messages */
pub const IXGBE_VT_MSGINFO_MASK: u32                    = 0xFF << IXGBE_VT_MSGINFO_SHIFT;

/* definitions to support mailbox API version negotiation */

/*
 * each element denotes a version of the API; existing numbers may not
 * change; any additions must go at the end
 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ixgbe_pfvf_api_rev {
    ixgbe_mbox_api_10,	/* API version 1.0, linux/freebsd VF driver */
    ixgbe_mbox_api_20,	/* API version 2.0, solaris Phase1 VF driver */
    ixgbe_mbox_api_11,	/* API version 1.1, linux/freebsd VF driver */
    ixgbe_mbox_api_12,	/* API version 1.2, linux/freebsd VF driver */
    ixgbe_mbox_api_13,	/* API version 1.3, linux/freebsd VF driver */
    /* This value should always be last */
    ixgbe_mbox_api_unknown,	/* indicates that API version is not known */
}

/* mailbox API, legacy requests */
pub const IXGBE_VF_RESET: u32                           = 0x01; /* VF requests reset */
pub const IXGBE_VF_SET_MAC_ADDR: u32                    = 0x02; /* VF requests PF to set MAC addr */
pub const IXGBE_VF_SET_MULTICAST: u32                   = 0x03; /* VF requests PF to set MC addr */
pub const IXGBE_VF_SET_VLAN: u32                        = 0x04; /* VF requests PF to set VLAN */

/* mailbox API, version 1.0 VF requests */
pub const IXGBE_VF_SET_LPE: u32                         = 0x05; /* VF requests PF to set VMOLR.LPE */
pub const IXGBE_VF_SET_MACVLAN: u32                     = 0x06; /* VF requests PF for unicast filter */
pub const IXGBE_VF_API_NEGOTIATE: u32                   = 0x08; /* negotiate API version */

/* mailbox API, version 1.1 VF requests */
pub const IXGBE_VF_GET_QUEUES: u32                      = 0x09; /* get queue configuration */

/* mailbox API, version 1.2 VF requests */
pub const IXGBE_VF_GET_RETA: u32                        = 0x0a;    /* VF request for RETA */
pub const IXGBE_VF_GET_RSS_KEY: u32                     = 0x0b;    /* get RSS key */
pub const IXGBE_VF_UPDATE_XCAST_MODE: u32               = 0x0c;

/* mode choices for IXGBE_VF_UPDATE_XCAST_MODE */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ixgbevf_xcast_modes {
    IXGBEVF_XCAST_MODE_NONE = 0,
    IXGBEVF_XCAST_MODE_MULTI,
    IXGBEVF_XCAST_MODE_ALLMULTI,
    IXGBEVF_XCAST_MODE_PROMISC,
}

/* GET_QUEUES return data indices within the mailbox */
pub const IXGBE_VF_TX_QUEUES: u32                       = 1; /* number of Tx queues supported */
pub const IXGBE_VF_RX_QUEUES: u32                       = 2; /* number of Rx queues supported */
pub const IXGBE_VF_TRANS_VLAN: u32                      = 3; /* Indication of port vlan */
pub const IXGBE_VF_DEF_QUEUE: u32                       = 4; /* Default queue offset */

/* length of permanent address message returned from PF */
pub const IXGBE_VF_PERMADDR_MSG_LEN: u32                = 4;
/* word in permanent address message with the current multicast type */
pub const IXGBE_VF_MC_TYPE_WORD: u32                    = 3;

pub const IXGBE_PF_CONTROL_MSG: u32                     = 0x0100; /* PF control message */

/* mailbox API, version 2.0 VF requests */
/* the following two constants were already defined earlier */
// pub const IXGBE_VF_API_NEGOTIATE: u32                = 0x08; /* negotiate API version */
// pub const IXGBE_VF_GET_QUEUES: u32                   = 0x09; /* get queue configuration */
pub const IXGBE_VF_ENABLE_MACADDR: u32                  = 0x0A; /* enable MAC address */
pub const IXGBE_VF_DISABLE_MACADDR: u32                 = 0x0B; /* disable MAC address */
pub const IXGBE_VF_GET_MACADDRS: u32                    = 0x0C; /* get all configured MAC addrs */
pub const IXGBE_VF_SET_MCAST_PROMISC: u32               = 0x0D; /* enable multicast promiscuous */
pub const IXGBE_VF_GET_MTU: u32                         = 0x0E; /* get bounds on MTU */
pub const IXGBE_VF_SET_MTU: u32                         = 0x0F; /* set a specific MTU */

/* mailbox API, version 2.0 PF requests */
pub const IXGBE_PF_TRANSPARENT_VLAN: u32                = 0x0101; /* enable transparent vlan */

pub const IXGBE_VF_MBX_INIT_TIMEOUT: u32                = 2000; /* number of retries on mailbox */
pub const IXGBE_VF_MBX_INIT_DELAY: u32                  = 500;  /* microseconds between retries */
