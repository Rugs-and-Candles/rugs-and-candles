import React from 'react';
import { Button } from '@chakra-ui/react';

const DiceRoll = ({ rollDice }) => {
  return (
    <Button onClick={rollDice} colorScheme="blue" size="lg">
      Roll Dice
    </Button>
  );
};

export default DiceRoll;
