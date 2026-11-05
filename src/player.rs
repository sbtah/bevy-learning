use bevy::prelude::*;

// Atlas constants
const TILE_SIZE: u32 = 64; // 64x64 tiles
const WALK_FRAMES: usize = 9; // 9 columns per walking row?
const MOVE_SPEED: f32 = 140.0; // pixels per second?
const ANIM_DT: f32 = 0.1; // seconds per framer (~10 FPS?)

#[derive(Component)]
struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct AnimationState {
    facing: Facing,
    moving: bool,
    was_moving: bool,
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    // What will be the type here?
    let texture = asset_server.load("character-spritesheet.png");

    // This created a game `world` ?
    // Do I need it here?
    let layout = atlas_layout.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE),
        WALK_FRAMES as u32,
        12,
        None,
        None,
    ));

    let facing = Facing::Down;
    let start_index = atlas_index_for(facing, 0); // Not yet implemented

    // What is happening here?
    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout,
                index: start_index,
            },
        ),
        Transform::from_translation(Vec3::ZERO),
        Player,
        AnimationState {
            facing,
            moving: false,
            was_moving: false,
        },
        AnimationTimer(Timer::from_seconds(ANIM_DT, TimerMode::Repeating)),
    ));
}

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&mut Transform, &mut AnimationState), With<Player>>,
) {
    // Is this is unpacking?
    // What is Transform?
    // How can we do Query for Transform and AnimationState above that are attached to Player?
    // How Player got transform? from Component ?
    let Ok((mut transform, mut anim)) = player.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    // Why all these if statements? Shouldn't this be else if?
    // What if press 2 buttons at once?
    if input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }

    if input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }

    if input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    //  This mean if player moved?
    if direction != Vec2::ZERO {
        // What is this?
        // I understand this is for diagonal right?
        // What this calculation return Vector2? again?
        let delta = direction.normalize() * MOVE_SPEED * time.delta_secs();

        // What is happening here?
        // We access translation field on struct which has x and we increment by delta.x
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
        anim.moving = true;

        // if absolute direction on x axis is bigger then absolute direction on y axis?
        if direction.x.abs() > direction.y.abs() {
            // Ok this seems simple if direction on x is bigger than 0 than we are facing Right else Left
            anim.facing = if direction.x > 0.0 {
                Facing::Right
            } else {
                Facing::Left
            };
        } else {
            anim.facing = if direction.y > 0.0 {
                Facing::Up
            } else {
                Facing::Down
            };
        }
    } else {
        anim.moving = false;
    }
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut AnimationTimer, &mut Sprite), With<Player>>,
) {
    let Ok((mut anim, mut timer, mut sprite)) = query.single_mut() else {
        warn!("No Player Entity with found");
        return;
    };

    // So atlas is basically a png file?
    let atlas = match sprite.texture_atlas.as_mut() {
        Some(a) => a,
        None => {
            warn!("Failed with loading sprite as TextureAtlas?");
            return;
        }
    };
    // This gives a row according to mapping
    // For example Facing::Up will give 8
    // So target_row is 8 at this point.
    let target_row = row_zero_based(anim.facing);

    //  What are these values for ? current_*?
    // what is atlas.index? why we are doing modulo and division here?
    // What are we getting from it?
    let mut current_col = atlas.index % WALK_FRAMES;
    let mut current_row = atlas.index / WALK_FRAMES;

    // What are we checking here?
    // When current row != target row?
    if current_row != target_row {
        atlas.index = row_start_index(anim.facing);
        // For starting a movement current collumn should always be 0 right?
        current_col = 0;
        current_row = target_row; // IDE complain that is is unused code? why do I even need current_row?
        timer.reset();
    }
    // just_started = anim in moving = true and was_moving = false
    let just_started = anim.moving && !anim.was_moving;
    // reverse...
    let just_stopped = !anim.moving && anim.was_moving;

    // If AnimationState has moving = true;
    if anim.moving {
        // If animation just stated:
        if just_started {
            // Shouldn't we set was_moving to false?
            //  What is happening here?
            let row_start = row_start_index(anim.facing);
            // What are we calculating here?
            // I think what I'm missing here is understanding that this will run each frame/tick
            // so current col increment with each button press?
            let next_col = (current_col + 1) % WALK_FRAMES;
            atlas.index = row_start + next_col;
        } else {
            // What is this?
            timer.tick(time.delta());
            // What are we checking here?
            if timer.just_finished() {
                let row_start = row_start_index(anim.facing);
                let next_col = (current_col + 1) % WALK_FRAMES;
                atlas.index = row_start + next_col;
            }
        }
    //
    } else if just_stopped {
        timer.reset();
    }
    anim.was_moving = anim.moving;
}

fn row_start_index(facing: Facing) -> usize {
    // Why this returns 72 for Facing::UP ?
    // Atlas will count all frames up to index 72 to get the frame for Facing::UP?
    row_zero_based(facing) * WALK_FRAMES
}

fn atlas_index_for(facing: Facing, frame_in_row: usize) -> usize {
    // Why do we do .min() on frame_in_row?
    // Why we do WALK_FRAMES - 1?
    //
    row_start_index(facing) + frame_in_row.min(WALK_FRAMES - 1)
}

fn row_zero_based(facing: Facing) -> usize {
    // Ok so this maps the row in .png file according to position
    // So if player is facing up the row is 8... etc.
    match facing {
        Facing::Up => 8,
        Facing::Left => 9,
        Facing::Down => 10,
        Facing::Right => 11,
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, animate_player));
    }
}
