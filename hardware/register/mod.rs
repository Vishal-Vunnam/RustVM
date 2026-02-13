pub struct Registers {
    pub r0: u16, 
    pub r1: u16, 
    pub r2: u16, 
    pub r3: u16, 
    pub r4: u16, 
    pub r5: u16, 
    pub r7: u16, 
    pub pc: u16, 
    pub cond: u16, 
} 




pub struct X86Registers { 
    pub rax: u64, 
    pub rbx: u64, 
    pub rcx: u64, 
    pub rdx: u64, 
    pub rsi: u64, 
    pub rdi: u64, 
    pub rsp: u64, 
    pub rbp: u64, 
    pub rip: u64, 
}

impl X86Registers { 
    pub fn new() -> X86Registers { 
        X86Registers { 
            rax: 0, 
            rbx: 0, 
            rcx: 0, 
            rdx: 0, 
            rsi: 0, 
            rdi: 0, 
            rsp: 0, 
            rbp: 0, 
            rip: 0,
        } 
    } 

    pub fn update(&mut self, index: u16, value: u64) { 
        match index { 
            0 => self.rax = value, 
            1 => self.rbx = value, 
            2 => self.rcx = value, 
            3 => self.rdx = value, 
            4 => self.rsi = value, 
            5 => self.rdi = value, 
            6 => self.rsp = value, 
            7 => self.rbp = value, 
            8 => self.rip = value, 
            _ => panic!("Index out of bound. "),
        } 
    }

    pub fn get(&self, index: u16) -> u64 { 
        match index { 
            0 => self.rax, 
            1 => self.rbx, 
            2 => self.rcx, 
            3 => self.rdx, 
            4 => self.rsi, 
            5 => self.rdi, 
            6 => self.rsp, 
            7 => self.rbp, 
            8 => self.rip, 
            _ => panic!("Index out of bound. "),
        } 
    }

    pub fn get_eax(&mut self) -> u32 { 
        self.rax as u32
    }

    pub fn set_eax(&mut self, value: u32) { 
        self.rax = value as u64;
    }

    pub fn get_ebx(&mut self) -> u32 { 
        self.rbx as u32
    }

    pub fn set_ebx(&mut self, value: u32) { 
        self.rbx = value as u64;
    }

    pub fn get_ecx(&mut self) -> u32 { 
        self.rcx as u32
    }

    pub fn set_ecx(&mut self, value: u32) { 
        self.rcx = value as u64;
    }


    pub fn get_edx(&mut self) -> u32 { 
        self.rdx as u32
    }

    pub fn set_edx(&mut self, value: u32) { 
        self.rdx = value as u64;
    }

    pub fn get_esi(&mut self) -> u32 { 
        self.rsi as u32
    }

    pub fn set_esi(&mut self, value: u32) { 
        self.rsi = value as u64;
    } 


}

impl Registers { 
    pub fn new() -> Registers { 
        Registers { 
            r0: 0, 
            r1: 0, 
            r2: 0, 
            r3: 0, 
            r4: 0, 
            r5: 0, 
            r5: 0, 
            r6: 0, 
            r7: 0, 
            pc: PC_START, 
            cond: 0,

        } 
    } 

    pub fn update(&mut self, index: u16, value: u16) {
        match index {
            0 => self.r0 = value,
            1 => self.r1 = value,
            2 => self.r2 = value,
            3 => self.r3 = value,
            4 => self.r4 = value,
            5 => self.r5 = value,
            6 => self.r6 = value,
            7 => self.r7 = value,
            8 => self.pc = value,
            9 => self.cond = value,
            _ => panic!("Index out of bound"),
        }
    }

    pub fn get(&self, index: u16) -> u16 {
        match index 
            0 => self.r0,
            1 => self.r1,
            2 => self.r2,
            3 => self.r3,
            4 => self.r4,
            5 => self.r5,
            6 => self.r6,
            7 => self.r7,
            8 => self.pc,
            9 => self.cond,
            _ => panic!("Index out of bound. "),
        }
    }

    enum ConditionFlag { 
        POS = 1 << 0, 
        ZRO = 1 << 1, 
        NEG = 1 << 2,
    } 

    pub fn update_r_cond_register(&mut self, r: u16) { 
        if self.get(r) == 0 { 
            self.update(9, ConditionFlag::ZRO as u16);
        } else if self.get(r) >> 15 == 1 { 
            self.update(9, ConditionFlag::NEG as u16);
        } else { 
            self.update(9, ConditionFlag::POS as u16);
        }
    }
} 
