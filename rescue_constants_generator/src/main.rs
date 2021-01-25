use franklin_crypto::bellman::bn256::{Bn256, Fr};
use franklin_crypto::bellman::to_hex;
use franklin_crypto::bellman::PrimeField;
use franklin_crypto::rescue::bn256::Bn256RescueParams;
use franklin_crypto::rescue::RescueHashParams;
use franklin_crypto::rescue::{rescue_hash, rescue_mimc};
use num_bigint::BigInt;
use serde::Serialize;
use std::fs::File;

#[derive(Default, Serialize)]
struct RescueConstants {
    round_constants: Vec<Vec<String>>,
    mds_matrix: Vec<Vec<String>>,
}

fn field_to_string(elem: &Fr) -> String {
    BigInt::parse_bytes(to_hex(elem).as_bytes(), 16)
        .unwrap()
        .to_str_radix(10)
}

fn u8_to_fe(u: &u8) -> Fr {
    let mut repr = <Fr as PrimeField>::Repr::default();
    repr.as_mut()[0] = *u as u64;
    let fe = <Fr as PrimeField>::from_repr(repr).unwrap();
    fe
}

fn generate_constants(output_file: &str) {
    let params = Bn256RescueParams::new_checked_2_into_1();
    println!("capacity {:?}", params.capacity());
    println!("rate {:?}", params.rate());
    println!("num_rounds {:?}", params.num_rounds());
    println!("security_level {:?}", params.security_level());
    println!("sbox_0.inv {:?}", params.sbox_0().inv);
    println!("sbox_0.power {:?}", params.sbox_0().power);
    assert_eq!(params.capacity() + params.rate(), params.state_width());
    let mut constants = RescueConstants::default();

    for i in 0..=(2 * (params.num_rounds())) {
        constants.round_constants.push(
            params
                .round_constants(i)
                .iter()
                .map(field_to_string)
                .collect(),
        );
    }
    for i in 0..params.state_width() {
        constants.mds_matrix.push(
            params
                .mds_matrix_row(i)
                .iter()
                .map(field_to_string)
                .collect(),
        );
    }
    serde_json::to_writer_pretty(&File::create(output_file).unwrap(), &constants).unwrap();
}

fn rescue_mimc_example() {
    let params = Bn256RescueParams::new_checked_2_into_1();
    // int.from_bytes(b'fluidex', byteorder='little')
    // int.from_bytes(b'hashtest', byteorder='little')
    let input = [
        Fr::from_str("28829699159647608").unwrap(),
        Fr::from_str("7521419745152037748").unwrap(),
        Fr::from_str("2").unwrap(),
    ];
    println!("Input = {:?}", input);
    let mimc_output = rescue_mimc::<Bn256>(&params, &input);
    println!("Mimc output = {:?}", mimc_output);
    assert_eq!(
        field_to_string(&mimc_output[0]),
        "16571241020258333354093353159575087257074492169409232867884029018069038774606"
    );
}

fn rescue_hash_example() {
    let params = Bn256RescueParams::new_checked_2_into_1();
    let input: Vec<Fr> = "迟迟钟鼓初长夜，耿耿星河欲曙天。"
        .as_bytes()
        .iter()
        .map(|u| u8_to_fe(u))
        .collect();
    let hash_output = rescue_hash::<Bn256>(&params, &input);
    println!("Hash output = {:?}", hash_output);
    assert_eq!(
        field_to_string(&hash_output[0]),
        "15131965683816686492029126038145678019083347981596432597977339723207837174957"
    );
}

fn main() {
    let output_file = "constants.json";
    generate_constants(output_file);
}
