use com::{co_class, interfaces::iunknown::IUnknown};
//#use winapi::shared::winerror::NOERROR;

#[co_class(implements(IUnknown))]
struct Test {
    field1: u32,
    field2: u32
}

impl Test {
    fn new() -> Box<Self> {
        Test::allocate(0, 0)
    }
}

fn main() {
    let _ = Test::new();
}
