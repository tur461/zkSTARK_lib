fn main() {
    let mut v1: Vec<usize> = vec![0];
    let v2: Vec<usize> = vec![4, 3, 2, 1];

    while let Some(&l) = v1.last() {
        println!("last: {}", l);
        if l == 0 {
            v1.pop();
        } else {
            break;
        }
    }
    println!("{:?}", v1.last());

    // let v3: Vec<(usize, usize)> = v1.into_iter().zip_longest(v2.into_iter()).collect();
    //
    // println!("zipped: {:?}", v3);
}
