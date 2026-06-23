mod restaurant {
    // Public module for front-of-house operation
    pub mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {
                println!("Added to wait list....");
            }
            pub fn seat_at_table() {
                println!("Seated at the table");
            }
        }

        pub mod serving {
            pub fn take_order(){
                println!("Order is taken..");
                super::super::back_of_house::prepare_food();
            }
            pub fn serve_order(){
                super::super::back_of_house::wash_dishes();
                println!("Order served..");
            }
            pub fn take_payment(){
                println!("Payment processed");
            }
        }
    }

    mod back_of_house{
        pub fn prepare_food(){
            println!("Food is prepared..");
        }
        pub fn wash_dishes(){
            println!("Wash dishes before serving..");
        }
    }
}

fn main() {
    // Using functions in the public modules of the restaurant
    restaurant::front_of_house::hosting::add_to_waitlist();
    restaurant::front_of_house::hosting::seat_at_table();
    restaurant::front_of_house::serving::take_order();
    restaurant::front_of_house::serving::serve_order();
    restaurant::front_of_house::serving::take_payment();

    // The following lines will cause compile errors if uncommented because they are private
    // restaurant::back_of_house::prepare_food();
    // restaurant::back_of_house::wash_dishes();
}