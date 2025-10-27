mod fireball;
mod catch;
mod missile;

pub fn install() {
    fireball::install();
    catch::install();
    missile::install();
}