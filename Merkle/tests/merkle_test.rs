
#[cfg(test)]
mod test {
    use merkle_lib::client::*;
    use mongodb::bson::doc;
    
    #[test]
    fn merkle_verify_path() {
        set_uri("mongodb://localhost:27017/");
        set_record_number(1000);
        insert_random_records().unwrap();
        set_query(doc! {"name": "ali"});
        read_query().unwrap();
        let result = verify_path().unwrap();
        
        assert!(result);
    } 
}