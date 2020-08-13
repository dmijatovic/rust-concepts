
mod transaction;


fn error_handling_match(){
    let res = transaction::get_transactions();

    match res{
        Ok(r)=>{println!("Transactions 1 {:?}", r)},
        Err(e)=>{
            println!("Failed to get transactions. Error: {}", e);
        }
    };
}

fn error_handling_combinators(){

    let res = transaction::get_transactions_combinators();

    if res.is_ok() {
        println!("Transactions 2 {:?}", res.unwrap());
    } else {
        println!("Failed to get transactions. Error: {}", res.unwrap_err());
    }

}

fn error_handling_custom_error(){
    match transaction::get_transactions_custom(){
        Ok(d)=>{println!("Transactions 3 {:?}", d)},
        Err(e)=>{
            println!("Failed to get transactions. Error: {}", e.to_string());
        }
    }
}


fn main() {
    println!("Hello, world!");

    error_handling_match();

    error_handling_combinators();

    error_handling_custom_error();

}
