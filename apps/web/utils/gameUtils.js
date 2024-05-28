// utils/gameUtils.js
import { SigningStargateClient } from '@cosmjs/stargate';
import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate';

const contractAddress = "YOUR_CONTRACT_ADDRESS_HERE";

export const connectKeplr = async () => {
  if (!window.getOfflineSigner || !window.keplr) {
    alert("Please install Keplr extension");
    return;
  }

  await window.keplr.enable("cosmoshub-4");
  const offlineSigner = window.getOfflineSigner("cosmoshub-4");
  const accounts = await offlineSigner.getAccounts();
  return accounts[0];
};

export const fetchInitialGameState = async () => {
  // Hardcoded initial state for now
  return Array.from({ length: 100 }, (_, i) => `Square ${i + 1}`);
};

export const fetchBlockTxHash = async () => {
  const response = await fetch("https://api.cosmos.network/blocks/latest");
  const data = await response.json();
  return data.block_id.hash;
};

export const calculateDiceRoll = (txHash) => {
  const number = parseInt(txHash.slice(0, 8), 16);
  return (number % 6) + 1;
};

export const movePlayer = (currentPosition, diceRoll) => {
  let newPosition = currentPosition + diceRoll;
  return newPosition;
};
