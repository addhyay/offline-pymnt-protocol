use offline_pymnt_protocol::setup;

fn main() {
    println!("Initiating...");
    let (pk, m_public_key) = setup();

    println!("mpk = {:?} and pk = {:?}", m_public_key, pk);
}
