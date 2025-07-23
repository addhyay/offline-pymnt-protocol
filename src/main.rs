use offline_pymnt_protocol::setup;

fn main() {
    println!("Initiating...");
    let (pk, m_public_key, local_address) = setup();
}
