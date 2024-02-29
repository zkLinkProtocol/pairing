use ff::{Field, PrimeField, PrimeFieldRepr};

cfg_if::cfg_if! {
    if #[cfg(feature = "asm")] {
        use core::arch::asm;
        use ff::PrimeFieldAsm;
        
        #[derive(PrimeFieldAsm)]
        #[PrimeFieldModulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
        #[PrimeFieldGenerator = "7"]
        #[UseADX = "true"]
        pub struct Fr(FrRepr);
    } else {
        #[derive(PrimeField)]
        #[PrimeFieldModulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
        #[PrimeFieldGenerator = "7"]
        pub struct Fr(FrRepr);
    }
}

#[test]
fn test_to_hex() {
    use ff::to_hex;
    assert_eq!(to_hex(&Fr::one()), "0000000000000000000000000000000000000000000000000000000000000001");
}

#[test]
fn test_fr_from_hex() {
    use ff::from_hex;
    let fr: Fr = from_hex("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    assert_eq!(fr, Fr::one());

    let fr: Fr = from_hex("0x0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    assert_eq!(fr, Fr::one());

    let fr: Fr = from_hex("0x01").unwrap();
    assert_eq!(fr, Fr::one());

    let fr: Fr = from_hex("0x00").unwrap();
    assert_eq!(fr, Fr::zero());

    let fr: Fr = from_hex("00").unwrap();
    assert_eq!(fr, Fr::zero());
}

#[test]
fn test_roots_of_unity() {
    assert_eq!(Fr::S, 28);
}

#[test]
fn test_default() {
    assert_eq!(Fr::default(), Fr::zero());
}
