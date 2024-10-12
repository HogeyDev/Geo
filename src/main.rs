pub mod generator;
pub mod io;

use generator::Generator;

fn main() {
    let mut gen = Generator::new();
    gen.load_gameplay("example/wave.gp");
    gen.print_gameplay();
}
