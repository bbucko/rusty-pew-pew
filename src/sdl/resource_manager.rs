use sdl::sdl2::image::LoadTexture;
use sdl::sdl2::render::Texture;
use sdl::sdl2::render::TextureCreator;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

pub struct ResourceManager<'l, K, R, L>
where
    K: Hash + Eq,
    L: 'l + ResourceLoader<'l, R>,
{
    loader: &'l L,
    cache: HashMap<K, Rc<R>>,
}

impl<'l, K, R, L> ResourceManager<'l, K, R, L>
where
    K: Hash + Eq,
    L: ResourceLoader<'l, R>,
{
    pub fn new(loader: &'l L) -> Self {
        let cache = HashMap::new();
        ResourceManager { cache, loader }
    }

    pub fn preload<D>(&mut self, details: &D, filename: &D) -> Result<(), String>
    where
        L: ResourceLoader<'l, R, Args = D>,
        D: Eq + Hash + ?Sized,
        K: Borrow<D> + for<'a> From<&'a D>,
    {
        let texture = self.loader.load(filename)?;
        self.cache.insert(details.into(), Rc::new(texture));
        Ok(())
    }

    // Generics magic to allow a HashMap to use String as a key
    // while allowing it to use &str for gets
    pub fn load<D>(&mut self, details: &D) -> Result<Rc<R>, String>
    where
        L: ResourceLoader<'l, R, Args = D>,
        D: Eq + Hash + ?Sized + Debug,
        K: Borrow<D> + for<'a> From<&'a D>,
    {
        self.cache
            .get(details)
            .cloned()
            .map(Ok)
            .unwrap_or_else(|| panic!("Invalid texture ID: {:?}", details))
    }
}

// TextureCreator knows how to load Textures
impl<'l, T> ResourceLoader<'l, Texture<'l>> for TextureCreator<T> {
    type Args = str;
    fn load(&'l self, path: &str) -> Result<Texture, String> {
        println!("loading a texture from path: {:?}", path);
        self.load_texture(path)
    }
}

// Generic trait to Load any Resource Kind
pub trait ResourceLoader<'l, R> {
    type Args: ?Sized;
    fn load(&'l self, data: &Self::Args) -> Result<R, String>;
}
