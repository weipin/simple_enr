# simple_enr

[Ethereum Node Records](https://github.com/ethereum/devp2p/blob/master/enr.md) implementation, PROOF OF CONCEPT.

# Immutable record

Records are immutable by default, meaning that once created there is no way to modify them.

## Create with builder

"Manually" creates a (immutable) record through a builder ([full sample code](https://github.com/weipin/simple_enr/blob/main/tests/common.rs#L20)):
```
let record = Builder::new()
    .with_ip4(Ipv4Addr::from(hex!("7f000001")))
    .with_udp4(u16::from_be_bytes(hex!("765f")))
    .sign_and_build::<Schemev4>(&private_key, &public_key)
    .unwrap();
```

## Create from textual form 

Creates a (immutable) record from its textual form ([full sample code](https://github.com/weipin/simple_enr/blob/main/tests/common.rs#L73)):

```
let record = Record::from_textual_form::<Schemev4>(EXAMPLE_RECORD_ADDRESS).unwrap();
```

# Mutable (publishable) record

To have a mutable record, you must create an immutable one first and then call its method `to_publishable`, 
constructing a "publishable" clone ([full sample code](https://github.com/weipin/simple_enr/blob/main/tests/common.rs#L45)):
```
let mut publishable_record = Builder::new()
        .with_ip4(ip4)
        .with_udp4(udp4)
        .sign_and_build::<Schemev4>(&private_key, &public_key)
        .unwrap()
        .to_publishable();
```

With a publishable record, you can modify its pair values and "publish" the textual form.
When you call the method `publish` and the record has any change, 
the sequence number will be increased as the spec requires ([full sample code](https://github.com/weipin/simple_enr/blob/main/tests/common.rs#L61)).

```
let (seq1, textual_form1) = publishable_record.publish::<Schemev4>().unwrap();

publishable_record.update_ip4(Ipv4Addr::from(hex!("7f000002")));
let (seq2, textual_form2) = publishable_record.publish::<Schemev4>().unwrap();
```

# Design

- Uses a [simple struct](https://github.com/weipin/simple_enr/blob/main/src/storage.rs) for record storage.
- Provides a simple [identity scheme abstraction](https://github.com/weipin/simple_enr/blob/main/src/scheme.rs) and the [v4 implementation](https://github.com/weipin/simple_enr/blob/main/src/scheme_v4.rs).
- Isolates implementations between [immutable record](https://github.com/weipin/simple_enr/blob/main/src/record.rs) and [mutable record](https://github.com/weipin/simple_enr/blob/main/src/publishable_record.rs).  
- Picks specific third-party crates for fast decoding/encoding.
