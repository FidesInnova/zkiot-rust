
#[cfg(test)]
mod test {
    use merkle_lib::client::*;
    use mongodb::bson::doc;
    
    #[test]
    fn merkle_verify_path() {
        set_uri("mongodb://localhost:27017/");
        set_database_name("DB1");
        set_collection_name("Col1");
        set_record_number(1000);
        insert_random_records().unwrap();
        set_query(doc! {"name": "hvshnmbmeb"});
        read_query().unwrap();
        let result = verify_path().unwrap();
        
        assert!(result);
    } 
}