use crate::{dtypes::Dtype, tensor::UniqueId, tensor_ops::Device};

use std::vec::Vec;

impl<E: Dtype, D: Device<E>, T: crate::nn_traits::BuildOnDevice<E, D>>
    crate::nn_traits::BuildOnDevice<E, D> for Vec<T>
{
    type Built = Vec<T::Built>;
    fn try_build_on_device(&self, device: &D) -> Result<Self::Built, <D>::Err> {
        self.iter()
            .map(|m_i| m_i.try_build_on_device(device))
            .collect()
    }
}

impl<E: Dtype, D: Device<E>, T: crate::nn_traits::ResetParams<E, D>>
    crate::nn_traits::ResetParams<E, D> for Vec<T>
{
    fn try_reset_params(&mut self) -> Result<(), <D>::Err> {
        for m_i in self.iter_mut() {
            m_i.try_reset_params()?;
        }
        Ok(())
    }
}

impl<E: Dtype, D: Device<E>, T: crate::nn_traits::UpdateParams<E, D>>
    crate::nn_traits::UpdateParams<E, D> for Vec<T>
{
    fn try_update_params<M, Optim: crate::nn_traits::Optimizer<M, E, D>>(
        &mut self,
        optimizer: &mut Optim,
        gradients: &crate::tensor::Gradients<E, D>,
        missing_tensors: &mut Vec<UniqueId>,
    ) -> Result<(), D::Err> {
        for m_i in self.iter_mut() {
            m_i.try_update_params(optimizer, gradients, missing_tensors)?;
        }
        Ok(())
    }
}

impl<E: Dtype, D: Device<E>, T: crate::nn_traits::ZeroGrads<E, D>> crate::nn_traits::ZeroGrads<E, D>
    for Vec<T>
{
    fn try_zero_grads(&self, grads: &mut crate::tensor::Gradients<E, D>) -> Result<(), <D>::Err> {
        for m_i in self.iter() {
            m_i.try_zero_grads(grads)?;
        }
        Ok(())
    }
}

#[cfg(feature = "safetensors")]
impl<T: crate::nn_traits::SaveSafeTensors> crate::nn_traits::SaveSafeTensors for Vec<T> {
    fn write_safetensors(
        &self,
        location: &str,
        tensors: &mut Vec<(String, safetensors::Dtype, Vec<usize>, Vec<u8>)>,
    ) {
        for (i, t) in self.iter().enumerate() {
            t.write_safetensors(&format!("{location}{i}."), tensors);
        }
    }
}

#[cfg(feature = "safetensors")]
impl<T: crate::nn_traits::LoadSafeTensors> crate::nn_traits::LoadSafeTensors for Vec<T> {
    fn read_safetensors(
        &mut self,
        location: &str,
        tensors: &safetensors::SafeTensors,
    ) -> Result<(), safetensors::SafeTensorError> {
        for (i, t) in self.iter_mut().enumerate() {
            t.read_safetensors(&format!("{location}{i}."), tensors)?;
        }
        Ok(())
    }
}

impl<Input, T: crate::nn_traits::Module<Input, Output = Input>> crate::nn_traits::Module<Input>
    for Vec<T>
{
    type Output = T::Output;
    type Error = T::Error;

    fn try_forward(&self, mut x: Input) -> Result<Self::Output, T::Error> {
        for m_i in self.iter() {
            x = m_i.try_forward(x)?;
        }
        Ok(x)
    }
    fn try_forward_mut(&mut self, mut x: Input) -> Result<Self::Output, Self::Error> {
        for m_i in self.iter_mut() {
            x = m_i.try_forward_mut(x)?;
        }
        Ok(x)
    }
}
