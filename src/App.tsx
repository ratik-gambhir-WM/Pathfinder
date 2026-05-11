import { Navigate, Route, Routes } from "react-router-dom";
import { HubPage } from "./pages/HubPage";
import { LoginPage } from "./pages/LoginPage";

function App() {
  return (
    <Routes>
      <Route element={<Navigate replace to="/login" />} path="/" />
      <Route element={<LoginPage />} path="/login" />
      <Route element={<HubPage />} path="/hub" />
      <Route element={<Navigate replace to="/login" />} path="*" />
    </Routes>
  );
}

export default App;
