use crate::key::gen_key;
use crate::round::{add_round_key, round_f_enc, round_f_dec};

#[derive(Debug)]
pub struct AES {
    key: Vec<u8>,
}

impl AES {
    pub fn new(key: &Vec<u8>) -> AES {
        let k = gen_key(&key);
        AES { key: k }
    }

    pub fn encrypt(&self, plaintext: &Vec<u8>) -> Vec<u8> {
        let mut m = plaintext.clone();
        let mut padding = 16;
        if m.len() % 16 != 0 {
            padding += m.len() % 16;
        }
        m.append(&mut vec![0u8; padding - 1]);
        m.push(padding as u8);

        let mut states = Vec::new();
        for i in 0..(m.len() / 16) {
            states.push(m[0+16*i..16+16*i].to_vec());
        }

        let key = self.key[0..16].to_vec();
        for i in 0..states.len() {
            states[i] = add_round_key(&states[i], &key);
        }

        for round in 1..11 {
            let key = self.key[0+16*round..16+16*round].to_vec();
            let last = round == 10;
            for i in 0..states.len() {
                states[i] = round_f_enc(&states[i], &key, last);
            }
        }

        let mut result = Vec::new();
        for state in states {
            for byte in state {
                result.push(byte);
            }
        }

        result
    }

    pub fn decrypt(&self, ciphertext: &Vec<u8>) -> Vec<u8> {
        let m = ciphertext.clone();

        let mut states = Vec::new();
        for i in 0..(m.len() / 16) {
            states.push(m[0+16*i..16+16*i].to_vec());
        }

        let key = self.key[160..176].to_vec();
        for i in 0..states.len() {
            states[i] = add_round_key(&states[i], &key);
        }

        for round in (1..11).rev() {
            let key = self.key[0+16*(round-1)..16+16*(round-1)].to_vec();
            let last = round == 1;
            for i in 0..states.len() {
                states[i] = round_f_dec(&states[i], &key, last);
            }
        }

        let mut result = Vec::new();
        for state in states {
            for byte in state {
                result.push(byte);
            }
        }

        let padding = result[result.len() - 1];
        result.truncate(result.len() - padding as usize);

        result
    }
}
