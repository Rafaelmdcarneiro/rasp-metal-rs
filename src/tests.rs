#![cfg(test)]

use gpio;

#[test]
fn test_get_gpsel() {
    assert_eq!(gpio::get_gpsel( 0), Some(gpio::GPSEL0));
    assert_eq!(gpio::get_gpsel( 9), Some(gpio::GPSEL0));
    assert_eq!(gpio::get_gpsel(10), Some(gpio::GPSEL1));
    assert_eq!(gpio::get_gpsel(19), Some(gpio::GPSEL1));
    assert_eq!(gpio::get_gpsel(20), Some(gpio::GPSEL2));
    assert_eq!(gpio::get_gpsel(29), Some(gpio::GPSEL2));
    assert_eq!(gpio::get_gpsel(30), Some(gpio::GPSEL3));
    assert_eq!(gpio::get_gpsel(39), Some(gpio::GPSEL3));
    assert_eq!(gpio::get_gpsel(40), Some(gpio::GPSEL4));
    assert_eq!(gpio::get_gpsel(49), Some(gpio::GPSEL4));
    assert_eq!(gpio::get_gpsel(50), Some(gpio::GPSEL5));
    assert_eq!(gpio::get_gpsel(53), Some(gpio::GPSEL5));
    assert_eq!(gpio::get_gpsel(54), None);
}

#[test]
fn test_shift_alt(){
    assert_eq!(gpio::shift_alt( 0, gpio::Modes::Alt2), (gpio::Modes::Alt2 as u32) <<  0);
    assert_eq!(gpio::shift_alt(11, gpio::Modes::Alt2), (gpio::Modes::Alt2 as u32) <<  3);
    assert_eq!(gpio::shift_alt(22, gpio::Modes::Alt2), (gpio::Modes::Alt2 as u32) <<  6);
    assert_eq!(gpio::shift_alt(33, gpio::Modes::Alt2), (gpio::Modes::Alt2 as u32) <<  9);
    assert_eq!(gpio::shift_alt(44, gpio::Modes::Alt2), (gpio::Modes::Alt2 as u32) << 12);
    assert_eq!(gpio::shift_alt( 5, gpio::Modes::Alt2), (gpio::Modes::Alt2 as u32) << 15);
    assert_eq!(gpio::shift_alt(16, gpio::Modes::Alt2), (gpio::Modes::Alt2 as u32) << 18);
    assert_eq!(gpio::shift_alt(27, gpio::Modes::Alt2), (gpio::Modes::Alt2 as u32) << 21);
    assert_eq!(gpio::shift_alt(38, gpio::Modes::Alt2), (gpio::Modes::Alt2 as u32) << 24);
    assert_eq!(gpio::shift_alt(49, gpio::Modes::Alt2), (gpio::Modes::Alt2 as u32) << 27);
    assert_eq!(gpio::shift_alt(52, gpio::Modes::Alt2), (gpio::Modes::Alt2 as u32) <<  6);
}
