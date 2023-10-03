use iak_api::prepaid_v2::{PricelistResponse, Product};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
struct UniqueProduct {
    product_description: String,
    product_category: String,
    product_type: String,
}

fn main() -> std::io::Result<()> {
    let file = File::open("examples/data/prepaid_v2_pricelist.json")?;
    let reader = BufReader::new(file);

    let pricelist: PricelistResponse = serde_json::from_reader(reader)?;

    let mut unique_products: HashSet<UniqueProduct> = HashSet::new();

    for product in pricelist.data.unwrap().pricelist.unwrap().iter() {
        if let (product_description, Some(product_category), Some(product_type)) = (
            product.product_description.clone(),
            product.product_category.clone(),
            product.product_type.clone(),
        ) {
            let product_type = product_type.to_string();
            let product_category = product_category.to_string();
            let unique_product = UniqueProduct {
                product_description,
                product_category,
                product_type,
            };
            unique_products.insert(unique_product);
        }
    }

    let mut writer = BufWriter::new(File::create("examples/data/unique_products.csv")?);
    writeln!(writer, "product_description,product_category,product_type,product_description")?;

    for unique_product in unique_products.iter() {
        // filter only the file name of product_description
        let product_description = unique_product
            .product_description
            .split("/")
            .last()
            .unwrap()
            .to_string();
        writeln!(
            writer,
            "{},{},{},{}",
            product_description,
            unique_product.product_category,
            unique_product.product_type,
            unique_product.product_description
        )?;
    }

    Ok(())
}
