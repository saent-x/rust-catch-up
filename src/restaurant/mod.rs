pub mod pizza_order {
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub toppings: String,
    }

    impl Pizza {
        pub fn make_pizza(dough: String, cheese: String, toppings: String) -> Pizza {
            Pizza {
                dough, cheese, toppings
            }
        }
    }

    pub mod help_customer{
        fn seat_at_table(){
            println!("customer seated at table")
        }

        pub fn take_order(){
            seat_at_table();

            let cus_pizza: super::Pizza =
                super::Pizza::make_pizza( String::from("regular"), String::from("mozzarella"), String::from("beef;chicken;mushrooms"));

            serve_customer(cus_pizza);
        }

        fn serve_customer(pizza: super::Pizza){
            println!("customer is served a pizza with [dough: {}, cheese: {}, toppings: {}]", pizza.dough, pizza.cheese, pizza.toppings);
        }
    }
}

pub fn order_food(){
    pizza_order::help_customer::take_order()
}