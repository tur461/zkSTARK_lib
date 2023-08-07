use sha2::{Digest, Sha256};

pub fn hash256_vec(bytes: &[u8]) -> Vec<u8> {
    let mut h = Sha256::new();
    h.update(bytes);
    h.finalize().to_vec()
}

pub fn hash256_str(bytes: &[u8]) -> String {
    u8_to_hexstr(&hash256_vec(bytes)[..])
}

pub fn u8_to_hexstr(v: &[u8]) -> String {
    v.iter()
        .map(|x| {
            let s = format!("{:x}", x);
            if x < &16 {
                return format!("0{}", s);
            }
            s
        })
        .collect::<Vec<String>>()
        .join("")
}
