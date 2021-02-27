// Thanks to juliand665 for allowing me to use his macro code for vectors.
// https://github.com/juliand665/raytracer-rust/blob/master/src/vectors/vector.rs

use std::fmt;
use std::ops;

pub type Component = f32;

pub trait Vector:
    'static
    + Copy
    + Clone
    + PartialEq
    + fmt::Display
    + ops::Neg<Output = Self>
    + ops::Add<Output = Self>
    + ops::Add<Component, Output = Self>
    + ops::AddAssign
    + ops::AddAssign<Component>
    + ops::Sub<Output = Self>
    + ops::Sub<Component, Output = Self>
    + ops::SubAssign
    + ops::SubAssign<Component>
    + ops::Mul<Component, Output = Self>
    + ops::MulAssign<Component>
    + ops::Div<Component, Output = Self>
    + ops::DivAssign<Component>
{
    fn zero() -> Self;

    fn length_squared(&self) -> f32;
    fn length(&self) -> f32;

    fn normalized(&self) -> Self;

    fn dot_product(&self, rhs: &Self) -> Component;
}

macro_rules! vec_type {
    ($type:ident($($component:ident)*)) => {
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub struct $type {
            $(pub $component: Component),*
        }

        impl $type {
            pub fn new($($component: Component,)*) -> Self {
                Self { $($component),* }
            }
        }

        impl fmt::Display for $type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({})", [$(self.$component.to_string()),*].join(", "))
            }
        }

        impl Vector for $type {
            fn zero() -> Self {
                Self { $($component: 0.0),* }
            }

            fn length_squared(&self) -> f32 {
                self.dot_product(&self)
            }

            fn length(&self) -> f32 {
                self.length_squared().sqrt()
            }

            fn normalized(&self) -> Self {
                let length = self.length();
                Self { $($component: self.$component / length),* }
            }

            fn dot_product(&self, rhs: &Self) -> Component {
                [$(self.$component * rhs.$component),*].iter().fold(0.0, |a, &b| a + b)
            }
        }

        impl ops::Neg for $type {
            type Output = Self;

            fn neg(self) -> Self {
                Self { $($component: -self.$component),* }
            }
        }

        macro_rules! bin_op {
            (cwise $rhs:ty, $op_name:ident, $fn:ident, $op:tt) => {
                impl ops::$op_name<$rhs> for $type {
                    type Output = Self;

                    fn $fn(self, rhs: $rhs) -> Self {
                        Self { $($component: self.$component $op rhs.$component),* }
                    }
                }
            };

            (cwise, $op_name:ident, $fn:ident, $op:tt) => {
                bin_op!(cwise Self, $op_name, $fn, $op);
            };

            (linear, $op_name:ident, $fn:ident, $op:tt) => {
                impl ops::$op_name<Component> for $type {
                    type Output = Self;

                    fn $fn(self, rhs: Component) -> Self {
                        Self { $($component: self.$component $op rhs),* }
                    }
                }
            };
        }

        bin_op!(cwise, Add, add, +);
        bin_op!(linear, Add, add, +);
        bin_op!(cwise, Sub, sub, -);
        bin_op!(linear, Sub, sub, -);
        bin_op!(linear, Mul, mul, *);
        bin_op!(linear, Div, div, /);

        macro_rules! bin_op_assign {
            (cwise $rhs:ty, $op_name:ident, $fn:ident, $op:tt) => {
                impl ops::$op_name<$rhs> for $type {
                    fn $fn(&mut self, rhs: $rhs) {
                        $(self.$component $op rhs.$component;)*
                    }
                }
            };

            (cwise, $op_name:ident, $fn:ident, $op:tt) => {
                bin_op_assign!(cwise Self, $op_name, $fn, $op);
            };

            (linear, $op_name:ident, $fn:ident, $op:tt) => {
                impl ops::$op_name<Component> for $type {
                    fn $fn(&mut self, rhs: Component) {
                        $(self.$component $op rhs;)*
                    }
                }
            };
        }

        bin_op_assign!(cwise, AddAssign, add_assign, +=);
        bin_op_assign!(linear, AddAssign, add_assign, +=);
        bin_op_assign!(cwise, SubAssign, sub_assign, -=);
        bin_op_assign!(linear, SubAssign, sub_assign, +=);
        bin_op_assign!(linear, MulAssign, mul_assign, *=);
        bin_op_assign!(linear, DivAssign, div_assign, /=);
    };
}

vec_type!(Vector2(x y));
vec_type!(Vector3(x y z));
vec_type!(Vector4(x y z w));

impl Vector3 {
    pub fn positive_x() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn negative_x() -> Self {
        Self::new(-1.0, 0.0, 0.0)
    }

    pub fn positive_y() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn negative_y() -> Self {
        Self::new(0.0, -1.0, 0.0)
    }

    pub fn positive_z() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn negative_z() -> Self {
        Self::new(0.0, 0.0, -1.0)
    }

    pub fn cross_product(&self, rhs: &Self) -> Self {
        Self {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }
}

impl From<&Vector2> for (Component, Component) {
    fn from(vec: &Vector2) -> Self {
        (vec.x, vec.y)
    }
}

impl From<&Vector3> for (Component, Component, Component) {
    fn from(vec: &Vector3) -> Self {
        (vec.x, vec.y, vec.z)
    }
}

impl From<&Vector4> for (Component, Component, Component, Component) {
    fn from(vec: &Vector4) -> Self {
        (vec.x, vec.y, vec.z, vec.w)
    }
}
