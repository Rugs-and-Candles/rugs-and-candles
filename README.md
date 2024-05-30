# Rugs and Candles: The Interchain Board Game

![image](https://github.com/Rugs-and-Candles/rugs-and-candles/assets/80094928/7d11a646-fc51-4b98-aa99-cfb337fc35c8)

## Overview

Rugs and Candles is an interactive board game built with React and Chakra UI, integrated with Cosmos Kit for seamless wallet connections. The game features a dynamic board that spans multiple Cosmos chains, allowing players to experience various chain-specific actions and events.

## Features

- **Wallet Integration**: Connect to multiple Cosmos-based wallets using Cosmos Kit.
- **Dynamic Board**: A board that adapts to different Cosmos chains, showcasing unique events and actions.
- **Interactive Gameplay**: Includes actions like lending, swapping, and more, represented by icons on specific tiles.
- **Candles and Ladders**: Navigate the board using green candles (ladders) to move up and red candles (snakes) to move down.
- **Winning Condition**: Reach the end of the board to win the game.

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/Rugs-and-Candles/rugs-and-candles.git
   cd rugs-and-candles
   ```

2. **Install dependencies**:

  ```bash
  npm install
  ```

3. **Start the development server**:

  ```bash
  npm run dev
  ```

## Usage
- **Connect Wallet**: Use the Connect Keplr button to connect your wallet.
- **Roll Dice**: Click the Roll Dice button to move your player across the board.
- **Navigate**: The player moves according to the dice roll, with additional moves dictated by landing on candles (ladders/snakes).

## Game Mechanics
- **Candles**: Green candles move the player up, red candles move the player down.
- **Action Tiles**: Some tiles trigger specific actions, indicated by icons. For example:

- **Tile 3**: Mint USK
- **Tile 5**: Lending on Kujira GHOST
- **Tile 7**: Bid on Orca
- **Tile 9**: Swap on FIN
- **Tile 16**: Proposal 16
- **Tile 23**: Leverage trade on Levana 
- **Tile 28**: Provide liquidity on Osnmosis
- **Tile 32**: Instantiate a smart contract
- **Tile 36**: Steady Lads, Deploying Funds
- **Tile 44**: Mint an NFT
- **Tile 50**: Finish the game

## Board Layout
The board layout is divided into sections, each representing a different chain with unique backgrounds and icons:

- **Kuji**: First 10 tiles
- **Juno**: Tiles 11-20
- **Osmosis**: Tiles 21-30
- **Luna**: Tiles 31-40
- **Stargaze**: Tiles 41-50

## Abstract SDK Integration
This project leverages the Abstract SDK, a comprehensive development platform for CosmWasm. The Abstract SDK provides a modular architecture and an extensive range of tools that streamline the development process for blockchain applications. Key components include:

- **Abstract SDK**: A Rust library for on-chain operations.
- **cw-orchestrator**: Facilitates smart contract deployment.
- **Abstract JS**: Enables browser-based smart contract interactions.

The repository structure includes:
- **apps**: Contains the front-end applications.
- **contracts**: Contains the CosmWasm smart contracts.
- **packages**: Shared libraries and utilities.
- **schema**: JSON schemas for the contracts.
- **scripts**: Scripts for various development tasks.

## Contributing
Contributions are welcome! Please fork the repository and submit pull requests for any enhancements or bug fixes. Let's make the Interchain Fun again!

## License
This project is licensed under the MIT License.
