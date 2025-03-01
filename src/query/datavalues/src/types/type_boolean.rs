// Copyright 2021 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::data_type::DataType;
use super::type_id::TypeID;
pub use crate::prelude::*;

#[derive(Default, Clone, Hash, serde::Deserialize, serde::Serialize)]
pub struct BooleanType {}

impl BooleanType {
    pub fn new_impl() -> DataTypeImpl {
        Self {}.into()
    }
}

impl DataType for BooleanType {
    fn data_type_id(&self) -> TypeID {
        TypeID::Boolean
    }

    fn name(&self) -> String {
        "Boolean".to_string()
    }
}

impl std::fmt::Debug for BooleanType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
