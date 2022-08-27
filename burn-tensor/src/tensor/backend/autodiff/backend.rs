use super::ADTensor;
use crate::graph::grad::Gradients;
use crate::tensor::backend::{ADBackend, Backend};
use crate::tensor::{Data, Distribution, Shape};

#[cfg(feature = "ndarray")]
use crate::NdArrayElement;

#[cfg(feature = "tch")]
use crate::TchElement;

macro_rules! define_impl {
    (
        name: $name:ident,
        backend: $backend:ty,
        element: $element:ident
    ) => {
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name<E> {
            _b: $backend,
        }

        impl<E: $element> Backend for $name<E> {
            type Device = <$backend as Backend>::Device;
            type Elem = E;
            type FullPrecisionElem = f32;
            type IntegerBackend = <$backend as Backend>::IntegerBackend;
            type FullPrecisionBackend = $name<<$backend as Backend>::FullPrecisionElem>;
            type TensorPrimitive<const D: usize> = ADTensor<D, $backend>;
            type BoolTensorPrimitive<const D: usize> =
                <$backend as Backend>::BoolTensorPrimitive<D>;

            fn from_data<const D: usize>(
                data: Data<Self::Elem, D>,
                device: Self::Device,
            ) -> Self::TensorPrimitive<D> {
                let tensor = <$backend as Backend>::from_data(data, device);
                ADTensor::from_tensor(tensor)
            }

            fn from_data_bool<const D: usize>(
                data: Data<bool, D>,
                device: Self::Device,
            ) -> Self::BoolTensorPrimitive<D> {
                <$backend as Backend>::from_data_bool(data, device)
            }

            fn random<const D: usize>(
                shape: Shape<D>,
                distribution: Distribution<Self::Elem>,
                device: Self::Device,
            ) -> Self::TensorPrimitive<D> {
                Self::from_data(Data::random(shape, distribution), device)
            }

            fn ad_enabled() -> bool {
                true
            }

            fn zeros<const D: usize>(
                shape: Shape<D>,
                device: Self::Device,
            ) -> Self::TensorPrimitive<D> {
                Self::from_data(Data::zeros(shape), device)
            }

            fn ones<const D: usize>(
                shape: Shape<D>,
                device: Self::Device,
            ) -> Self::TensorPrimitive<D> {
                Self::from_data(Data::ones(shape), device)
            }

            fn name() -> String {
                format!("autodiff<{}>", <$backend as Backend>::name())
            }
        }

        impl<E: $element> ADBackend for $name<E> {
            type InnerBackend = $backend;

            fn backward<const D: usize>(tensor: &Self::TensorPrimitive<D>) -> Gradients {
                tensor.backward()
            }
            fn grad<const D: usize>(
                tensor: &Self::TensorPrimitive<D>,
                grads: &Gradients,
            ) -> Option<<$backend as Backend>::TensorPrimitive<D>> {
                grads.wrt(tensor).map(|grad| grad.clone())
            }

            fn inner<const D: usize>(
                tensor: &Self::TensorPrimitive<D>,
            ) -> <Self::InnerBackend as Backend>::TensorPrimitive<D> {
                tensor.tensor()
            }

            fn from_inner<const D: usize>(
                tensor: <Self::InnerBackend as Backend>::TensorPrimitive<D>,
            ) -> Self::TensorPrimitive<D> {
                ADTensor::from_tensor(tensor)
            }
        }
    };
}

#[cfg(feature = "ndarray")]
define_impl!(
    name: ADBackendNdArray,
    backend: crate::tensor::backend::ndarray::NdArrayBackend<E>,
    element: NdArrayElement
);
#[cfg(feature = "tch")]
define_impl!(
    name: ADBackendTch,
    backend: crate::tensor::backend::tch::TchBackend<E>,
    element: TchElement
);

#[macro_export]
macro_rules! register_ndarray {
    () => {
        #[cfg(feature = "ndarray")]
        mod ndarray_impl {
            use super::*;
            use crate::NdArrayElement;

            define_impl!(
                crate::tensor::backend::autodiff::ADBackendNdArray::<E>,
                crate::tensor::backend::ndarray::NdArrayBackend::<E>,
                NdArrayElement
            );
        }
    };
}

#[macro_export]
macro_rules! register_tch {
    () => {
        #[cfg(feature = "tch")]
        mod tch_impl {
            use super::*;
            use crate::TchElement;

            define_impl!(
                $crate::tensor::backend::autodiff::ADBackendTch::<E>,
                $crate::tensor::backend::tch::TchBackend::<E>,
                TchElement
            );
        }
    };
}
