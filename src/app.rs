use std::sync::Arc;

use glam::{Quat, Vec3};
use gobs::scene::shape::Shapes;
use hecs::World;

use gobs::core::entity::{camera::Camera, instance::InstanceFlag, light::Light};
use gobs::core::geometry::vertex::VertexFlag;
use gobs::game::{app::Run, input::Input};
use gobs::scene::{
    Gfx, MaterialBuilder, Model, ModelBuilder, PipelineFlag, RenderError, Scene, Shader,
};

use crate::components::{self, Name, Orientation, Player, Position};
use crate::events::Event;
use crate::map::TileMap;
use crate::movement::Facing;
use crate::systems;

pub struct App {
    map: TileMap<Arc<Model>>,
    scene: Scene,
    light_model: Arc<Model>,
    world: World,
    events: Vec<Event>,
}

impl Run for App {
    async fn create(gfx: &Gfx) -> Self {
        let camera = Camera::perspective(
            (0., 0., 0.),
            gfx.width() as f32 / gfx.height() as f32,
            (70. as f32).to_radians(),
            0.1,
            150.,
            (0. as f32).to_radians(),
            (0. as f32).to_radians(),
            Vec3::Y,
        );

        let light = Light::new((0., 20., 0.), (1., 1., 0.9));

        let phong_shader = Self::phong_shader(gfx).await;
        let solid_shader = Self::solid_shader(gfx).await;
        let wire_shader = Self::wire_shader(gfx).await;

        let mut scene = Scene::new(gfx, camera, light, &[wire_shader]).await;
        scene.toggle_pass(crate::WIRE_PASS);

        let material = MaterialBuilder::new("diffuse")
            .diffuse_texture(crate::WALL_TEXTURE)
            .await
            .normal_texture(crate::WALL_TEXTURE_N)
            .await
            .build();

        let wall_model = ModelBuilder::new()
            .add_mesh(
                Shapes::cube(3, 2, &[5, 5, 5, 5, 6, 4]),
                Some(material.clone()),
            )
            .build(phong_shader.clone());

        let floor_model = ModelBuilder::new()
            .add_mesh(Shapes::cube(3, 2, &[4]), Some(material))
            .build(phong_shader);

        let mut map = TileMap::new();

        map.load(crate::MAP, crate::TILE_SIZE, wall_model, floor_model)
            .unwrap();

        Self::load_scene(&mut scene, &map);

        scene.camera.position = map.start.into();

        let light_model = scene
            .load_model(crate::LIGHT, None, solid_shader)
            .await
            .unwrap();

        scene.add_node(
            "light",
            scene.light.position,
            Quat::from_axis_angle(Vec3::Z, 0.),
            Vec3::splat(1.),
            light_model.clone(),
        );

        let mut world = World::new();

        world.spawn((
            Name { name: "Bob".into() },
            Player,
            components::Camera::new(),
            Position {
                x: map.start.x,
                y: map.start.y,
                z: map.start.z,
            },
            Orientation::new(Facing::North),
        ));

        App {
            map,
            scene,
            light_model,
            world,
            events: Vec::new(),
        }
    }

    fn update(&mut self, delta: f32, gfx: &Gfx) {
        systems::update(
            delta,
            &mut self.world,
            &self.events,
            &self.map,
            &mut self.scene,
        );

        let angular_speed = 10.;

        let mut light_position: Vec3 = self.scene.light.position;
        light_position =
            (Quat::from_axis_angle((0., 0., 1.).into(), (angular_speed * delta).to_radians())
                * light_position)
                .into();

        self.scene.light.update(light_position);

        for node in self.scene.layer_mut("light").nodes_mut() {
            if node.model().id == self.light_model.id {
                node.move_to_position(light_position);
            }
        }

        self.scene.update(gfx);

        self.events.clear();
    }

    fn render(&mut self, gfx: &Gfx) -> Result<(), RenderError> {
        self.scene.render(gfx)
    }

    fn input(&mut self, _gfx: &Gfx, input: Input) {
        self.events.push(Event::Input(input));
    }

    fn resize(&mut self, width: u32, height: u32, _gfx: &Gfx) {
        self.scene.resize(width, height)
    }
}

impl App {
    fn load_scene(scene: &mut Scene, map: &TileMap<Arc<Model>>) {
        for tile in &map.tiles {
            let layer = match tile.tile {
                crate::map::TileSet::WALL(_) => "wall",
                crate::map::TileSet::FLOOR(_) => "floor",
            };

            scene.add_node(
                layer,
                tile.position,
                Quat::IDENTITY,
                Vec3::ONE,
                tile.tile.model(),
            );
        }
    }

    async fn phong_shader(gfx: &Gfx) -> Arc<Shader> {
        Shader::new(
            gfx,
            "Phong",
            "phong.wgsl",
            VertexFlag::POSITION | VertexFlag::TEXTURE | VertexFlag::NORMAL,
            InstanceFlag::MODEL | InstanceFlag::NORMAL,
            PipelineFlag::CULLING | PipelineFlag::DEPTH_TEST | PipelineFlag::DEPTH_WRITE,
        )
        .await
    }

    async fn solid_shader(gfx: &Gfx) -> Arc<Shader> {
        Shader::new(
            gfx,
            "Solid",
            "solid.wgsl",
            VertexFlag::POSITION | VertexFlag::COLOR,
            InstanceFlag::MODEL,
            PipelineFlag::CULLING | PipelineFlag::DEPTH_TEST | PipelineFlag::DEPTH_WRITE,
        )
        .await
    }

    pub async fn wire_shader(gfx: &Gfx) -> Arc<Shader> {
        Shader::new(
            gfx,
            crate::WIRE_PASS,
            "wire.wgsl",
            VertexFlag::POSITION,
            InstanceFlag::MODEL,
            PipelineFlag::CULLING
                | PipelineFlag::DEPTH_TEST
                | PipelineFlag::LINE
                | PipelineFlag::ALPHA,
        )
        .await
    }
}
