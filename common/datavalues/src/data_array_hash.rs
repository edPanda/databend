// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::marker::PhantomData;
use std::sync::Arc;

use common_arrow::arrow::array::Array;
use common_arrow::arrow::array::BinaryArray;
use common_arrow::arrow::array::GenericStringArray;
use common_arrow::arrow::array::LargeBinaryArray;
use common_arrow::arrow::array::StringOffsetSizeTrait;
use common_arrow::arrow::array::UInt64Array;
use common_arrow::arrow::datatypes::DataType;
use common_exception::ErrorCodes;
use common_exception::Result;

use crate::DataArrayRef;
use crate::DataColumnarValue;
use crate::DataValue;

pub struct DataArrayHash<Hasher: std::hash::Hasher + Default>(PhantomData<Hasher>);

impl<Hasher: std::hash::Hasher + Default> DataArrayHash<Hasher> {
    pub fn data_array_hash(input: &DataColumnarValue) -> Result<DataColumnarValue> {
        match input {
            DataColumnarValue::Array(input) => Ok(DataColumnarValue::Array(
                Self::data_array_hash_with_array(input)?
            )),
            DataColumnarValue::Constant(input, rows) => Ok(DataColumnarValue::Constant(
                Self::data_array_hash_with_scalar(input)?,
                *rows
            ))
        }
    }

    fn data_array_hash_with_scalar(input: &DataValue) -> Result<DataValue> {
        match input {
            DataValue::Null => Ok(DataValue::Null),
            DataValue::Utf8(str) => match str {
                None => Ok(DataValue::Null),
                Some(str) => {
                    let mut hasher = Hasher::default();
                    hasher.write(str.as_bytes());
                    Ok(DataValue::UInt64(Some(hasher.finish())))
                }
            },
            _ => Result::Err(ErrorCodes::BadDataValueType(
                "DataArray Error: data_array_hash_with_array must be string or binary type."
            ))
        }
    }

    fn data_array_hash_with_array(input: &DataArrayRef) -> Result<DataArrayRef> {
        match input.data_type() {
            DataType::Utf8 => Self::string_data_array_hash_with_array::<i32>(input),
            DataType::LargeUtf8 => Self::string_data_array_hash_with_array::<i64>(input),
            DataType::Binary => Self::binary_data_array_hash_with_array(input),
            DataType::LargeBinary => Self::large_binary_data_array_hash_with_array(input),
            _ => Result::Err(ErrorCodes::BadDataValueType(
                " DataArray Error: data_array_hash_with_array must be string or binary type."
            ))
        }
    }

    fn string_data_array_hash_with_array<T: StringOffsetSizeTrait>(
        data: &DataArrayRef
    ) -> Result<DataArrayRef> {
        let binary_data = data
            .as_any()
            .downcast_ref::<GenericStringArray<T>>()
            .ok_or_else(|| {
                ErrorCodes::BadDataValueType(format!(
                    "DataArray Error: Cannot downcast_array from datatype:{:?} item to:{}",
                    data.data_type(),
                    stringify!(GenericStringArray<T>)
                ))
            })?;

        let mut hasher = Hasher::default();
        let value_size = binary_data.value_data().len();
        let mut hash_builder = UInt64Array::builder(value_size);

        for index in 0..binary_data.len() {
            match binary_data.is_null(index) {
                true => {
                    let _ = hash_builder.append_null()?;
                }
                false => {
                    hasher.write(binary_data.value(index).as_bytes());
                    let _ = hash_builder.append_value(hasher.finish())?;
                }
            };
        }

        Ok(Arc::new(hash_builder.finish()))
    }

    fn binary_data_array_hash_with_array(data: &DataArrayRef) -> Result<DataArrayRef> {
        let binary_data = data.as_any().downcast_ref::<BinaryArray>().ok_or_else(|| {
            ErrorCodes::BadDataValueType(format!(
                "DataValue Error: Cannot downcast_array from datatype:{:?} item to:{}",
                data.data_type(),
                stringify!(BinaryArray)
            ))
        })?;

        let value_size = binary_data.value_data().len();
        let mut hash_builder = UInt64Array::builder(value_size);

        for index in 0..binary_data.len() {
            let mut hasher = Hasher::default();
            match binary_data.is_null(index) {
                true => {
                    let _ = hash_builder.append_null()?;
                }
                false => {
                    hasher.write(binary_data.value(index));
                    let _ = hash_builder.append_value(hasher.finish())?;
                }
            };
        }

        Ok(Arc::new(hash_builder.finish()))
    }

    fn large_binary_data_array_hash_with_array(data: &DataArrayRef) -> Result<DataArrayRef> {
        let binary_data = data
            .as_any()
            .downcast_ref::<LargeBinaryArray>()
            .ok_or_else(|| {
                ErrorCodes::BadDataValueType(format!(
                    "DataValue Error: Cannot downcast_array from datatype:{:?} item to:{}",
                    data.data_type(),
                    stringify!(LargeBinaryArray)
                ))
            })?;

        let mut hasher = Hasher::default();
        let value_size = binary_data.value_data().len();
        let mut hash_builder = UInt64Array::builder(value_size);

        for index in 0..binary_data.len() {
            match binary_data.is_null(index) {
                true => {
                    let _ = hash_builder.append_null()?;
                }
                false => {
                    hasher.write(binary_data.value(index));
                    let _ = hash_builder.append_value(hasher.finish())?;
                }
            };
        }

        Ok(Arc::new(hash_builder.finish()))
    }
}
