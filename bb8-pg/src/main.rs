
mod pg_db;
mod todo_list;
mod todo_item;

fn main() {
    println!("Hello, world!");

    let ok = pg_db::main();
    if ok.is_err(){
        println!("pg_db.main failed with error");
    }else{
        println!("pg_db.main completed");
    }

}
