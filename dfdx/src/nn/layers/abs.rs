use crate::prelude::*;

/// Calls [crate::tensor_ops::abs()]
#[derive(Default, Debug, Clone, Copy, crate::nn::CustomModule)]
pub struct Abs;
impl<S: Shape, E: Dtype, D: Device<E>, T: Tape<E, D>> crate::nn::Module<Tensor<S, E, D, T>>
    for Abs
{
    type Output = Tensor<S, E, D, T>;
    type Error = D::Err;
    fn try_forward(&self, x: Tensor<S, E, D, T>) -> Result<Self::Output, Self::Error> {
        x.try_abs()
    }
}
