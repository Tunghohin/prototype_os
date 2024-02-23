pub trait AddressMetaData {
    const PA_WIDTH: usize;
    const VA_WIDTH: usize;
    const PPN_WIDTH: usize;
    const VPN_WIDTH: usize;
}

pub trait GenericAddress: AddressMetaData {}
