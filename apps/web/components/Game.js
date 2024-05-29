import React, { useState, useEffect } from 'react';
import { VStack, Box, Text, Button, Image, Flex, Heading } from '@chakra-ui/react';
import { CHAIN_NAME, Wallet as WalletButton } from './wallet/Wallet';
import Board from './Board';
import { connectKeplr, fetchRandomNumber, movePlayer, fetchInitialGameState } from '../utils/gameUtils';

const Game = () => {
  const [gameState, setGameState] = useState([]);
  const [playerPosition, setPlayerPosition] = useState(1);
  const [account, setAccount] = useState(null);
  const [chainName, setChainName] = useState(CHAIN_NAME);

  function onChainChange(chainName) {
    setChainName(chainName);
  }
  
  useEffect(() => {
    const initGame = async () => {
      const initialState = await fetchInitialGameState();
      setGameState(initialState);
    };
    initGame();
  }, []);

  const rollDice = () => {
    const diceRoll = fetchRandomNumber();
    const newPosition = movePlayer(playerPosition, diceRoll);
    setPlayerPosition(newPosition);
  };

  const connectWallet = async () => {
    const account = await connectKeplr();
    setAccount(account);
  };

  return (
    <Flex
      direction="column"
      align="center"
      justify="center"
      minH="100vh"
      backgroundImage="url('/icons/board-background.png')"
      backgroundSize="cover"
      backgroundPosition="center"
    >
      <VStack spacing={4} bg="rgba(255, 255, 255, 0.6)" p={4} borderRadius="md" boxShadow="md">
        <Flex align="center" justify="center" position="relative" width="100%">
          <Heading as="h1" size="lg" textAlign="center" bg="none">
            Rugs and Candles: The Interchain Board Game
          </Heading>
        </Flex>
        <WalletButton 
          colorScheme="teal"
          size="lg"
          chainName={chainName}
          onChainChange={onChainChange}
        >
          Connect Keplr
        </WalletButton>
        {account && <Text>Connected: {account.address}</Text>}
        <Board gameState={gameState} playerPosition={playerPosition} onPlayerMove={setPlayerPosition} />
        <Button onClick={rollDice} mt={4} colorScheme="blue">Roll Dice</Button>
        <Box>
          <Text>Player Position: {playerPosition}</Text>
        </Box>
      </VStack>
    </Flex>
  );
};

export default Game;
