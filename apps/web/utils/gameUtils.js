import { SigningStargateClient } from '@cosmjs/stargate';
import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate';

const contractAddress = "YOUR_CONTRACT_ADDRESS_HERE";

export const connectKeplr = async () => {
  if (!window.getOfflineSigner || !window.keplr) {
    alert("Please install Keplr extension");
    return;
  }

  await window.keplr.enable("neutron-1");
  const offlineSigner = window.getOfflineSigner("neutron-1");
  const accounts = await offlineSigner.getAccounts();
  return accounts[0];
};

export const fetchInitialGameState = async () => {
  return Array.from({ length: 50 }, (_, i) => `Square ${i + 1}`);
};

export const fetchRandomNumber = () => {
  return Math.floor(Math.random() * 6) + 1;
};

export const movePlayer = (currentPosition, diceRoll) => {
  let newPosition = currentPosition + diceRoll;

  // Handle snakes and ladders
  const snakesAndLadders = {
    2: 22,
    14: 4,
    19: 39,
    27: 47,
    35: 15,
    41: 21,
  };

  return snakesAndLadders[newPosition] || newPosition;
};
