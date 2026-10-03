use dw23::describe_load;
use dw23::wagon::Cargo;

fn main() {
    let standard = [Cargo::Flour(400), Cargo::Bullets(50), Cargo::Oxen(4), Cargo::Flour(1150)];
    let full = [Cargo::Flour(1900), Cargo::Bullets(50)];
    let heavy = [Cargo::Flour(2000), Cargo::Bullets(1)];

    println!("{}", describe_load(&standard));
    println!("{}", describe_load(&full));
    println!("{}", describe_load(&heavy));
}
