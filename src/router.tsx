import React from 'react'
import { createBrowserRouter, createRoutesFromElements, Route } from 'react-router-dom';
import ZustandPage from '@/pages/ZustandPage';
import SidebarLayout from '@/common/components/Common/Layouts/SidebarLayout';
import WelcomePage from '@/pages/WelcomePage/WelcomePage';
import TauriPage from '@/pages/TauriPage/TauriPage';
import WalletPage from '@/pages/WalletPage/WalletPage';
import IdentityPage from '@/pages/IdentityPage/IdentityPage';

export const router = createBrowserRouter(
  createRoutesFromElements(
    <Route path="/" element={<SidebarLayout />}>
      <Route index element={<WelcomePage />} />
      <Route path="/wallet" element={<WalletPage />} />
      <Route path="/identity" element={<IdentityPage />} />
      <Route path="/tauri" element={<TauriPage />} />
      <Route path="/zustand" element={<ZustandPage />} />
    </Route>
  )
)
