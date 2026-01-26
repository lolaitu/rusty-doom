use std::io::Result;
use renet::{RenetServer, ConnectionConfig, ClientId};
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
    pub time_of_last_loop: std::time::Instant,
    pub client_map: std::collections::HashMap<ClientId, u32>,
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
            time_of_last_loop: std::time::Instant::now(),
            client_map: std::collections::HashMap::new(),
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
                    self.client_map.insert(client_id, player_id);
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    println!("Client {} disconnected: {:?}", client_id, reason);
                    if let Some(player_id) = self.client_map.remove(&client_id) {
                        self.world.remove_entity(player_id);
                    }
                }
            }
        }

        for client_id in self.server.clients_id() {
            while let Some(message) = self.server.receive_message(client_id, 0) { // Channel 0 for reliable/ordered
                if let Ok(msg) = bincode::deserialize::<ClientMessage>(&message) {
                    match msg {
                        ClientMessage::Input(input) => {
                            if let Some(&player_id) = self.client_map.get(&client_id) {
                                // Apply input to client's player entity
                                let move_speed = if input.move_forward || input.move_backward || input.strafe_left || input.strafe_right {
                                     crate::common::entity::PLAYER_SPEED 
                                } else { 0.0 };
                                let rot_speed = crate::common::entity::PLAYER_ROTATION_SPEED;

                                if input.move_forward {
                                    Physics::move_entity_forward(&mut self.world, player_id, move_speed * 0.016, &self.level);
                                }
                                if input.move_backward {
                                    Physics::move_entity_forward(&mut self.world, player_id, -move_speed * 0.016, &self.level);
                                }
                                if input.rotate_left {
                                    Physics::rotate_entity(&mut self.world, player_id, -rot_speed * 0.016);
                                }
                                if input.rotate_right {
                                    Physics::rotate_entity(&mut self.world, player_id, rot_speed * 0.016);
                                }
                                if input.strafe_left {
                                    Physics::strafe_entity(&mut self.world, player_id, -move_speed * 0.016, &self.level);
                                }
                                if input.strafe_right {
                                    Physics::strafe_entity(&mut self.world, player_id, move_speed * 0.016, &self.level);
                                }
                            }
                        }
                    }
                }
            }
        }



        // Calculate delta time
        let now = std::time::Instant::now();
        let delta_time = now.duration_since(self.time_of_last_loop).as_secs_f64();
        self.time_of_last_loop = now;

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
            Physics::move_entity_forward(&mut self.world, self.player_id, move_speed * delta_time, &self.level);
        }
        if input_manager.is_active(crate::input::Action::MoveBackward) {
            Physics::move_entity_forward(&mut self.world, self.player_id, -move_speed * delta_time, &self.level);
        }
        if input_manager.is_active(crate::input::Action::RotateLeft) {
            Physics::rotate_entity(&mut self.world, self.player_id, -rot_speed * delta_time);
        }
        if input_manager.is_active(crate::input::Action::RotateRight) {
            Physics::rotate_entity(&mut self.world, self.player_id, rot_speed * delta_time);
        }
        if input_manager.is_active(crate::input::Action::StrafeLeft) {
            Physics::strafe_entity(&mut self.world, self.player_id, -move_speed * delta_time, &self.level);
        }
        if input_manager.is_active(crate::input::Action::StrafeRight) {
            Physics::strafe_entity(&mut self.world, self.player_id, move_speed * delta_time, &self.level);
        }
        
        // Sync player struct with entity (for rendering)
        if let Some(entity) = self.world.get_entity(self.player_id) {
            self.player.transform = entity.transform;
            self.player.health = entity.health as u32;
        }

        // Run physics
        Physics::update(&mut self.world, delta_time, &self.level);

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
        
        // Frame limiting
        let target_duration = std::time::Duration::from_secs_f64(1.0 / 60.0);
        let elapsed = self.time_of_last_loop.elapsed();
        if elapsed < target_duration {
            std::thread::sleep(target_duration - elapsed);
        }

        Ok(false)
    }
}
