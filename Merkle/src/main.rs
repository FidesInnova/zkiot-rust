use anyhow::{anyhow, Result};
use merkle_lib::client::*;
use mongodb::bson::doc;


fn main() -> Result<()> {
    set_uri("mongodb://localhost:27017/");
    set_record_number(1000);
    insert_random_records().unwrap();
    set_query(doc! {"name": "jrclkilwhw"});
    read_query().unwrap();
    let result = verify_path();

    println!("{result:?}");
    Ok(())
}
