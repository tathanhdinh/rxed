extern crate failure;
#[macro_use] extern crate failure_derive;

pub mod gen;
pub mod error;

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


pub struct DecodedInstruction {
    inst: gen::xed_decoded_inst_t,
}

impl std::fmt::Display for DecodedInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unimplemented!()
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

    pub fn decode(itext: &[u8]) -> Result<DecodedInstruction, error::Error> {
        let mut inst: gen::xed_decoded_inst_t = unsafe { 
            let mut inst: gen::xed_decoded_inst_t = std::mem::zeroed();
            gen::xed_decoded_inst_zero(&mut inst);
            inst
        };
        let xed_error = unsafe {
            gen::xed_decode(&mut inst, itext.as_ptr(), itext.len() as u32)
        };
        if xed_error == gen::XED_ERROR_NONE {
            Ok(DecodedInstruction { inst })
        }
        else {
            Err(error::Error { xed_error })
        }
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