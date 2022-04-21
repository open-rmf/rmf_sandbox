use bevy::prelude::*;
use rand::prelude::*;

use bevy::utils::HashSet;
use bevy::utils::HashMap;

// Usage map["actor_name"]["animation_type"] = handle
// animation type can be idle or walking
type AnimationHashMap = HashMap<String, HashMap<String, Handle<AnimationClip>>>;

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
        animations: &mut AnimationHashMap,
        _asset_server: &Res<AssetServer>,
    ) {
        for n in 0..10000 {
            let actor_name = "MaleVisitorOnPhone";
            let model = _asset_server.load("/home/luca/ws_sim/rmf_sandbox/rmf_sandbox/assets/models/MaleVisitorPhoneWalk/MaleVisitorOnPhone_Anim.glb#Scene0");
            // Only add resource if not existing
            if !animations.contains_key(actor_name) {
                let idle_path = "/home/luca/ws_sim/rmf_sandbox/rmf_sandbox/assets/models/MaleVisitorPhoneWalk/MaleVisitorOnPhone_Anim.glb#Animation0";
                let walking_path = "/home/luca/ws_sim/rmf_sandbox/rmf_sandbox/assets/models/MaleVisitorPhoneWalk/MaleVisitorOnPhone_Anim.glb#Animation1";
                let mut anim_hash = HashMap::<String, Handle<AnimationClip>>::default();
                anim_hash.insert("idle".to_string(), _asset_server.load(idle_path));
                anim_hash.insert("walking".to_string(), _asset_server.load(walking_path));
                animations.insert(actor_name.to_string(), anim_hash);
            }

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
        animations: &mut AnimationHashMap,
        mut commands: &mut Commands,
        _asset_server: Res<AssetServer>,
    ) {
        self.spawn_asset(&mut commands, &_asset_server);
        self.spawn_actor(&mut commands, animations, &_asset_server);
    }
}

pub fn initialize_actors(
    sm: ResMut<ActorControl>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut animations = AnimationHashMap::default();
    sm.spawn(&mut animations, &mut commands, asset_server);
    // Add the resource
    commands.insert_resource(animations);
}

// Once the scene is loaded, start the animation
fn setup_scene_once_loaded(
    animations: Res<AnimationHashMap>,
    mut q : Query<(Entity, &mut AnimationPlayer)>,
    mut actor_init: Local<HashSet<Entity>>,
    mut done: Local<bool>,
) {
    if !*done {
        for (e, mut player) in q.iter_mut() {
            // Only initialize actor that was not initialized before
            if !actor_init.contains(&e) {
                let mut rng = rand::thread_rng();
                let anim_speed = rng.gen::<f32>();
                let anim_name = if anim_speed > 0.5 {"idle"} else {"walking"};
                player.play(animations["MaleVisitorOnPhone"][anim_name].clone_weak()).repeat().set_speed(anim_speed);
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
