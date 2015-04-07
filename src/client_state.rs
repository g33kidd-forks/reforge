use std::rc::Rc;
use std::cell::RefCell;

use opengl_graphics::Gl;
use opengl_graphics::glyph_cache::GlyphCache;
use sdl2_window::Sdl2Window;

use asset_store::AssetStore;
use battle_context::BattleContext;
use sector_client::{ClientBattleState, ExitMode};
use net::Client;
use sector_data::SectorData;
use ship::{ShipNetworked};

pub enum ClientState {
    JoinSector,
    Respawn,
}

pub fn run_client_state_manager(window: &Rc<RefCell<Sdl2Window>>, gl: &mut Gl, glyph_cache: &mut GlyphCache, asset_store: &AssetStore, mut client: Client) {
    // Receive the star map
    let mut packet = client.receive();
    let sectors: Vec<SectorData> = packet.read().ok().expect("Failed to read star map");
    
    loop {
        // Receive the ships from the server
        let mut packet = client.receive();
        let my_ship: ShipNetworked = packet.read().ok().expect("Failed to read my Ship");
        let server_results_sent = packet.read().ok().expect("Failed to read server_results_sent from server");
        let ships: Vec<ShipNetworked> = match packet.read() {
            Ok(ships) => ships,
            Err(e) => panic!("Unable to receive ships froms server: {}", e),
        };
        
        // Create the battle state
        let mut battle_context = BattleContext::new(vec!());
        
        // Add the ships
        battle_context.add_networked_ships(ships);
        battle_context.add_networked_ship(my_ship);
        
        let mut battle = ClientBattleState::new(&mut client, battle_context);

        let exit_mode = battle.run(window, gl, glyph_cache, asset_store, sectors.clone(), server_results_sent);
        
        if exit_mode == ExitMode::Logout {
            break;
        }
    }
}