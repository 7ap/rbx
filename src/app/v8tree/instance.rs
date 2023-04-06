use std::ops::Deref;

use crate::app::reflection::DescribedBase;
use crate::app::util::{GuidItem, UniqueId};
use crate::base::rbx::diagnostics::Countable;
use crate::base::rbx::{Noncopyable, Signal};
use crate::stl::memory::{SharedPtr, UniquePtr, WeakPtr};
use crate::stl::string::String as CxxString;
use crate::stl::vector::Vector;

#[derive(Debug)]
#[repr(C)]
pub struct Completeness {
    pub num_local_replicated_children: i32,
    pub num_expected_direct_children: i32,
    pub num_computed_complete_direct_children: u32, // : 24
    pub is_complete: bool,                          // : true
    pub new_complete: bool,                         // : true
    pub ack_complete: bool,                         // : true
    pub enqueued: bool,                             // : true
    pub finalization_enqueued: bool,                // : true
    pub completeness_tracked: bool,                 // : true
    pub changed_parent_locally: bool,               // : true
    pub processing_replicated_parent_change: bool,  // : true
}

#[derive(Debug)]
#[repr(C)]
pub struct Instance {
    _super0: DescribedBase,
    _super1: GuidItem<Self>,
    _super2: Countable<Self>,
    _super3: Noncopyable,
    pub archivable: bool,
    pub is_parent_locked: bool,
    pub is_setting_parent: bool,
    pub ancestor_permission_mask: u8,
    pub name: *mut CxxString,
    pub children: SharedPtr<Vector<SharedPtr<*mut Instance>>>,
    pub parent: *mut Instance,
    pub edit_time_metadata: UniquePtr<usize>, // UniquePtr<EditTimeMetadata>
    pub deprecated_unique_id: UniqueId,
    pub completeness: Completeness,
    pub on_demand_ptr: UniquePtr<usize>, // UniquePtr<OnDemandInstance>
    pub owned_by_actor: WeakPtr<usize>,  // WeakPtr<Actor>,
    pub ancestry_changed_signal: Signal<fn(SharedPtr<Instance>, SharedPtr<Instance>)>,
    pub property_changed_signal: Signal<fn()>, // Signal<fn(*const PropertyDescriptor)>
    pub combined_signal: Signal<fn()>, // Signal<fn(CombinedSignalType, *const CombinedSignalData)>,
}

impl Deref for Instance {
    type Target = DescribedBase;

    fn deref(&self) -> &Self::Target {
        &self._super0
    }
}
