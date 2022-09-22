use ethers::core::k256::ecdsa::SigningKey;
use rand_core::OsRng;

#[tokio::main]
async fn main() {
    println!("--------------------------");
    let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
    println!("signing_key: {:?}", signing_key); 

    let calculated_address = ethers::utils::secret_key_to_address(&signing_key);
    println!("calculated_address: {:?}", calculated_address);
    
    let secret_key_bytes = signing_key.to_bytes();
    println!("secret_key_sign_bytes: {:?}", secret_key_bytes);

    let secret_key_hex = hex::encode(&secret_key_bytes);
    println!("secret_key_hex   : {:?}", secret_key_hex);

    println!(".");    
    println!("--------------------------");

    for seed in 0000000001..0000100000 {
        //let seed = 4000000000;
        let seed_array = transform_u64_to_array_of_u8(seed);
        let signing_key = SigningKey::from_bytes(&seed_array).unwrap(); // Serialize with `::to_bytes()`
        let calculated_address = ethers::utils::secret_key_to_address(&signing_key);
        
        println!("seed     : {:?}", seed); 
        //println!("key      : {:?}", signing_key.to_bytes()); 
        //println!("address  : {:?}", calculated_address);
    }


    //[0, 0, 0, 0, 238, 107, 40, 0] 
    // bytes: &[u8] 
    // let num = u64::from_be_bytes(seed_bytes);
    // println!("num: {:?}", num);

}


fn transform_u64_to_array_of_u8(x:u64) -> [u8;32] {
    let b1 : u8 = ((x >> 56) & 0xff) as u8;
    let b2 : u8 = ((x >> 48) & 0xff) as u8;
    let b3 : u8 = ((x >> 40) & 0xff) as u8;
    let b4 : u8 = ((x >> 32) & 0xff) as u8;
    let b5 : u8 = ((x >> 24) & 0xff) as u8;
    let b6 : u8 = ((x >> 16) & 0xff) as u8;
    let b7 : u8 = ((x >> 8) & 0xff) as u8;
    let b8 : u8 = (x & 0xff) as u8;

    return  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, b1, b2, b3, b4, b5, b6, b7, b8]
}


//[0, 0, 0, 0, 238, 107, 40, 0]
// k256::ecdsa::SigningKey
// Initialize [`SigningKey`] from a raw scalar value (big endian).
// pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
//     let inner = SecretKey::from_bytes(bytes)
//         .map(|sk| sk.to_secret_scalar())
//         .map_err(|_| Error::new())?;

//     Ok(Self { inner })
// }