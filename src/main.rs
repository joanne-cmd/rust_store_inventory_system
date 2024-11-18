mod inventory;
use inventory::{Inventory, Product};

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

    // Edit product
    match inventory.edit_product("slippers", Some("Updated description"), Some(30.99), Some(120)) {
        Ok(_) => println!("Product updated."),
        Err(err) => println!("Error updating product: {}", err),
    }

    // List all products
    println!("Current Inventory:");
    for (product) in inventory.list_all_products() {
        println!(" {:?}", product);
    }

    // Delete a product
    match inventory.delete_product("summer tshirt") {
        Ok(_) => println!("Product deleted."),
        Err(err) => println!("Error deleting product: {}", err),
    }

    // Save inventory to file
    match inventory.save_to_file("inventory.json") {
        Ok(_) => println!("Inventory saved to inventory.json"),
        Err(err) => println!("Error saving inventory: {}", err),
    }
}
