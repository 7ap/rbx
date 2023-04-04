use std::ops::Deref;

use crate::app::v8tree::Instance;

#[derive(Debug)]
#[repr(C)]
pub struct DataModel {
    _super0: Instance,
}

impl Deref for DataModel {
    type Target = Instance;

    fn deref(&self) -> &Self::Target {
        &self._super0
    }
}
