import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { HelmetProvider } from 'react-helmet-async'
import './i18n'
import './index.css'
import App from './App.jsx'
import { PWAInstall } from './components/PWAInstall.jsx'

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <PWAInstall />
    <App />
import ProfilePage from './components/ProfilePage.jsx'
import { BrowserRouter, Routes, Route } from 'react-router-dom'

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<App />} />                {/* Main App route */}
        <Route path="/profile" element={<ProfilePage />} />  {/* Profile Page route */}
      </Routes>
    </BrowserRouter>
  </StrictMode>,
)