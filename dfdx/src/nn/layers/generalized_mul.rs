use crate::prelude::*;

/// A residual connection around two modules: `T(x) * U(x)`.
///
/// # Generics
/// - `T`: The underlying module to do a skip connection around.
/// - `U`: The underlying residual module
///
/// # Examples
/// ```rust
/// # use dfdx::prelude::*;
/// # use dfdx::*;
/// # let dev: Cpu = Default::default();
/// type Model = GeneralizedMul<ReLU, Square>;
/// let model = dev.build_module::<f32>(Model::default());
/// let x = dev.tensor([-2.0, -1.0, 0.0, 1.0, 2.0]);
/// let y = model.forward(x);
/// assert_eq!(y.array(), [0.0, 0.0, 0.0, 1.0, 8.0]);
/// ```
#[derive(
    Default, Clone, Debug, ResetParams, ZeroGrads, UpdateParams, LoadSafeTensors, SaveSafeTensors,
)]
pub struct GeneralizedMul<T, U> {
    #[module]
    #[serialize]
    pub t: T,
    #[module]
    #[serialize]
    pub u: U,
}

impl<E: Dtype, D: Device<E>, T: BuildOnDevice<E, D>, U: BuildOnDevice<E, D>> BuildOnDevice<E, D>
    for GeneralizedMul<T, U>
{
    type Built = GeneralizedMul<T::Built, U::Built>;
    fn try_build_on_device(&self, device: &D) -> Result<Self::Built, <D>::Err> {
        let t = self.t.try_build_on_device(device)?;
        let u = self.u.try_build_on_device(device)?;
        Ok(GeneralizedMul { t, u })
    }
}

impl<X: WithEmptyTape, T: Module<X>, U: Module<X, Error = T::Error>> Module<X>
    for GeneralizedMul<T, U>
where
    T::Output: TryMul<U::Output, Err = T::Error>,
{
    type Output = <T::Output as TryMul<U::Output>>::Output;
    type Error = T::Error;
    fn try_forward(&self, x: X) -> Result<Self::Output, Self::Error> {
        let t = self.t.try_forward(x.with_empty_tape())?;
        let u = self.u.try_forward(x)?;
        t.try_mul(u)
    }

    fn try_forward_mut(&mut self, x: X) -> Result<Self::Output, Self::Error> {
        let t = self.t.try_forward_mut(x.with_empty_tape())?;
        let u = self.u.try_forward_mut(x)?;
        t.try_mul(u)
    }
}
