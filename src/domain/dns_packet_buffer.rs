pub struct DnsPacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
}

impl DnsPacketBuffer {
    pub fn new() -> DnsPacketBuffer {
        DnsPacketBuffer {
            buf: [0; 512],
            pos: 0,
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Step the buffer position forward a specific number of steps
    pub fn step(&mut self, steps: usize) -> Result<(), &'static str> {
        if self.pos + steps >= 512 {
            return Err("End of buffer");
        }

        self.pos += steps;

        Ok(())
    }

    /// Change the buffer position
    pub fn seek(&mut self, pos: usize) -> Result<(), &'static str> {
        if self.pos >= 512 {
            return Err("End of buffer");
        }

        self.pos = pos;

        Ok(())
    }

    /// Read a single byte and move the position one step forward
    pub fn read(&mut self) -> Result<u8, &'static str> {
        if self.pos >= 512 {
            return Err("End of buffer");
        }
        let res = self.buf[self.pos];
        self.pos += 1;

        Ok(res)
    }

    /// Get a single byte, without changing the buffer position
    pub fn get(&mut self, pos: usize) -> Result<u8, &'static str> {
        if pos >= 512 {
            return Err("End of buffer");
        }
        Ok(self.buf[pos])
    }

    /// Get a range of bytes
    pub fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8], &'static str> {
        let end = match start.checked_add(len) {
            None => return Err("End of buffer"),
            Some(result) => {
                if start + len >= 512 {
                    return Err("End of buffer");
                }
                result
            }
        };
        Ok(&self.buf[start..end])
    }

    /// Read two bytes, stepping two steps forward
    pub fn read_u16(&mut self) -> Result<u16, &'static str> {
        let res = ((self.read()? as u16) << 8) | (self.read()? as u16);

        Ok(res)
    }

    /// Read four bytes, stepping four steps forward
    pub fn read_u32(&mut self) -> Result<u32, &'static str> {
        let res = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | ((self.read()? as u32) << 0);

        Ok(res)
    }
}
