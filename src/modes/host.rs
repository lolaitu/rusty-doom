use std::io::Result;
use renet::{RenetServer, ConnectionConfig};
use matchbox_socket::WebRtcSocket;
use futures::future::BoxFuture;
use crate::common::world::World;
use crate::common::physics::Physics;
use crate::common::level::Level;
use crate::network::connection::setup_server;
use crate::input::InputManager;
use crate::graphics::RenderBuffer;
use renet::ServerEvent;
use crate::common::protocol::{ClientMessage, ServerMessage, PlayerInput};
use crate::network::connection::PROTOCOL_ID;

use crate::graphics::draw;
use crossterm::terminal;

use crate::player::Player;

pub struct HostGame {
    pub server: RenetServer,
    pub socket: WebRtcSocket,
    pub world: World,
    pub level: Level,
    pub runtime: tokio::runtime::Runtime,
    pub render_buffer: RenderBuffer,
    pub term_size: (u16, u16),
    pub player: Player,
    pub player_id: u32,
}

impl HostGame {
    pub fn new(level: Level) -> Result<Self> {
        let runtime = tokio::runtime::Runtime::new()?;
        let (server, socket, message_loop) = setup_server();
        
        // Spawn the message loop on the runtime
        runtime.spawn(message_loop);
        
        let mut world = World::new();
        
        // Spawn local player
        let player_entity = crate::common::entity::Entity::new_player(0, 3.5, 3.5);
        let player_id = world.spawn_entity(player_entity);

        let (w, h) = terminal::size()?;
        let render_buffer = RenderBuffer::new(w, h);
        let player = Player::new()?;
        
        Ok(Self {
            server,
            socket,
            world,
            level,
            runtime,
            render_buffer,
            term_size: (w, h),
            player,
            player_id,
        })
    }
}

use crate::modes::gamemode::GameMode;

impl GameMode for HostGame {
    fn update(&mut self, input_manager: &InputManager) -> Result<bool> {
        // TODO: Drive the message_loop future if possible.
        // Since we can't easily block, we assume the future is being driven elsewhere 
        // OR we just poll the server which might not be enough for Matchbox without the loop.
        // However, for this implementation step, we focus on the logic.
        
        self.server.update(std::time::Duration::from_millis(16)); // Assume 60 FPS

        while let Some(event) = self.server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    println!("Client {} connected", client_id);
                    // Spawn player for client
                    let player_id = self.world.spawn_entity(crate::common::entity::Entity::new_player(0, 3.5, 3.5));
                    // TODO: Map client_id to player_id
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    println!("Client {} disconnected: {:?}", client_id, reason);
                    // Remove player
                }
            }
        }

        for client_id in self.server.clients_id() {
            while let Some(message) = self.server.receive_message(client_id, 0) { // Channel 0 for reliable/ordered
                if let Ok(msg) = bincode::deserialize::<ClientMessage>(&message) {
                    match msg {
                        ClientMessage::Input(input) => {
                            // Apply input to client's player
                            // Need mapping from client_id to entity_id
                        }
                    }
                }
            }
        }



        // Local Player Movement
        let mut move_speed = if input_manager.is_active(crate::input::Action::Sprint) { 
            crate::common::entity::PLAYER_SPEED * 2.0 
        } else { 
            crate::common::entity::PLAYER_SPEED 
        };
        
        let mut rot_speed = crate::common::entity::PLAYER_ROTATION_SPEED;

        // Apply weapon weight penalty
        let penalty = self.player.get_current_weapon().movement_penalty;
        move_speed *= 1.0 - penalty;
        rot_speed *= 1.0 - (penalty * 0.5);

        if input_manager.is_active(crate::input::Action::MoveForward) {
            Physics::move_entity_forward(&mut self.world, self.player_id, move_speed * 0.016, &self.level);
        }
        if input_manager.is_active(crate::input::Action::MoveBackward) {
            Physics::move_entity_forward(&mut self.world, self.player_id, -move_speed * 0.016, &self.level);
        }
        if input_manager.is_active(crate::input::Action::RotateLeft) {
            Physics::rotate_entity(&mut self.world, self.player_id, -rot_speed * 0.016);
        }
        if input_manager.is_active(crate::input::Action::RotateRight) {
            Physics::rotate_entity(&mut self.world, self.player_id, rot_speed * 0.016);
        }
        if input_manager.is_active(crate::input::Action::StrafeLeft) {
            Physics::strafe_entity(&mut self.world, self.player_id, -move_speed * 0.016, &self.level);
        }
        if input_manager.is_active(crate::input::Action::StrafeRight) {
            Physics::strafe_entity(&mut self.world, self.player_id, move_speed * 0.016, &self.level);
        }
        
        // Sync player struct with entity (for rendering)
        if let Some(entity) = self.world.get_entity(self.player_id) {
            self.player.transform = entity.transform;
            self.player.health = entity.health as u32;
        }

        // Run physics
        Physics::update(&mut self.world, 0.016, &self.level);

        // Broadcast state
        let snapshot = ServerMessage::WorldSnapshot(self.world.clone());
        if let Ok(data) = bincode::serialize(&snapshot) {
            self.server.broadcast_message(0, data);
        }

        // Render
        let mut stdout = std::io::stdout();
        self.term_size = terminal::size()?;
        if self.term_size.0 != self.render_buffer.width as u16 || self.term_size.1 != self.render_buffer.height as u16 {
             self.render_buffer = RenderBuffer::new(self.term_size.0, self.term_size.1);
        }
        
        draw(&self.world, &self.player, &self.level, self.term_size, &mut self.render_buffer)?;
        self.render_buffer.flush(&mut stdout)?;
        
        Ok(false)
    }
}
