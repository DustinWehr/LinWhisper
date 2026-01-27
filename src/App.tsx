import { useEffect } from "react";
import { Routes, Route, useNavigate, useLocation } from "react-router-dom";
import { listen } from "@tauri-apps/api/event";
import Layout from "./components/Layout";
import HomePage from "./pages/HomePage";
import HistoryPage from "./pages/HistoryPage";
import SettingsPage from "./pages/SettingsPage";
import ModesPage from "./pages/ModesPage";
import RecordingIndicator from "./pages/RecordingIndicator";

function App() {
  const navigate = useNavigate();
  const location = useLocation();

  useEffect(() => {
    // Listen for navigation events from the backend
    const unlisten = listen<string>("navigate", (event) => {
      navigate(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [navigate]);

  // Recording indicator window has no layout
  if (location.pathname === "/recording") {
    return <RecordingIndicator />;
  }

  return (
    <Layout>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/history" element={<HistoryPage />} />
        <Route path="/settings" element={<SettingsPage />} />
        <Route path="/modes" element={<ModesPage />} />
      </Routes>
    </Layout>
  );
}

export default App;
