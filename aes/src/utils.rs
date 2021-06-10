pub fn xtime(b: u8) -> u8 {
    let mut r = b;
    r = r.rotate_left(1);
    r &= 0b11111110;
    if b / 0b10000000 == 1 {
        r ^= 0x1B;
    }
    r
}

pub fn mul(a: u8, d: u8) -> u8 {
    match d {
        0x01 => a,
        0x02 => xtime(a),
        0x03 => xtime(a) ^ a,
        0x0B => a ^ xtime(a) ^ xtime(xtime(xtime(a))),
        0x09 => xtime(xtime(xtime(a))) ^ a,
        0x0D => a ^ xtime(xtime(a)) ^ xtime(xtime(xtime(a))),
        0x0E => xtime(a) ^ xtime(xtime(a)) ^ xtime(xtime(xtime(a))),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xtime() {
        assert_eq!(xtime(0x57), 0xAE);
        assert_eq!(xtime(0xAE), 0x47);
    }
}
