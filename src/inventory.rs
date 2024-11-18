use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Write, Read};

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}
impl Product {
    pub fn new(name: &str, description: &str, price: f64, quantity: u32) -> Self {
        Product {
            name: name.to_string(),
            description: description.to_string(),
            price,
            quantity,
        }
    }
}
use std::collections::HashMap;

pub struct Inventory {
    products: HashMap<String, Product>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            products: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) -> Result<(), String> {
        if self.products.contains_key(&product.name) {
            Err(format!("Product '{}' already exists.", product.name))
        } else {
            self.products.insert(product.name.clone(), product);
            Ok(())
        }
    }

    pub fn edit_product(&mut self, name: &str, new_description: Option<&str>, new_price: Option<f64>, new_quantity: Option<u32>)-> Result<(), String>{
        if let Some(product) = self.products.get_mut(name) {
            if let Some(description) = new_description {
                product.description = description.to_string();
            }
            if let Some(price) = new_price {
                product.price = price;
            }
            if let Some(quantity) = new_quantity {
                product.quantity = quantity;
            }
            Ok(())
        } else {
            Err(format!("Product '{}' not found.", name))
        }
    }


    pub fn delete_product(&mut self, name: &str)-> Result<(), String>{
        if self.products.remove(name).is_some(){
            Ok(())
        }else{
            Err(format!("Product '{}' not found.", name))
        }

    }

    pub fn list_all_products(&self) -> Vec<&Product>{
        self.products.values().collect()
    }


    pub fn save_to_file(&self, path: &str) -> Result<(), String> {
        let data = serde_json::to_string(&self.products).map_err(|e| e.to_string())?;
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        file.write_all(data.as_bytes()).map_err(|e| e.to_string())
    }

    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut data = String::new();
        file.read_to_string(&mut data).map_err(|e| e.to_string())?;
        let products: HashMap<String, Product> = serde_json::from_str(&data).map_err(|e| e.to_string())?;
        Ok(Inventory { products })
    }
}
