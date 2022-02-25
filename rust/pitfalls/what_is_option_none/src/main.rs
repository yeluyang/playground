use std::mem::{size_of, transmute};
fn main() {
    // Option<T>的本质是用某个值映射None
    dbg!(size_of::<Option<bool>>());
    dbg!(size_of::<Option<u8>>());
    dbg!(size_of::<Option<u16>>());
    unsafe {
        dbg!(transmute::<Option<bool>, u8>(None)); // 使用Bool的无效值2去映射None
        dbg!(transmute::<Option<bool>, u8>(Some(false)));
        dbg!(transmute::<Option<bool>, u8>(Some(true)));

        dbg!(transmute::<Option<u8>, u16>(None)); // 使用0映射None
        dbg!(transmute::<Option<u8>, u16>(Some(u8::MIN))); // 使用1映射0u8
        dbg!(transmute::<Option<u8>, u16>(Some(u8::MAX)));

        dbg!(transmute::<Option<u8>, i16>(None)); // 使用0映射None
        dbg!(transmute::<Option<u8>, i16>(Some(u8::MIN))); // 使用1映射0u8, 即使转换后的类型是符号类型
        dbg!(transmute::<Option<u8>, i16>(Some(u8::MAX)));
    }
}
