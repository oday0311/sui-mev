use fastcrypto::ed25519::Ed25519KeyPair;
use fastcrypto::traits::KeyPair;
use rand::thread_rng;
use sui_types::base_types::SuiAddress;
use sui_types::crypto::PublicKey;
use hex;  // 添加这一行

fn generate_random_keypair() -> () {
    let rng = &mut thread_rng();
    let key1 = Ed25519KeyPair::generate(rng);
    let privatekey = key1.private();

    privatekey
        .as_ref()
        .iter()
        .for_each(|byte| print!("{:02x}", byte));  // 输出私钥的十六进制表示

    // 将privatekey转换为hex字符串并输出
    }

fn main() {
    generate_random_keypair();
    println!("Hello, world!");
}
