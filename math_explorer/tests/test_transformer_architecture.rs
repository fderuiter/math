use math_explorer::ai::transformer::traits::NormalizationLayer;
use math_explorer::ai::transformer::{EncoderLayer, FeedForward, MultiHeadAttention};
use nalgebra::DMatrix;

/// A mock normalization layer that does nothing (Identity).
struct IdentityNorm;

impl NormalizationLayer for IdentityNorm {
    fn forward(&self, x: &DMatrix<f64>) -> DMatrix<f64> {
        x.clone()
    }
}

#[test]
fn test_transformer_dependency_injection() {
    let d_model = 64;
    let h = 4;
    let d_ff = 256;

    // Create standard components
    let attn = MultiHeadAttention::new(d_model, h).unwrap();
    let ff = FeedForward::new(d_model, d_ff);

    // Create our custom component
    let identity_norm1 = IdentityNorm;
    let identity_norm2 = IdentityNorm;

    // Inject them into the EncoderLayer
    // We explicitly specify the types to prove the generic parameters work
    let layer: EncoderLayer<MultiHeadAttention, FeedForward, IdentityNorm> =
        EncoderLayer::new_with_components(attn, ff, identity_norm1, identity_norm2);

    // Create a dummy input
    let x = DMatrix::from_element(5, d_model, 1.0);

    // Run forward pass
    let output = layer.forward(&x, None);

    // Verify dimensions
    assert_eq!(output.nrows(), 5);
    assert_eq!(output.ncols(), d_model);

    // Since we used IdentityNorm, the values will be different than standard LayerNorm,
    // but the fact that it compiled and ran proves the architectural flexibility.
}

#[test]
fn test_transformer_builder() {
    use math_explorer::ai::transformer::TransformerBuilder;

    let builder = TransformerBuilder::new()
        .d_model(64)
        .heads(4)
        .d_ff(128)
        .layers(2);

    let transformer = builder.build().expect("Should build successfully");

    assert_eq!(transformer.encoder.layers.len(), 2);
    assert_eq!(transformer.decoder.layers.len(), 2);
}

#[test]
fn test_transformer_builder_invalid() {
    use math_explorer::ai::transformer::TransformerBuilder;

    // d_model (65) is not divisible by heads (4)
    let builder = TransformerBuilder::new()
        .d_model(65)
        .heads(4)
        .d_ff(128)
        .layers(2);

    let result = builder.build();
    assert!(result.is_err());

    // Check error type
    match result {
        Err(math_explorer::ai::error::AIError::DimensionMismatch { .. }) => (),
        _ => panic!("Expected DimensionMismatch error"),
    }
}
