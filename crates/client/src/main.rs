use std::{net::UdpSocket, time::SystemTime};

use anyhow::Result;
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_replicon_renet::{
    RenetChannelsExt, RepliconRenetPlugins,
    netcode::{ClientAuthentication, NetcodeClientTransport},
    renet::{ConnectionConfig, RenetClient},
};
use clap::Parser;
use core::{NetTransform, PROTOCOL_ID, register_replication};

#[derive(Parser, Debug, Clone, Resource)]
struct Args {
    #[arg(long, default_value = "127.0.0.1:5000")]
    server: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut app = App::new();
    app.insert_resource(args);

    app.add_plugins((DefaultPlugins, RepliconPlugins, RepliconRenetPlugins));

    register_replication(&mut app);

    app.add_systems(Startup, (setup_scene, init_client));
    app.add_systems(
        Update,
        (
            attach_sprite_to_replicated_entities,
            sync_visual_transform_from_net,
            log_client_state,
        ),
    );

    app.run();
    Ok(())
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d::default());
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

    commands.insert_resource(client);
    commands.insert_resource(transport);
}

#[derive(Component)]
struct NetEntityVisual;

fn attach_sprite_to_replicated_entities(
    mut commands: Commands,
    q: Query<
        Entity,
        (
            With<Replicated>,
            With<NetTransform>,
            Without<NetEntityVisual>,
        ),
    >,
) {
    for e in &q {
        commands.entity(e).insert((
            NetEntityVisual,
            Sprite {
                color: Color::srgb(1.0, 0.1, 0.1),
                custom_size: Some(Vec2::new(12.0, 12.0)),
                ..Default::default()
            },
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

fn log_client_state(time: Res<Time>, state: Res<State<ClientState>>) {
    if (time.elapsed().as_secs_f32() as i32) % 2 == 0 {
        debug!("client state: {:?}", state.get());
    }
}
