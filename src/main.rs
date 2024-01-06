mod tft;

fn main() {
    println!("It alive");
    let mut r = create_spi().unwrap();
    half_duplex(&mut r);
}
