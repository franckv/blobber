use anyhow::Result;
use glam::Vec3;
use log::*;

pub enum TileSet<M> {
    WALL(M),
    FLOOR(M),
}

impl<M: Copy> TileSet<M> {
    pub fn model(&self) -> M {
        match self {
            TileSet::WALL(m) => *m,
            TileSet::FLOOR(m) => *m,
        }
    }
}

pub struct Tile<M> {
    pub position: Vec3,
    pub tile: TileSet<M>,
}

pub struct TileMap<M> {
    pub tiles: Vec<Tile<M>>,
    pub start: Vec3,
}

impl<M: Copy> TileMap<M> {
    pub fn new() -> Self {
        TileMap {
            tiles: Vec::new(),
            start: Vec3::new(0., 0., 0.),
        }
    }

    pub fn add_tile(&mut self, tile: TileSet<M>, position: Vec3) {
        let tile = Tile { position, tile };

        self.tiles.push(tile);
    }

    pub fn collides(&self, position: Vec3) -> bool {
        self.tiles.iter().find(|t| t.position == position).is_some()
    }

    pub fn load(&mut self, data: &str, tile_size: f32, wall_id: M, floor_id: M) -> Result<()> {
        info!("Load scene");

        let offset = 16.;

        let (mut i, mut j) = (0., 0.);

        let (mut pos_x, pos_y, mut pos_z) = (0., 0., 0.);

        for c in data.chars() {
            match c {
                'w' => {
                    i += tile_size;
                    let mut position = Vec3 {
                        x: i - offset,
                        y: 0.,
                        z: j - offset,
                    };

                    self.add_tile(TileSet::WALL(wall_id), position);
                    position.y = -crate::TILE_SIZE;
                    self.add_tile(TileSet::FLOOR(floor_id), position);
                }
                '@' => {
                    i += crate::TILE_SIZE;
                    let position = Vec3 {
                        x: i - offset,
                        y: -crate::TILE_SIZE,
                        z: j - offset,
                    };
                    (pos_x, pos_z) = (position.x, position.z);
                    self.add_tile(TileSet::FLOOR(floor_id), position);
                }
                '.' => {
                    i += crate::TILE_SIZE;
                    let position = Vec3 {
                        x: i - offset,
                        y: -crate::TILE_SIZE,
                        z: j - offset,
                    };
                    self.add_tile(TileSet::FLOOR(floor_id), position);
                }
                '\n' => {
                    j += crate::TILE_SIZE;
                    i = 0.;
                }
                _ => (),
            }
        }

        self.start = Vec3::new(pos_x, pos_y, pos_z);

        Ok(())
    }
}
