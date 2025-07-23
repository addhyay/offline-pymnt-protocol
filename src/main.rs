use offline_pymnt_protocol::setup;

fn main() {
    println!("Initiating...");
    let (_pk, _mpublic_key, _local_address) = setup();
    println!("Treasury-address : {}", _local_address);
}
