import React, { useState, useEffect } from 'react';
import { VStack, Box, Text, Button } from '@chakra-ui/react';
import Board from './Board';
import DiceRoll from './DiceRoll';
import { connectKeplr, fetchBlockTxHash, movePlayer, calculateDiceRoll, fetchInitialGameState } from '../utils/gameUtils';

const Game = () => {
  const [gameState, setGameState] = useState([]);
  const [playerPosition, setPlayerPosition] = useState(0);
  const [account, setAccount] = useState(null);

  useEffect(() => {
    const initGame = async () => {
      const initialState = await fetchInitialGameState();
      setGameState(initialState);
    };
    initGame();
  }, []);

  const rollDice = async () => {
    const txHash = await fetchBlockTxHash();
    const diceRoll = calculateDiceRoll(txHash);
    const newPosition = movePlayer(playerPosition, diceRoll);
    setPlayerPosition(newPosition);
  };

  const connectWallet = async () => {
    const account = await connectKeplr();
    setAccount(account);
  };

  return (
    <VStack spacing={4}>
      <Button onClick={connectWallet} colorScheme="teal" size="lg">
        Connect Keplr
      </Button>
      {account && <Text>Connected: {account.address}</Text>}
      <Board gameState={gameState} />
      <DiceRoll rollDice={rollDice} />
      <Box>
        <Text>Player Position: {playerPosition}</Text>
      </Box>
    </VStack>
  );
};

export default Game;
