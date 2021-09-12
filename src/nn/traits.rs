use crate::gradients::traits::Params;
use crate::randomize::Randomize;
use crate::tensor::traits::{Batch, Tensor};

pub trait Module: Randomize + Params + Default {
    type Input: Tensor + Batch;
    type Output: Tensor + Batch;

    fn forward<const B: usize>(
        &mut self,
        input: &mut <Self::Input as Batch>::Batched<B>,
    ) -> <Self::Output as Batch>::Batched<B>;
}