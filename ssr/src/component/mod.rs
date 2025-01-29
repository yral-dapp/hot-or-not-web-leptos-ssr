pub mod icpump;
pub mod yral; 
pub mod shared;

#[derive(Clone, Copy, PartialEq)] 
pub enum AppType {
    Yral,
    IcPump,
}