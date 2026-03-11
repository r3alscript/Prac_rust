import { useEffect, useMemo, useState } from "react";
import "./LotPage.css";
import { useNavigate } from "react-router-dom";



function pad2(n: number): string {
  return String(n).padStart(2, "0");
}

function msToHMS(ms: number): string {
  const total = Math.max(0, Math.floor(ms / 1000));
  const h = Math.floor(total / 3600);
  const m = Math.floor((total % 3600) / 60);
  const s = total % 60;
  return `${pad2(h)}:${pad2(m)}:${pad2(s)}`;
}

type Lot = {
  author: string;
  seller: string;
  title: string;
  bidsCount: number;
  maxBid: number;
  watchers: number;
  createdAt: string;
  endsAtLabel: string;
  currentPrice: number;
  description: string;
  endsInMsFromNow: number;
  images: string[];
};

const lots = [
  { id: 1, title: "Стіл дерев’яний", price: 125, image: "/img/chair.jpg" },
  { id: 2, title: "Будинок", price: 125, image: "/img/house.png" },
  { id: 3, title: "Будинок", price: 125, image: "/img/house.png" },
  { id: 4, title: "Будинок", price: 125, image: "/img/house.png" },
];
export default function LotPage() {

  const navigate = useNavigate();

  const goToLots = () => {
    navigate("/lots");
  };

  const lot: Lot = useMemo(
    () => ({
      author: "RjhbcnefX",
      seller: "PPvdmo",
      title: "Стіл дерев’яний",
      bidsCount: 20,
      maxBid: 2000,
      watchers: 12,
      createdAt: "12.06.2026",
      endsAtLabel: "12.06.2026, 21:00",
      currentPrice: 125,
      description:
        "Стіл має класичний дизайн, який гармонійно поєднується як із сучасним, так і з традиційним інтер’єром. Конструкція міцна та стійка, ніжки надійно закріплені, що забезпечує довговічність і комфорт у використанні.",
      endsInMsFromNow: (11 * 3600 + 17 * 60 + 26) * 1000,
      images: [
        "https://images.unsplash.com/photo-1616628182505-4c2a5f87b84d?auto=format&fit=crop&w=1400&q=80",
        "https://images.unsplash.com/photo-1598300056393-4aac492f4344?auto=format&fit=crop&w=1400&q=80",
        "https://images.unsplash.com/photo-1505693416388-ac5ce068fe85?auto=format&fit=crop&w=1400&q=80",
      ],
    }),
    []
  );

  const [currentPrice, setCurrentPrice] = useState<number>(lot.currentPrice);
  const [bidValue, setBidValue] = useState<number>(lot.currentPrice);
  const [endsInMs, setEndsInMs] = useState<number>(lot.endsInMsFromNow);
  const [activeImg, setActiveImg] = useState<number>(0);

  const nextImg = () => setActiveImg((i) => (i + 1) % lot.images.length);
  const prevImg = () =>
    setActiveImg((i) => (i - 1 + lot.images.length) % lot.images.length);

  useEffect(() => {
    const t = window.setInterval(() => {
      setEndsInMs((prev) => Math.max(0, prev - 1000));
    }, 1000);
    return () => window.clearInterval(t);
  }, []);

  const step = 5;

  const decBid = () => setBidValue((v) => Math.max(step, v - step));
  const incBid = () => setBidValue((v) => v + step);

  const placeBid = () => {
    if (bidValue <= currentPrice) {
      alert("Ставка має бути більшою за поточну ціну.");
      return;
    }
    setCurrentPrice(bidValue);
    alert(`Ставку ${bidValue} грн прийнято!`);
  };

  return (
    <div className="lp-page">
      <div className="lp-topbar">
        <div className="lp-user">
          <div className="lp-avatar" />
          <div className="lp-username">{lot.author}</div>
          <button className="lp-logoutBtn" type="button" aria-label="logout" >
           
            <img src="/img/Logout.jpg" alt="logout" />
          </button>
        </div>
      </div>

      <div className="lp-container">
        <div className="lp-layout">
         <div className="lp-titleRow">
  <button
    className="lp-backBtn"
    aria-label="back"
    onClick={goToLots}
  >
    ←
  </button>
  <h1 className="lp-title">{lot.title}</h1>
</div>

          <div className="lp-chips">
            <div className="lp-chip">
              <span className="lp-chip-text">Продавець</span>
              <span className="lp-chip-num">{lot.seller}</span>
            </div>

            <div className="lp-chip">
              <span className="lp-chip-text">Ставок</span>
              <span className="lp-chip-num">{lot.bidsCount}</span>
            </div>

            <div className="lp-chip">
              <span className="lp-chip-text">Макс. ставка</span>
              <span className="lp-chip-num">{lot.maxBid}</span>
            </div>

            <div className="lp-chip">
              <span className="lp-chip-text">Стежать за лотом</span>
              <span className="lp-chip-num">{lot.watchers}</span>
            </div>

            <div className="lp-chip">
              <span className="lp-chip-text">Додано</span>
              <span className="lp-chip-num">{lot.createdAt}</span>
            </div>
          </div>

          <div className="lp-imageCard">
            <button
              className="lp-imgArrow lp-left"
              onClick={prevImg}
              type="button"
              aria-label="prev"
            >
              <span className="lp-arrowIcon">‹</span>
            </button>

            <img className="lp-mainImage" src={lot.images[activeImg]} alt="lot" />

            <button
              className="lp-imgArrow lp-right"
              onClick={nextImg}
              type="button"
              aria-label="next"
            >
              <span className="lp-arrowIcon">›</span>
            </button>

            <div className="lp-thumbs">
              {lot.images.map((img, i) => (
                <button
                  key={img}
                  className={`lp-thumbBtn ${i === activeImg ? "active" : ""}`}
                  onClick={() => setActiveImg(i)}
                  type="button"
                  aria-label={`thumb-${i + 1}`}
                >
                  <img className="lp-thumbImg" src={img} alt={`thumb ${i + 1}`} />
                </button>
              ))}
            </div>
          </div>

          <div className="lp-right">
            <div className="lp-card lp-ends">
              <div className="lp-ends-row">
                <div>
                  <div className="lp-muted">Закінчення</div>
                  <div className="lp-strong">{lot.endsAtLabel}</div>
                </div>

                <div className="lp-ends-right">
                  <div className="lp-muted">Закінчується через</div>
                  <div className="lp-timer">{msToHMS(endsInMs)}</div>
                </div>
              </div>
            </div>

            <div className="lp-card lp-priceCard">
              <div className="lp-priceRow">
                <div className="lp-priceLabel">Поточна ціна</div>
                <div className="lp-priceTag">
                  <span>{currentPrice}</span>
                  <span className="lp-currency">грн</span>
                </div>
              </div>
            </div>

            <div className="lp-card lp-bidCard">
              <div className="lp-bidRow">
                <div className="lp-stepper">
                  <button
                    className="lp-stepBtn"
                    onClick={decBid}
                    aria-label="minus"
                    type="button"
                  >
                    –
                  </button>

                  <div className="lp-stepValue">
                    {bidValue} <span className="lp-currency">грн</span>
                  </div>

                  <button
                    className="lp-stepBtn"
                    onClick={incBid}
                    aria-label="plus"
                    type="button"
                  >
                    +
                  </button>
                </div>

                <button className="lp-primaryBtn" onClick={placeBid} type="button">
                  Зробити ставку
                </button>
              </div>
            </div>

            <div className="lp-card lp-desc">{lot.description}</div>
          </div>

        
          <aside className="lp-side">
            <div className="lp-recommend">Можливо вам сподобається</div>

            <div className="lp-sideInner">
              {lots.map((x) => (
                <a key={x.id} href={`/lot/${x.id}`} className="lp-sideCard">
                  <div className="lp-sideImageWrap">
                    <img src={x.image} alt={x.title} />
                  </div>

                  <div className="lp-sideTitleText">{x.title}</div>

                  <div className="lp-priceTag2">{x.price} грн</div>
                </a>
              ))}
            </div>
          </aside>
        </div>
      </div>
    </div>
  );
}