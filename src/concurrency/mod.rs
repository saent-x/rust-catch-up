use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

pub fn threads(){
    let thread_1 = thread::spawn(|| {
       for i in 1..50{
           println!("spawned thread: {}", i);
           thread::sleep(Duration::from_millis(1));
       }
    });

    for i in 1..30{
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    thread_1.join().unwrap();
}

pub fn bank_ref_example(){
    pub struct Bank{
        balance: f32
    }

    fn withdraw(the_bank: &mut Bank, amt: f32){
        the_bank.balance -= amt;
    }

    let mut my_bank = Bank{balance: 100_000.00};

    withdraw(&mut my_bank, 2000.0);

    println!("balance: {}", my_bank.balance);

    fn customer_withdraw(the_bank: &mut Bank){
        withdraw(the_bank, 1000.0);
    }

    // thread::spawn(|| {
    //     customer_withdraw(&mut my_bank); // shared variable possible failure condition
    // }).join().unwrap();

    // fix
    fn withdraw_2(my_bank: &Arc<Mutex<Bank>>, amt: f32){
        // get bank ref
        let mut bank_ref = my_bank.lock().unwrap();

        if bank_ref.balance < 1000.0{
            println!("current balance too low!")
        }else {
            bank_ref.balance -= 1000.0;
            println!("withdrawal successful: ${}", bank_ref.balance);
        }
    }

    fn customer_2(my_bank: Arc<Mutex<Bank>>){
        withdraw_2(&my_bank, 1000.0);
    }

    let new_bank = Arc::new(Mutex::new(Bank {balance: 20_000.0}));

    let bank_handles = (0..10).map(|_| {
        let bank_rf = new_bank.clone();

        thread::spawn(|| {
            customer_2(bank_rf);
        })
    });

    for handles in bank_handles {
        handles.join().unwrap();
    }

    println!("total balance: {}", new_bank.lock().unwrap().balance);

}