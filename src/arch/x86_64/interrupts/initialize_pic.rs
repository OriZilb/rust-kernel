use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::Mutex;

pub(super) const PIC_1_OFFSET: u8 = 32;
pub(super) const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

lazy_static! {
    pub static ref PICS: Mutex<ChainedPics> =
        Mutex::new(unsafe {ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)});
}

pub unsafe fn init_pic() {
    let mut pics = PICS.lock();
    pics.initialize();
    pics.write_masks(0xff, 0xff); // Mask all IRQs initially
}