//! # Pebbles Game Test Module
//!
//! 这个模块包含了对 `pebbles_game_io` 游戏逻辑的单元测试，用于验证游戏的初始化、玩家行为处理、
//! 游戏状态管理以及重启游戏等功能的正确性。

use gtest::{Program, System}; // 引入gtest框架的Program和System类
use pebbles_game_io::*; // 引入游戏逻辑和数据结构

/// 测试 `pebbles_game_io` 的核心游戏逻辑。
///
/// 该测试验证以下几点：
/// - 游戏可以正确初始化。
/// - 玩家能够执行回合操作，并且游戏状态正确更新。
/// - 玩家可以放弃游戏，游戏状态能够正确标识赢家。
/// - 游戏可以被重启，重启后状态应该符合新的游戏设置。
#[test]
fn test_game_logic() {
    let system = System::new(); // 初始化测试系统
    let program = Program::current(&system); // 获取当前的程序实例
    let pid = program.id(); // 获取程序ID
    let sender_id = 100; // 定义一个发送者ID

    // 初始化游戏状态
    let init_message = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 4,
    };

    // 发送初始化消息到程序
    program.send(sender_id, init_message);

    // 从程序获取并验证初始状态
    let state: GameState = program.read_state(pid)
        .expect("Failed to get the initial state of the game");
    assert!(state.pebbles_remaining <= 10);
    assert_eq!(state.pebbles_count, 10);
    assert_eq!(state.max_pebbles_per_turn, 4);
    assert_eq!(state.difficulty, DifficultyLevel::Easy);
    assert_eq!(state.winner, None::<Player>);

    // 用户回合：尝试拿走2个石子
    program.send(sender_id, PebblesAction::Turn(2));

    // 验证用户回合后的游戏状态
    let state: GameState = program.read_state(pid)
        .expect("Failed to get the state of the game after user's turn");
    assert!(state.pebbles_remaining <= 8);

    // 用户选择放弃游戏
    program.send(sender_id, PebblesAction::GiveUp);

    // 验证用户放弃后的游戏状态
    let state: GameState = program.read_state(pid)
        .expect("Failed to get the state of the game after giving up");
    assert_eq!(state.winner, Some(Player::Program));

    // 发送重启游戏的指令
    let restart_message = PebblesAction::Restart {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 15,
        max_pebbles_per_turn: 10,
    };
    program.send(sender_id, restart_message);

    // 验证重启后的游戏状态
    let state: GameState = program.read_state(pid)
        .expect("Failed to get the state of the game after restart");
    assert!(state.pebbles_remaining <= 15);
    assert_eq!(state.pebbles_count, 15);
    assert_eq!(state.max_pebbles_per_turn, 10);
    assert_eq!(state.difficulty, DifficultyLevel::Hard);
    assert_eq!(state.winner, None);
}
