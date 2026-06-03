import { Navigate, Route, Routes } from "react-router-dom";
import { DataRoomPage } from "./pages/DataRoomPage";
import { HubPage } from "./pages/HubPage";
import { DealRoomPage } from "./pages/DealRoomPage";
import { GlobalVaultPage } from "./pages/GlobalVaultPage";
import { LoginPage } from "./pages/LoginPage";
import { SummarizePage } from "./pages/SummarizePage";

function App() {
  return (
    <Routes>
      <Route element={<Navigate replace to="/login" />} path="/" />
      <Route element={<LoginPage />} path="/login" />
      <Route element={<HubPage />} path="/hub" />
      <Route element={<GlobalVaultPage />} path="/hub/vault" />
      <Route element={<SummarizePage />} path="/hub/summarize" />
      <Route element={<DealRoomPage />} path="/hub/deals/:dealId" />
      <Route element={<DataRoomPage />} path="/hub/deals/:dealId/data-room" />
      <Route element={<Navigate replace to="/login" />} path="*" />
    </Routes>
  );
}

export default App;
