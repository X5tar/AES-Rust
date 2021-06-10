use crate::round::sub_byte;

const RC: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];

pub fn gen_key(key: &Vec<u8>) -> Vec<u8> {
    let mut result = key.clone();
    for n in 1..11 {
        let mut temp = result[0+4*(4*n-1)..4+4*(4*n-1)].to_vec();
        temp.rotate_left(1);
        temp = sub_byte(&temp, false);
        temp[0] ^= RC[n-1];
        for i in 0..4 {
            temp[i] ^= result[i+4*(4*(n-1))];
        }
        result.append(&mut temp);
        for j in 1..4 {
            for i in 0..4 {
                temp.push(result[i+4*(4*n+j-1)] ^ result[i+4*(4*(n-1)+j)]);
            }
            result.append(&mut temp);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_key() {
        let key: Vec<u8> = vec![
            0x3C, 0xA1, 0x0B, 0x21,
            0x57, 0xF0, 0x19, 0x16,
            0x90, 0x2E, 0x13, 0x80,
            0xAC, 0xC1, 0x07, 0xBD,
        ];
        assert_eq!(gen_key(&key)[16..32], [
            0x45, 0x64, 0x71, 0xB0,
            0x12, 0x94, 0x68, 0xA6,
            0x82, 0xBA, 0x7B, 0x26,
            0x2E, 0x7B, 0x7C, 0x9B,
        ]);
    }
}
