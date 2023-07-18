import React from 'react'
import { Box, Heading, Flex, Text } from '@chakra-ui/react'
import { Technology, TechStack } from '@/pages/WelcomePage/components';

const WelcomePage: React.FC = () => {

  return (
    <Flex justifyContent="center" flexDirection="column" h="100%">
      <Box as="section" whiteSpace="nowrap" textAlign="center">
        <Heading size="2xl">Welcome to</Heading>
        <Heading size="lg" color="primary.400">zkWallet</Heading>
      </Box>

      <Box as="section" whiteSpace="nowrap" textAlign="center">
        <Text>
          A desktop zkSync Wallet with privacy in mind.

          <Heading size="md" mt={5}>
            Present Features
          </Heading>
          <ul>
            <li>Create ZkSync Era Wallet</li>
            <li>Retrieve balance</li>
            <li>Transfer between accounts</li>
            <li>Create KYC proof</li>
          </ul>

          <Heading size="md" mt={5}>
            Potential features
          </Heading>
          <ul>
            <li>Limit daily spending</li>
            <li>Submit proofs on-chain</li>
          </ul>
        </Text>
      </Box>
    </Flex>

  )
}

export default WelcomePage;
