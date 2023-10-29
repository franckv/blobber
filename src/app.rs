use std::sync::Arc;

use glam::{Quat, Vec3};
use hecs::World;

use crate::components::{self, Name, Orientation, Player, Position};
use crate::controller::CameraController;
use crate::events::Event;
use crate::map::TileMap;
use crate::movement::Facing;
use crate::systems;
use gobs_game as game;
use gobs_scene as scene;

use game::{app::Run, input::Input};
use scene::scene::Scene;
use scene::{
    camera::{Camera, CameraProjection},
    RenderError,
};
use scene::{light::Light, MaterialBuilder, ModelBuilder};
use scene::{Gfx, Model};

pub struct App {
    camera_controller: CameraController,
    map: TileMap<Arc<Model>>,
    scene: Scene,
    light_model: Arc<Model>,
    world: World,
    events: Vec<Event>,
}

impl Run for App {
    async fn create(gfx: &mut Gfx) -> Self {
        let camera = Camera::new(
            (0., 0., 0.),
            CameraProjection::new(
                gfx.width(),
                gfx.height(),
                (70. as f32).to_radians(),
                0.1,
                150.,
            ),
            (0. as f32).to_radians(),
            (0. as f32).to_radians(),
        );

        let light = Light::new((0., 20., 0.), (1., 1., 0.9));
        let light_position = light.position;

        let mut scene = Scene::new(gfx, camera, light).await;

        let material = MaterialBuilder::new("diffuse")
            .diffuse_texture(gfx, crate::WALL_TEXTURE)
            .await
            .normal_texture(gfx, crate::WALL_TEXTURE_N)
            .await
            .build(gfx, &scene.phong_shader);

        let wall_model = ModelBuilder::new()
            .add_mesh(
                scene::shape::Shapes::cube(
                    gfx,
                    scene.phong_shader.vertex_flags(),
                    3,
                    2,
                    &[5, 5, 5, 5, 6, 4],
                ),
                Some(material.clone()),
            )
            .build();

        let floor_model = ModelBuilder::new()
            .add_mesh(
                scene::shape::Shapes::cube(gfx, scene.phong_shader.vertex_flags(), 3, 2, &[4]),
                Some(material),
            )
            .build();

        let mut map = TileMap::new();

        map.load(crate::MAP, crate::TILE_SIZE, wall_model, floor_model)
            .unwrap();

        Self::load_scene(&mut scene, &map);

        scene.camera.position = map.start.into();

        let light_model = scene
            .load_model(gfx, crate::LIGHT, scene.solid_shader.clone(), 0.3)
            .await
            .unwrap();

        scene.add_node(
            light_position,
            Quat::from_axis_angle(Vec3::Z, 0.),
            light_model.clone(),
            scene.solid_shader.clone(),
        );

        let camera_controller = CameraController::new(0.7);

        let mut world = World::new();

        world.spawn((
            Name { name: "Bob".into() },
            Player,
            components::Camera,
            Position {
                x: map.start.x,
                y: map.start.y,
                z: map.start.z,
            },
            Orientation::new(Facing::North),
        ));

        App {
            camera_controller,
            map,
            scene,
            light_model,
            world,
            events: Vec::new(),
        }
    }

    fn update(&mut self, delta: f32, gfx: &mut Gfx) {
        systems::update(&mut self.world, &self.events, &self.map, &mut self.scene);

        let angular_speed = 10.;

        self.camera_controller
            .update_camera(&mut self.scene.camera, delta);

        let mut light_position: Vec3 = self.scene.light.position;
        light_position =
            (Quat::from_axis_angle((0., 0., 1.).into(), (angular_speed * delta).to_radians())
                * light_position)
                .into();

        self.scene.light.update(light_position);

        for node in &mut self.scene.nodes {
            if node.model().id == self.light_model.id {
                node.set_transform(light_position, node.transform().rotation);
            }
        }

        self.scene.update(gfx);

        self.events.clear();
    }

    fn render(&mut self, gfx: &mut Gfx) -> Result<(), RenderError> {
        self.scene.render(gfx)
    }

    fn input(&mut self, _gfx: &mut Gfx, input: Input) {
        self.events.push(Event::Input(input));

        match input {
            Input::KeyPressed(key) => {
                self.camera_controller.key_pressed(key);
            }
            Input::KeyReleased(key) => {
                self.camera_controller.key_released(key);
            }
            Input::MousePressed => {
                self.camera_controller.mouse_pressed();
            }
            Input::MouseReleased => {
                self.camera_controller.mouse_released();
            }
            Input::MouseWheel(delta) => {
                self.camera_controller.mouse_scroll(delta);
            }
            Input::MouseMotion(dx, dy) => {
                self.camera_controller.mouse_drag(dx, dy);
            }
        }
    }

    fn resize(&mut self, width: u32, height: u32, gfx: &mut Gfx) {
        self.scene.resize(gfx, width, height)
    }
}

impl App {
    pub fn load_scene(scene: &mut Scene, map: &TileMap<Arc<Model>>) {
        let rotation = Quat::from_axis_angle(Vec3::Z, 0.);

        for tile in &map.tiles {
            scene.add_node(
                tile.position,
                rotation,
                tile.tile.model(),
                scene.phong_shader.clone(),
            );
        }
    }
}
