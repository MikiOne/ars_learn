use ring::aead::{self, Aad, Nonce, UnboundKey, LessSafeKey};
use ring::rand::{SecureRandom, SystemRandom};

#[test]
fn test_aes_256_gcm() {
    // 生成随机密钥
    let key_bytes = [0u8; 32]; // AES-256 需要 32 字节的密钥
    let key = UnboundKey::new(&aead::AES_256_GCM, &key_bytes).unwrap();
    let less_safe_key = LessSafeKey::new(key);

    // 创建随机数生成器
    let rng = SystemRandom::new();
    let mut nonce_bytes = [0u8; 12]; // GCM 模式需要 12 字节的 nonce
    rng.fill(&mut nonce_bytes).unwrap();
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);

    // 明文数据
    let plaintext = b"Hello, world! This is a test message.";

    // 创建一个缓冲区来存储加密后的数据
    let mut in_out = plaintext.to_vec();
    println!("in_out: {:?}", in_out);

    // 加密
    let additional_data = Aad::empty(); // 附加数据（可选）
    less_safe_key.seal_in_place_append_tag(nonce, additional_data, &mut in_out).unwrap();

    // println!("Ciphertext: {:?}", in_out);

    // 解密
    let mut decrypted_data = in_out.clone();
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);
    let result = less_safe_key.open_in_place(nonce, additional_data, &mut decrypted_data).unwrap();
    println!("in_out: {:?}", result);

    // 输出解密后的明文
    println!("Decrypted: {:?}", std::str::from_utf8(result).unwrap());
}