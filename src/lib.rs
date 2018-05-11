extern crate failure;

pub mod gen;

pub fn get_copyright() -> String {
    let cpyr = unsafe {
        std::ffi::CStr::from_ptr(gen::xed_get_copyright())
    };
    cpyr.to_str().unwrap().to_owned()
}

pub fn get_version() -> String {
    let vers = unsafe {
        std::ffi::CStr::from_ptr(gen::xed_get_version())
    };
    vers.to_str().unwrap().to_owned()
}

#[derive(Debug)]
pub struct Error {
    xed_error: gen::xed_error_enum_t,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
        let err_str = unsafe {
            std::ffi::CStr::from_ptr(gen::xed_error_enum_t2str(self.xed_error))
        };
        write!(f, "{}", err_str.to_str().unwrap())
    }
}

pub struct Decoder {
    state: gen::xed_state_s,
}

impl Decoder {
    pub fn new(mmode: gen::xed_machine_mode_enum_t, stack_addr_width: gen::xed_address_width_enum_t) -> Decoder {
        unsafe {
            gen::xed_tables_init();
        }
        let state = gen::xed_state_s { mmode, stack_addr_width };
        Decoder { state }
    }
}

// pub fn initialize_tables() {
//     unsafe {
//         gen::xed_tables_init();
//     }
// }

// fn main() {
//     println!("Hello, world!");
// }

#[cfg(test)]
mod tests {
    use gen::*;
    use super::*;

    #[test]
    fn xed_min() {
        let mmode_64 = gen::XED_MACHINE_MODE_LONG_64;
        let stack_addr_width_64 = gen::XED_ADDRESS_WIDTH_64b;


    }
}