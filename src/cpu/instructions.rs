fn opcode_handler(opcode : u8) -> u16 {

    match opcode {
        0x00 => 1, //NOP
        0x01 => 1, //ld r16, imm16
        0x02 => 1,
        0x03 => 1,
        0x04 => 1,
        0x05 => 1,
        0x06 => 1,
        0x07 => 1,
        0x08 => 1,
        0x09 => 1,
        0x0A => 1,
        0x0B => 1,
        0x0C => 1,
        0x0D => 1,
        0x0E => 1,
        0x0F => 1,
        _    => eprintln!("ERROR: INVALID OPCODE"),
    }
    // return 10
}

