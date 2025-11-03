pub unsafe fn init_pic() {
    let mut master_control = Port::new(MASTER_CONTROL);
    let mut master_data = Port::new(MASTER_DATA);
    let mut slave_control = Port::new(SLAVE_CONTROL);
    let mut slave_data = Port::new(SLAVE_CONTROL);

    // Start initialization (ICW1)
    master_control.write(0x11u8);
    slave_control.write(0x11u8);

    // Set vector offsets (ICW2)
    master_data.write(MASTER_CONTROL as u8); // Master offset
    slave_data.write(SLAVE_CONTROL as u8); // Slave offset

    // Tell master about slave (ICW3)
    master_data.write(0x04u8); // IRQ2 has slave
    slave_data.write(0x02u8); // Slave identity

    // ICW4: 8086 mode
    master_data.write(0x01u8);
    slave_data.write(0x01u8);

    // Mask all IRQs (just in case)
    master_data.write(0x0u8);
    slave_data.write(0x0u8);
}
