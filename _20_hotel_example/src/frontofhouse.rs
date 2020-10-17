pub mod hosting;
pub mod serving;

pub fn add_to_waitlist(order_number: u32) {
    // Relative path
    hosting::add_to_waitlist(order_number);
}

pub fn take_order(order_number: u32) {
    // Relative path
    serving::take_order(order_number);
}

pub fn serve_order(order_number: u32) {
    // Relative path
    serving::serve_order(order_number);
}

pub fn take_payment(order_number: u32) {
    // Relative path
    serving::take_payment(order_number);
}
