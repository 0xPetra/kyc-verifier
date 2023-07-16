import { Box, Tabs, TabList, TabPanels, Tab, TabPanel, Text } from '@chakra-ui/react';
import React, { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { IoLogInOutline } from 'react-icons/io5'
import { useWalletStore } from '../../stores/useWalletStore';  // replace with your actual store path
import { BoxAction } from '@/pages/IdentityPage/components';

interface CustomResponse {
  message: string
}

const IdentityPage: React.FC = () => {
  const { eraPk, setEraPk } = useWalletStore((state) => ({
    eraPk: state.eraPk,
    setEraPk: state.setEraPk
  }));

  const IdentityInstance = () => {
    return (
      <>
        <Tabs isLazy>
          <TabList>
            <Tab>Identity</Tab>
          </TabList>

          <TabPanels>
            <TabPanel>
              {/* Balances UI */}
              <Box p={4} bg="gray.200" borderRadius="md">
                {/* Balances content goes here */}
                <p>Display your balances here</p>
              </Box>
            </TabPanel>
          </TabPanels>
        </Tabs>
      </>
    )
  }

  return (
    <BoxAction title="Wallet" icon={IoLogInOutline}>
      {eraPk ? <IdentityInstance /> : <Text>You must create a wallet first</Text>}
    </BoxAction>
  )
}

export default IdentityPage;
