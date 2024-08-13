// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
pub struct Order {
    product_name: String,
    quantity: i32,
    unit_price: i32,
}

impl Order {
    pub fn new(product_name: String, quantity: i32, unit_price: i32) -> Order {
        let order = Order {
            product_name,
            quantity,
            unit_price,
        };
        order.validate();
        order
    }

    fn validate(&self) {
        if self.product_name.is_empty() {
            panic!("");
        }
        if self.product_name.len() > 300 {
            panic!("")
        }
        if self.quantity == 0 {
            panic!("");
        }
        if self.unit_price == 0 {
            panic!("");
        }
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn quantity(&self) -> &i32 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &i32 {
        &self.unit_price
    }
    pub fn total(&self) -> i32 {
        self.unit_price * self.quantity
    }

    pub fn set_product_name(&mut self, product_name: String) {
        self.product_name = product_name;
        self.validate()
    }
    pub fn set_quantity(&mut self, quantity: i32) {
        self.quantity = quantity;
        self.validate()
    }
    pub fn set_unit_price(&mut self, unit_price: i32) {
        self.unit_price = unit_price;
        self.validate()
    }
}
