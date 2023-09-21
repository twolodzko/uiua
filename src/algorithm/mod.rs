use std::convert::Infallible;

use crate::{
    array::{Array, ArrayValue, Shape},
    Uiua, UiuaError, UiuaResult,
};

mod dyadic;
pub mod fork;
pub(crate) mod invert;
pub mod loops;
mod monadic;
pub mod pervade;

fn max_shape(a: &[usize], b: &[usize]) -> Shape {
    let shape_len = a.len().max(b.len());
    let mut new_shape = Shape::with_capacity(shape_len);
    for _ in 0..shape_len {
        new_shape.push(0);
    }
    for i in 0..new_shape.len() {
        let j = new_shape.len() - i - 1;
        if a.len() > i {
            new_shape[j] = a[a.len() - i - 1];
        }
        if b.len() > i {
            new_shape[j] = new_shape[j].max(b[b.len() - i - 1]);
        }
    }
    new_shape
}

pub trait FillContext: Copy {
    type Error;
    fn error(self, msg: impl ToString) -> Self::Error;
    fn fill<T: ArrayValue>(self) -> Option<T>;
}

impl FillContext for &Uiua {
    type Error = UiuaError;
    fn error(self, msg: impl ToString) -> Self::Error {
        self.error(msg)
    }
    fn fill<T: ArrayValue>(self) -> Option<T> {
        T::get_fill(self)
    }
}

impl FillContext for () {
    type Error = Infallible;
    fn error(self, msg: impl ToString) -> Self::Error {
        panic!("{}", msg.to_string())
    }
    fn fill<T: ArrayValue>(self) -> Option<T> {
        None
    }
}

fn op_bytes_retry_fill<T>(
    bytes: Array<u8>,
    on_bytes: impl FnOnce(Array<u8>) -> UiuaResult<T>,
    on_nums: impl FnOnce(Array<f64>) -> UiuaResult<T>,
) -> UiuaResult<T> {
    match on_bytes(bytes.clone()) {
        Ok(res) => Ok(res),
        Err(err) => {
            if err.is_fill() {
                on_nums(bytes.convert())
            } else {
                Err(err)
            }
        }
    }
}

fn op_bytes_ref_retry_fill<T>(
    bytes: &Array<u8>,
    on_bytes: impl FnOnce(&Array<u8>) -> UiuaResult<T>,
    on_nums: impl FnOnce(&Array<f64>) -> UiuaResult<T>,
) -> UiuaResult<T> {
    match on_bytes(bytes) {
        Ok(res) => Ok(res),
        Err(err) => {
            if err.is_fill() {
                on_nums(&bytes.clone().convert())
            } else {
                Err(err)
            }
        }
    }
}
