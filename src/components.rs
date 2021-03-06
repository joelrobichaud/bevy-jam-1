use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::AudioSource;

use std::collections::HashMap;

pub enum TileType {
    Wall,
    Ladder,
}

#[derive(Default)]
pub struct TileMap(pub HashMap<(i32, i32), TileType>);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    Credits,
}

pub struct UiSounds {
    pub button_clicked_sfx: Handle<AudioSource>,
}

pub struct GameTextures {
    pub player_health_standing: Handle<Image>,
    pub player_speed_standing: Handle<Image>,
    pub player_strength_standing: Handle<Image>,

    pub player_health_climbing: Handle<Image>,
    pub player_speed_climbing: Handle<Image>,
    pub player_strength_climbing: Handle<Image>,

    pub player_health_falling: Handle<Image>,
    pub player_speed_falling: Handle<Image>,
    pub player_strength_falling: Handle<Image>,
}

pub struct GameSounds {
    pub player_movement_sfxs: Vec<Handle<AudioSource>>,
    pub player_climb_up_sfxs: Vec<Handle<AudioSource>>,
    pub player_climb_down_sfxs: Vec<Handle<AudioSource>>,
    pub player_hit_sfxs: Vec<Handle<AudioSource>>,
    pub player_attack_sfx: Handle<AudioSource>,
    pub falling_ice_sfx: Handle<AudioSource>,
    pub goal_sfx: Handle<AudioSource>,
}

pub enum Advantage {
    Speed,
    Strength,
    Health,
}

pub struct GameState {
    pub player_previous_pos: Vec3,
    pub world_should_update: bool,
    pub player_num_actions_taken: u32,
    pub player_is_falling: bool,
    pub player_advantage: Option<Advantage>,
    pub level_index: usize,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            player_previous_pos: Vec3::ZERO,
            world_should_update: false,
            player_num_actions_taken: 0,
            player_is_falling: false,
            player_advantage: None,
            level_index: 0,
        }
    }
}

#[derive(Clone, Component)]
pub struct Speed(pub u8);

#[derive(Clone, Component)]
pub struct Damage(pub i32);

#[derive(Clone, Component)]
pub struct Health(pub i32);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Bundle)]
pub struct PlayerBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub player: Player,
    pub speed: Speed,
    pub damage: Damage,
    pub health: Health,
}

impl LdtkEntity for PlayerBundle {
    fn bundle_entity(
        _: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("Player.png"),
                ..Default::default()
            },
            player: Player::default(),
            speed: Speed(1),
            damage: Damage(0),
            health: Health(100),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Obstacle;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Blocking(pub bool);

#[derive(Clone, Bundle)]
pub struct ObstacleSpikeBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub obstacle: Obstacle,
    pub damage: Damage,
    pub health: Health,
    pub blocking: Blocking,
}

impl LdtkEntity for ObstacleSpikeBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Self {
        let mut blocking = Blocking(false);
        if let Some(blocking_field) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "IsBlocking".to_string())
        {
            if let FieldValue::Bool(blocking_value) = blocking_field.value {
                blocking = Blocking(blocking_value);
            }
        }

        let mut damage = Damage(0);
        if let Some(damage_field) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "Damage".to_string())
        {
            if let FieldValue::Int(Some(damage_value)) = damage_field.value {
                damage = Damage(damage_value);
            }
        }

        let mut health = Health(100);
        if let Some(health_field) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "Health".to_string())
        {
            if let FieldValue::Int(Some(health_value)) = health_field.value {
                health = Health(health_value);
            }
        }

        Self {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("ObstacleSpike.png"),
                ..Default::default()
            },
            obstacle: Obstacle::default(),
            damage,
            health,
            blocking,
        }
    }
}

#[derive(Clone, Bundle)]
pub struct ObstacleBlockBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub obstacle: Obstacle,
    pub damage: Damage,
    pub health: Health,
    pub blocking: Blocking,
}

impl LdtkEntity for ObstacleBlockBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Self {
        let mut blocking = Blocking(false);
        if let Some(blocking_field) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "IsBlocking".to_string())
        {
            if let FieldValue::Bool(blocking_value) = blocking_field.value {
                blocking = Blocking(blocking_value);
            }
        }

        let mut damage = Damage(0);
        if let Some(damage_field) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "Damage".to_string())
        {
            if let FieldValue::Int(Some(damage_value)) = damage_field.value {
                damage = Damage(damage_value);
            }
        }

        let mut health = Health(100);
        if let Some(health_field) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "Health".to_string())
        {
            if let FieldValue::Int(Some(health_value)) = health_field.value {
                health = Health(health_value);
            }
        }

        Self {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("ObstacleBlock.png"),
                ..Default::default()
            },
            obstacle: Obstacle::default(),
            damage,
            health,
            blocking,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Goal;

#[derive(Bundle)]
pub struct GoalBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub goal: Goal,
}

impl LdtkEntity for GoalBundle {
    fn bundle_entity(
        _: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("Goal.png"),
                ..Default::default()
            },
            goal: Goal::default(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct WallTile;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallTileBundle {
    wall: WallTile,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct ClimbableTile;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ClimableTileBundle {
    pub climbable: ClimbableTile,
}

// The actual falling ice, when the player goes underneath
// the falling ice tile
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct FallingIce;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct StaticIce;

#[derive(Clone, Bundle)]
pub struct FallingIceBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub damage: Damage,
    pub health: Health,
    pub static_ice: StaticIce,
    //pub falling_ice: FallingIce,
}

impl LdtkEntity for FallingIceBundle {
    fn bundle_entity(
        _: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("FallingIce.png"),
                ..Default::default()
            },
            health: Health(1),
            damage: Damage(100),
            static_ice: StaticIce::default(),
            //falling_ice: FallingIce::default(),
        }
    }
}
