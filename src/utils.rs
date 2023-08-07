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
