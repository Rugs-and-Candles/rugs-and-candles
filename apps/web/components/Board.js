import React, { useEffect } from 'react';
import { Box, SimpleGrid, Image, Text, keyframes } from '@chakra-ui/react';

const chainThemes = {
  neutron: { bg: 'blue.800', icon: '/icons/neutron.png' },
  kuji: { bg: 'red.100', icon: '/icons/kuji.png' },
  juno: { bg: 'peachpuff', icon: '/icons/juno.png' },
  archway: { bg: 'yellow.100', icon: '/icons/archway.png' },
  osmosis: { bg: 'purple.500', icon: '/icons/osmosis.png' },
};

const candles = [
  { type: 'green', start: 2, span: 3 },
  { type: 'red', start: 14, span: 2 },
  { type: 'green', start: 27, span: 3 },
  { type: 'green', start: 19, span: 3 },
  { type: 'red', start: 35, span: 3 },
  { type: 'red', start: 41, span: 3 },
];

const getChainFromIndex = (index) => {
  if (index < 10) return 'neutron';
  if (index < 20) return 'kuji';
  if (index < 30) return 'juno';
  if (index < 40) return 'archway';
  return 'osmosis';
};

const slideAnimation = keyframes`
  from { transform: translateY(0); }
  to { transform: translateY(-100%); }
`;

const Board = ({ gameState, playerPosition, onPlayerMove }) => {
  const playerIcon = '/icons/player.png';

  useEffect(() => {
    const handlePlayerMove = () => {
      const currentCandle = candles.find(c => c.start === playerPosition);
      if (currentCandle) {
        const newPosition = currentCandle.type === 'green'
          ? playerPosition + currentCandle.span
          : playerPosition - currentCandle.span;
        setTimeout(() => onPlayerMove(newPosition), 1000);
      }
    };

    handlePlayerMove();
  }, [playerPosition, onPlayerMove]);

  return (
    <Box display="flex" flexDirection="column-reverse" position="relative">
      {Array.from({ length: 5 }, (_, rowIndex) => {
        const chain = getChainFromIndex(rowIndex * 10);
        return (
          <Box key={rowIndex} display="flex" alignItems="center" position="relative">
            <Image src={chainThemes[chain].icon} alt={`${chain} icon`} boxSize="80px" mr={4} />
            <SimpleGrid columns={10} spacing={4} flex="1">
              {gameState.slice(rowIndex * 10, (rowIndex + 1) * 10).map((square, index) => {
                const globalIndex = rowIndex * 10 + index + 1; // Adjusting index to match tile numbers
                const candle = candles.find(c => c.start === globalIndex);
                const isPlayerHere = globalIndex === playerPosition;
                return (
                  <Box
                    key={index}
                    position="relative"
                    border="1px"
                    borderColor="gray.200"
                    p={4}
                    bg={chainThemes[chain].bg}
                    borderRadius="md"
                    boxShadow="md"
                    width="100px"
                    height="100px"
                    transition="all 0.3s"
                    _hover={{ boxShadow: 'xl', transform: 'scale(1.05)' }}
                  >
                    {candle && (
                      <Image
                        src={`/icons/${candle.type}candle.png`}
                        alt={`${candle.type === 'green' ? 'Ladder' : 'Snake'}`}
                        position="absolute"
                        top={candle.type === 'green' ? 'auto' : '0'}
                        bottom={candle.type === 'green' ? '0' : 'auto'}
                        left="50%"
                        transform="translateX(-50%)"
                        width="70px"
                        height="auto"
                        maxHeight={`${candle.span * 100}px`}
                        opacity="0.85"
                        zIndex="10"
                        style={{
                          objectFit: 'cover',
                          height: `${candle.span * 100}px`,
                        }}
                      />
                    )}
                    <Text fontFamily="Arial, sans-serif" fontSize="lg" fontWeight="bold" color={chain === 'neutron' ? 'white' : 'black'} zIndex="modal">
                      {globalIndex}
                    </Text>
                    {isPlayerHere && (
                      <Image
                        src={playerIcon}
                        alt="Player"
                        position="absolute"
                        top="50%"
                        left="50%"
                        transform="translate(-50%, -50%)"
                        boxSize="60px"
                        zIndex="modal"
                        animation={`${slideAnimation} 1s ease-in-out`}
                      />
                    )}
                  </Box>
                );
              })}
            </SimpleGrid>
          </Box>
        );
      })}
    </Box>
  );
};

export default Board;
