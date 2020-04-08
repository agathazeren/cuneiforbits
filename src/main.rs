mod job;
mod orbit;
mod rocket;
mod sats;

mod units {
    pub struct Mass(u64); //kg
    pub struct Isp(u64); //s
    pub struct Volume(u64); //L
    pub struct Preasure(u64); //Pa
}
fn main() {
    println!("Hello, world!");
}
