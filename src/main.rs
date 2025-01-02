use image::{ImageBuffer, Rgba};
use xcf::Xcf;

fn main() {
    let raw_image = Xcf::open("sample.xcf").unwrap();
    let layer0 = &raw_image.layers[0];

    let mut img = ImageBuffer::new(layer0.width, layer0.height);

    let layer_width = &layer0.width;
    
    let rgba_buf = layer0.raw_rgba_buffer();
    
    for pixel in rgba_buf.iter().enumerate() {
        let layer_height = pixel.0 as u32 / *layer_width;
        
        let rgba = Rgba([pixel.1.r(), pixel.1.g(), pixel.1.b(), pixel.1.a()]);
        img.put_pixel((pixel.0 as u32) - (layer_height * *layer_width as u32), layer_height, rgba);
    }

    img.save("output.png").expect("fuck");
}
