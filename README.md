# Pebbles Game Smart Contract

## 概述
本项目是一个基于 Web3 技术的石子游戏智能合约。玩家通过智能合约调用参与游戏，轮流拿走一定数量的石子，最终目标是成为最后一个拿走石子的玩家。

## 特点
- **智能合约交互**：玩家通过区块链交互方式参与游戏。
- **可配置游戏设置**：支持自定义游戏难度、石子总数和每回合最多可拿走的石子数。
- **适应不同难度**：提供不同难度级别的游戏，通过智能算法调整游戏策略。

## 功能说明
- **初始化游戏** (`init`): 根据传入的参数初始化游戏状态，包括难度、石子总数和每回合拿走的石子上限。
- **处理玩家操作** (`handle`): 根据玩家的行动（拿走石子、放弃或重启游戏）更新游戏状态。
- **查询游戏状态** (`state`): 提供当前游戏的状态信息，如剩余石子数和当前的赢家。

## 结构说明
- **PebblesInit**: 初始化游戏的参数结构体。
- **PebblesAction**: 玩家可以执行的操作类型，如进行回合、放弃或重启游戏。
- **GameState**: 记录当前游戏状态的结构体，包括剩余石子数、游戏难度等信息。

## 使用指南
1. **合约部署**：将智能合约编译为 WebAssembly 后部署到 Gear 协议。
2. **游戏初始化**：玩家通过调用 `init` 函数并传入相应参数开始新游戏。
3. **玩家操作**：玩家通过调用 `handle` 函数，传入自己的操作，根据合约逻辑处理相应的行动。
4. **状态查询**：通过调用 `state` 函数来查询当前的游戏状态。

## 开发背景
此项目旨在通过实际的合约开发，教育开发者理解和构建基于 Gear 协议的智能合约游戏。游戏逻辑简单明了，便于新手理解和上手操作，同时也提供了与区块链技术交互的实践机会。
