#![allow(non_snake_case)]
use super::render;

use render::BmpImageInfo;
use render::BmpImageErrs;
use render::BmpHeaderFuncs;
use render::BmpImageInfoFuncs;
use render::BmpHEADER;
use render::ErrInfo;
use render::BmpDIBHEADER;
use render::BmpDibHeaderFuncs;
use render::ErrInfoFuncs;
use render::BmpImageErrsFuncs;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/*
 * Implementing errors for throughout the project.
 */
 
//  ErrInfo will store the information about the current BMP image being rendered.
impl ErrInfoFuncs for ErrInfo
{
    fn set_vals(file: PathBuf, error_info: String, bmp_header: BmpHEADER, DIB_header: BmpDIBHEADER) -> Self
    {
        Self {
            err_file: PathBuf::from(file),
            err_info: error_info,
            bmp_header: bmp_header,
            bmp_dib_header: DIB_header,
        }
    }
}

// This is all of the possible errors to be thrown throughout the program.
impl BmpImageErrsFuncs for BmpImageErrs
{
    fn err_creating(file: PathBuf, error_info: String, header: BmpHEADER, dib_header: BmpDIBHEADER) -> BmpImageErrs
    {
        BmpImageErrs::ErrCreating(ErrInfo::set_vals(file, error_info, header, dib_header))
    }
    fn invalid_img_size(image_size: u32) -> BmpImageErrs
    {
        BmpImageErrs::InvalidImageSize(image_size)
    }
}

/*
 * Implementing functionality for the BMP rendering.
 */
 
//  Information about the BMP header.
impl BmpHeaderFuncs for BmpHEADER
{
    fn new_header() -> Self
    {
        Self {
            header_field: vec!['B', 'M'], // Default. All BMP files support 'B' 'M'
            bmp_file_size: 0,
            starting_addr: 0
        }
    }
    
    fn assign(&mut self, file_size: u32, starting_address: u32) -> BmpHEADER
    {
        self.bmp_file_size = file_size;
        self.starting_addr = starting_address;
        
        return self.clone();
    }
}

// Information about the DIB header.
impl BmpDibHeaderFuncs for BmpDIBHEADER
{
    fn new_dib_header(width: u32, height: u32) -> Self
    {
        Self {
            header_size: 0, // should be 40, but default to zero just in case.
            width: width,
            height: height,
            color_planes: 1, // should always be one.
            bits_per_pixel: 0,
            compression: 0, // zero in our case.
            image_size: 0,
            horizontal_res: 0,
            vertical_res: 0,
            color_palette: 0,
            important_c: 0 // All colors are important.
        }
    }
    fn assign_dib_header(&mut self, h_res: u32, v_res: u32, image_size: u32, header_size: u32, bpp: u8) -> BmpDIBHEADER
    {
        self.horizontal_res = h_res;
        self.vertical_res = v_res;
        self.image_size = image_size;
        self.header_size = header_size;
        self.bits_per_pixel = bpp;
        
        return self.clone();
    }
}
 
//  All information about the image.
impl BmpImageInfoFuncs for BmpImageInfo
{
    fn new_bmp(height: u32, width: u32) -> Self
    {
        Self {
            header: BmpHEADER::new_header(),
            dib_header: BmpDIBHEADER::new_dib_header(height, width),
            header_arr: Vec::new(),
            dib_header_arr: Vec::new(),
            pixel_array: Vec::new()
        }
    }
    
    fn configure_bmp(&mut self, pixel_array: Vec<u8>) -> Result<BmpImageInfo, BmpImageErrs>
    {
        /*
         * Using RGB, and knowing we render 4 bytes at a time:
         *  Lets say we have a 2 x 2. The pixel array would then look something like:
         *      [
         *      255, 0, 0,
         *      0, 255, 0,
         *      0, 0,-> padding to make it 8 bytes.
         *      255, 0, 0,      
         *      0, 255, 0,
         *      0, 0, -> padding to make it 16 bytes.
         *      ]
         *
         *  So the calculation of adding the height and width, multiplying it by 4
         *  should give us the accurate size of the pixel array(image size).
         */
        self.dib_header.assign_dib_header(0x002e23, 0x002e23, (self.dib_header.width * self.dib_header.height) * 4, 40, 24);
        
        // Make sure the size is divisible by 2.
        if !(self.dib_header.image_size % 2 == 0)
        {
            return Err(BmpImageErrs::invalid_img_size(self.dib_header.image_size));
        }
        
        // Knowing the full header size is 54, we can then just calculate the size of the bmp
        // file.
        self.header.assign(54 + self.dib_header.image_size, 54);
        
        self.pixel_array = pixel_array;
        
        Ok(self.clone())
    }
    
    fn write_bmp(&mut self, filename: String) -> Result<BmpImageInfo, BmpImageErrs>
    {
        let file_path: PathBuf = PathBuf::from(filename);
        
        if file_path.exists() {
            return Err(BmpImageErrs::err_creating(file_path, "File Already Exists".to_string(), self.header.clone(), self.dib_header.clone()));
        }
        
        self.header_arr = vec![
            self.header.header_field[0] as u8, self.header.header_field[1] as u8,
            self.header.bmp_file_size as u8, 0, 0, 0,
            0, 0, 0, 0,
            self.header.starting_addr as u8, 0, 0, 0
        ];
        
        self.dib_header_arr = vec![
            self.dib_header.header_size as u8, 0, 0, 0,
            self.dib_header.width as u8, 0, 0, 0,
            self.dib_header.height as u8,0, 0, 0,
            self.dib_header.color_planes as u8, 0, 
            self.dib_header.bits_per_pixel as u8, 0,
            self.dib_header.compression as u8, 0, 0, 0,
            self.dib_header.image_size as u8, 0, 0, 0,
            self.dib_header.horizontal_res as u8, 0, 0, 0, self.dib_header.vertical_res as u8, 0, 0, 0,
            self.dib_header.color_palette as u8, 0, 0, 0,
            self.dib_header.important_c as u8, 0, 0, 0,
        ];
        
        let mut f = File::create(file_path)?;
        
        f.write_all(&self.header_arr)?;
        f.write_all(&self.dib_header_arr)?;
        f.write_all(&self.pixel_array)?;
        
        Ok(self.clone())
    }
}