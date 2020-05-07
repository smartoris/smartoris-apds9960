macro_rules! apds9960_reg {
    (
        $(#[$($attr:meta)*])*
        $name:ident $type:ident $size:literal $addr:literal $reset:literal
        $mode:ident { $($mode_tt:tt)* }
        [ $($field:ident($($field_tt:tt)*),)* ]
    ) => {
        $(#[$($attr)*])*
        #[derive(Clone, Copy, Bitfield)]
        #[bitfield($($field($mode, $($field_tt)*),)*)]
        pub struct $name($type);
        impl Default for $name {
            fn default() -> Self {
                $name($reset)
            }
        }
        apds9960_reg!($name $type $size $addr $mode { $($mode_tt)* });
    };

    (
        $name:ident $type:ident $size:literal $addr:literal r {
            #[$($load_attr:meta)*] fn $load:ident;
        }
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
        $name:ident $type:ident $size:literal $addr:literal w {
            $(#[$($store_val_attr:meta)*])* fn $store_val:ident;
            $(#[$($store_attr:meta)*])* fn $store:ident;
        }
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
        $name:ident $type:ident $size:literal $addr:literal rw {
            $(#[$($load_attr:meta)*])* fn $load:ident;
            $(#[$($store_val_attr:meta)*])* fn $store_val:ident;
            $(#[$($store_attr:meta)*])* fn $store:ident;
        }
    ) => {
        apds9960_reg! {
            $name $type $size $addr r {
                $(#[$($load_attr)*])* fn $load;
            }
        }
        apds9960_reg! {
            $name $type $size $addr w {
                $(#[$($store_val_attr)*])* fn $store_val;
                $(#[$($store_attr)*])* fn $store;
            }
        }
    };
}

macro_rules! apds9960_reg_raw {
    (
        $type:ident $size:literal $addr:literal r {
            $(#[$($load_attr:meta)*])* fn $load:ident;
        }
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
        $type:ident $size:literal $addr:literal w {
            $(#[$($store_attr:meta)*])* fn $store:ident;
        }
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
        $type:ident $size:literal $addr:literal rw {
            $(#[$($load_attr:meta)*])* fn $load:ident;
            $(#[$($store_attr:meta)*])* fn $store:ident;
        }
    ) => {
        apds9960_reg_raw! {
            $type $size $addr r {
                $(#[$($load_attr)*])* fn $load;
            }
        }
        apds9960_reg_raw! {
            $type $size $addr w {
                $(#[$($store_attr)*])* fn $store;
            }
        }
    };
}

macro_rules! apds9960_reg_touch {
    (
        $addr:literal;
        $(#[$($attr:meta)*])*
        fn $name:ident;
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
