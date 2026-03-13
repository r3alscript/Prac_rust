import { BrowserRouter, Routes, Route } from "react-router-dom";
import LotsPage from "./ListLot";
import LotPage from "./LotPage";
import BidHistory from "./BidHistoryPage";
import MyLots from "./MyLotsPage";
import EditProfile from "./EditProfilePage";
import OAuthP from "./OAuthPage";


function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/lots" element={<LotsPage />} />
        <Route path="/lot/:id" element={<LotPage />} />
        <Route path="/bidhistory" element={<BidHistory />} />
        <Route path="/mylot" element={<MyLots />} />
        <Route path="/profile" element={<EditProfile />} />
        <Route path="*" element={<LotsPage />} />
        <Route path="/authorization" element={<OAuthP />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;