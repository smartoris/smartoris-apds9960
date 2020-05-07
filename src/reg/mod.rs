//! APDS-9960 registers.

#[macro_use]
mod macros;

use crate::{Apds9960Drv, Apds9960I2CPort};
use drone_core::bitfield::Bitfield;
use futures::prelude::*;

apds9960_reg! {
    /// Enable states and interrupts.
    Enable u8 1 0x80 0x00 rw {
        /// Reads contents of ENABLE register.
        fn load_enable;
        /// Writes `value` to ENABLE register.
        fn store_enable_val;
        /// Passes the reset value to the closure `f`, then writes the result of
        /// the closure to ENABLE register.
        fn store_enable;
    }
    [
        pon(0, "Power ON."),
        aen(1, "ALS Enable."),
        pen(2, "Proximity Detect Enable."),
        wen(3, "Wait Enable."),
        aien(4, "ALS Interrupt Enable."),
        pien(5, "Proximity Interrupt Enable."),
        gen(6, "Gesture Enable."),
    ]
}

apds9960_reg_raw! {
    u8 1 0x81 rw {
        /// Reads contents of ATIME register.
        fn load_atime;
        /// Writes `value` to ATIME register.
        fn store_atime;
    }
}

apds9960_reg_raw! {
    u8 1 0x83 rw {
        /// Reads contents of WTIME register.
        fn load_wtime;
        /// Writes `value` to WTIME register.
        fn store_wtime;
    }
}

apds9960_reg_raw! {
    u16 2 0x84 rw {
        /// Reads contents of AILT register.
        fn load_ailt;
        /// Writes `value` to AILT register.
        fn store_ailt;
    }
}

apds9960_reg_raw! {
    u16 2 0x86 rw {
        /// Reads contents of AIHT register.
        fn load_aiht;
        /// Writes `value` to AIHT register.
        fn store_aiht;
    }
}

apds9960_reg_raw! {
    u8 1 0x89 rw {
        /// Reads contents of PILT register.
        fn load_pilt;
        /// Writes `value` to PILT register.
        fn store_pilt;
    }
}

apds9960_reg_raw! {
    u8 1 0x8B rw {
        /// Reads contents of PIHT register.
        fn load_piht;
        /// Writes `value` to PIHT register.
        fn store_piht;
    }
}

apds9960_reg! {
    /// Interrupt persistence filters (non-gesture.)
    Pers u8 1 0x8C 0x00 rw {
        /// Reads contents of PERS register.
        fn load_pers;
        /// Writes `value` to PERS register.
        fn store_pers_val;
        /// Passes the reset value to the closure `f`, then writes the result of
        /// the closure to PERS register.
        fn store_pers;
    }
    [
        apers(0, 4, "ALS Interrupt Persistence."),
        ppers(4, 4, "Proximity Interrupt Persistence."),
    ]
}

apds9960_reg! {
    /// Configuration register one.
    Config1 u8 1 0x8D 0x60 rw {
        /// Reads contents of CONFIG1 register.
        fn load_config1;
        /// Writes `value` to CONFIG1 register.
        fn store_config1_val;
        /// Passes the reset value to the closure `f`, then writes the result of
        /// the closure to CONFIG1 register.
        fn store_config1;
    }
    [
        wlong(1, "Wait Long."),
    ]
}

apds9960_reg! {
    /// Proximity pulse count and length.
    Ppulse u8 1 0x8E 0x40 rw {
        /// Reads contents of PPULSE register.
        fn load_ppulse;
        /// Writes `value` to PPULSE register.
        fn store_ppulse_val;
        /// Passes the reset value to the closure `f`, then writes the result of
        /// the closure to PPULSE register.
        fn store_ppulse;
    }
    [
        ppulse(0, 6, "Proximity Pulse Count."),
        pplen(6, 2, "Proximity Pulse Length."),
    ]
}

apds9960_reg! {
    /// Gain control.
    Control u8 1 0x8F 0x00 rw {
        /// Reads contents of CONTROL register.
        fn load_control;
        /// Writes `value` to CONTROL register.
        fn store_control_val;
        /// Passes the reset value to the closure `f`, then writes the result of
        /// the closure to CONTROL register.
        fn store_control;
    }
    [
        again(0, 2, "ALS and Color Gain Control."),
        pgain(2, 2, "Proximity Gain Control."),
        ldrive(6, 2, "LED Drive Strength."),
    ]
}

apds9960_reg! {
    /// Configuration register two.
    Config2 u8 1 0x90 0x01 rw {
        /// Reads contents of CONFIG2 register.
        fn load_config2;
        /// Writes `value` to CONFIG2 register.
        fn store_config2_val;
        /// Passes the reset value to the closure `f`, then writes the result of
        /// the closure to CONFIG2 register.
        fn store_config2;
    }
    [
        led_boost(4, 2, "Additional LDR current during proximity and gesture LED pulses."),
        cpsien(6, "Clear Photodiode Saturation Interrupt Enable."),
        psien(7, "Proximity Saturation Interrupt Enable."),
    ]
}

apds9960_reg_raw! {
    u8 1 0x92 r {
        /// Reads contents of ID register.
        fn load_id;
    }
}

apds9960_reg! {
    /// Device status.
    Status u8 1 0x93 0x00 r {
        /// Reads contents of STATUS register.
        fn load_status;
    }
    [
        avalid(0, "ALS Valid."),
        pvalid(1, "Proximity Valid."),
        gint(2, "Gesture Interrupt."),
        aint(4, "ALS Interrupt."),
        pint(5, "Proximity Interrupt."),
        pgsat(6, "Indicates that an analog saturation event occurred during a previous proximity \
                  or gesture cycle."),
        cpsat(7, "Clear Photodiode Saturation."),
    ]
}

apds9960_reg_raw! {
    u16 2 0x94 r {
        /// Reads contents of CDATA register.
        fn load_cdata;
    }
}

apds9960_reg_raw! {
    u16 2 0x96 r {
        /// Reads contents of RDATA register.
        fn load_rdata;
    }
}

apds9960_reg_raw! {
    u16 2 0x98 r {
        /// Reads contents of GDATA register.
        fn load_gdata;
    }
}

apds9960_reg_raw! {
    u16 2 0x9A r {
        /// Reads contents of BDATA register.
        fn load_bdata;
    }
}

apds9960_reg_raw! {
    u8 1 0x9C r {
        /// Reads contents of BDATA register.
        fn load_pdata;
    }
}

apds9960_reg_raw! {
    u8 1 0x9D rw {
        /// Reads contents of POFFSET_UR register.
        fn load_poffset_ur;
        /// Writes `value` to POFFSET_UR register.
        fn store_poffset_ur;
    }
}

apds9960_reg_raw! {
    u8 1 0x9E rw {
        /// Reads contents of POFFSET_DL register.
        fn load_poffset_dl;
        /// Writes `value` to POFFSET_DL register.
        fn store_poffset_dl;
    }
}

apds9960_reg! {
    /// Configuration register three.
    Config3 u8 1 0x9F 0x00 rw {
        /// Reads contents of CONFIG3 register.
        fn load_config3;
        /// Writes `value` to CONFIG3 register.
        fn store_config3_val;
        /// Passes the reset value to the closure `f`, then writes the result of
        /// the closure to CONFIG3 register.
        fn store_config3;
    }
    [
        pmask_r(0, "Proximity Mask RIGHT Enable."),
        pmask_l(1, "Proximity Mask LEFT Enable."),
        pmask_d(2, "Proximity Mask DOWN Enable."),
        pmask_u(3, "Proximity Mask UP Enable."),
        sai(4, "Sleep After Interrupt."),
        pcmp(5, "Proximity Gain Compensation Enable."),
    ]
}

apds9960_reg_raw! {
    u8 1 0xA0 rw {
        /// Reads contents of GPENTH register.
        fn load_gpenth;
        /// Writes `value` to GPENTH register.
        fn store_gpenth;
    }
}

apds9960_reg_raw! {
    u8 1 0xA1 rw {
        /// Reads contents of GEXTH register.
        fn load_gexth;
        /// Writes `value` to GEXTH register.
        fn store_gexth;
    }
}

apds9960_reg! {
    /// Gesture configuration one.
    Gconf1 u8 1 0xA2 0x00 rw {
        /// Reads contents of GCONF1 register.
        fn load_gconf1;
        /// Writes `value` to GCONF1 register.
        fn store_gconf1_val;
        /// Passes the reset value to the closure `f`, then writes the result of
        /// the closure to GCONF1 register.
        fn store_gconf1;
    }
    [
        gexpers(0, 2, "Gesture Exit Persistence."),
        gexmsk(2, 4, "Gesture Exit Mask."),
        gfifoth(6, 2, "Gesture FIFO Threshold."),
    ]
}

apds9960_reg! {
    /// Gesture configuration two.
    Gconf2 u8 1 0xA3 0x00 rw {
        /// Reads contents of GCONF2 register.
        fn load_gconf2;
        /// Writes `value` to GCONF2 register.
        fn store_gconf2_val;
        /// Passes the reset value to the closure `f`, then writes the result of
        /// the closure to GCONF2 register.
        fn store_gconf2;
    }
    [
        gwtime(0, 3, "Gesture Wait Time."),
        gldrive(3, 2, "Gesture LED Drive Strength."),
        ggain(5, 2, "Gesture Gain Control."),
    ]
}

apds9960_reg_raw! {
    u8 1 0xA4 rw {
        /// Reads contents of GOFFSET_U register.
        fn load_goffset_u;
        /// Writes `value` to GOFFSET_U register.
        fn store_goffset_u;
    }
}

apds9960_reg_raw! {
    u8 1 0xA5 rw {
        /// Reads contents of GOFFSET_D register.
        fn load_goffset_d;
        /// Writes `value` to GOFFSET_D register.
        fn store_goffset_d;
    }
}

apds9960_reg_raw! {
    u8 1 0xA7 rw {
        /// Reads contents of GOFFSET_L register.
        fn load_goffset_l;
        /// Writes `value` to GOFFSET_L register.
        fn store_goffset_l;
    }
}

apds9960_reg_raw! {
    u8 1 0xA9 rw {
        /// Reads contents of GOFFSET_R register.
        fn load_goffset_r;
        /// Writes `value` to GOFFSET_R register.
        fn store_goffset_r;
    }
}

apds9960_reg! {
    /// Gesture pulse count and length.
    Gpulse u8 1 0xA6 0x40 rw {
        /// Reads contents of GPULSE register.
        fn load_gpulse;
        /// Writes `value` to GPULSE register.
        fn store_gpulse_val;
        /// Passes the reset value to the closure `f`, then writes the result of
        /// the closure to GPULSE register.
        fn store_gpulse;
    }
    [
        gpulse(0, 6, "Number of Gesture Pulses."),
        gplen(6, 2, "Gesture Pulse Length."),
    ]
}

apds9960_reg! {
    /// Gesture configuration three.
    Gconf3 u8 1 0xAA 0x00 rw {
        /// Reads contents of GCONF3 register.
        fn load_gconf3;
        /// Writes `value` to GCONF3 register.
        fn store_gconf3_val;
        /// Passes the reset value to the closure `f`, then writes the result of
        /// the closure to GCONF3 register.
        fn store_gconf3;
    }
    [
        gdims(0, 2, "Gesture Dimension Select."),
    ]
}

apds9960_reg! {
    /// Gesture configuration four.
    Gconf4 u8 1 0xAB 0x00 rw {
        /// Reads contents of GCONF4 register.
        fn load_gconf4;
        /// Writes `value` to GCONF4 register.
        fn store_gconf4_val;
        /// Passes the reset value to the closure `f`, then writes the result of
        /// the closure to GCONF4 register.
        fn store_gconf4;
    }
    [
        gmode(0, "Gesture Mode."),
        gien(1, "Gesture interrupt enable."),
        gfifo_clr(2, "Setting this bit to `1` clears GFIFO, GINT, GVALID, GFIFO_OV and GFIFO_LVL."),
    ]
}

apds9960_reg_raw! {
    u8 1 0xAE r {
        /// Reads contents of GFLVL register.
        fn load_gflvl;
    }
}

apds9960_reg! {
    /// Gesture status.
    Gstatus u8 1 0xAF 0x00 r {
        /// Reads contents of GSTATUS register.
        fn load_gstatus;
    }
    [
        gvalid(0, "Gesture FIFO Data."),
        gfov(1, "Gesture FIFO Overflow."),
    ]
}

apds9960_reg_touch! {
    0xE4;
    /// Accesses IFORCE register - force interrupt.
    fn touch_iforce;
}

apds9960_reg_touch! {
    0xE5;
    /// Accesses PICLEAR register - proximity interrupt clear.
    fn touch_piclear;
}

apds9960_reg_touch! {
    0xE6;
    /// Accesses CICLEAR register - ALS clear channel interrupt clear.
    fn touch_ciclear;
}

apds9960_reg_touch! {
    0xE7;
    /// Accesses AICLEAR register - All non-gesture interrupts clear.
    fn touch_aiclear;
}

apds9960_reg_raw! {
    u8 1 0xFC r {
        /// Reads contents of GFIFO_U register.
        fn load_gfifo_u;
    }
}

apds9960_reg_raw! {
    u8 1 0xFD r {
        /// Reads contents of GFIFO_D register.
        fn load_gfifo_d;
    }
}

apds9960_reg_raw! {
    u8 1 0xFE r {
        /// Reads contents of GFIFO_L register.
        fn load_gfifo_l;
    }
}

apds9960_reg_raw! {
    u8 1 0xFF r {
        /// Reads contents of GFIFO_R register.
        fn load_gfifo_r;
    }
}
