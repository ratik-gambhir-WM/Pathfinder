import { Navigate, Route, Routes } from "react-router-dom";
import { HubPage } from "./pages/HubPage";
import { DealRoomPage } from "./pages/DealRoomPage";
import { LoginPage } from "./pages/LoginPage";

function App() {
  return (
    <Routes>
      <Route element={<Navigate replace to="/login" />} path="/" />
      <Route element={<LoginPage />} path="/login" />
      <Route element={<HubPage />} path="/hub" />
      <Route element={<DealRoomPage />} path="/hub/deals/:dealId" />
      <Route element={<Navigate replace to="/login" />} path="*" />
    </Routes>
  );
}

export default App;
