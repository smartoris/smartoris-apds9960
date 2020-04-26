macro_rules! apds9960_reg {
    (
        #[$($attr:meta)*]
        $vis:vis struct $name:ident {
            type: $type:ident,
            size: $size:literal,
            addr: $addr:literal,
            reset: $reset:literal,
            mode: $mode:ident,
            $($mode_tt:tt)*
        }
        $(
            $bitfield:ident {
                $($field:ident($($field_tt:tt)*),)*
            }
        )?
    ) => {
        #[$($attr)*]
        #[derive(Clone, Copy$(, $bitfield)?)]
        $(
            #[bitfield(
                $($field($mode, $($field_tt)*),)*)
            ]
        )?
        $vis struct $name($type);
        impl Default for $name {
            fn default() -> Self {
                $name($reset)
            }
        }
        apds9960_reg!($mode { $($mode_tt)* } $name, $type, $size, $addr);
    };

    (
        r {
            #[$($load_attr:meta)*] fn $load:ident,
        }
        $name:ident, $type:ident, $size:literal, $addr:literal
    ) => {
        impl<A> Apds9960Drv<A> {
            #[$($load_attr)*]
            pub fn $load<'a>(
                &'a mut self,
                i2c: &'a mut impl Apds9960I2CPort<A>,
            ) -> impl Future<Output = $name> + 'a {
                self.load_reg(i2c, $addr, $size).map(|value| $name(value as $type))
            }
        }
    };

    (
        w {
            $(#[$($store_val_attr:meta)*])* fn $store_val:ident,
            $(#[$($store_attr:meta)*])* fn $store:ident,
        }
        $name:ident, $type:ident, $size:literal, $addr:literal
    ) => {
        impl<A> Apds9960Drv<A> {
            $(#[$($store_val_attr)*])*
            pub fn $store_val<'a>(
                &'a mut self,
                i2c: &'a mut impl Apds9960I2CPort<A>,
                value: $name,
            ) -> impl Future<Output = ()> + 'a {
                self.store_reg(i2c, u16::from(value.0), $addr, $size)
            }

            $(#[$($store_attr)*])*
            pub fn $store<'a>(
                &'a mut self,
                i2c: &'a mut impl Apds9960I2CPort<A>,
                f: impl FnOnce(&mut $name) -> &mut $name,
            ) -> impl Future<Output = ()> + 'a {
                let value = f(&mut $name::default()).0;
                self.store_reg(i2c, u16::from(value), $addr, $size)
            }
        }
    };

    (
        rw {
            $(#[$($load_attr:meta)*])* fn $load:ident,
            $(#[$($store_val_attr:meta)*])* fn $store_val:ident,
            $(#[$($store_attr:meta)*])* fn $store:ident,
        }
        $name:ident, $type:ident, $size:literal, $addr:literal
    ) => {
        apds9960_reg! {
            r {
                $(#[$($load_attr)*])* fn $load,
            }
            $name, $type, $size, $addr
        }
        apds9960_reg! {
            w {
                $(#[$($store_val_attr)*])* fn $store_val,
                $(#[$($store_attr)*])* fn $store,
            }
            $name, $type, $size, $addr
        }
    };

    (
        r_raw {
            $(#[$($load_attr:meta)*])* fn $load:ident,
        }
        $name:ident, $type:ident, $size:literal, $addr:literal
    ) => {
        impl<A> Apds9960Drv<A> {
            $(#[$($load_attr)*])*
            pub fn $load<'a>(
                &'a mut self,
                i2c: &'a mut impl Apds9960I2CPort<A>,
            ) -> impl Future<Output = $type> + 'a {
                self.load_reg(i2c, $addr, $size).map(|value| value as $type)
            }
        }
    };

    (
        w_raw {
            $(#[$($store_attr:meta)*])* fn $store:ident,
        }
        $name:ident, $type:ident, $size:literal, $addr:literal
    ) => {
        impl<A> Apds9960Drv<A> {
            $(#[$($store_attr)*])*
            pub fn $store<'a>(
                &'a mut self,
                i2c: &'a mut impl Apds9960I2CPort<A>,
                value: $type,
            ) -> impl Future<Output = ()> + 'a {
                self.store_reg(i2c, u16::from(value), $addr, $size)
            }
        }
    };

    (
        rw_raw {
            $(#[$($load_attr:meta)*])* fn $load:ident,
            $(#[$($store_attr:meta)*])* fn $store:ident,
        }
        $name:ident, $type:ident, $size:literal, $addr:literal
    ) => {
        apds9960_reg! {
            r_raw {
                $(#[$($load_attr)*])* fn $load,
            }
            $name, $type, $size, $addr
        }
        apds9960_reg! {
            w_raw {
                $(#[$($store_attr)*])* fn $store,
            }
            $name, $type, $size, $addr
        }
    };
}

macro_rules! apds9960_reg_touch {
    (
        $addr:literal,
        $(#[$($attr:meta)*])*
        fn $name:ident,
    ) => {
        impl<A> Apds9960Drv<A> {
            $(#[$($attr)*])*
            pub fn $name<'a>(
                &'a mut self,
                i2c: &'a mut impl Apds9960I2CPort<A>,
            ) -> impl Future<Output = ()> + 'a {
                self.touch_reg(i2c, $addr)
            }
        }
    };
}