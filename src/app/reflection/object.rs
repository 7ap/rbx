use std::ops::Deref;

use crate::app::util::Name;
use crate::base::rbx::DenseHashMap;
use crate::stl::memory::{SharedPtr, UniquePtr};
use crate::stl::string::String as CxxString;
use crate::stl::vector::Vector;

use super::{Descriptor, EventSource, MemberDescriptor, MemberDescriptorContainer};

#[derive(Debug)]
#[repr(C)]
pub struct ClassDescriptor {
    _super0: Descriptor,
    pub property_descriptors: MemberDescriptorContainer,
    pub event_descriptors: MemberDescriptorContainer,
    pub function_descriptors: MemberDescriptorContainer,
    pub yield_function_descriptors: MemberDescriptorContainer,
    pub callback_descriptors: MemberDescriptorContainer,
    pub security: u8, // crate::app::security::Permissions
    pub is_public: bool,
    pub replicate_type: bool,
    pub can_write_xml: bool,
    pub is_script_creatable: bool,
    pub is_service: bool,
    pub index: u16,
    pub descendant_count: u16,
    pub memory_category: *const usize,
    pub base: *const ClassDescriptor,
    pub derived_classes: Vector<*mut ClassDescriptor>,
    pub descriptor_by_name: DenseHashMap<*const Name, DescriptorEntry>,
}

impl Deref for ClassDescriptor {
    type Target = Descriptor;

    fn deref(&self) -> &Self::Target {
        &self._super0
    }
}

#[derive(Debug)]
pub struct DescriptorEntry {
    pub desc: *mut MemberDescriptor,
    pub r#type: DescriptorType,
}

#[derive(Debug)]
#[repr(C)]
pub enum DescriptorType {
    Property = 0,
    Event = 1,
    Function = 2,
    YieldFunction = 3,
    Callback = 4,
}

#[derive(Debug)]
#[repr(C)]
pub struct DescribedBase {
    _super0: EventSource,
    _super1: SharedPtr<Self>,
    pub descriptor: *const ClassDescriptor,
    pub xml_id: UniquePtr<CxxString>,
}

impl Deref for DescribedBase {
    type Target = EventSource;

    fn deref(&self) -> &Self::Target {
        &self._super0
    }
}
