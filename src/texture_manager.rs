use std::path::Path;

pub struct TextureManager
{
    pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>
}

impl TextureManager
{
    pub fn get_texture(&self, path: String) -> sdl2::render::Texture<'_>
    {
        let temp_surface = sdl2::surface::Surface::load_bmp(Path::new(&path));
        let surface = match temp_surface {
            Ok(file) => file,
            Err(error) => {
                panic!("Problem opening a file: {:?}", error)
            },
        };
        let temp_texture = self.texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string());
        let texture = match temp_texture {
            Ok(file) => file,
            Err(error) => {
                panic!("Problem creating texutre surface: {:?}", error)
            },
        };
        return texture;
    }
}