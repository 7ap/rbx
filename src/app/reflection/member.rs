use std::ffi::c_char;
use std::ops::Deref;

use crate::app::security::Permissions;
use crate::app::util::Name;
use crate::base::rbx::DenseHashMap;
use crate::stl::functional::Function;
use crate::stl::vector::Vector;

use super::{ClassDescriptor, Descriptor};

#[derive(Debug)]
#[repr(C)]
pub struct MemberDescriptor {
    _super0: Descriptor,
    pub category: *const Name,
    pub owner: *const ClassDescriptor,
    pub security: Permissions,
}

impl Deref for MemberDescriptor {
    type Target = Descriptor;

    fn deref(&self) -> &Self::Target {
        &self._super0
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct MemberDescriptorContainer {
    pub descriptors: Vector<*mut MemberDescriptor>,
    pub descriptor_lookup: DenseHashMap<*const c_char, *mut MemberDescriptor>,
    pub derived_containers: Vector<*mut MemberDescriptorContainer>,
    pub base: *const MemberDescriptorContainer,
    pub descriptor_added_callback: Function<fn(*mut MemberDescriptor)>,
}
