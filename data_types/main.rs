use std::mem;

fn main() {
    let mut a:u8 = 129;
    println!("a={}", a);
    a = 129;
    println!("{}", a);

    let mut c = 123456789; // 32 bits i32
    println!("{} {}", c, mem::size_of_val(&c));

    let z:isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {}, {} bit os", z, size_of_z, size_of_z*8);
    let d:char = 'x';
    println!("d = {} {}", d, mem::size_of_val(&d));

}   