use futures::executor::block_on;

fn main() {
    if let Err(err) = block_on(db::run()) {
        println!("Error: {:?}", err);
    }
}
