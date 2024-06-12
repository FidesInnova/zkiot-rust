use algorithms::Sha256;
use anyhow::{anyhow, Ok, Result};
use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use light_poseidon::{Poseidon, PoseidonHasher};
use merkle::Merkle;
use mongodb::{
    bson::doc,
    sync::{Client, Collection},
};
use rand::{thread_rng, Rng};
use rs_merkle::*;
use serde::{de::value, Deserialize, Serialize};
use std::time::{Duration, Instant};


pub type HashType = [u8; 32];
const RECORD_NUM: u32 = 10_000;
static mut TIMER_HASH: Duration = Duration::from_secs(0);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RootHash {
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

impl AsBytes for &[u8] {
    fn as_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

fn hash_poseidon<T: AsBytes>(a: T) -> HashType {
    let a = Fr::from_be_bytes_mod_order(&a.as_bytes());
    let mut hasher = Poseidon::<Fr>::new_circom(1).unwrap();
    let hash = hasher.hash(&[a]).unwrap();
    let mut result = [0u8; 32];
    let hash_bytes = hash.into_bigint().to_bytes_be();
    result.copy_from_slice(&hash_bytes[0..32]);
    result
}

fn hash_leaf<T1, T2>(a: T1, b: T2) -> HashType
where
    T1: AsBytes,
    T2: AsBytes,
{
    let a = Fr::from_be_bytes_mod_order(&a.as_bytes());
    let b = Fr::from_be_bytes_mod_order(&b.as_bytes());
    let mut hasher = Poseidon::<Fr>::new_circom(2).unwrap();
    let hash = hasher.hash(&[a, b]).unwrap();
    let mut result = [0u8; 32];
    let hash_bytes = hash.into_bigint().to_bytes_be();
    result.copy_from_slice(&hash_bytes[0..32]);
    result
}

fn user_record_create() -> UserRecord {
    let timer = Instant::now();

    let mut rng = rand::thread_rng();
    let name: String = (0..10).map(|_| rng.gen_range(b'a'..b'z') as char).collect();
    let number = rng.gen_range(10000..99999);
    let hash = hash_leaf(name.clone(), number);

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

fn database_store_root(root: RootHash) -> Result<()> {
    let root_collection = database_connection::<RootHash>("DB1", "Root")?;
    root_collection.insert_one(root, None)?;
    Ok(())
}

mod merkle {
    use rs_merkle::{MerkleProof, MerkleTree};

    use crate::{database_store_root, HashType, PoseidonHash, RootHash, UserRecord};

    pub struct Merkle {
        merkle: MerkleTree<PoseidonHash>,
        root: HashType,
        leaves: Vec<HashType>,
        data: Vec<UserRecord>,
        data_len: usize,
    }
    impl Merkle {
        pub fn new(data: Vec<UserRecord>) -> Self {
            let data_len = data.len();
            let mut leaves = vec![];
            for value in &data {
                leaves.push(value.hash);
            }
            let merkle = MerkleTree::<PoseidonHash>::from_leaves(&leaves);
            let root = merkle.root().unwrap();
            Self {
                merkle,
                root,
                leaves,
                data,
                data_len,
            }
        }

        pub fn root(&self) -> HashType {
            self.root
        }

        pub fn path_checker(&self, record: UserRecord) -> Option<(Vec<HashType>, UserRecord)> {
            let index = self.data.iter().position(|r| *r == record);
            if index.is_none() {
                return None;
            }
            let index = index.unwrap();
            let indices_to_prove = vec![index];
            let merkle_proof = self.merkle.proof(&indices_to_prove);

            Some((merkle_proof.proof_hashes().to_vec(), record))
        }

        pub fn verify(&self, record: UserRecord) -> Option<bool> {
            // TODO: Remove duplicate code
            let index = self.data.iter().position(|r| *r == record);
            if index.is_none() {
                return None;
            }
            let index = index.unwrap();

            let leaves_to_prove = self
                .leaves
                .get(index..index + 1)
                .ok_or("can't get leaves to prove")
                .unwrap();

            let indices_to_prove = vec![index];
            let leaves_to_prove = self
                .leaves
                .get(index..index + 1)
                .ok_or("can't get leaves to prove")
                .unwrap();

            let merkle_proof = self.merkle.proof(&indices_to_prove);

            let proof_bytes = merkle_proof.to_bytes();

            let proof = MerkleProof::<PoseidonHash>::try_from(proof_bytes);
            if proof.is_err() {
                return None;
            }
            Some(proof.unwrap().verify(
                self.root,
                &indices_to_prove,
                leaves_to_prove,
                self.data_len,
            ))
        }
    }
}

#[derive(Clone, Copy)]
struct PoseidonHash;
impl Hasher for PoseidonHash {
    type Hash = HashType;

    fn hash(data: &[u8]) -> Self::Hash {
        hash_poseidon(data)
    }
}

fn main() -> Result<()> {
    let user_collection = database_connection::<UserRecord>("DB1", "Col1")?;
    let timer = Instant::now();
    // insert data to database
    // user_collection.insert_many((0..RECORD_NUM).map(|_| user_record_create()), None)?;

    statistics(timer);

    // get leaves
    let mut leaves: Vec<HashType> = vec![];
    // get data
    let mut users: Vec<UserRecord> = vec![];
    let cursor = user_collection.find(None, None);

    if let std::result::Result::Ok(data) = cursor {
        for value in data {
            leaves.push(value?.hash);
            users.push(value?);
        }
    }

    // println!("{:?}", leaves);

    if leaves.is_empty() {
        return Err(anyhow!("leaves data is empty"));
    }
    if users.is_empty() {
        return Err(anyhow!("users data is empty"));
    }
    // --------------------------------------------------------------------------------------
    let merklee = Merkle::new();
    // --------------------------------------------------------------------------------------
    Ok(())
}
