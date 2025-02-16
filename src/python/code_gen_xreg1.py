import re

def generate_code(reg_name, reg_addr):
    return f'''\
bitflags! {{
    struct {reg_name}: u32 {{
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }}
}}

impl FlagReg for {reg_name} {{
    const REG: u32 = {reg_addr};
}}

impl From<u32> for {reg_name} {{
    fn from(x: u32) -> Self {{
        Self::from_bits_truncate(x)
    }}
}}

impl Into<u32> for {reg_name} {{
    fn into(self) -> u32 {{
        self.bits()
    }}
}}

impl XReg1 for {reg_name} {{

}}
'''

def extract_macro_parameters(input_str):
    # This regular expression matches the macro parameters within X_REG0!(...)
    pattern = r'X_REG1!\((\w+),\s*(\w+)\);'
    match = re.match(pattern, input_str)
    
    if match:
        reg_name = match.group(1)
        reg_addr = match.group(2)
        return reg_name, reg_addr
    else:
        raise ValueError("Input string does not match the expected format.")

if __name__ == '__main__':
    # 逐行读取./input.rs
    with open('./input_xreg1.rs', 'r') as file:
        input_str = file.readline()
        while input_str:
            reg_name, reg_addr = extract_macro_parameters(input_str)
            print(generate_code(reg_name, reg_addr))
            input_str = file.readline()