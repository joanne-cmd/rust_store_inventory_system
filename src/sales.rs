use crate::inventory::Inventory;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use serde_json;
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct Sale {
    name: String,
    quantity_sold: u32,
    sale_price: f64,
    date: NaiveDate,
}

impl Sale {
    pub fn new(name: &str, quantity_sold: u32, sale_price: f64, date: NaiveDate) -> Sale {
        Sale {
            name: name.to_string(),
            quantity_sold,
            sale_price,
            date,
        }
    }

    pub fn record_sale(
        &self,
        inventory: &mut Inventory,
        name: &str,
        quantity_sold: u32,
        sale_price: f64,
    ) -> Result<(), String> {
        if let Some(product) = inventory.get_product(name) {
            if product.quantity >= quantity_sold {
                let total_sale = sale_price * quantity_sold as f64;
                let profit = (sale_price - product.price) * quantity_sold as f64;

                if let Some(product) = inventory.products.get_mut(name) {
                    product.quantity -= quantity_sold;
                }

                println!(
                    "Sale recorded for '{}': Quantity sold: {}, Total sale: ${:.2}, Profit: ${:.2}",
                    name, quantity_sold, total_sale, profit
                );

                Ok(())
            } else {
                Err(format!(
                    "Not enough stock for '{}'. Available: {}",
                    name, product.quantity
                ))
            }
        } else {
            Err(format!("Product '{}' not found in inventory.", name))
        }
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<(), String> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path);

        match file {
            Ok(mut f) => {
                let record = serde_json::to_string(&self).map_err(|e| e.to_string())?;
                writeln!(f, "{}", record).map_err(|e| e.to_string())?;
                Ok(())
            }
            Err(e) => Err(format!("Failed to open file: {}", e)),
        }
    }
}
