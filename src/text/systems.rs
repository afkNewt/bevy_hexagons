use bevy::prelude::*;

use crate::{board::components::HexTile, hexagon::cursor_to_hex, player::resources::PlayerCoins};

use super::components::{CoinText, TileText};

pub fn generate_tile_variant_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Tile: ",
                TextStyle {
                    font: asset_server.load("fonts/arial.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/arial.ttf"),
                font_size: 40.0,
                color: Color::GOLD,
            }),
        ]),
        TileText,
    ));
}

pub fn update_tile_variant_text(
    windows: Query<&Window>,
    hexes: Query<&HexTile>,
    mut tile_text: Query<&mut Text, With<TileText>>,
) {
    let Some(hovered_hex) = cursor_to_hex(windows) else {
        return;
    };

    let mut hex_info = "None".to_string();
    for hex in &hexes {
        if hex.coordinate == hovered_hex {
            hex_info = format!("{:?} {}", hex.variant, hex.coordinate);
        }
    }

    for mut text in &mut tile_text {
        // Update the value of the second section
        text.sections[1].value = hex_info.to_string();
    }
}

pub fn generate_player_coin_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Coins: ",
                TextStyle {
                    font: asset_server.load("fonts/arial.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/arial.ttf"),
                font_size: 40.0,
                color: Color::GOLD,
            }),
        ]),
        CoinText,
    ));
}

pub fn update_player_coin_text(
    player_coins: Res<PlayerCoins>,
    mut coin_text: Query<&mut Text, With<CoinText>>,
) {
    for mut text in &mut coin_text {
        // Update the value of the second section
        text.sections[1].value = format!("{}", player_coins.0);
    }
}
