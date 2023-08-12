use crate::stl::vector::Vector;

#[derive(Debug)]
#[repr(C)]
pub struct OrderedVectorSet<Type, Order> {
    pub storage: Vector<Type>,
    pub order: Order,
    _pad0: [u8; 0x007],
}
