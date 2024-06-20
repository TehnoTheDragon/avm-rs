use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign, Not};

pub trait RegisterTrait<T> {
    fn get(&self) -> T;
    fn set(&mut self, value: T);
}

pub type RegisterType<T> = dyn RegisterTrait<T>;

#[macro_export]
macro_rules! define_register {
    ( $name:ident, $type:ty ) => {
        pub struct $name(pub $type);
        impl RegisterTrait<$type> for $name {
            fn get(&self) -> $type {
                self.0
            }
            fn set(&mut self, value: $type) {
                self.0 = value;
            }
        }
        impl Add for $name {
            type Output = $name;
            fn add(self, other: Self) -> Self::Output {
                Self(self.0 + other.0)
            }
        }
        impl AddAssign for $name {
            fn add_assign(&mut self, other: Self) {
                *self = Self(self.0 + other.0)
            }
        }
        impl Sub for $name {
            type Output = $name;
            fn sub(self, other: Self) -> Self::Output {
                Self(self.0 - other.0)
            }
        }
        impl SubAssign for $name {
            fn sub_assign(&mut self, other: Self) {
                *self = Self(self.0 - other.0)
            }
        }
        impl Mul for $name {
            type Output = $name;
            fn mul(self, other: Self) -> Self::Output {
                Self(self.0 * other.0)
            }
        }
        impl MulAssign for $name {
            fn mul_assign(&mut self, other: Self) {
                *self = Self(self.0 * other.0)
            }
        }
        impl Div for $name {
            type Output = $name;
            fn div(self, other: Self) -> Self::Output {
                Self(self.0 / other.0)
            }
        }
        impl DivAssign for $name {
            fn div_assign(&mut self, other: Self) {
                *self = Self(self.0 / other.0)
            }
        }
    };
}

#[macro_export]
macro_rules! register_bitwise_extension {
    ( $strukt:ident ) => {
        impl BitAnd for $strukt {
            type Output = $strukt;
            fn bitand(self, other: Self) -> Self::Output {
                Self(self.0 & other.0)
            }
        }
        impl BitAndAssign for $strukt {
            fn bitand_assign(&mut self, other: Self) {
                *self = Self(self.0 & other.0)
            }
        }

        impl BitOr for $strukt {
            type Output = $strukt;
            fn bitor(self, other: Self) -> Self::Output {
                Self(self.0 | other.0)
            }
        }
        impl BitOrAssign for $strukt {
            fn bitor_assign(&mut self, other: Self) {
                *self = Self(self.0 | other.0)
            }
        }

        impl BitXor for $strukt {
            type Output = $strukt;
            fn bitxor(self, other: Self) -> Self::Output {
                Self(self.0 ^ other.0)
            }
        }
        impl BitXorAssign for $strukt {
            fn bitxor_assign(&mut self, other: Self) {
                *self = Self(self.0 ^ other.0)
            }
        }

        impl Not for $strukt {
            type Output = $strukt;
            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }

        impl Shl for $strukt {
            type Output = $strukt;
            fn shl(self, other: Self) -> Self::Output {
                Self(self.0 << other.0)
            }
        }
        impl ShlAssign for $strukt {
            fn shl_assign(&mut self, other: Self) {
                *self = Self(self.0 << other.0)
            }
        }

        impl Shr for $strukt {
            type Output = $strukt;
            fn shr(self, other: Self) -> Self::Output {
                Self(self.0 >> other.0)
            }
        }
        impl ShrAssign for $strukt {
            fn shr_assign(&mut self, other: Self) {
                *self = Self(self.0 >> other.0)
            }
        }
    };
}

define_register!(RegisterU8, u8);
register_bitwise_extension!(RegisterU8);
define_register!(RegisterU16, u16);
register_bitwise_extension!(RegisterU16);
define_register!(RegisterU32, u32);
register_bitwise_extension!(RegisterU32);
define_register!(RegisterU64, u64);
register_bitwise_extension!(RegisterU64);
define_register!(RegisterU128, u128);
register_bitwise_extension!(RegisterU128);

define_register!(RegisterI8, i8);
register_bitwise_extension!(RegisterI8);
define_register!(RegisterI16, i16);
register_bitwise_extension!(RegisterI16);
define_register!(RegisterI32, i32);
register_bitwise_extension!(RegisterI32);
define_register!(RegisterI64, i64);
register_bitwise_extension!(RegisterI64);
define_register!(RegisterI128, i128);
register_bitwise_extension!(RegisterI128);

define_register!(RegisterF32, f32);
define_register!(RegisterF64, f64);