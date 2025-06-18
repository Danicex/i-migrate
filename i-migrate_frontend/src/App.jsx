import { Routes, Route } from "react-router-dom"
import './App.css'
import {AppDataProvider} from './Context/AppContext'
import MainPage from "./Landingpage/MainPage"
import { PaymentFlow } from "./SignUser/PaymentFlow"
function App() {

  return (
    <AppDataProvider>
    <Routes>
    <Route path="/" element={ <MainPage/> } />
    <Route path="/pay" element={ < PaymentFlow /> } />
  </Routes>
    </AppDataProvider>
  )
}

export default App
