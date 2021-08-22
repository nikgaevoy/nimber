macro_rules! nimber_nimber_forward_binop {
    (impl $imp:ident, $method:ident as $bound:ident, $func:ident) => {
        impl<F: $bound<S>, S> $imp<Nimber<S>> for Nimber<F> {
            type Output = Nimber<<F as $bound<S>>::Output>;

            #[inline]
            fn $method(self, rhs: Nimber<S>) -> Self::Output {
                Nimber {
                    x: $bound::$func(self.x, rhs.x),
                }
            }
        }

        impl<'a, F: 'a, S> $imp<Nimber<S>> for &'a Nimber<F>
        where
            &'a F: $bound<S>,
        {
            type Output = Nimber<<&'a F as $bound<S>>::Output>;

            #[inline]
            fn $method(self, rhs: Nimber<S>) -> Self::Output {
                Nimber {
                    x: $bound::$func(&self.x, rhs.x),
                }
            }
        }

        impl<'b, F: $bound<&'b S>, S> $imp<&'b Nimber<S>> for Nimber<F> {
            type Output = Nimber<<F as $bound<&'b S>>::Output>;

            #[inline]
            fn $method(self, rhs: &'b Nimber<S>) -> Self::Output {
                Nimber {
                    x: $bound::$func(self.x, &rhs.x),
                }
            }
        }

        impl<'a, 'b, F, S> $imp<&'b Nimber<S>> for &'a Nimber<F>
        where
            &'a F: $bound<&'b S>,
        {
            type Output = Nimber<<&'a F as $bound<&'b S>>::Output>;

            #[inline]
            fn $method(self, rhs: &'b Nimber<S>) -> Self::Output {
                Nimber {
                    x: $bound::$func(&self.x, &rhs.x),
                }
            }
        }
    };
    (impl $imp:ident, $method:ident) => {
        nimber_nimber_forward_binop!(impl $imp, $method as $imp, $method);
    };
}

macro_rules! nimber_nimber_forward_binop_assign {
    (impl $imp:ident, $method:ident as $bound:ident, $func:ident) => {
        impl<F: $bound<S>, S> $imp<Nimber<S>> for Nimber<F> {
            #[inline]
            fn $method(&mut self, rhs: Nimber<S>) {
                $bound::$func(&mut self.x, rhs.x)
            }
        }

        impl<'b, F: $bound<&'b S>, S> $imp<&'b Nimber<S>> for Nimber<F> {
            #[inline]
            fn $method(&mut self, rhs: &'b Nimber<S>) {
                $bound::$func(&mut self.x, &rhs.x)
            }
        }
    };
    (impl $imp:ident, $method:ident) => {
        nimber_nimber_forward_binop_assign!(impl $imp, $method as $imp, $method);
    };
}

macro_rules! nimber_val_forward_binop {
    (impl $imp:ident, $method:ident as $bound:ident, $func:ident) => {
        impl<F: $bound<S>, S> $imp<S> for Nimber<F> {
            type Output = Nimber<<F as $bound<S>>::Output>;

            #[inline]
            fn $method(self, rhs: S) -> Self::Output {
                Nimber {
                    x: $bound::$func(self.x, rhs),
                }
            }
        }

        impl<'a, F: 'a, S> $imp<S> for &'a Nimber<F>
        where
            &'a F: $bound<S>,
        {
            type Output = Nimber<<&'a F as $bound<S>>::Output>;

            #[inline]
            fn $method(self, rhs: S) -> Self::Output {
                Nimber {
                    x: $bound::$func(&self.x, rhs),
                }
            }
        }
    };
    (impl $imp:ident, $method:ident) => {
        nimber_val_forward_binop!(impl $imp, $method as $imp, $method);
    };
}

macro_rules! nimber_val_forward_binop_assign {
    (impl $imp:ident, $method:ident as $bound:ident, $func:ident) => {
        impl<F: $bound<S>, S> $imp<S> for Nimber<F> {
            #[inline]
            fn $method(&mut self, rhs: S) {
                $bound::$func(&mut self.x, rhs)
            }
        }
    };
    (impl $imp:ident, $method:ident) => {
        nimber_val_forward_binop_assign!(impl $imp, $method as $imp, $method);
    };
}

macro_rules! nimber_ref_binop {
    (impl $imp:ident, $method:ident) => {
        impl<T> $imp for Nimber<T>
        where
            for<'x, 'y> &'x Nimber<T>: $imp<&'y Nimber<T>, Output = Nimber<T>>,
        {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: Self) -> Self::Output {
                $imp::$method(&self, &rhs)
            }
        }

        impl<'a, T> $imp<Nimber<T>> for &'a Nimber<T>
        where
            for<'x, 'y> &'x Nimber<T>: $imp<&'y Nimber<T>, Output = Nimber<T>>,
        {
            type Output = Nimber<T>;

            #[inline]
            fn $method(self, rhs: Nimber<T>) -> Self::Output {
                $imp::$method(self, &rhs)
            }
        }

        impl<'b, T> $imp<&'b Nimber<T>> for Nimber<T>
        where
            for<'x, 'y> &'x Nimber<T>: $imp<&'y Nimber<T>, Output = Nimber<T>>,
        {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: &'b Self) -> Self::Output {
                $imp::$method(&self, rhs)
            }
        }
    };
}

macro_rules! nimber_ref_binop_assign {
    (impl $imp:ident, $method:ident use $bound:ident, $func:ident) => {
        impl<T> $imp<Nimber<T>> for Nimber<T>
        where
            for<'x, 'y> &'x Nimber<T>: $bound<&'y Nimber<T>, Output = Nimber<T>>,
        {
            #[inline]
            fn $method(&mut self, rhs: Nimber<T>) {
                *self = $bound::$func(&*self, &rhs);
            }
        }

        impl<'b, T> $imp<&'b Nimber<T>> for Nimber<T>
        where
            for<'x, 'y> &'x Nimber<T>: $bound<&'y Nimber<T>, Output = Nimber<T>>,
        {
            #[inline]
            fn $method(&mut self, rhs: &'b Nimber<T>) {
                *self = $bound::$func(&*self, rhs);
            }
        }
    };
}
