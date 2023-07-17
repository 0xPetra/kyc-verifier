import { Text } from '@chakra-ui/react';
import React from 'react'
import { IoLogInOutline } from 'react-icons/io5'
import { useWalletStore } from '../../stores/useWalletStore';  // replace with your actual store path
import { BoxAction } from '@/pages/IdentityPage/components';
import { VerifyUser } from '@/pages/IdentityPage/components/index';
interface CustomResponse {
  message: string
}

const IdentityPage: React.FC = () => {
  const { eraPk, setEraPk } = useWalletStore((state) => ({
    eraPk: state.eraPk,
    setEraPk: state.setEraPk
  }));

  return (
    <BoxAction title="Identity" icon={IoLogInOutline}>
      {eraPk ? <VerifyUser /> : <Text>You must create a wallet first</Text>}
    </BoxAction>
  )
}

export default IdentityPage;
