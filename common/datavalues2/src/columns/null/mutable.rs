// Copyright 2021 Datafuse Labs.
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

use std::sync::Arc;

use common_arrow::arrow::bitmap::MutableBitmap;

use crate::columns::mutable::MutableColumn;
use crate::types::DataTypePtr;
use crate::ColumnRef;
use crate::NullColumn;
use crate::NullType;

#[derive(Debug, Default)]
pub struct MutableNullColumn {
    length: usize,
}

impl MutableNullColumn {
    pub fn finish(&mut self) -> NullColumn {
        self.length = 0;
        NullColumn {
            length: self.length,
        }
    }
}

impl MutableColumn for MutableNullColumn {
    fn data_type(&self) -> DataTypePtr {
        Arc::new(NullType {})
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_column(&mut self) -> ColumnRef {
        Arc::new(self.finish())
    }

    fn append_default(&mut self) {
        self.length += 1;
    }

    fn shrink_to_fit(&mut self) {}

    fn append_null(&mut self) -> bool {
        self.length += 1;
        true
    }

    fn validity(&self) -> Option<&MutableBitmap> {
        None
    }
}
