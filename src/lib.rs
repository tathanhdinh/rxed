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

    pub fn decode(&self, itext: &[u8]) -> Result<DecodedInstruction, error::Error> {
        // let mut inst: gen::xed_decoded_inst_t = unsafe { 
        //     // let mut inst: gen::xed_decoded_inst_t = std::mem::zeroed();
        //     let mut inst = std::mem::uninitialized();
        //     gen::xed_decoded_inst_zero(&mut inst);
        //     inst
        // };
        let mut inst = unsafe { std::mem::uninitialized() };
        let xed_error = unsafe {
            gen::xed_decoded_inst_zero_set_mode(&mut inst, &self.state);
            gen::xed_decode(&mut inst, itext.as_ptr(), itext.len() as u32)
        };
        if xed_error == gen::XED_ERROR_NONE {
            Ok(DecodedInstruction { inst })
        }
        else {
            Err(error::Error::new(xed_error))
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
    // use gen::*;
    // use super::*;
    use super::*;

    #[test]
    fn xed_min_mode_legacy_32() {
        let itext = [0xf, 0x85, 0x99, 0x00, 0x00, 0x00];
        let decoder = Decoder::new(gen::XED_MACHINE_MODE_LEGACY_32, gen::XED_ADDRESS_WIDTH_32b);
        assert!(decoder.decode(&itext).is_ok())
    }

    #[test]
    fn xed_min_mode_long_64() {
        let itext = [0xf, 0x85, 0x99, 0x00, 0x00, 0x00];
        let decoder = Decoder::new(gen::XED_MACHINE_MODE_LONG_64, gen::XED_ADDRESS_WIDTH_64b);
        assert!(decoder.decode(&itext).is_ok())
    }

    #[test]
    fn xed_min() {
        let mmode_64 = gen::XED_MACHINE_MODE_LONG_64;
        let stack_addr_width_64 = gen::XED_ADDRESS_WIDTH_64b;
    }
}