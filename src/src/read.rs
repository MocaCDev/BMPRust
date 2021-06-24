use std::path::PathBuf;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Gather
{
    ordered     BTreeMap<u32, Vec<u8>>
}

#[derive(Debug, Clone)]
pub struct PixelArray
{
    pixels  Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct BMPFileInfo
{
    valid_header        bool,
    valid_dib_header    bool,
    image_size          u32,
    file_size           u32,
    start_addr          u32,
    pixel_arr           pixel_arr
}