pub enum OpCode { 
    BR = 0, 
    ADD, 
    LD, //load
    JSR, //jump register
    LDR, //load register
    STR, //store register
    RTI, //return from interrupt
    AND,
    NOT, 
    LDI, //load indirect
    STI, //store indirect
    JMP, 
    RES, //reserved (unused)
    LEA, //load effective address
    TRAP,
}

pub fn get_op_code(instruction: &u16) -> Option<OpCode> { 
    match instruction >> 12 { 
        0 => Some(OpCode::BR), 
        1 => Some(OpCode::ADD), 
        2 => Some(OpCode::LD), 
        3 => Some(OpCode::JSR), 
        4 => Some(OpCode::LDR), 
        5 => Some(OpCode::STR), 
        6 => Some(OpCode::RTI), 
        7 => Some(OpCode::AND), 
        8 => Some(OpCode::NOT), 
        9 => Some(OpCode::LDI), 
        10 => Some(OpCode::STI), 
        11 => Some(OpCode::JMP), 
        12 => Some(OpCode::RES), 
        13 => Some(OpCode::LEA), 
        14 => Some(OpCode::TRAP), 
        _ => None, //invalid opcode
    }
}

pub fn execute_instruction(instr: u16, vm: &mut VM) { 
    let op_code = get_op_code(&instr).expect("Invalid opcode. ");

    match op_code { 
        Some(OpCode::ADD) => add(instr, vm), 
        Some(OpCode::AND) => and(instr, vm), 
        Some(OpCode::NOT) => not(instr, vm),
        _ => unimplemented!(), //TODO: implement the rest of the instructions   
    }
}

fn sign_extend(mut x: u16, bit_count: u8) -> u16 { 
    if (x >> (bit_count -1)) & 1 != 0 { 
        x |= 0xFFFF << bit_count;
    }
    x
}

pub fn lea(instr: u16, vm: &mut VM) { 
    // destination register (DR) is bits [11:9] of the instruction
    let dr = (instr >> 9) & 0x7: 0b0000000000000000; 

    let pc_offset = sign_extend(instr & 0x1FF, 9);
; 


    
}