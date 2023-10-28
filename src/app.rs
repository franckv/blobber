use glam::{Quat, Vec3};

use crate::controller::{CameraController, Facing};
use crate::map::TileMap;
use gobs_game as game;
use gobs_scene as scene;

use game::{app::Run, input::Input};
use scene::scene::Scene;
use scene::Gfx;
use scene::{
    camera::{Camera, CameraProjection},
    RenderError, ShaderType,
};
use scene::{light::Light, MaterialBuilder, ModelBuilder};
use uuid::Uuid;

pub struct App {
    camera_controller: CameraController,
    map: TileMap<Uuid>,
    scene: Scene,
    light_model: Uuid,
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

        let wall_model = ModelBuilder::new()
            .add_mesh(
                scene::shape::Shapes::cube(
                    gfx,
                    ShaderType::Phong.vertex_flags(),
                    3,
                    2,
                    &[5, 5, 5, 5, 6, 4],
                ),
                0,
            )
            .add_material(
                MaterialBuilder::new("diffuse")
                    .diffuse_texture(gfx, crate::WALL_TEXTURE)
                    .await
                    .normal_texture(gfx, crate::WALL_TEXTURE_N)
                    .await
                    .build(gfx, &scene.phong_shader),
            )
            .build();

        let floor_model = ModelBuilder::new()
            .add_mesh(
                scene::shape::Shapes::cube(gfx, ShaderType::Phong.vertex_flags(), 3, 2, &[4]),
                0,
            )
            .add_material(
                MaterialBuilder::new("diffuse")
                    .diffuse_texture(gfx, crate::WALL_TEXTURE)
                    .await
                    .normal_texture(gfx, crate::WALL_TEXTURE_N)
                    .await
                    .build(gfx, &scene.phong_shader),
            )
            .build();

        let wall_id = scene.add_model(wall_model, ShaderType::Phong);
        let floor_id = scene.add_model(floor_model, ShaderType::Phong);

        let mut map = TileMap::new();

        map.load(crate::MAP, crate::TILE_SIZE, wall_id, floor_id)
            .unwrap();

        Self::load_scene(&mut scene, &map);

        scene.camera.position = map.start.into();

        let light_model = scene
            .load_model(gfx, crate::LIGHT, ShaderType::Solid, 0.3)
            .await
            .unwrap();
        scene.add_node(
            light_position,
            Quat::from_axis_angle(Vec3::Z, 0.),
            light_model,
        );

        let camera_controller = CameraController::new(Facing::North, 0.7);

        App {
            camera_controller,
            map,
            scene,
            light_model,
        }
    }

    fn update(&mut self, delta: f32, gfx: &mut Gfx) {
        let angular_speed = -10.;

        self.camera_controller
            .update_camera(&mut self.scene.camera, &self.map, delta);

        let mut light_position: Vec3 = self.scene.light.position;
        light_position =
            (Quat::from_axis_angle((0., 0., 1.).into(), (angular_speed * delta).to_radians())
                * light_position)
                .into();

        self.scene.light.update(light_position);

        for node in &mut self.scene.nodes {
            if node.model() == self.light_model {
                node.set_transform(light_position, node.transform().rotation);
            }
        }

        self.scene.update(gfx);
    }

    fn render(&mut self, gfx: &mut Gfx) -> Result<(), RenderError> {
        self.scene.render(gfx)
    }

    fn input(&mut self, _gfx: &mut Gfx, input: Input) {
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
    pub fn load_scene(scene: &mut Scene, map: &TileMap<Uuid>) {
        let rotation = Quat::from_axis_angle(Vec3::Z, 0.);

        for tile in &map.tiles {
            scene.add_node(tile.position, rotation, tile.tile.model());
        }
    }
}
