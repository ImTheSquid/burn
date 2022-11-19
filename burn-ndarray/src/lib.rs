#[macro_use]
extern crate derive_new;

#[cfg(any(
    feature = "blas-netlib",
    feature = "blas-openblas",
    feature = "blas-openblas-system",
))]
extern crate blas_src;

mod backend;
mod element;
mod module_ops;
mod ops;
mod tensor;
mod tensor_ops;

pub use backend::*;
pub(crate) use tensor::*;

#[cfg(test)]
mod tests {
    type TestBackend = crate::NdArrayBackend<f32>;

    burn_tensor::testgen_all!();
}