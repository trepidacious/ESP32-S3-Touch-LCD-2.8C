//! Canonical init sequences for known ST7701-based panels.
//!
//! Each sequence is a `&[Step]` table that can be passed straight to
//! [`crate::St7701::init`] / [`crate::St7701::run`]. The byte values
//! come from each panel's reference driver — *don't* mix and match
//! between sequences: the gamma table, MADCTL byte, and pixel-format
//! command are all subtly panel-specific.

use crate::Step;

/// Commands from table 1, these should always be supported
#[repr(u8)]
enum Command1 {
    SwReset = 0x01,    // Software Reset
    SlpOut = 0x11,     // Sleep Out
    PtlOn = 0x12,      // Partial Display Mode On
    NorOn = 0x13,      // Normal Display Mode On
    InvOff = 0x20,     // Display Inversion Off
    InvOn = 0x21,      // Display Inversion On
    AllPOff = 0x22,    // All Pixels Off
    AllPon = 0x23,     // All Pixels On
    GamSet = 0x26,     // Gamma Set
    DispOff = 0x28,    // Display Off
    DispOn = 0x29,     // Display On
    TeOff = 0x34,      // Tearing Effect Line Off (kinda vsync)
    TeOn = 0x35,       // Tearing Effect Line On (kinda vsync)
    MadCtl = 0x36,     // Display data access control
    IdmOff = 0x38,     // Idle Mode Off
    IdmOn = 0x39,      // Idle Mode On
    ColMod = 0x3A,     // Interface Pixel Format
    Gsl = 0x45,        // Get Scan Line
    Cnd2BkxSel = 0xFF, // Select which command2 banks are enabled
}

// Commands from table 2, bank 0 - this bank must be enabled before using
#[repr(u8)]
enum Command2Bank0 {
    PvGamCtrl = 0xB0, // Positive Voltage Gamma Control
    NvGamCtrl = 0xB1, // Negative Voltage Gamma Control
    DgmEn = 0xB8,     // Digital Gamma Enable
    DgmLutR = 0xB9,   // Digital Gamma LUT for Red
    DgmLutB = 0xBA,   // Digital Gamma Lut for Blue
    LneSet = 0xC0,    // Display Line Setting
    PorCtrl = 0xC1,   // Porch Control
    InvSet = 0xC2,    // Inversion Selection & Frame Rate Control
    RgbCtrl = 0xC3,   // RGB Control
    ParCtrl = 0xC5,   // Partial Mode Control
    SDir = 0xC7,      // X-direction Control
    PdoSet = 0xC8,    // Pseudo-Dot Inversion Diving Settign
    ColCtrl = 0xCD,   // Colour Control
    SreCtrl = 0xE0,   // Sunlight Readable Enhancement
    NrCtrl = 0xE1,    // Noise Reduce Control
    SeCtrl = 0xE2,    // Sharpness Control
    CcCtrl = 0xE3,    // Color Calibration Control
    SkCtrl = 0xE4,    // Skin Tone Preservation Control
}

// Commands from table 2, bank 1 - this bank must be enabled before using
#[repr(u8)]
enum Command2Bank1 {
    Vhrs = 0xB0,    // Vop amplitude
    VcomS = 0xB1,   // VCOM amplitude
    VghSs = 0xB2,   // VGH voltage
    TestCmd = 0xB3, // TEST command
    VglS = 0xB5,    // VGL voltage
    VrhDv = 0xB6,   // VRH_DV voltage
    PwCtrl1 = 0xB7, // Power Control 1
    PwCtrl2 = 0xB8, // Power Control 2
    PClkS1 = 0xBA,  // Power pumping clock selection 1
    PClkS2 = 0xBC,  // Power pumping clock selection 2
    Pdr1 = 0xC1,    // Source pre_drive timing set 1
    Pdr2 = 0xC2,    // Source pre_drive timing set 2
}

// Commands from table 2, bank 3 - this bank must be enabled before using
#[repr(u8)]
enum Command2Bank3 {
    NvmEn = 0xC8,   // NVM enable
    NvmSet = 0xCA,  // NVM manual control
    PromAct = 0xCC, // NVM program active
}

/// Step to disable all command2 banks
pub const COMMAND2_BANKS_DISABLE: Step = Step {
    cmd: Command1::Cnd2BkxSel as u8,
    data: &[0x77, 0x01, 0x00, 0x00, 0x00],
    delay_ms: 0,
};

/// Step to enable command2 bank 0
pub const COMMAND2_BANK0_ENABLE: Step = Step {
    cmd: Command1::Cnd2BkxSel as u8,
    data: &[0x77, 0x01, 0x00, 0x00, 0x10],
    delay_ms: 0,
};

/// Step to enable command2 bank 1
pub const COMMAND2_BANK1_ENABLE: Step = Step {
    cmd: Command1::Cnd2BkxSel as u8,
    data: &[0x77, 0x01, 0x00, 0x00, 0x11],
    delay_ms: 0,
};

/// Step to enable command2 bank 3
pub const COMMAND2_BANK3_ENABLE: Step = Step {
    cmd: Command1::Cnd2BkxSel as u8,
    data: &[0x77, 0x01, 0x00, 0x00, 0x13],
    delay_ms: 0,
};

/// Init sequence for the Pimoroni Presto board  —
/// a 480×480 square IPS panel with an ST7701 controller.
///
/// Values are taken from the [presto micropython driver](https://github.com/pimoroni/presto/blob/main/drivers/st7701/st7701.cpp)
pub const PIMORONI_PRESTO: &[Step] = &[
    Step {
        cmd: Command1::SwReset as u8,
        data: &[],
        delay_ms: 150,
    },
    //
    // Bank 0
    //
    COMMAND2_BANK0_ENABLE,
    // (59 + 1) * 8 = 480 lines
    Step {
        cmd: Command2Bank0::LneSet as u8,
        data: &[0x3b, 0x00],
        delay_ms: 0,
    },
    // Display porch settings: 13 VBP, 2 VFP (these should not be changed)
    Step {
        cmd: Command2Bank0::PorCtrl as u8,
        data: &[0x0d, 0x02],
        delay_ms: 0,
    },
    Step {
        cmd: Command2Bank0::InvSet as u8,
        data: &[0x31, 0x01],
        delay_ms: 0,
    },
    // LED polarity reversed
    Step {
        cmd: Command2Bank0::ColCtrl as u8,
        data: &[0x08],
        delay_ms: 0,
    },
    Step {
        cmd: Command2Bank0::PvGamCtrl as u8,
        data: &[
            0x00, 0x11, 0x18, 0x0e, 0x11, 0x06, 0x07, 0x08, 0x07, 0x22, 0x04, 0x12, 0x0f, 0xaa,
            0x31, 0x18,
        ],
        delay_ms: 0,
    },
    Step {
        cmd: Command2Bank0::NvGamCtrl as u8,
        data: &[
            0x00, 0x11, 0x19, 0x0e, 0x12, 0x07, 0x08, 0x08, 0x08, 0x22, 0x04, 0x11, 0x11, 0xa9,
            0x32, 0x18,
        ],
        delay_ms: 0,
    },
    // HV mode, H and V back porch + sync
    Step {
        cmd: Command2Bank0::RgbCtrl as u8,
        data: &[0x80, 0x2e, 0x0e],
        delay_ms: 0,
    },
    //
    // Bank 1
    //
    COMMAND2_BANK1_ENABLE,
    // 4.7375v
    Step {
        cmd: Command2Bank1::Vhrs as u8,
        data: &[0x60],
        delay_ms: 0,
    },
    // 0.725v
    Step {
        cmd: Command2Bank1::VcomS as u8,
        data: &[0x32],
        delay_ms: 0,
    },
    // 15v
    Step {
        cmd: Command2Bank1::VghSs as u8,
        data: &[0x07],
        delay_ms: 0,
    },
    // y tho?
    Step {
        cmd: Command2Bank1::TestCmd as u8,
        data: &[0x80],
        delay_ms: 0,
    },
    // -10.17v
    Step {
        cmd: Command2Bank1::VglS as u8,
        data: &[0x49],
        delay_ms: 0,
    },
    // Middle/Min/Min bias
    Step {
        cmd: Command2Bank1::PwCtrl1 as u8,
        data: &[0x85],
        delay_ms: 0,
    },
    // 6.6 / -4.6
    Step {
        cmd: Command2Bank1::PwCtrl2 as u8,
        data: &[0x21],
        delay_ms: 0,
    },
    // 1.6uS
    Step {
        cmd: Command2Bank1::Pdr1 as u8,
        data: &[0x78],
        delay_ms: 0,
    },
    // 6.4uS
    Step {
        cmd: Command2Bank1::Pdr2 as u8,
        data: &[0x78],
        delay_ms: 0,
    },
    // Begin Forbidden Knowledge
    // This sequence is probably specific to TL040WVS03CT15-H1263A.
    // It is not documented in the ST7701s datasheet.
    // Display does not work when removing those
    Step {
        cmd: 0xE0,
        data: &[0x00, 0x1b, 0x02],
        delay_ms: 0,
    },
    Step {
        cmd: 0xE1,
        data: &[
            0x08, 0xa0, 0x00, 0x00, 0x07, 0xa0, 0x00, 0x00, 0x00, 0x44, 0x44,
        ],
        delay_ms: 0,
    },
    Step {
        cmd: 0xE2,
        data: &[
            0x11, 0x11, 0x44, 0x44, 0xed, 0xa0, 0x00, 0x00, 0xec, 0xa0, 0x00, 0x00,
        ],
        delay_ms: 0,
    },
    Step {
        cmd: 0xE3,
        data: &[0x00, 0x00, 0x11, 0x11],
        delay_ms: 0,
    },
    Step {
        cmd: 0xE4,
        data: &[0x44, 0x44],
        delay_ms: 0,
    },
    Step {
        cmd: 0xE5,
        data: &[
            0x0a, 0xe9, 0xd8, 0xa0, 0x0c, 0xeb, 0xd8, 0xa0, 0x0e, 0xed, 0xd8, 0xa0, 0x10, 0xef,
            0xd8, 0xa0,
        ],
        delay_ms: 0,
    },
    Step {
        cmd: 0xE6,
        data: &[0x00, 0x00, 0x11, 0x11],
        delay_ms: 0,
    },
    Step {
        cmd: 0xE7,
        data: &[0x44, 0x44],
        delay_ms: 0,
    },
    Step {
        cmd: 0xE8,
        data: &[
            0x09, 0xe8, 0xd8, 0xa0, 0x0b, 0xea, 0xd8, 0xa0, 0x0d, 0xec, 0xd8, 0xa0, 0x0f, 0xee,
            0xd8, 0xa0,
        ],
        delay_ms: 0,
    },
    Step {
        cmd: 0xEB,
        data: &[0x02, 0x00, 0xe4, 0xe4, 0x88, 0x00, 0x40],
        delay_ms: 0,
    },
    Step {
        cmd: 0xEC,
        data: &[0x3c, 0x00],
        delay_ms: 0,
    },
    Step {
        cmd: 0xED,
        data: &[
            0xab, 0x89, 0x76, 0x54, 0x02, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x20, 0x45, 0x67,
            0x98, 0xba,
        ],
        delay_ms: 0,
    },
    Step {
        cmd: 0x36,
        data: &[0x00],
        delay_ms: 0,
    },
    //
    // Command 2 Bank 3
    //
    COMMAND2_BANK3_ENABLE,
    Step {
        cmd: 0xE5,
        data: &[0xe4],
        delay_ms: 0,
    },
    // End Forbidden Knowledge
    //
    // Disable all Command 2 banks
    COMMAND2_BANKS_DISABLE,
    // 24 bits per pixel...
    // Step {
    //     cmd: Command1::ColMod as u8,
    //     data: &[0x77],
    //     delay_ms: 0,
    // },
    // 18 bits per pixel...
    Step {
        cmd: Command1::ColMod as u8,
        data: &[0x66],
        delay_ms: 0,
    },
    // 16 bits per pixel...
    // Step {
    //     cmd: Command1::ColMod as u8,
    //     data: &[0x55],
    //     delay_ms: 0,
    // },

    // TODO: Support colour mode RGB/BGR by either reading current MADCTL from the display or using a data member

    // Rotate 0
    Step {
        cmd: Command1::MadCtl as u8,
        data: &[0x00],
        delay_ms: 0,
    },
    COMMAND2_BANK0_ENABLE,
    Step {
        cmd: Command2Bank0::SDir as u8,
        data: &[0x00],
        delay_ms: 0,
    },
    COMMAND2_BANKS_DISABLE,
    // Rotate 180
    // Step {
    //     cmd: Command1::MadCtl as u8,
    //     data: &[0x10],
    //     delay_ms: 0,
    // },
    // COMMAND2_BANK0_ENABLE,
    // Step {
    //     cmd: Command2Bank0::SDir as u8,
    //     data: &[0x04],
    //     delay_ms: 0,
    // },
    // COMMAND2_BANKS_DISABLE,
    Step {
        cmd: Command1::InvOn as u8,
        data: &[],
        delay_ms: 1,
    },
    Step {
        cmd: Command1::SlpOut as u8,
        data: &[],
        delay_ms: 120,
    },
    Step {
        cmd: Command1::DispOn as u8,
        data: &[],
        delay_ms: 50,
    },
];
