//ch2-2-10
fn main(){
    let a = 42;
    let r = &a;
    let b = a + *r;

    println!("a + a = {}", b);
}