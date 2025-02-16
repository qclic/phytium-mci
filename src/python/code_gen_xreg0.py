import re
def generate_code(reg_name, reg_addr):
    return f'''\
bitflags! {{
    struct {reg_name}: u32 {{
        const PULL_MASK = genmask!(9, 8);
        const DRIVE_MASK = genmask!(7, 4);
        const FUNC_MASK = genmask!(2, 0);
        const FUNC_BIT0 = 1 << 0;
        const FUNC_BIT1 = 1 << 1;
        const FUNC_BIT2 = 1 << 2;
        const DRIVE_BIT0 = 1 << 4;
        const DRIVE_BIT1 = 1 << 5;
        const DRIVE_BIT2 = 1 << 6;
        const DRIVE_BIT3 = 1 << 7;
        const PULL_BIT0 = 1 << 8;
        const PULL_BIT1 = 1 << 9;
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

impl XReg0 for {reg_name} {{

}}
'''

def extract_macro_parameters(input_str):
    # This regular expression matches the macro parameters within X_REG0!(...)
    pattern = r'X_REG0!\((\w+),\s*(\w+)\);'
    match = re.match(pattern, input_str)
    
    if match:
        reg_name = match.group(1)
        reg_addr = match.group(2)
        return reg_name, reg_addr
    else:
        raise ValueError("Input string does not match the expected format.")

if __name__ == '__main__':
    # 逐行读取./input.rs
    with open('./input.rs', 'r') as file:
        input_str = file.readline()
        while input_str:
            reg_name, reg_addr = extract_macro_parameters(input_str)
            print(generate_code(reg_name, reg_addr))
            input_str = file.readline()