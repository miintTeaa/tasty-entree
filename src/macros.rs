macro_rules! new_array {
    ( $size:expr, $typ:ty, $default:expr ) => {
        unsafe {
            let mut ret: [$typ; $size] = std::mem::MaybeUninit::uninit().assume_init();

            if $size != 0 {
                let ret_first: *mut $typ = ret.get_mut(0).unwrap();

                for i in 0..$size {
                    std::ptr::write(ret_first.add(i), $default)
                }
            }

            ret
        }
    };
}

pub(crate) use new_array;