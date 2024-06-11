use std::time::{Duration, Instant};
use algorithms::Sha256;
use rs_merkle::*;
use anyhow::{Ok, Result};
use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use light_poseidon::{Poseidon, PoseidonHasher};
use mongodb::{bson::doc, sync::{Client, Collection}};
use rand::{thread_rng, Rng};
use serde::{de::value, Deserialize, Serialize};


pub type HashType = [u8; 32];
const RECORD_NUM: u32 = 10_000;
static mut TIMER_HASH: Duration = Duration::from_secs(0);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserRecord {
    name: String,
    number: u32,
    hash: HashType,
}
impl UserRecord {
    fn new(name: String, number: u32, hash: HashType) -> Self {
        Self { name, number, hash }
    }
}

struct HashNode {
    level: Vec<u32>,
    hash: HashType,
}

fn database_connection<T>(database: &str, collection: &str) -> Result<Collection<T>> {
    let uri = "mongodb://localhost:27017/";
    let client = Client::with_uri_str(uri)?;
    Ok(client.database(database).collection::<T>(collection))
}

trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

impl AsBytes for String {
    fn as_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl AsBytes for u32 {
    fn as_bytes(&self) -> Vec<u8> {
        self.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect()
    }
}

impl AsBytes for HashType {
    fn as_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

fn hash_gen<T1, T2>(a: T1, b: T2) -> Result<HashType>
where
    T1: AsBytes,
    T2: AsBytes,
{
    let input_name = Fr::from_be_bytes_mod_order(&a.as_bytes());
    let input_number = Fr::from_be_bytes_mod_order(&b.as_bytes());

    let mut hasher = Poseidon::<Fr>::new_circom(2)?;
    let hash = hasher.hash(&[input_name, input_number])?;
    let mut result = [0u8; 32];
    let hash_bytes = hash.into_bigint().to_bytes_be();
    result.copy_from_slice(&hash_bytes[0..32]);
    Ok(result)
}

fn user_record_create() -> UserRecord {
    let timer = Instant::now();

    let mut rng = rand::thread_rng();
    let name: String = (0..10).map(|_| rng.gen_range(b'a'..b'z') as char).collect();
    let number = rng.gen_range(10000..99999);
    let hash = hash_gen(name.clone(), number).unwrap();

    unsafe { TIMER_HASH += timer.elapsed() }

    UserRecord::new(name, number, hash)
}

fn statistics(timer: Instant) {
    let all = timer.elapsed();
    println!(
        "all: {:?}\nGen_users: {:?}\nInsert: {:?}",
        all,
        unsafe { TIMER_HASH },
        unsafe { all - TIMER_HASH }
    );
}

fn main() -> Result<()> {
    let user_collection = database_connection::<UserRecord>("DB1","Col1")?;
    let timer = Instant::now();
    // insert data to database 
    user_collection.insert_many((0..RECORD_NUM).map(|_| user_record_create()), None)?;
    
    statistics(timer);

    // get leaves 
    let mut leaves: Vec<HashType> = vec![];
    let cursor = user_collection.find(None, None);

    if let std::result::Result::Ok(data) = cursor {
        for value in data {
            leaves.push(value?.hash);
        }
    }

    // println!("{:?}", leaves);


    // ----- = rs_merkle lib test = ---------------------------------------------------------
    let leaf_values = ["a", "b", "c", "d", "e", "f"];
    let leaves: Vec<HashType> = leaf_values
        .iter()
        .map(|x| hash_gen(x.to_string(), x.to_string()).unwrap())
        .collect::<Vec<HashType>>();

    // create merkel tree and get root
    // TODO: implement for Poseidon
    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
    
    // println!("{:#?}", merkle_tree);

    let merkle_root = merkle_tree.root().ok_or("couldn't get the merkle root").unwrap();
    


    // item to prove
    // why take a vector? 
    let indices_to_prove = vec![3];
    let leaves_to_prove = leaves.get(3..4).ok_or("can't get leaves to prove").unwrap();


    let merkle_proof = merkle_tree.proof(&indices_to_prove);
    
    // serialize proof to pass it to the client
    let proof_bytes = merkle_proof.to_bytes();

    // parse proof back on the client
    let proof = MerkleProof::<algorithms::Sha256>::try_from(proof_bytes)?;

    println!("Data: {:?}, Hash:{:?}\nProof result:", leaf_values.get(3..4), leaves.get(3..4));
    if proof.verify(merkle_root, &indices_to_prove, leaves_to_prove, leaves.len()) {
        println!("Valid");
    } else {
        println!("Invalid")
    }
    // --------------------------------------------------------------------------------------
    Ok(())
}