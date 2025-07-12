use image::ImageFormat;

pub fn resize_png_image(image: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    resize_image(ImageFormat::Png, image, width, height)

}

fn resize_image(format: ImageFormat, image: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    use image::imageops::FilterType;
    use std::io::Cursor;

    let img = image::ImageReader::with_format(Cursor::new(image), format)
        .decode()
        .expect("Failed to decode image")
        .resize_exact(width, height, FilterType::Lanczos3);

    let mut buf = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)
        .expect("Failed to encode image as PNG");
    buf
}
