macro_rules! none_array {
    ( $size:expr, $typ:ty ) => {
        unsafe {
            let mut ret_maybe = std::mem::MaybeUninit::<[_; $size]>::uninit();

            let ret = ret_maybe.as_mut_ptr();

            if $size != 0 {
                let ret_first: *mut Option<$typ> = (*ret).get_mut(0).unwrap();

                for i in 0..$size {
                    std::ptr::write(ret_first.add(i), None)
                }
            }

            ret_maybe.assume_init()
        }
    };
}

pub(super) use none_array;