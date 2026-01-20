use std::{net::UdpSocket, time::SystemTime};

use anyhow::Result;
use bevy::camera::Viewport;
use bevy::{camera::ScalingMode, prelude::*, window::WindowResolution};
use bevy_replicon::prelude::*;
use bevy_replicon_renet::{
    RenetChannelsExt, RepliconRenetPlugins,
    netcode::{ClientAuthentication, NetcodeClientTransport},
    renet::{ConnectionConfig, RenetClient},
};
use clap::Parser;
use core::{NetTransform, PROTOCOL_ID, Player, PlayerInputCommand, register_replication};

// Virtual resolution for pixel art. Adjust to change the visible world.
const VIRTUAL_WIDTH: f32 = 320.0;
const VIRTUAL_HEIGHT: f32 = 180.0;
// Scale the 8x8 cat sprite for readability.
const SPRITE_SCALE: f32 = 2.0;

#[derive(Parser, Debug, Clone, Resource)]
struct Args {
    #[arg(long, default_value = "127.0.0.1:5000")]
    server: String,
}

#[derive(Resource)]
struct LocalClientId(u64);

#[derive(Component)]
struct NetEntityVisual;

#[derive(Component)]
struct LocalPlayer;

#[derive(Component)]
struct FollowCamera;

#[derive(Resource)]
struct CameraVirtualResolution {
    width: f32,
    height: f32,
}

#[derive(Resource)]
struct LocalInputState {
    move_dir: Vec2,
    sprint: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut app = App::new();
    app.insert_resource(args);

    app.insert_resource(CameraVirtualResolution {
        width: VIRTUAL_WIDTH,
        height: VIRTUAL_HEIGHT,
    });
    app.insert_resource(LocalInputState {
        move_dir: Vec2::ZERO,
        sprint: false,
    });

    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Al-cat-raz".to_string(),
                    resolution: WindowResolution::new(
                        (VIRTUAL_WIDTH as u32) * 4,
                        (VIRTUAL_HEIGHT as u32) * 4,
                    )
                    .with_scale_factor_override(1.0),

                    ..Default::default()
                }),
                ..Default::default()
            }),
        RepliconPlugins,
        RepliconRenetPlugins,
    ));
    register_replication(&mut app);
    // Inputs are latency-sensitive; unordered delivery is fine for this prototype.
    app.add_client_message::<PlayerInputCommand>(Channel::Unordered);

    app.add_systems(Startup, (setup_scene, init_client));
    app.add_systems(
        Update,
        (
            capture_player_input,
            send_player_input,
            attach_sprite_to_replicated_entities,
            sync_visual_transform_from_net,
            tag_local_player,
            update_camera_follow,
            update_camera_viewport,
            log_client_state,
        ),
    );

    app.run();
    Ok(())
}

fn setup_scene(mut commands: Commands, virtual_resolution: Res<CameraVirtualResolution>) {
    let projection = OrthographicProjection {
        scaling_mode: ScalingMode::Fixed {
            width: virtual_resolution.width,
            height: virtual_resolution.height,
        },
        ..OrthographicProjection::default_2d()
    };

    // The viewport will be letterboxed to preserve pixel-perfect scaling.
    commands
        .spawn(Camera2d)
        .insert(Projection::Orthographic(projection))
        .insert(FollowCamera);
}

fn init_client(mut commands: Commands, args: Res<Args>, channels: Res<RepliconChannels>) {
    let server_addr = args.server.parse().expect("invalid --server");
    let socket = UdpSocket::bind("0.0.0.0:0").expect("failed to bind UDP socket");

    let connection_config = ConnectionConfig {
        server_channels_config: channels.server_configs(),
        client_channels_config: channels.client_configs(),
        ..Default::default()
    };

    let client = RenetClient::new(connection_config);

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("system time before unix epoch");

    let client_id = fastrand::u64(..);

    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(now, authentication, socket)
        .expect("failed to create NetcodeClientTransport");

    commands.insert_resource(LocalClientId(client_id));
    commands.insert_resource(client);
    commands.insert_resource(transport);
}

fn capture_player_input(keys: Res<ButtonInput<KeyCode>>, mut input_state: ResMut<LocalInputState>) {
    let mut move_dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        move_dir.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        move_dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        move_dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        move_dir.x += 1.0;
    }

    input_state.move_dir = move_dir;
    input_state.sprint = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
}

fn send_player_input(
    keys: Res<ButtonInput<KeyCode>>,
    input_state: Res<LocalInputState>,
    mut input_writer: MessageWriter<PlayerInputCommand>,
) {
    let dash = keys.just_pressed(KeyCode::Space);

    // Server expects normalized 8-way input.
    let x = input_state.move_dir.x.clamp(-1.0, 1.0).round() as i8;
    let y = input_state.move_dir.y.clamp(-1.0, 1.0).round() as i8;

    input_writer.write(PlayerInputCommand {
        x,
        y,
        sprint: input_state.sprint,
        dash,
    });
}

fn attach_sprite_to_replicated_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q: Query<
        Entity,
        (
            With<Replicated>,
            With<NetTransform>,
            Without<NetEntityVisual>,
        ),
    >,
) {
    let texture = asset_server.load("cat.png");

    for e in &q {
        commands.entity(e).insert((
            NetEntityVisual,
            Sprite {
                image: texture.clone(),
                custom_size: Some(Vec2::new(8.0, 8.0)),
                ..Default::default()
            },
            Transform::from_scale(Vec3::splat(SPRITE_SCALE)),
        ));
    }
}

fn sync_visual_transform_from_net(
    mut q: Query<(&NetTransform, &mut Transform), With<NetEntityVisual>>,
) {
    for (nt, mut t) in &mut q {
        t.translation.x = nt.x;
        t.translation.y = nt.y;
    }
}

fn tag_local_player(
    local_client: Res<LocalClientId>,
    mut commands: Commands,
    q: Query<(Entity, &Player), (With<Replicated>, Without<LocalPlayer>)>,
) {
    for (entity, player) in &q {
        if player.network_id == local_client.0 {
            commands.entity(entity).insert(LocalPlayer);
        }
    }
}

// Keep the camera centered on the local player.
fn update_camera_follow(
    mut q_camera: Query<&mut Transform, (With<FollowCamera>, Without<LocalPlayer>)>,
    q_player: Query<&Transform, (With<LocalPlayer>, Without<FollowCamera>)>,
) {
    let Ok(player_transform) = q_player.single() else {
        return;
    };
    for mut camera_transform in &mut q_camera {
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}

fn update_camera_viewport(
    windows: Query<&Window>,
    virtual_resolution: Res<CameraVirtualResolution>,
    mut cameras: Query<&mut Camera, With<FollowCamera>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let width = window.resolution.physical_width();
    let height = window.resolution.physical_height();
    let scale_x = width as f32 / virtual_resolution.width;
    let scale_y = height as f32 / virtual_resolution.height;
    let scale = scale_x.min(scale_y).floor().max(1.0) as u32;

    let viewport_width = (virtual_resolution.width as u32).saturating_mul(scale);
    let viewport_height = (virtual_resolution.height as u32).saturating_mul(scale);
    let viewport_x = width.saturating_sub(viewport_width) / 2;
    let viewport_y = height.saturating_sub(viewport_height) / 2;

    // Integer scaling + centered viewport keeps pixels crisp on any screen.
    // The resulting letterbox ensures we never render at a fractional scale.

    for mut camera in &mut cameras {
        camera.viewport = Some(Viewport {
            physical_position: UVec2::new(viewport_x, viewport_y),
            physical_size: UVec2::new(viewport_width, viewport_height),
            depth: 0.0..1.0,
        });
    }
}

fn log_client_state(time: Res<Time>, state: Res<State<ClientState>>) {
    if (time.elapsed().as_secs_f32() as i32) % 2 == 0 {
        debug!("client state: {:?}", state.get());
    }
}
