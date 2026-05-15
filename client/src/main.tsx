import { BrowserRouter, Route, Routes } from 'react-router'
import { createRoot } from 'react-dom/client'
import { StrictMode, Suspense } from 'react'

import App from './designSystem/App/App'
import Home from './pages/home/Home'
import NotFound from './pages/notFound/NotFound'
import Profile from './pages/profile/Profile'
import './main.css'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <Suspense fallback="Loading...">
      <BrowserRouter>
        <Routes>
          <Route index element={<App><Home /></App>} />
          <Route path="/404" element={<App><NotFound /></App>} />
          <Route path=":login" element={<App><Profile /></App>} />
        </Routes>
      </BrowserRouter>
    </Suspense>
  </StrictMode>
)
