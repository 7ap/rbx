use std::ops::Deref;

use crate::app::reflection::DescribedBase;
use crate::app::util::GuidItem;
use crate::base::rbx::diagnostics::Countable;
use crate::base::rbx::Noncopyable;
use crate::stl::memory::SharedPtr;
use crate::stl::string::String as CxxString;
use crate::stl::vector::Vector;

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
    // pub edit_time_metadata: EditTimeMetadata,
    // pub deprecated_unique_id: UniqueId,
    // pub completeness: Completeness,
    // pub on_demand_ptr: OnDemandInstance,
    // pub owned_by_actor: WeakPtr<Actor>,
    // pub ancestry_changed_signal: Signal<fn(SharedPtr<Instance>, SharedPtr<Instance>)>,
    // pub property_changed_signal: Signal<fn(*const PropertyDescriptor)>,
    // pub combined_signal: Signal<fn(CombinedSignalType, *const CombinedSignalData)>,
}

impl Deref for Instance {
    type Target = DescribedBase;

    fn deref(&self) -> &Self::Target {
        &self._super0
    }
}
