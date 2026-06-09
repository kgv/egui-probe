/// Probe default
pub trait ProbeDefault {
    fn probe_default() -> Self;
}

// impl<T: Default> ProbeDefault for T {
//     fn probe_default() -> Self {
//         Default::default()
//     }
// }

impl<T: ProbeDefault> ProbeDefault for Option<T> {
    fn probe_default() -> Self {
        Some(ProbeDefault::probe_default())
    }
}

macro_rules! impl_probe_default {
    ($type:ident) => {
        impl ProbeDefault for $type {
            #[inline(always)]
            fn probe_default() -> Self {
                Self::default()
            }
        }
    };
    ($($type:ident),*) => {
        $(impl_probe_default!($type);)*
    };
}

impl_probe_default!(
    i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool
);

// impl<T: Default> ProbeDefault for T {
//     fn probe_default() -> Self {
//         Default::default()
//     }
// }
