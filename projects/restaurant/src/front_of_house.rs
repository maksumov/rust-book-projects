// Loaded by `mod front_of_house;` in lib.rs. Child modules (hosting,
// serving) are still inline here; `mod hosting;` would similarly map
// to src/front_of_house/hosting.rs.

pub mod hosting {
    pub fn add_to_waitlist() {}

    fn seat_at_table() {}
}

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}
