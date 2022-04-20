use bevy::prelude::*;
use rand::prelude::*;

use bevy::utils::HashSet;

//struct Animations(Vec<Handle<AnimationClip>>);
struct Animations(Handle<AnimationClip>);

pub struct ActorControl {
    // Properties here
}

impl Default for ActorControl {
    fn default() -> Self {
        ActorControl {
        }
    }
}

impl ActorControl {

    fn spawn_asset(
        &self,
        commands: &mut Commands,
        _asset_server: &Res<AssetServer>,
    ) {
        let gltf = _asset_server.load("/home/luca/ws_sim/rmf_sandbox/rmf_sandbox/assets/models/AmbulanceStretcher/AmbulanceStretcher.gltf#Scene0");
        let gltf_pbr = _asset_server.load("/home/luca/ws_sim/rmf_sandbox/rmf_sandbox/assets/models/AmbulanceStretcher/AmbulanceStretcher.glb#Scene0");

        commands.spawn_bundle((
            Transform::from_xyz(1.0, 0.0, 0.0),
            GlobalTransform::identity(),
        )).with_children(|parent| {
            parent.spawn_scene(gltf);
        });

        commands.spawn_bundle((
            Transform::from_xyz(2.0, 0.0, 0.0),
            GlobalTransform::identity(),
        )).with_children(|parent| {
            parent.spawn_scene(gltf_pbr);
        });
    }

    fn spawn_actor(
        &self,
        commands: &mut Commands,
        _asset_server: &Res<AssetServer>,
    ) {

        for n in 0..10000 {
            let model = _asset_server.load("/home/luca/ws_sim/rmf_sandbox/rmf_sandbox/assets/models/MaleVisitorPhoneWalk/MaleVisitorPhoneWalk.gltf#Scene0");
            let animation_path = "/home/luca/ws_sim/rmf_sandbox/rmf_sandbox/assets/models/MaleVisitorPhoneWalk/MaleVisitorPhoneWalk.gltf#Animation0";
            commands.insert_resource(Animations(_asset_server.load(animation_path)));
            //commands.insert_resource(Animations(vec![_asset_server.load(animation_path)]));
            //let animation_path = "/home/luca/ws_sim/rmf_sandbox/rmf_sandbox/assets/models/AnimatedFox/Fox.glb#Animation0";
            //let model = _asset_server.load("/home/luca/ws_sim/rmf_sandbox/rmf_sandbox/assets/models/AnimatedFox/Fox.glb#Scene0");

            // Now add the animation
            // And the model
            commands.spawn_bundle((
                Transform::from_xyz(-n as f32, 0.0, 0.0),
                GlobalTransform::identity(),
            )).with_children(|parent| {
                parent.spawn_scene(model);
            });

        }


    }

    pub fn spawn(
        &self,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        _asset_server: Res<AssetServer>,
    ) {
        self.spawn_asset(&mut commands, &_asset_server);
        self.spawn_actor(&mut commands, &_asset_server);
    }
}

pub fn initialize_actors(
    mut sm: ResMut<ActorControl>,
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    sm.spawn(commands, meshes, materials, asset_server);
}

// Once the scene is loaded, start the animation
fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut q : Query<(Entity, &mut AnimationPlayer)>,
    mut actor_init: Local<HashSet<Entity>>,
    mut done: Local<bool>,
) {
    // TODO consider passing a list of actors to avoid iterating through all of them at all times
    if !*done {
        for (e, mut player) in q.iter_mut() {
            // Only initialize actor that was not initialized before
            if !actor_init.contains(&e) {
                let mut rng = rand::thread_rng();
                let anim_speed = rng.gen::<f32>();
                player.play(animations.0.clone_weak()).repeat().set_speed(anim_speed);
                actor_init.insert(e);
                //*done = true;
            }
        }
    }
    if actor_init.len() == 10000 {
        *done = true;
    }
}

#[derive(Default)]
pub struct ActorControlPlugin;

impl Plugin for ActorControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorControl>()
           .add_startup_system(initialize_actors)
           .add_system(setup_scene_once_loaded);
    }
}
