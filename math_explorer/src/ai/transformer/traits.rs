use nalgebra::DMatrix;

/// Trait representing an attention mechanism (e.g., Scaled Dot-Product, Multi-Head).
pub trait AttentionMechanism {
    /// Performs the forward pass of the attention mechanism.
    ///
    /// # Arguments
    /// * `q`: Queries matrix.
    /// * `k`: Keys matrix.
    /// * `v`: Values matrix.
    /// * `mask`: Optional mask.
    fn forward(
        &self,
        q: &DMatrix<f64>,
        k: &DMatrix<f64>,
        v: &DMatrix<f64>,
        mask: Option<&DMatrix<f64>>,
    ) -> DMatrix<f64>;
}

/// Trait representing a feed-forward network component.
pub trait FeedForwardNetwork {
    /// Performs the forward pass.
    ///
    /// # Arguments
    /// * `x`: Input matrix.
    fn forward(&self, x: &DMatrix<f64>) -> DMatrix<f64>;
}

/// Trait representing a normalization layer (e.g., LayerNorm, RMSNorm).
pub trait NormalizationLayer {
    /// Performs the normalization.
    ///
    /// # Arguments
    /// * `x`: Input matrix.
    fn forward(&self, x: &DMatrix<f64>) -> DMatrix<f64>;
}
