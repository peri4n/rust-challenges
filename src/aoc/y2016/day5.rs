use md5::compute;
use nom::AsChar;

static HEX_LUT: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
];

const INPUT: &str = "ugkcyxxp";

pub fn day5_fst() -> String {
    let mut result = String::with_capacity(8);
    let mut i = 0;

    loop {
        let digest = compute([INPUT, &i.to_string()].concat());

        if starts_with_five_zeros(&digest.0) {
            let sixth = HEX_LUT[(digest.0[2] & 0x0F) as usize] as char;
            result.push(sixth);
        }

        if result.len() == 8 {
            return result;
        }

        i += 1;
    }
}

pub fn starts_with_five_zeros(bytes: &[u8]) -> bool {
    bytes[0] == 0 && bytes[1] == 0 && bytes[2] >> 4 == 0
}

pub fn day5_snd() -> String {
    let mut result = String::from("________");
    let mut i = 0;
    let mut found = 0;

    loop {
        let digest = compute([INPUT, &i.to_string()].concat());

        if starts_with_five_zeros(&digest.0) {
            let seventh = HEX_LUT[(digest.0[3] >> 4) as usize] as char;
            let sixth = HEX_LUT[(digest.0[2] & 0x0F) as usize] as char;

            // if valid position
            if sixth.is_dec_digit() && sixth != '9' && sixth != '8' {
                let bytes = unsafe { result.as_bytes_mut() };

                // if position is not already set
                if bytes[sixth as usize - '0' as usize] as char == '_' {
                    bytes[sixth as usize - '0' as usize] = seventh as u8;
                    found += 1;
                }
            }
        }

        if found == 8 {
            return result;
        }

        i += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day5_fst(), String::from("d4cd2ee1"));
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day5_snd(), String::from("f2c730e5"));
    }
}
