use std::io::Result;
use renet::{RenetClient, ConnectionConfig};
use matchbox_socket::WebRtcSocket;
use futures::future::BoxFuture;
use crate::common::world::World;
use crate::common::level::Level;
use crate::network::connection::setup_client;
use crate::input::{InputManager, Action};
use crate::graphics::RenderBuffer;
use crate::common::protocol::{ClientMessage, ServerMessage, PlayerInput};
use crate::graphics::draw;
use crossterm::terminal;

use crate::player::Player;

pub struct ClientGame {
    pub client: RenetClient,
    pub socket: WebRtcSocket,
    pub world: World,
    pub runtime: tokio::runtime::Runtime,
    pub render_buffer: RenderBuffer,
    pub term_size: (u16, u16),
    pub player: Player,
    pub level: Level,
}

impl ClientGame {
    pub fn new() -> Result<Self> {
        let runtime = tokio::runtime::Runtime::new()?;
        let (client, socket, message_loop) = setup_client();
        
        runtime.spawn(message_loop);
        
        let world = World::new();
        let (w, h) = terminal::size()?;
        let render_buffer = RenderBuffer::new(w, h);
        let player = Player::new()?;
        let level = Level::debug_1()?;
        
        Ok(Self {
            client,
            socket,
            world,
            runtime,
            render_buffer,
            term_size: (w, h),
            player,
            level,
        })
    }
}

use crate::modes::gamemode::GameMode;

impl GameMode for ClientGame {
    fn update(&mut self, input_manager: &InputManager) -> Result<bool> {
        self.client.update(std::time::Duration::from_millis(16));

        // Frame limiting to prevent 100% CPU usage if waiting
        std::thread::sleep(std::time::Duration::from_millis(1));

        // Send Input
        let input = PlayerInput {
            move_forward: input_manager.is_active(Action::MoveForward),
            move_backward: input_manager.is_active(Action::MoveBackward),
            strafe_left: input_manager.is_active(Action::StrafeLeft),
            strafe_right: input_manager.is_active(Action::StrafeRight),
            rotate_left: input_manager.is_active(Action::RotateLeft),
            rotate_right: input_manager.is_active(Action::RotateRight),
            shoot: input_manager.is_active(Action::Shoot),
            reload: input_manager.is_active(Action::Reload),
            view_angle: 0.0, // TODO: Get from player entity if we have one
        };
        
        let message = ClientMessage::Input(input);
        if let Ok(data) = bincode::serialize(&message) {
            self.client.send_message(0, data);
        }

        // Receive State
        while let Some(message) = self.client.receive_message(0) {
            if let Ok(msg) = bincode::deserialize::<ServerMessage>(&message) {
                match msg {
                    ServerMessage::WorldSnapshot(snapshot) => {
                        self.world = snapshot;
                    }
                }
            }
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
