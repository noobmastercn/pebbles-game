//! # Pebbles Game Module
//!
//! 本模块实现了一个简单的石子游戏（Pebbles Game），游戏的核心逻辑包括初始化游戏、处理玩家操作和管理游戏状态。
//! 游戏通过外部调用（如智能合约调用）来接收指令，并根据游戏规则更新状态。

#![no_std] // 指出本 crate 不使用标准库，适用于裸机或嵌入式系统。

use pebbles_game_io::*; // 引入游戏相关的数据结构和类型。
use gstd::{exec, msg};  // 引入用于执行和消息传递的库。

static mut PEBBLES_GAME: Option<GameState> = None; // 全局静态可变状态，存储当前的游戏状态。

/// 初始化游戏状态。
/// 加载外部消息作为游戏初始化参数，设置游戏的初始状态。
#[no_mangle]
pub extern "C" fn init() {
    let init_message: PebblesInit = msg::load().expect("Can't load init message"); // 加载初始化消息。
    let first_player = get_first_player(); // 随机确定首个行动的玩家。
    let pebbles_remaining = get_init_pebbles_remain(
        init_message.pebbles_count,
        init_message.max_pebbles_per_turn,
        first_player.clone(),
        init_message.difficulty,
    ); // 计算初始剩余石子数。

    let initial_state = GameState { // 创建游戏初始状态。
        pebbles_count: init_message.pebbles_count,
        max_pebbles_per_turn: init_message.max_pebbles_per_turn,
        pebbles_remaining,
        difficulty: init_message.difficulty,
        first_player,
        winner: None,
    };

    unsafe {
        PEBBLES_GAME = Some(initial_state); // 安全地更新全局状态。
    }
}

/// 处理玩家操作。
/// 根据玩家的行动更新游戏状态，可以是玩家的回合、放弃或重启游戏。
#[no_mangle]
pub extern "C" fn handle() {
    let action: PebblesAction = msg::load().expect("Unable to decode PebblesAction"); // 加载玩家操作。
    let mut game_state = unsafe { PEBBLES_GAME.take().expect("Game state is not initialized") }; // 取出当前游戏状态。

    match action {
        PebblesAction::Turn(pebbles_taken) => { // 玩家回合：尝试拿走一定数量的石子。
            if pebbles_taken > game_state.max_pebbles_per_turn || pebbles_taken == 0 {
                panic!("Invalid number of pebbles taken"); // 操作无效时触发 panic。
            }
            if pebbles_taken > game_state.pebbles_remaining {
                panic!("Not enough pebbles remaining"); // 石子不足时触发 panic。
            }
            game_state.pebbles_remaining -= pebbles_taken; // 更新剩余石子数。
            if game_state.pebbles_remaining == 0 {
                game_state.winner = Some(Player::User); // 如果石子取完，玩家获胜。
                msg::reply(PebblesEvent::Won(Player::User), 0)
                    .expect("Failed to reply with Won event"); // 发送获胜事件。
            } else {
                update_game_state(&mut game_state); // 如果游戏未结束，更新状态。
            }
        }
        PebblesAction::GiveUp => { // 玩家放弃游戏。
            game_state.winner = Some(Player::Program); // 程序获胜。
            msg::reply(PebblesEvent::Won(Player::Program), 0)
                .expect("Failed to reply with Won event"); // 发送获胜事件。
        }
        PebblesAction::Restart { // 重启游戏。
            difficulty,
            pebbles_count,
            max_pebbles_per_turn,
        } => {
            game_state = restart_game(difficulty, pebbles_count, max_pebbles_per_turn); // 根据指定参数重置游戏状态。
        }
    }

    unsafe {
        PEBBLES_GAME = Some(game_state); // 保存更新后的游戏状态。
    }
}

/// 返回当前游戏状态。
/// 用于外部查询当前游戏的详细状态。
#[no_mangle]
pub extern "C" fn state() {
    let game_state = unsafe { PEBBLES_GAME.clone().expect("Game state is not initialized") };
    msg::reply(game_state, 0).expect("Failed to share state"); // 回复当前状态。
}

/// 生成一个随机的 u32 整数。
/// 使用消息ID作为随机数生成的盐值。
///
/// # 返回
/// 返回一个随机生成的 u32 整数。
fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

/// 根据游戏难度和当前的游戏状态计算程序应该拿走的石子数。
///
/// # 参数
/// * `pebbles_remaining` - 游戏中剩余的石子数。
/// * `max_pebbles_per_turn` - 每个回合中，玩家最多可以拿走的石子数。
/// * `difficulty` - 游戏难度。
///
/// # 返回
/// 返回程序应该拿走的石子数。
fn get_contract_pebbles_taken(
    pebbles_remaining: u32,
    max_pebbles_per_turn: u32,
    difficulty: DifficultyLevel,
) -> u32 {
    match difficulty {
        DifficultyLevel::Easy => {
            let random_number = get_random_u32();
            (random_number % max_pebbles_per_turn + 1).min(pebbles_remaining)
        },
        DifficultyLevel::Hard => {
            let optimal_pebbles_taken = pebbles_remaining % (max_pebbles_per_turn + 1);
            if optimal_pebbles_taken == 0 {
                1
            } else {
                optimal_pebbles_taken
            }
        }
    }
}

/// 随机确定首个行动的玩家。
///
/// # 返回
/// 返回随机确定的首个行动玩家。
fn get_first_player() -> Player {
    let random_number = get_random_u32();
    if random_number % 2 == 0 {
        Player::User
    } else {
        Player::Program
    }
}

/// 根据游戏初始设置计算初始剩余的石子数。
///
/// # 参数
/// * `pebbles_count` - 游戏开始时的石子总数。
/// * `max_pebbles_per_turn` - 每个回合中，玩家最多可以拿走的石子数。
/// * `first_player` - 首个行动的玩家。
/// * `difficulty` - 游戏难度。
///
/// # 返回
/// 返回计算得出的初始剩余石子数。
fn get_init_pebbles_remain(
    pebbles_count: u32,
    max_pebbles_per_turn: u32,
    first_player: Player,
    difficulty: DifficultyLevel,
) -> u32 {
    let mut pebbles_remaining = pebbles_count;

    if first_player == Player::Program {
        let counter_pebbles_taken = get_contract_pebbles_taken(pebbles_count, max_pebbles_per_turn, difficulty);
        pebbles_remaining -= counter_pebbles_taken;
        msg::reply(PebblesEvent::CounterTurn(counter_pebbles_taken), 0)
            .expect("Failed to reply with CounterTurn event");
    }

    pebbles_remaining
}

/// 更新游戏状态。
///
/// # 参数
/// * `game_state` - 可变引用到当前的游戏状态。
fn update_game_state(game_state: &mut GameState) {
    let counter_pebbles_taken = get_contract_pebbles_taken(
        game_state.pebbles_remaining,
        game_state.max_pebbles_per_turn,
        game_state.difficulty,
    );

    game_state.pebbles_remaining -= counter_pebbles_taken;

    if game_state.pebbles_remaining == 0 {
        game_state.winner = Some(Player::Program);
        msg::reply(PebblesEvent::Won(Player::Program), 0)
            .expect("Failed to reply with Won event");
    } else {
        msg::reply(PebblesEvent::CounterTurn(counter_pebbles_taken), 0)
            .expect("Failed to reply with CounterTurn event");
    }
}

/// 重置游戏状态。
///
/// # 参数
/// * `difficulty` - 游戏难度。
/// * `pebbles_count` - 游戏开始时的石子总数。
/// * `max_pebbles_per_turn` - 每个回合中，玩家最多可以拿走的石子数。
///
/// # 返回
/// 返回重置后的游戏状态。
fn restart_game(difficulty: DifficultyLevel, pebbles_count: u32, max_pebbles_per_turn: u32) -> GameState {
    let first_player = get_first_player();
    let pebbles_remaining = get_init_pebbles_remain(
        pebbles_count,
        max_pebbles_per_turn,
        first_player.clone(),
        difficulty,
    );

    GameState {
        pebbles_count,
        max_pebbles_per_turn,
        pebbles_remaining,
        difficulty,
        first_player,
        winner: None,
    }
}
