use std::{net::UdpSocket, time::SystemTime};

use anyhow::Result;
use bevy::{prelude::*, state::app::StatesPlugin};
use bevy_replicon::prelude::*;
use bevy_replicon_renet::{
    RenetChannelsExt, RepliconRenetPlugins,
    netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    renet::{ConnectionConfig, RenetServer},
};
use clap::Parser;
use core::{NetTransform, PROTOCOL_ID, register_replication};

#[derive(Parser, Debug, Clone, Resource)]
struct Args {
    #[arg(long, default_value = "127.0.0.1:5000")]
    addr: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut app = App::new();
    app.insert_resource(args);

    app.add_plugins((
        MinimalPlugins,
        StatesPlugin,
        RepliconPlugins,
        RepliconRenetPlugins,
    ));

    register_replication(&mut app);

    app.add_systems(Startup, (init_server, spawn_replicated_entity));
    app.add_systems(Update, (move_entity_server_authoritative, log_server_state));

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

fn spawn_replicated_entity(mut commands: Commands) {
    commands.spawn((Replicated, NetTransform { x: 50.0, y: 50.0 }));
}

fn move_entity_server_authoritative(
    time: Res<Time>,
    mut q: Query<&mut NetTransform, With<Replicated>>,
) {
    let t = time.elapsed().as_secs_f32();
    for mut nt in &mut q {
        nt.x = 200.0 + (t * 1.2).sin() * 120.0;
        nt.y = 140.0 + (t * 0.9).cos() * 80.0;
    }
}

fn log_server_state(time: Res<Time>, state: Res<State<ServerState>>) {
    if (time.elapsed().as_secs_f32() as i32) % 2 == 0 {
        debug!("server state: {:?}", state.get());
    }
}
