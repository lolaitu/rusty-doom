classDiagram
    %% Core Interfaces & Main
    class Main {
        +InputManager input_manager
        +Box~GameMode~ current_mode
        +RenderBuffer render_buffer
        +run()
    }
    class GameMode {
        <<interface>>
        +update(input: &InputManager, dt: f64) -> Result~bool~
        +render(buffer: &mut RenderBuffer) -> Result~()~
    }
    Main --> GameMode
    Main --> InputManager
    Main --> RenderBuffer
    %% Game Modes
    class SoloGame {
        +World world
        +Physics physics
        +Player local_player
        +update(...)
        +render(...)
    }
    class HostGame {
        +World world
        +Physics physics
        +NetworkServer server
        +HashMap~ClientId, Player~ players
        +update(...)
        +render(...)
        -process_network_messages()
        -broadcast_state()
    }
    class ClientGame {
        +World predicted_world
        +NetworkClient client
        +Player local_player
        +update(...)
        +render(...)
        -send_input()
        -reconcile_state(server_state: World)
    }
    GameMode <|.. SoloGame
    GameMode <|.. HostGame
    GameMode <|.. ClientGame
    %% Shared Logic (Common)
    class World {
        +Level level
        +HashMap~u32, Entity~ entities
        +u32 next_entity_id
        +spawn_entity(e: Entity)
        +remove_entity(id: u32)
        +get_snapshot() -> WorldState
    }
    class Entity {
        +u32 id
        +EntityType type
        +Transform transform
        +Vector2 velocity
        +bool active
    }
    class Physics {
        +update_entity(e: &mut Entity, level: &Level, dt: f64)
        +check_collisions(world: &World)
        +move_entity(...)
    }
    SoloGame --> World
    HostGame --> World
    ClientGame --> World
    
    World "1" *-- "*" Entity
    SoloGame ..> Physics
    HostGame ..> Physics
    ClientGame ..> Physics
    %% Networking
    class NetworkServer {
        +RenetServer server
        +MatchboxSocket socket
        +send_message(client_id, msg: ServerMessage)
        +receive_messages() -> Vec~ClientMessage~
    }
    class NetworkClient {
        +RenetClient client
        +MatchboxSocket socket
        +send_message(msg: ClientMessage)
        +receive_messages() -> Vec~ServerMessage~
    }
    class Protocol {
        <<enumeration>>
        ClientMessage
        ServerMessage
    }
    
    class ClientMessage {
        <<variant>>
        Input(PlayerInput)
        Command(String)
    }
    
    class ServerMessage {
        <<variant>>
        WorldSnapshot(World)
        Event(GameEvent)
    }
    HostGame --> NetworkServer
    ClientGame --> NetworkClient
    NetworkServer ..> Protocol
    NetworkClient ..> Protocol
    %% Input & Graphics
    class InputManager {
        +update()
        +get_action_state(action: Action) -> bool
    }
    class RenderBuffer {
        +resize(w, h)
        +set_pixel(x, y, char, color)
    }
    class Renderer {
        +draw_world(world: &World, buffer: &mut RenderBuffer)
        +draw_hud(player: &Player, buffer: &mut RenderBuffer)
    }
    SoloGame ..> Renderer
    HostGame ..> Renderer
    ClientGame ..> Renderer