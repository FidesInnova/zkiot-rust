use ::mongodb::bson::Document;
use client::HashType;
use lazy_static::lazy_static;
use poseidon_hash::PoseidonHash;
use records::UserRecord;
use rs_merkle::MerkleTree;
use std::sync::Mutex;

lazy_static! {
    static ref MONGODB_URI: Mutex<String> = Mutex::new(String::new());
    static ref RECORD_NUM: Mutex<usize> = Mutex::new(0);
    static ref QUERY: Mutex<Document> = Mutex::new(Document::new());
    static ref MERKLE_ORIGINAL_ROOT: Mutex<HashType> = Mutex::new([0u8; 32]);
    static ref MERKLE_PROOF_PATH: Mutex<Vec<(UserRecord, usize, Vec<HashType>)>> = Mutex::new(vec![]);
    static ref MERKLE_QUERY_RESULT: Mutex<Vec<UserRecord>> = Mutex::new(vec![]);

    static ref MERKLE_TREE: Mutex<MerkleTree<PoseidonHash>> = Mutex::new(MerkleTree::<PoseidonHash>::new());
}

pub mod client {
    pub type HashType = [u8; 32];

    use crate::{
        mongodb::{database_connection, database_save_root_hash},
        poseidon_hash::PoseidonHash,
        records::UserRecord,
        MERKLE_ORIGINAL_ROOT, MERKLE_PROOF_PATH, MERKLE_QUERY_RESULT, MERKLE_TREE, MONGODB_URI,
        QUERY, RECORD_NUM,
    };
    use anyhow::{anyhow, Ok, Result};
    use mongodb::bson::{doc, Document};
    use rs_merkle::{MerkleProof, MerkleTree};

    fn merkle_root(leaves: &Vec<HashType>) -> Result<HashType> {
        let merkle_tree = MerkleTree::<PoseidonHash>::from_leaves(&leaves);
        let merkle_root = merkle_tree
            .root()
            .ok_or(anyhow!("couldn't get the merkle root"))?;
        Ok(merkle_root)
    }

    pub fn set_uri(uri: &str) {
        *MONGODB_URI.lock().unwrap() = uri.to_string();
    }

    pub fn set_record_number(num: usize) {
        *RECORD_NUM.lock().unwrap() = num;
    }

    // set original root
    pub fn insert_random_records() -> Result<()> {
        let user_collection = database_connection::<UserRecord>("DB1", "Col1")?;
        // user_collection.insert_many((0..*RECORD_NUM.lock().unwrap()).map(|_| user_record_create()), None)?;

        let mut leaves: Vec<HashType> = vec![];
        let cursor = user_collection.find(None, None)?;

        for value in cursor {
            leaves.push(value?.hash);
        }

        let merkle_tree = MerkleTree::<PoseidonHash>::from_leaves(&leaves);
        let merkle_root = merkle_root(&leaves)?;

        *MERKLE_ORIGINAL_ROOT.lock().unwrap() = merkle_root;
        *MERKLE_TREE.lock().unwrap() = merkle_tree;

        // save to database
        database_save_root_hash(merkle_root)?;
        Ok(())
    }

    pub fn set_query(query: Document) {
        *QUERY.lock().unwrap() = query
    }

    // set the Result and Proof variables
    // returns Ok(false) when no results are found in the database
    pub fn read_query() -> Result<bool> {
        let mut query_res = vec![];
        let user_collection = database_connection::<UserRecord>("DB1", "Col1")?;
        let cursor = user_collection.find(QUERY.lock().unwrap().clone(), None)?;
        for value in cursor {
            query_res.push(value?);
        }
        if query_res.is_empty() {
            return Ok(false);
        }
        // set Result
        *MERKLE_QUERY_RESULT.lock().unwrap() = query_res.clone();

        for rec in query_res {
            let index: usize = MERKLE_TREE
                .lock()
                .unwrap()
                .leaves()
                .unwrap()
                .iter()
                .position(|h| *h == rec.hash)
                .unwrap();
            let indices_to_prove = vec![index];
            let merkle_proof = MERKLE_TREE.lock().unwrap().proof(&indices_to_prove);
            let hashes = merkle_proof.proof_hashes().to_vec();
            MERKLE_PROOF_PATH
                .lock()
                .unwrap()
                .push((rec.clone(), index, hashes));
        }

        Ok(true)
    }

    // Calculate a new root and compare it with the original root. Return true if the original root and the new root are the same
    pub fn verify_path() -> Result<bool> {
        let leaves = &MERKLE_TREE.lock().unwrap().leaves().unwrap();

        for (_, index, _) in MERKLE_PROOF_PATH.lock().unwrap().clone().into_iter() {
            let indices_to_prove = vec![index];
            let leaves_to_prove = leaves
                .get(index..index + 1)
                .ok_or("can't get leaves to prove")
                .unwrap();
            let merkle_proof = MERKLE_TREE.lock().unwrap().proof(&indices_to_prove);
            let proof_bytes = merkle_proof.to_bytes();

            let proof = MerkleProof::<PoseidonHash>::try_from(proof_bytes)?;

            if !proof.verify(
                *MERKLE_ORIGINAL_ROOT.lock().unwrap(),
                &indices_to_prove,
                leaves_to_prove,
                leaves.len(),
            ) {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

mod records {
    use crate::{client::HashType, poseidon_hash::hash_leaf};
    use rand::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct UserRecord {
        pub name: String,
        pub number: u32,
        pub hash: HashType,
    }
    impl UserRecord {
        fn new(name: String, number: u32, hash: HashType) -> Self {
            Self { name, number, hash }
        }
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct RootHash {
        pub hash: HashType,
    }

    pub fn user_record_create() -> UserRecord {
        let mut rng = rand::thread_rng();
        let name: String = (0..10).map(|_| rng.gen_range(b'a'..b'z') as char).collect();
        let number = rng.gen_range(10000..99999);
        let hash = hash_leaf(name.clone(), number);
        UserRecord::new(name, number, hash)
    }
}

mod mongodb {
    use crate::client::HashType;
    use crate::records::RootHash;
    use crate::MONGODB_URI;
    use anyhow::Result;
    use mongodb::sync::Client;
    use mongodb::sync::Collection;

    pub fn database_connection<T>(database: &str, collection: &str) -> Result<Collection<T>> {
        let uri = MONGODB_URI.lock().unwrap().clone();
        let client = Client::with_uri_str(uri)?;
        Ok(client.database(database).collection::<T>(collection))
    }

    pub fn database_save_root_hash(root: HashType) -> Result<()> {
        let root_collection = database_connection::<RootHash>("DB1", "Root")?;

        root_collection.insert_one(RootHash { hash: root }, None)?;
        Ok(())
    }
}

mod poseidon_hash {
    use ark_bn254::Fr;
    use ark_ff::{BigInteger, PrimeField};
    use light_poseidon::{Poseidon, PoseidonHasher};
    use rs_merkle::Hasher;

    #[derive(Clone, Copy)]
    pub struct PoseidonHash;
    impl Hasher for PoseidonHash {
        type Hash = HashType;

        fn hash(data: &[u8]) -> Self::Hash {
            hash_poseidon(data)
        }
    }

    use crate::client::HashType;

    pub trait AsBytes {
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

    pub fn hash_poseidon<T>(a: T) -> HashType
    where
        T: AsBytes,
    {
        let a = Fr::from_le_bytes_mod_order(&a.as_bytes());
        let mut hasher = Poseidon::<Fr>::new_circom(1).unwrap();
        let hash = hasher.hash(&[a]).unwrap();
        let mut result = [0u8; 32];
        let hash_bytes = hash.into_bigint().to_bytes_be();
        result.copy_from_slice(&hash_bytes[0..32]);
        result
    }

    pub fn hash_leaf<T1, T2>(a: T1, b: T2) -> HashType
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
}
