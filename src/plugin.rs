pub mod plugin {
    use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
    pub struct MainGame;

    #[derive(Component)]
    struct Triangle;

    impl Plugin for MainGame {
        fn build(&self, app: &mut App) {
            app
                .add_systems(Startup, setup)
                .add_systems(FixedUpdate, rotate_triangle);
        }
    }

    fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
        commands.spawn(Camera2dBundle::default());

        let triangle = Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::new(0.0, 50.0),
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        )));

        let color = Color::rgb(1.0, 0.0, 0.0);

        commands.spawn((MaterialMesh2dBundle {
            mesh: triangle,
            material: materials.add(color),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }, Triangle));
    }

    fn rotate_triangle(mut triangles: Query<&mut Transform, With<Triangle>>, time: Res<Time>) {
        for mut triangle in &mut triangles {
            triangle.rotate_z(1.0 * time.delta_seconds());
        }
    }
}