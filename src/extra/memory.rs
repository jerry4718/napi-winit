use std::alloc::Layout;

struct Trunk<const N: usize>;

impl<const N: usize> Trunk<N> {
    const LAYOUT: Layout = Layout::new::<[u8; N]>();
}

pub mod namespace {
    use std::alloc::Layout;
    use napi::bindgen_prelude::*;

    struct DynamicBuffer {
        pub(crate) count: f64,
        pub(crate) per: f64,
        pub(crate) ratio: Option<f64>,
    }

    impl DynamicBuffer {
        pub fn new(count: f64, per: f64, ratio: Option<f64>) -> Result<Self> {
            if count != count.floor() || count <= 0. {
                return napi_reason!("The count is not an int or lower than 0");
            }
            if per != per.floor() || per <= 0. {
                return napi_reason!("The per is not an int or lower than 0");
            }
            let ratio = ratio.unwrap_or(1.5);
            
            if ratio <= 1. {
                return napi_reason!("The ratio must be greater than 1");
            }

            let init_size = count * per * ratio;

            if init_size % per != 0. {
                return napi_reason!("The init_size is not a multiple of per");
            }

            let count = count as usize;
            let per = per as usize;
            
            let size = per * ratio;
            let layout = Layout::from_size_align(1, per).unwrap();
            todo!();
        }
    }
}