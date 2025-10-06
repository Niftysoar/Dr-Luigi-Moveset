use super::*;

mod acmd;
mod status;

pub static ARTICLE_INSTANCE_WORK_ID_FLOAT_ROTATION: i32 = 0;

pub fn install() {

    acmd::install();
    status::install();

}