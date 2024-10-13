pub mod generator;
pub mod io;
pub mod neural;

use generator::Generator;
use rand::random;

fn main() {
    let mut gen = Generator::new();
    for layer in gen.network.layers.iter_mut() {
        for node in &mut layer.nodes {
            node.value = random();
        }
    }
    println!("{:#?}", gen.network);
    gen.load_gameplay("example/wave.gp");
    gen.run();
    gen.print_gameplay();
}
