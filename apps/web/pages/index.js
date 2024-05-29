import React from 'react';
import { Box } from '@chakra-ui/react';
import Game from '../components/Game';

const Home = () => {
  return (
    <Box textAlign="center" py={10}>
      <Game />
    </Box>
  );
};

export default Home;
