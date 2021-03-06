use core::State;
use core::command::{self, Command};
use core::movement;
use core::map;
use core::{Attacks, Moves};

pub fn check(state: &State, command: &Command) -> Result<(), Error> {
    match *command {
        Command::Create(ref command) => check_create(state, command),
        Command::MoveTo(ref command) => check_move_to(state, command),
        Command::Attack(ref command) => check_attack(state, command),
        Command::EndTurn(ref command) => check_end_turn(state, command),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    NotEnoughMovePoints,
    BadActorId,
    BadTargetId,
    ObjectAlreadyExists,
    TileIsOccupied,
    DistanceIsTooBig,
    CanNotCommandEnemyUnits,
    NotEnoughMoves,
    NotEnoughAttacks,
}

fn check_move_to(state: &State, command: &command::MoveTo) -> Result<(), Error> {
    let unit = match state.unit_opt(command.id) {
        Some(unit) => unit,
        None => return Err(Error::BadActorId),
    };
    if unit.player_id != state.player_id() {
        return Err(Error::CanNotCommandEnemyUnits);
    }
    if unit.moves == Moves(0) {
        return Err(Error::NotEnoughMoves);
    }
    let cost = movement::path_cost(state, unit, &command.path);
    if cost > unit.move_points {
        return Err(Error::NotEnoughMovePoints);
    }
    Ok(())
}

fn check_create(state: &State, command: &command::Create) -> Result<(), Error> {
    if state.unit_opt(command.id).is_some() {
        return Err(Error::ObjectAlreadyExists);
    }
    if command.unit.player_id != state.player_id() {
        return Err(Error::CanNotCommandEnemyUnits);
    }
    if !state.units_at(command.unit.pos).is_empty() {
        return Err(Error::TileIsOccupied);
    }
    Ok(())
}

fn check_attack(state: &State, command: &command::Attack) -> Result<(), Error> {
    let attacker = match state.unit_opt(command.attacker_id) {
        Some(unit) => unit,
        None => return Err(Error::BadActorId),
    };
    if attacker.player_id != state.player_id() {
        return Err(Error::CanNotCommandEnemyUnits);
    }
    let target = match state.unit_opt(command.target_id) {
        Some(unit) => unit,
        None => return Err(Error::BadTargetId),
    };
    if attacker.attacks == Attacks(0) {
        return Err(Error::NotEnoughAttacks);
    }
    let dist = map::distance_hex(attacker.pos, target.pos);
    let max_dist = 1;
    if dist > max_dist {
        return Err(Error::DistanceIsTooBig);
    }
    Ok(())
}

fn check_end_turn(_: &State, _: &command::EndTurn) -> Result<(), Error> {
    Ok(())
}
