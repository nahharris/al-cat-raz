use std::{net::UdpSocket, time::SystemTime};

use anyhow::Result;
use bevy::{log::LogPlugin, prelude::*, state::app::StatesPlugin};
use bevy_replicon::prelude::*;
use bevy_replicon::shared::backend::connected_client::NetworkId;
use bevy_replicon_renet::{
    RenetChannelsExt, RepliconRenetPlugins,
    netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    renet::{ConnectionConfig, RenetServer},
};
use clap::Parser;
use core::{NetTransform, PROTOCOL_ID, Player, PlayerInputCommand, register_replication};

// Tuned for snappy top-down movement on a 320x180 virtual resolution.
// Adjust these without touching networking or input code.
const BASE_MOVE_SPEED: f32 = 70.0;
const SPRINT_MULTIPLIER: f32 = 1.5;
const DASH_SPEED_MULTIPLIER: f32 = 3.0;
const DASH_DURATION_SECS: f32 = 0.18;
const DASH_COOLDOWN_SECS: f32 = 0.8;

#[derive(Parser, Debug, Clone, Resource)]
struct Args {
    #[arg(long, default_value = "127.0.0.1:5000")]
    addr: String,
}

#[derive(Component)]
struct PlayerMovementState {
    dash_timer: Timer,
    dash_cooldown: Timer,
    dash_direction: Vec2,
}

#[derive(Component)]
struct PlayerOwner {
    client: Entity,
}

#[derive(Component)]
struct PlayerInput {
    move_dir: Vec2,
    sprint: bool,
    dash_pressed: bool,
}

#[derive(Resource)]
struct DashConfig {
    /// True to lock dash direction to the input when dash starts.
    /// If false, the dash direction will update while the dash is active.
    lock_direction: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut app = App::new();
    app.insert_resource(args);

    app.add_plugins((
        MinimalPlugins,
        LogPlugin::default(),
        StatesPlugin,
        RepliconPlugins,
        RepliconRenetPlugins,
    ));

    register_replication(&mut app);
    // Inputs are latency-sensitive; unordered delivery is fine for this prototype.
    app.add_client_message::<PlayerInputCommand>(Channel::Unordered);

    // Dash direction locking is a gameplay choice. Keep it configurable here.
    app.insert_resource(DashConfig {
        lock_direction: true,
    });

    app.add_systems(Startup, init_server);
    app.add_systems(
        Update,
        (
            spawn_player_on_connect,
            despawn_player_on_disconnect,
            receive_player_inputs,
            apply_player_movement,
            log_server_state,
        ),
    );

    app.run();
    Ok(())
}

fn init_server(mut commands: Commands, args: Res<Args>, channels: Res<RepliconChannels>) {
    let server_addr = args.addr.parse().expect("invalid --addr");
    let socket = UdpSocket::bind(server_addr).expect("failed to bind UDP socket");

    let connection_config = ConnectionConfig {
        server_channels_config: channels.server_configs(),
        client_channels_config: channels.client_configs(),
        ..Default::default()
    };

    let server = RenetServer::new(connection_config);

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("system time before unix epoch");

    let transport = NetcodeServerTransport::new(
        ServerConfig {
            max_clients: 32,
            protocol_id: PROTOCOL_ID,
            public_addresses: vec![server_addr],
            authentication: ServerAuthentication::Unsecure,
            current_time: now,
        },
        socket,
    )
    .expect("failed to create NetcodeServerTransport");

    commands.insert_resource(server);
    commands.insert_resource(transport);
}

fn spawn_player_on_connect(
    mut commands: Commands,
    q: Query<(Entity, &NetworkId), Added<ConnectedClient>>,
) {
    for (client_entity, network_id) in &q {
        let mut dash_timer = Timer::from_seconds(DASH_DURATION_SECS, TimerMode::Once);
        dash_timer.finish();
        let mut dash_cooldown = Timer::from_seconds(DASH_COOLDOWN_SECS, TimerMode::Once);
        dash_cooldown.finish();

        let player_entity = commands
            .spawn((
                Replicated,
                Player {
                    network_id: network_id.get(),
                },
                PlayerOwner {
                    client: client_entity,
                },
                NetTransform { x: 0.0, y: 0.0 },
                PlayerInput {
                    move_dir: Vec2::ZERO,
                    sprint: false,
                    dash_pressed: false,
                },
                PlayerMovementState {
                    dash_timer,
                    dash_cooldown,
                    dash_direction: Vec2::X,
                },
            ))
            .id();

        info!(
            "player joined: entity={player_entity:?} client={client_entity:?} network_id={}",
            network_id.get()
        );
    }
}

fn despawn_player_on_disconnect(
    mut commands: Commands,
    mut removed: RemovedComponents<ConnectedClient>,
    players: Query<(Entity, &PlayerOwner)>,
) {
    for client_entity in removed.read() {
        for (entity, owner) in &players {
            if owner.client == client_entity {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn receive_player_inputs(
    mut commands: Commands,
    mut inputs: MessageReader<FromClient<PlayerInputCommand>>,
    network_ids: Query<&NetworkId>,
    players: Query<(Entity, &Player)>,
) {
    for input in inputs.read() {
        let client_entity = match input.client_id {
            ClientId::Client(entity) => entity,
            ClientId::Server => continue,
        };

        let Ok(network_id) = network_ids.get(client_entity) else {
            continue;
        };

        let Some((entity, _)) = players
            .iter()
            .find(|(_, player)| player.network_id == network_id.get())
        else {
            continue;
        };

        let move_dir =
            Vec2::new(input.message.x as f32, input.message.y as f32).clamp_length_max(1.0);

        commands.entity(entity).insert(PlayerInput {
            move_dir,
            sprint: input.message.sprint,
            dash_pressed: input.message.dash,
        });
    }
}

fn apply_player_movement(
    time: Res<Time>,
    dash_config: Res<DashConfig>,
    mut q: Query<(
        &mut NetTransform,
        &mut PlayerInput,
        &mut PlayerMovementState,
    )>,
) {
    let delta = time.delta_secs();

    for (mut transform, mut input, mut movement) in &mut q {
        movement.dash_timer.tick(time.delta());
        movement.dash_cooldown.tick(time.delta());

        if input.dash_pressed
            && movement.dash_timer.is_finished()
            && movement.dash_cooldown.is_finished()
            && input.move_dir.length_squared() > 0.0
        {
            // Dash uses the current input direction.
            movement.dash_direction = input.move_dir.normalize_or_zero();
            movement.dash_timer.reset();
            movement.dash_cooldown.reset();
        }

        let dash_active = !movement.dash_timer.is_finished();
        let mut base_dir = input.move_dir;

        if dash_active {
            if dash_config.lock_direction {
                base_dir = movement.dash_direction;
            } else if base_dir.length_squared() > 0.0 {
                movement.dash_direction = base_dir.normalize_or_zero();
                base_dir = movement.dash_direction;
            } else {
                base_dir = movement.dash_direction;
            }
        } else if base_dir.length_squared() > 0.0 {
            movement.dash_direction = base_dir.normalize_or_zero();
        }

        if base_dir.length_squared() > 0.0 {
            let mut speed = BASE_MOVE_SPEED;
            if input.sprint {
                speed *= SPRINT_MULTIPLIER;
            }
            if dash_active {
                speed *= DASH_SPEED_MULTIPLIER;
            }

            transform.x += base_dir.x * speed * delta;
            transform.y += base_dir.y * speed * delta;
        }

        input.dash_pressed = false;
    }
}

fn log_server_state(time: Res<Time>, state: Res<State<ServerState>>) {
    if (time.elapsed().as_secs_f32() as i32) % 2 == 0 {
        debug!("server state: {:?}", state.get());
    }
}
