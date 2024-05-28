import React from 'react';
import { Box, SimpleGrid } from '@chakra-ui/react';

const Board = ({ gameState }) => {
  return (
    <SimpleGrid columns={10} spacing={4}>
      {gameState.map((square, index) => (
        <Box key={index} border="1px" borderColor="gray.200" p={4}>
          {square}
        </Box>
      ))}
    </SimpleGrid>
  );
};

export default Board;
