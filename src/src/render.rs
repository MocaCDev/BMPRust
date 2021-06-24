use std::path::PathBuf;
use std::io;

#[derive(Debug, Clone)]
pub struct BmpHEADER
{
    pub header_field:    Vec<char>, // 'B', 'M'
    pub bmp_file_size:   u32,
    pub starting_addr:   u32,
}

#[derive(Debug, Clone)]
pub struct BmpDIBHEADER
{
    pub header_size:     u32,    // Normally 40. In our case, will be 40.
    pub width:           u32,
    pub height:          u32,
    pub color_planes:    u8,
    pub bits_per_pixel:  u8,
    pub compression:     u32,
    pub image_size:      u32,
    pub horizontal_res:  u32,
    pub vertical_res:    u32,
    pub color_palette:   u8,
    pub important_c:     u8,
}

#[derive(Debug, Clone)]
pub struct BmpImageInfo
{
    pub header:          BmpHEADER,
    pub dib_header:      BmpDIBHEADER,
    /* The above will be converted into 2 arrays, header and dib header. */
    pub header_arr:      Vec<u8>,
    pub dib_header_arr:  Vec<u8>,
    /* Pixel array to be written. */
    pub pixel_array:     Vec<u8>,
}

#[derive(Debug)]
pub struct ErrInfo
{
    pub err_file:        PathBuf,
    pub err_info:        String,
    /* Extra information to debug the error. */
    pub bmp_header:      BmpHEADER,
    pub bmp_dib_header:  BmpDIBHEADER,
}

#[derive(Debug)]
pub enum BmpImageErrs {
    FileError(io::Error),
    ErrCreating(ErrInfo),
    InvalidImageSize(u32), // image size should be able to be divisible by 2.
}

// Primary error for file handling.
impl From<io::Error> for BmpImageErrs
{
    fn from(err: io::Error) -> BmpImageErrs
    {
        BmpImageErrs::FileError(err)
    }
}

pub trait BmpImageInfoFuncs
{
    fn new_bmp(height: u32, width: u32) -> Self;
    fn configure_bmp(&mut self, pixel_array: Vec<u8>) -> Result<BmpImageInfo, BmpImageErrs>;
    fn write_bmp(&mut self, filename: String) -> Result<BmpImageInfo, BmpImageErrs>;
}

pub trait BmpHeaderFuncs
{
    fn new_header() -> Self;
    fn assign(&mut self, file_size: u32, starting_address: u32) ->  BmpHEADER;
}

pub trait BmpDibHeaderFuncs
{
    fn new_dib_header(width: u32, height: u32) -> Self;
    fn assign_dib_header(&mut self, h_res: u32, v_res: u32, image_size: u32, header_size: u32, bpp: u8) -> BmpDIBHEADER;
}

pub trait ErrInfoFuncs
{
    fn set_vals(file: PathBuf, error_info: String, bmp_header: BmpHEADER, DIB_header: BmpDIBHEADER) -> Self;
}

pub trait BmpImageErrsFuncs
{
    fn err_creating(file: PathBuf, error_info: String, header: BmpHEADER, dib_header: BmpDIBHEADER) -> BmpImageErrs;
    fn invalid_img_size(image_size: u32) -> BmpImageErrs;
}
