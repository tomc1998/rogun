use common::vec::Vec2f32;
use common::color::RGBf32;

/// Tile object, describes a tile's data
#[derive(Clone, Copy)]
pub struct Tile {
  /// A tile's ID. Using a flyweight pattern in TileMap16 to refer to
  /// this.
  pub id: u16,
  pub color: RGBf32,
}

impl Tile {
  pub fn new(id: u16) -> Tile {
    Tile {id: id, color: RGBf32::new(0.0, 0.0, 0.0)}
  }
}

/// Bank of tiles, add tiles to the lib_state's TileBank object to register them for use in the tile map's flyweight pattern.
pub struct TileBank {
  tiles: Vec<Tile>,
}

impl TileBank {
  pub fn new() -> TileBank {
    TileBank { tiles: Vec::new() }
  }

  /// Inserts a tile into the tile bank. Maintains sorted order. 
  /// O(log(n)).
  pub fn register_tile(&mut self, tile: Tile) {
    if self.tiles.len() == 0 {
      self.tiles.push(tile);
      return;
    }
    let (mut l_bound, mut u_bound) = (0, self.tiles.len() - 1);
    let mut target;
    loop {
      if u_bound - l_bound <= 1 {
        if u_bound == self.tiles.len()-1 && 
          self.tiles[u_bound].id < tile.id { // End of the tiles?
            self.tiles.push(tile);
          }
        else if l_bound == 0 &&
          self.tiles[l_bound].id > tile.id { // Start of tiles?
            self.tiles.insert(0, tile);
          }
        else { // In between u_bound and l_bound
          self.tiles.insert(u_bound, tile);
        }
        break;
      }
      target = (u_bound + l_bound) / 2;
      if self.tiles[target].id > tile.id {
        u_bound = target;
      }
      else if self.tiles[target].id < tile.id {
        l_bound = target;
      }
      else { // Same tile ID, replace the tile at this index
        self.tiles[target] = tile;
        return;
      }
    }
  }

  /// Binary searches to find index of a tile in the bank. O(log(n)).
  pub fn get_tile_index(&self, id: u16) -> Option<usize> {
    if self.tiles.len() == 0 {
      return None;
    }
    let (mut l_bound, mut u_bound) = (0, self.tiles.len() - 1);
    let mut target : usize;
    loop {
      // Check if we've searched everything...
      if u_bound - l_bound <= 1 
        && self.tiles[u_bound].id != id 
          && self.tiles[l_bound].id != id {
            return None;
          }

      target = (u_bound + l_bound) / 2;
      if self.tiles[target].id == id {
        return Some(target);
      }
      if self.tiles[target].id > id {
        u_bound = target;
      }
      else {
        l_bound = target + 1;
      }
    }
  }

  pub fn get_tile(&self, id: u16) -> Option<Tile> {
    let index = self.get_tile_index(id);
    if index.is_none() { return None; }
    else {
      return Some(self.tiles[index.unwrap()]);
    }
  }
}

/// 16 x 16 tile map
pub struct TileMap16 {
  /// World position of tile map
  pub world_pos: Vec2f32,

  /// Width / height of tile in world coords
  pub tile_size: f32,

  /// Flyweight array of tiles
  pub tiles: [u16; 16*16],
}

impl TileMap16 {
  pub fn new() -> TileMap16 {
    TileMap16 { 
      world_pos: Vec2f32(0.0, 0.0),
      tile_size: 16.0,
      tiles: [0; 16*16] 
    }
  }
}
