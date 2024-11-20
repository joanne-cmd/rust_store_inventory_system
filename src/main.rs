mod inventory;
mod sales;

use inventory::{Inventory, Product};
use sales::Sale;
use chrono::NaiveDate;

fn main() {
    let mut inventory = Inventory::new();

    // Add multiple products
    let products = vec![
        Product::new("summer tshirt", "it is golden with a touch of white", 19.99, 100),
        Product::new("slippers", "white in color good for a beach walk", 29.99, 50),
        Product::new("hand bag", "black in color medium size", 9.99, 200),
    ];

    for product in products {
        match inventory.add_product(product) {
            Ok(_) => println!("Product added."),
            Err(err) => println!("Error: {}", err),
        }
    }

    // Edit a product
    match inventory.edit_product("slippers", Some("Updated description"), Some(30.99), Some(120)) {
        Ok(_) => println!("Product updated."),
        Err(err) => println!("Error updating product: {}", err),
    }

    // List all products
    println!("Current Inventory:");
    for product in inventory.list_all_products() {
        println!("{:?}", product);
    }

    /// Create an instance of Sale
    let sale_date = NaiveDate::from_ymd(2024, 11, 20); // Set sale date
    let mut sale = Sale::new("slippers", 2, 29.99, sale_date); // Create sale instance

    // Record sale: The inventory will be updated when this is called
    match sale.record_sale(&mut inventory, "slippers", 2, 29.99) {
        Ok(_) => println!("Sale recorded!"),
        Err(err) => println!("Error recording sale: {}", err),
    }
     // Save the sale to a file
     match sale.save_to_file("sales_records.json") {
         Ok(_) => println!("Sale record saved successfully."),
         Err(err) => println!("Error saving sale record: {}", err),
     }

    // Delete a product
    // match inventory.delete_product("summer tshirt") {
    //     Ok(_) => println!("Product deleted."),
    //     Err(err) => println!("Error deleting product: {}", err),
    // }

    // Save inventory to file
    match inventory.save_to_file("inventory.json") {
        Ok(_) => println!("Inventory saved to inventory.json"),
        Err(err) => println!("Error saving inventory: {}", err),
    }
}
