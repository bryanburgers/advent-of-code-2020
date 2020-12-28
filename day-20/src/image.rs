use super::IdentifiedTile;
use super::Tile;
use super::TileIndex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::mem::{self, MaybeUninit};

#[derive(Clone)]
pub struct Image {
    tiles: [ImageTile; 144],
}

impl Image {
    pub(crate) fn build(
        solution: HashMap<(usize, usize), TileIndex>,
        tiles: &[IdentifiedTile],
    ) -> Image {
        let mut r = Vec::with_capacity(144);

        for y in 0..12 {
            for x in 0..12 {
                if let Some(tile_idx) = solution.get(&(x, y)) {
                    let tile = tiles
                        .into_iter()
                        .find(|&&tile| tile.id == tile_idx.id)
                        .unwrap();
                    let tile = tile.tile.transform(tile_idx.transform);
                    let image_tile = ImageTile::from(tile);
                    r.push(image_tile)
                } else {
                    panic!("No tile at {},{}", x, y);
                }
            }
        }

        let mut data: [MaybeUninit<ImageTile>; 144] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..144 {
            data[i] = MaybeUninit::new(r[i]);
        }
        let tiles = unsafe { mem::transmute::<_, [ImageTile; 144]>(data) };

        Self { tiles }
    }
}

impl Image {
    pub const SIZE: usize = 12 * 8;

    pub fn get(&self, x: usize, y: usize) -> bool {
        let tile_x = x / 8;
        let tile_y = y / 8;
        let tile_idx = tile_y * 12 + tile_x;
        let tile = &self.tiles[tile_idx];
        tile.get(x % 8, y % 8)
    }

    pub fn write<T>(
        &self,
        writer: &mut T,
        monsters: &HashSet<(usize, usize)>,
    ) -> std::io::Result<()>
    where
        T: std::io::Write,
    {
        write!(
            writer,
            r##"<svg version="1.0" xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" viewBox="0 0 {size} {size}">"##,
            size = Self::SIZE
        )?;
        write!(
            writer,
            r##"  <rect x="0" y="0" width="{size}" height="{size}" fill="#000088"/>"##,
            size = Self::SIZE
        )?;
        for x in 0..Self::SIZE {
            for y in 0..Self::SIZE {
                if self.get(x, y) {
                    write!(
                        writer,
                        r##"  <rect x="{x}" y="{y}" width="1" height="1" fill="#000044"/>"##,
                        x = x,
                        y = y
                    )?;
                }
            }
        }
        for (x, y) in monsters {
            write!(
                writer,
                r##"  <rect x="{x}" y="{y}" width="1" height="1" fill="#00ff00"/>"##,
                x = x,
                y = y
            )?;
        }

        write!(writer, r##"</svg>"##)?;
        Ok(())
    }
}

#[derive(Copy, Clone)]
struct ImageTile {
    inner: u64,
}

impl From<Tile> for ImageTile {
    fn from(tile: Tile) -> ImageTile {
        let mut t = ImageTile { inner: 0 };
        for x in 0..8 {
            for y in 0..8 {
                t.set(x, y, tile.get(x + 1, y + 1));
            }
        }
        t
    }
}

impl ImageTile {
    fn idx(&self, x: usize, y: usize) -> usize {
        y * 8 + x
    }
    fn set(&mut self, x: usize, y: usize, v: bool) {
        let idx = self.idx(x, y);

        if v {
            self.inner |= 1 << idx;
        } else {
            self.inner &= !(1 << idx);
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        let idx = self.idx(x, y);
        (self.inner & (1 << idx)) > 0
    }
}
