import { Box, Tabs, TabList, TabPanels, Tab, TabPanel } from '@chakra-ui/react';
import React, { useEffect, useState } from 'react'
import { IoLogInOutline } from 'react-icons/io5'
import { BoxAction, CreateWallet } from '@/pages/WalletPage/components';

interface CustomResponse {
  message: string
}

const WalletPage: React.FC = () => {
  const [hasWallet, setHasWallet] = useState<boolean>(false);

  const WalletInstance = () => {
    return (
      <>
        <Tabs isLazy>
          <TabList>
            <Tab>Balances</Tab>
            <Tab>Transfer</Tab>
            <Tab>Receive</Tab>
          </TabList>

          <TabPanels>
            <TabPanel>
              {/* Balances UI */}
              <Box p={4} bg="gray.200" borderRadius="md">
                {/* Balances content goes here */}
                <p>Display your balances here</p>
              </Box>
            </TabPanel>

            <TabPanel>
              {/* Transfer UI */}
              <Box p={4} bg="gray.200" borderRadius="md">
                {/* Transfer content goes here */}
                <p>Transfer crypto here</p>
              </Box>
            </TabPanel>

            <TabPanel>
              {/* Receive UI */}
              <Box p={4} bg="gray.200" borderRadius="md">
                {/* Receive content goes here */}
                <p>Receive crypto here</p>
              </Box>
            </TabPanel>
          </TabPanels>
        </Tabs>
      </>
    )
  }

  return (
    <BoxAction title="Wallet" icon={IoLogInOutline}>
      {hasWallet ? <WalletInstance /> : <CreateWallet />}
    </BoxAction>
  )
}

export default WalletPage;
