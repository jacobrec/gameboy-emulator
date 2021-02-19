pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn is_add_half_carry(target: u8, value: u8) -> bool {
    ((target & 0x0F).wrapping_add(value & 0x0F)) & 0x10 == 0x10
}

pub fn is_add_carry(target: u8, value: u8) -> bool {
    let (val, overflow) = target.overflowing_add(value);
    return overflow
}

pub fn is_add_half_carry16(target: u16, value: u16) -> bool {
    ((target & 0x00FF).wrapping_add(value & 0x00FF)) & 0x0100 == 0x0100
}

pub fn is_add_carry16(target: u16, value: u16) -> bool {
    let (val, overflow) = target.overflowing_add(value);
    return overflow
}

pub fn is_subtract_half_carry(target: u8, value: u8) -> bool {
    let (val, overflow) = (target & 0x0F).overflowing_sub(value & 0x0F);
    return if overflow { true } else { val < 0 }
}

pub fn is_subtract_carry(target: u8, value: u8) -> bool {
    let (val, overflow) = target.overflowing_sub(value);
    return overflow
}

pub fn is_subtract_half_carry16(target: u16, value: u16) -> bool {
    let (val, overflow) = (target & 0x00FF).overflowing_sub(value & 0x00FF);
    return if overflow { true } else { val < 0 }
}

pub fn is_subtract_carry16(target: u16, value: u16) -> bool {
    let (val, overflow) = target.overflowing_sub(value);
    return overflow
}