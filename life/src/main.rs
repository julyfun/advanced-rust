fn main() {
    let mut d = vec![1, 2];
    let mref = &mut d;
    mref.push(3);
    let sref = &d;
    println!("{sref:?}");
}

