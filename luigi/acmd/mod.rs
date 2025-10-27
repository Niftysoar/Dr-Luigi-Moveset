mod catch;
mod throw;
mod aerials;
mod specials;
mod tilts;
mod jabs;
mod smashes;
mod ground;
mod win;
mod other;
// mod final_smash;

pub fn install() {
    catch::install();
    throw::install();
    aerials::install();
    specials::install();
    tilts::install();
    jabs::install();
    smashes::install();
    ground::install();
    win::install();
    other::install();
    // final_smash::install();
}