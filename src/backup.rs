fn set_fetch(&mut self, fetch: u8) -> u8 
    {
        self.fetch = fetch;
        return self.fetch;
    }

    fn get_fetch(& self) -> u8 
    {
        return self.fetch;
    }

    fn set_temp(&mut self, temp: u16) -> u16
    {
        self.temp = temp;
        return self.temp;
    }

    fn get_temp(& self) -> u16
    {
        return self.temp;
    }

    fn set_addr_abs(&mut self, addr_abs: u16) -> u16
    {
        self.addr_abs = addr_abs;
        return self.addr_abs;
    }

    fn get_addr_abs(& self) -> u16
    {
        return self.addr_abs;
    }

    fn set_addr_rel(&mut self, addr_rel: u16) -> u16
    {
        self.addr_rel = addr_rel;
        return self.addr_rel;
    }

    fn get_addr_rel(& self) -> u16
    {
        return self.addr_rel;
    }

    fn set_opcode(&mut self, opcode: u8) -> u8
    {
        self.opcode = opcode;
        return self.opcode;
    }

    fn get_opcode(& self) -> u8
    {
        return self.opcode;
    }


    fn set_cycles(&mut self, cycles: u8) -> u8
    {
        self.cycles = cycles;
        return self.cycles;
    }

    fn get_cycles(& self) -> u8
    {
        return self.cycles
    }

    fn set_clockcount(&mut self, clock_count: u32) -> u32
    {
        self.clock_count = clock_count;
        return self.clock_count;
    }
    fn get_clockcount(& self) -> u32
    {
        return self.clock_count;
    }

    fn set_fetch(&mut self, fetch: u8) -> u8;
    fn get_fetch(& self) -> u8;

    fn set_temp(&mut self, temp: u16) -> u16;
    fn get_temp(& self) -> u16;

    fn set_addr_abs(&mut self, addr_abs: u16) -> u16;
    fn get_addr_abs(& self) -> u16;

    fn set_addr_rel(&mut self, addr_rel: u16) -> u16;
    fn get_addr_rel(& self) -> u16;

    fn set_opcode(&mut self, opcode: u8) -> u8;
    fn get_opcode(& self) -> u8;

    fn set_cycles(&mut self, cycles: u8) -> u8;
    fn get_cycles(& self) -> u8;

    fn set_clockcount(&mut self, clock_count: u32) -> u32;
    fn get_clockcount(& self) -> u32;