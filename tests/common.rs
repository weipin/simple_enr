use hex_literal::hex;
use secp256k1::{PublicKey, SecretKey, SECP256K1};
use simple_enr::{Builder, Record, Schemev4};
use std::net::Ipv4Addr;

const EXAMPLE_RECORD_ADDRESS: &str = concat!(
    "enr:-IS4QHCYrYZbAKWCBRlAy5zzaDZXJBGkcnh4MHcBFZntXNFrdvJjX04jRzjz",
    "CBOonrkTfj499SZuOh8R33Ls8RRcy5wBgmlkgnY0gmlwhH8AAAGJc2VjcDI1Nmsx",
    "oQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8"
);

#[test]
fn build_immutable_record() {
    let private_key = SecretKey::from_slice(&hex!(
        "b71c71a67e1177ad4e901695e1b4b9ee17ae16c6668d313eac2f96dbcda3f291"
    ))
    .unwrap();
    let public_key = PublicKey::from_secret_key(SECP256K1, &private_key);
    let ip4 = Ipv4Addr::from(hex!("7f000001"));
    let udp4 = u16::from_be_bytes(hex!("765f"));
    let record = Builder::new()
        .with_ip4(ip4)
        .with_udp4(udp4)
        .sign_and_build::<Schemev4>(&private_key, &public_key)
        .unwrap();

    assert_eq!(
        record.textual_form::<Schemev4>().unwrap(),
        EXAMPLE_RECORD_ADDRESS
    );
    assert_eq!(record.ip4().unwrap(), ip4);
    assert_eq!(record.udp4().unwrap(), udp4);
}

#[test]
fn build_update_mutable_record() {
    let private_key = SecretKey::from_slice(&hex!(
        "b71c71a67e1177ad4e901695e1b4b9ee17ae16c6668d313eac2f96dbcda3f291"
    ))
    .unwrap();
    let public_key = PublicKey::from_secret_key(SECP256K1, &private_key);
    let ip4 = Ipv4Addr::from(hex!("7f000001"));
    let udp4 = u16::from_be_bytes(hex!("765f"));
    let mut publishable_record = Builder::new()
        .with_ip4(ip4)
        .with_udp4(udp4)
        .sign_and_build::<Schemev4>(&private_key, &public_key)
        .unwrap()
        .to_publishable();

    assert_eq!(publishable_record.ip4().unwrap(), ip4);
    let (seq1, textual_form1) = publishable_record.publish::<Schemev4>().unwrap();
    assert_eq!(seq1, 1);
    assert_eq!(textual_form1, EXAMPLE_RECORD_ADDRESS);

    let ip4_2 = Ipv4Addr::from(hex!("7f000002"));
    publishable_record.update_ip4(ip4_2);
    assert_eq!(publishable_record.ip4().unwrap(), ip4_2);
    let (seq2, textual_form2) = publishable_record.publish::<Schemev4>().unwrap();
    assert_eq!(seq2, 2);
    assert_ne!(textual_form1, textual_form2);
}

#[test]
fn record_from_textual() {
    let record = Record::from_textual_form::<Schemev4>(EXAMPLE_RECORD_ADDRESS).unwrap();
    let ip4 = Ipv4Addr::from(hex!("7f000001"));
    let udp4 = u16::from_be_bytes(hex!("765f"));

    assert_eq!(record.ip4().unwrap(), ip4);
    assert_eq!(record.udp4().unwrap(), udp4);
}
