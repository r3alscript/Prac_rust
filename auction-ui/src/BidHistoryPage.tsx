import { useMemo, useState } from "react";
import "./BidHistoryPage.css";
import "./ListLot.css";

type BidHistoryItem = {
  id: number;
  lotTitle: string;
  seller: string;
  endDate: string;
  bid: number;
  image: string;
};

const bidHistory: BidHistoryItem[] = [
  {
    id: 1,
    lotTitle: "Стіл дерев’яний",
    seller: "Rjhbcnedfx",
    endDate: "22.03.2026 19:00",
    bid: 1500,
    image: "/img/chair.jpg",
  },
  {
    id: 2,
    lotTitle: "Будинок",
    seller: "PPvdmo",
    endDate: "25.03.2026 14:30",
    bid: 2300,
    image: "/img/house.png",
  },
  {
    id: 3,
    lotTitle: "Шафа",
    seller: "SellerOne",
    endDate: "26.03.2026 12:00",
    bid: 4100,
    image: "/img/chair.jpg",
  },
  {
    id: 4,
    lotTitle: "Картина",
    seller: "ArtHouse",
    endDate: "27.03.2026 18:45",
    bid: 3200,
    image: "/img/house.png",
  },
  {
    id: 5,
    lotTitle: "Лампа",
    seller: "LightPro",
    endDate: "28.03.2026 11:15",
    bid: 980,
    image: "/img/chair.jpg",
  },
  {
    id: 6,
    lotTitle: "Комод",
    seller: "WoodMax",
    endDate: "29.03.2026 16:20",
    bid: 5100,
    image: "/img/house.png",
  },
  {
    id: 7,
    lotTitle: "Полиця",
    seller: "HomeLine",
    endDate: "30.03.2026 13:10",
    bid: 1750,
    image: "/img/chair.jpg",
  },
  {
    id: 8,
    lotTitle: "Крісло",
    seller: "DecorLux",
    endDate: "31.03.2026 20:00",
    bid: 2600,
    image: "/img/house.png",
  },
];

export default function BidHistoryPage() {
  const username = "RjhbcnefX";

  const itemsPerPage = 6;
  const [currentPage, setCurrentPage] = useState(1);

  const totalPages = Math.ceil(bidHistory.length / itemsPerPage);

  const currentItems = useMemo(() => {
    const start = (currentPage - 1) * itemsPerPage;
    const end = start + itemsPerPage;
    return bidHistory.slice(start, end);
  }, [currentPage]);

  const goToPage = (page: number) => {
    if (page < 1 || page > totalPages) return;
    setCurrentPage(page);
  };

  return (
    <div className="lots-page">
      <aside className="lots-sidebarStatic">
        <div className="lots-sidebarProfile">
          <div className="lots-avatar" />
          <div className="lots-username">{username}</div>
        </div>

        <div className="lots-sidebarMenu">
          <button
            className="lots-sidebarItem"
            type="button"
            onClick={() => (window.location.href = "/lots")}
          >
            <img src="/img/Purchaseorder.svg" className="lots-menuIcon" alt="" />
            Список лотів
          </button>

          <button
            className="lots-sidebarItem active"
            type="button"
            onClick={() => (window.location.href = "/bidhistory")}
          >
            <img src="/img/Auction.svg" className="lots-menuIcon" alt="" />
            Історія ставок
          </button>

          <button
            className="lots-sidebarItem"
            type="button"
            onClick={() => (window.location.href = "/mylot")}
          >
            <img src="/img/Purchaseorder.svg" className="lots-menuIcon" alt="" />
            Мої лоти
          </button>

          <button
            className="lots-sidebarItem"
            type="button"
            onClick={() => (window.location.href = "/profile")}
          >
            <img src="/img/Usermale.svg" className="lots-menuIcon" alt="" />
            Редагувати профіль
          </button>
        </div>
      </aside>

      <div className="lots-main">
        <div className="lots-container">
          <div className="lots-head">
            <h1 className="lots-title">Історія ставок</h1>

            <button className="lots-logoutBtn" type="button">
              <img src="/img/Logout.jpg" alt="logout" />
            </button>
          </div>

          <div className="bh-head">
            <div>Лот</div>
            <div>Продавець</div>
            <div>Дата завершення</div>
            <div>Ставка</div>
          </div>

          <div className="bh-list">
            {currentItems.map((item) => (
              <div className="bh-row" key={item.id}>
                <div className="bh-lotCell">
                  <div className="bh-thumbWrap">
                    <img src={item.image} alt={item.lotTitle} className="bh-thumb" />
                  </div>

                  <div className="bh-lotTitle">{item.lotTitle}</div>
                </div>

                <div className="bh-seller">{item.seller}</div>
                <div className="bh-date">{item.endDate}</div>
                <div className="bh-bid">{item.bid} грн</div>
              </div>
            ))}
          </div>

          {totalPages > 1 && (
            <div className="bh-pagination">
              {currentPage > 1 && (
                <button
                  className="bh-pageBtn bh-navBtn"
                  type="button"
                  onClick={() => goToPage(currentPage - 1)}
                >
                  ←
                </button>
              )}

              {Array.from({ length: totalPages }, (_, index) => {
                const page = index + 1;
                return (
                  <button
                    key={page}
                    type="button"
                    className={`bh-pageBtn ${currentPage === page ? "active" : ""}`}
                    onClick={() => goToPage(page)}
                  >
                    {page}
                  </button>
                );
              })}

              {currentPage < totalPages && (
                <button
                  className="bh-pageBtn bh-navBtn"
                  type="button"
                  onClick={() => goToPage(currentPage + 1)}
                >
                  →
                </button>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}