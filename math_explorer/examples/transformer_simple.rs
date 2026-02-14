//! A simple example demonstrating how to assemble and run a Transformer model.
//!
//! To run this example:
//! `cargo run --example transformer_simple`

use math_explorer::ai::transformer::TransformerBuilder;
use nalgebra::DMatrix;

fn main() {
    // 1. Configure Model Parameters
    let d_model = 64; // Embedding dimension
    let heads = 4; // Number of attention heads
    let d_ff = 128; // Feed-forward dimension
    let layers = 2; // Number of layers

    // 2. Instantiate Components & Assemble Transformer
    // We use the new Builder pattern for safe construction.
    let transformer = TransformerBuilder::new()
        .d_model(d_model)
        .heads(heads)
        .d_ff(d_ff)
        .layers(layers)
        .build()
        .expect("Failed to build Transformer configuration");

    println!("Transformer model assembled successfully!");

    // 4. Create Dummy Input (Batch Size = 1 for simplicity, Sequence Length = 10)
    // Shape: (Sequence Length, Embedding Dimension)
    let seq_len = 10;
    // Using constant values to simulate embeddings
    let input_src = DMatrix::from_element(seq_len, d_model, 0.5);
    let input_tgt = DMatrix::from_element(seq_len, d_model, 0.5);

    // 5. Run Forward Pass
    // A. Encoder Pass
    println!("Running Encoder...");
    // The encoder takes ownership or reference? Let's check docs.
    // Encoder::forward takes `mut x: DMatrix<f64>`. So it consumes input.
    // We should clone if we needed it again, but here we don't.
    let enc_output = transformer.encoder.forward(input_src, None);
    assert_eq!(enc_output.shape(), (seq_len, d_model));

    // B. Decoder Pass
    // Decoder::forward takes `mut x: DMatrix<f64>` and `enc_output: &DMatrix`.
    println!("Running Decoder...");
    let dec_output = transformer
        .decoder
        .forward(input_tgt, &enc_output, None, None);
    assert_eq!(dec_output.shape(), (seq_len, d_model));

    println!(
        "Forward pass complete. Output shape: {:?}",
        dec_output.shape()
    );
}
