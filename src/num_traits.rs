macro_rules! num_traits {
($name:ident, [$($bit:tt),*]) => {
    /// Allows writing `pixel + pixel`
    impl<T: Add> Add for $name<T> {
        type Output = $name<<T as Add>::Output>;

        #[inline(always)]
        fn add(self, other: $name<T>) -> Self::Output {
            $name {
                $(
                    $bit: self.$bit + other.$bit,
                )+
            }
        }
    }
    /// Allows writing `pixel + 1`
    impl<T> Add<T> for $name<T> where
        T: Copy + Add<Output=T>
    {
        type Output = $name<T>;

        #[inline(always)]
        fn add(self, r: T) -> Self::Output {
            $name {
                $(
                    $bit: self.$bit + r,
                )+
            }
        }
    }

    /// Allows writing `pixel += pixel`
    impl<T> AddAssign for $name<T> where
        T: Add<Output = T> + Copy
    {
        #[inline(always)]
        fn add_assign(&mut self, other: $name<T>) {
            *self = Self {
                $(
                    $bit: self.$bit + other.$bit,
                )+
            };
        }
    }
    /// Allows writing `pixel += 1`
    impl<T> AddAssign<T> for $name<T> where
        T: Copy + Add<Output=T>
    {
        #[inline(always)]
        fn add_assign(&mut self, r: T) {
            *self = $name {
                $(
                    $bit: self.$bit + r,
                )+
            };
        }
    }

    /// Allows writing `pixel - pixel`
    impl<T: Sub> Sub for $name<T> {
        type Output = $name<<T as Sub>::Output>;

        #[inline(always)]
        fn sub(self, other: $name<T>) -> Self::Output {
            $name {
                $(
                    $bit: self.$bit - other.$bit,
                )+
            }
        }
    }
    /// Allows writing `pixel - 1`
    impl<T> Sub<T> for $name<T> where
        T: Copy + Sub<Output=T>
    {
        type Output = $name<<T as Sub>::Output>;

        #[inline(always)]
        fn sub(self, r: T) -> Self::Output {
            $name {
                $(
                    $bit: self.$bit - r,
                )+
            }
        }
    }

    /// Allows writing `pixel -= pixel`
    impl<T> SubAssign for $name<T> where
        T: Sub<Output = T> + Copy
    {
        #[inline(always)]
        fn sub_assign(&mut self, other: $name<T>) {
            *self = Self {
                $(
                    $bit: self.$bit - other.$bit,
                )+
            };
        }
    }
    /// Allows writing `pixel -= 1`
    impl<T> SubAssign<T> for $name<T> where
        T: Copy + Sub<Output=T>
    {
        #[inline(always)]
        fn sub_assign(&mut self, r: T) {
            *self = $name {
                $(
                    $bit: self.$bit - r,
                )+
            };
        }
    }

    /// Allows writing `pixel * pixel`
    impl<T: Mul> Mul for $name<T> {
        type Output = $name<<T as Mul>::Output>;

        #[inline(always)]
        fn mul(self, other: $name<T>) -> Self::Output {
            $name {
                $(
                    $bit: self.$bit * other.$bit,
                )+
            }
        }
    }
    /// Allows writing `pixel * 2`
    impl<T> Mul<T> for $name<T> where
        T: Copy + Mul<Output=T>
    {
        type Output = $name<T>;

        #[inline(always)]
        fn mul(self, r: T) -> Self::Output {
            $name {
                $(
                    $bit: self.$bit * r,
                )+
            }
        }
    }

    /// Allows writing `pixel *= pixel`
    impl<T> MulAssign for $name<T> where
        T: Mul<Output = T> + Copy
    {
        #[inline(always)]
        fn mul_assign(&mut self, other: $name<T>) {
            *self = Self {
                $(
                    $bit: self.$bit * other.$bit,
                )+
            };
        }
    }
    /// Allows writing `pixel *= 2`
    impl<T> MulAssign<T> for $name<T> where
        T: Copy + Mul<Output=T>
    {
        #[inline(always)]
        fn mul_assign(&mut self, r: T) {
            *self = $name {
                $(
                    $bit: self.$bit * r,
                )+
            };
        }
    }

    /// Allows writing `pixel / pixel`
    impl<T: Div> Div for $name<T> {
        type Output = $name<<T as Div>::Output>;

        #[inline(always)]
        fn div(self, other: $name<T>) -> Self::Output {
            $name {
                $(
                    $bit: self.$bit / other.$bit,
                )+
            }
        }
    }
    /// Allows writing `pixel / 2`
    impl<T> Div<T> for $name<T> where
        T: Copy + Div<Output=T>
    {
        type Output = $name<T>;

        #[inline(always)]
        fn div(self, r: T) -> Self::Output {
            $name {
                $(
                    $bit: self.$bit / r,
                )+
            }
        }
    }

    /// Allows writing `pixel /= pixel`
    impl<T> DivAssign for $name<T> where
        T: Div<Output = T> + Copy
    {
        #[inline(always)]
        fn div_assign(&mut self, other: $name<T>) {
            *self = Self {
                $(
                    $bit: self.$bit / other.$bit,
                )+
            };
        }
    }
    /// Allows writing `pixel /= 2`
    impl<T> DivAssign<T> for $name<T> where
        T: Copy + Div<Output=T>
    {
        #[inline(always)]
        fn div_assign(&mut self, r: T) {
            *self = $name {
                $(
                    $bit: self.$bit / r,
                )+
            };
        }
    }
};
}

num_traits!(Rgb, [r, g, b]);
