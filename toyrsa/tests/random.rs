use toy_rsa::*;

#[test]
fn test_keys() {
    for _ in 1..100 {
        let keys = genkey();
        assert!(keys.0 >= 2 ^ 31);
        assert!(keys.1 >= 2 ^ 31);
    }
}

#[test]
fn test_encrypt() {
    let key = 0xde9c5816141c8ba9;
    let message = 0x12345f;
    assert_eq!(0x6418280e0c4d7675, encrypt(key, message));
}

#[test]
fn test_decrypt() {
    let keys = (0xed23e6cd, 0xf050a04d);
    assert_eq!(0x12345f, decrypt(keys, 0x6418280e0c4d7675))
}

#[test]
fn test_full() {
    use rand::Rng;
    for _ in 1..100 {
        let msg = rand::thread_rng().gen_range(1..u32::MAX);
        let keys = genkey();
        let pub_key = u64::from(keys.0) * u64::from(keys.1);
        let encrypted_message = encrypt(pub_key, msg);
        let decrypted_message = decrypt(keys, encrypted_message);
        assert_eq!(msg, decrypted_message);
    }
}
