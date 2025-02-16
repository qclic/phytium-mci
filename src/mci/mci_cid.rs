#[derive(Debug, Default)]
pub struct MCICid {
    pub manufacturer_id: u8,
    pub application_id: u16,
    pub product_name: [u8; 5],
    pub product_version: u8,
    pub serial_number: u32,
    pub manufacturing_data: u16,
}
