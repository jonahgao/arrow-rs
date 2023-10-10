// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! Cast kernel for [Apache Arrow](https://docs.rs/arrow)

pub mod cast;
use arrow_array::{cast::AsArray, Array};
pub use cast::*;
pub mod display;
pub mod parse;

#[cfg(feature = "prettyprint")]
pub mod pretty;

#[test]
fn test_can() {
    use arrow_array::Decimal128Array;
    use arrow_schema::DataType;

    let input_type = DataType::Decimal128(10, 8);
    let output_type = DataType::Decimal128(8, 6);
    let array = Decimal128Array::from(vec![1000_000000])
        .with_precision_and_scale(10, 6)
        .unwrap();

    let options = CastOptions {
        safe: false,
        ..Default::default()
    };
    let out_array = cast_with_options(&array, &output_type, &options).unwrap();
    let out_array: &Decimal128Array = out_array.as_primitive();
    // out_array
    //     .validate_decimal_precision(out_array.precision())
    //     .unwrap();

    println!("{:?}", array);
    println!("{:?}", array.value_as_string(0));
    println!("{:?}", out_array);
    println!("{:?}", out_array.value_as_string(0));
}
