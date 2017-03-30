use traits::{Chainable};

pub fn build(layers: Vec<Box<Chainable>>) -> Box<[u8]> {
    let mut out = Vec::new();
    for layer in &layers {
        layer.to_binary(&mut out);
    }
    out.into_boxed_slice()
}
