
#[cfg(test)]
mod test {
    use merkle_lib::{client::*, poseidon_hash::hash_leaf, records::UserRecord};
    use mongodb::bson::doc;
    
    #[test]
    fn merkle_verify_path() {
        reset_all();

        set_uri("mongodb://localhost:27017/");
        set_database_name("DB1");
        let collection = set_collection_name("Col1").unwrap();
        collection.drop(None).unwrap();

        collection.insert_one(UserRecord::new("alireza0".to_string(), 12345, hash_leaf("alireza0".to_string(), 12345)), None).unwrap();
        
        set_record_number(1000);
        insert_random_records().unwrap();
        
        set_query(doc! {"name": "alireza0"});
        read_query().unwrap();
        let result = verify_path().unwrap();
        
        assert!(result);
    }

    #[test]
    #[should_panic]
    fn merkle_verify_path_failure() {
        reset_all();

        set_uri("mongodb://localhost:27017/");
        set_database_name("DB1");
        let collection = set_collection_name("Col1").unwrap();
        collection.drop(None).unwrap();

        set_record_number(10);
        insert_random_records().unwrap();

        set_query(doc! {"name": "NO_RECORD_NAME"});
        read_query().unwrap();
        let result = verify_path().unwrap();        
        assert!(result);
    }

    #[test]
    fn merkle_duplicate_inputs() {
        reset_all();

        set_uri("mongodb://localhost:27017/");
        set_database_name("DB1");
        let collection = set_collection_name("Col1").unwrap();
        collection.drop(None).unwrap();

        collection.insert_one(UserRecord::new("alireza1".to_string(), 12345, hash_leaf("alireza1".to_string(), 12345)), None).unwrap();
        collection.insert_one(UserRecord::new("alireza1".to_string(), 45668, hash_leaf("alireza1".to_string(), 45668)), None).unwrap();
        collection.insert_one(UserRecord::new("alireza1".to_string(), 55555, hash_leaf("alireza1".to_string(), 55555)), None).unwrap();
        
        set_record_number(10);
        insert_random_records().unwrap();
        
        set_query(doc! {"name": "alireza1"});
        read_query().unwrap();
        let result = verify_path().unwrap();
        
        assert!(result);
    }
}