use franklin_crypto::bellman::bn256::{Bn256, Fr};
use franklin_crypto::bellman::{from_hex, to_hex};
use franklin_crypto::bellman::{PrimeField, PrimeFieldRepr};
use franklin_crypto::rescue::bn256::Bn256RescueParams;
use franklin_crypto::rescue::rescue_hash;
use web_sys::console;

thread_local! {
    pub static RESCUE_PARAMS: Bn256RescueParams = Bn256RescueParams::new_checked_2_into_1();
}

use wasm_bindgen::prelude::*;

// executed automatically ...
#[wasm_bindgen(start)]
pub fn init() {
    //console::log_1(&"init rescue wasm".into());
    console_error_panic_hook::set_once();
    RESCUE_PARAMS.with(|_| {});
    //console::log_1(&"init rescue wasm done".into());
}

fn field_to_bytes(elem: &Fr) -> Vec<u8> {
    let repr = elem.into_repr();
    let required_length = repr.as_ref().len() * 8;
    let mut buf: Vec<u8> = Vec::with_capacity(required_length);
    repr.write_be(&mut buf).unwrap();
    buf
}

fn byte_to_field(u: &u8) -> Fr {
    let mut repr = <Fr as PrimeField>::Repr::default();
    repr.as_mut()[0] = *u as u64;
    let field = <Fr as PrimeField>::from_repr(repr).unwrap();
    field
}

#[wasm_bindgen(js_name=rescueHash)]
pub fn rescue_hash_bytes(msg: &[u8]) -> Vec<u8> {
    RESCUE_PARAMS.with(|params| {
        let input: Vec<Fr> = msg.iter().map(byte_to_field).collect();
        let hash_output = rescue_hash::<Bn256>(&params, &input);
        field_to_bytes(&hash_output[0])
    })
}

#[wasm_bindgen(js_name=rescueHashHex)]
pub fn rescue_hash_hex(msgs: js_sys::Array) -> String {
    RESCUE_PARAMS.with(|params| {
        // msgs can either start with '0x' or not
        let inputs: Vec<Fr> = msgs
            .iter()
            .map(|s| from_hex(&s.as_string().unwrap()).unwrap())
            .collect();
        let hash_output = rescue_hash::<Bn256>(&params, &inputs);
        format!("0x{}", to_hex(&hash_output[0]))
    })
}
