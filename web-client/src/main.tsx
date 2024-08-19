import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import {App} from './App.tsx'
import { OpenAPI } from './api/index.ts'

OpenAPI.BASE = 'http://192.168.2.56:8080'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
