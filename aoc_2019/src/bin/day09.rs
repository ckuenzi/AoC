mod intscript;

fn main() {
    let mut computer = intscript::Computer::new_from_text(include_str!("inputs\\day09a.txt"));
    println!("{:?}", computer.siso(1));
    println!("{:?}", computer.reset().siso(2));
}
