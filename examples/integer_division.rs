use clap::Parser;
use halo2_base::gates::RangeChip;
use halo2_base::safe_types::RangeInstructions;
use halo2_base::utils::ScalarField;
use halo2_base::AssignedValue;
#[allow(unused_imports)]
use halo2_base::{
    Context,
    QuantumCell::{Constant, Existing, Witness},
};
use halo2_scaffold::scaffold::cmd::Cli;
use halo2_scaffold::scaffold::run;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::env::var;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitInput {
    pub x: String, // field element, but easier to deserialize as a string
}

// public inputs:
// * A non-negative integer x, which is guaranteed to be at most 16-bits

// public outputs:
// * The non-negative integer (x / 32), where "/" represents integer division.
fn integer_division<F: ScalarField>(
    ctx: &mut Context<F>,
    input: CircuitInput,
    make_public: &mut Vec<AssignedValue<F>>,
) {
    let x = F::from_str_vartime(&input.x).expect("deserialize field element should not fail");

    let lookup_bits =
        var("LOOKUP_BITS").unwrap_or_else(|_| panic!("LOOKUP_BITS not set")).parse().unwrap();
    let x = ctx.load_witness(x);

    make_public.push(x);

    let range = RangeChip::default(lookup_bits);

    let (div, rem) = range.div_mod(ctx, x, BigUint::from(32 as u32), 16);

    make_public.push(div);

    println!("x: {:?}", x.value());
    println!("div value: {:?}", div.value());
    assert_eq!(*div.value() * F::from(32) + *rem.value(), *x.value());
}

fn main() {
    env_logger::init();

    let args = Cli::parse();

    run(integer_division, args);
}
