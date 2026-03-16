import { useEffect, useState } from "react";
import "./LotPage.css";
import { useNavigate, useParams } from "react-router-dom";

function pad2(n: number): string {
  return String(n).padStart(2, "0");
}

function msToHMS(ms: number): string {
  const safeMs = Number.isFinite(ms) ? ms : 0;
  const total = Math.max(0, Math.floor(safeMs / 1000));
  const h = Math.floor(total / 3600);
  const m = Math.floor((total % 3600) / 60);
  const s = total % 60;
  return `${pad2(h)}:${pad2(m)}:${pad2(s)}`;
}

function toDateMs(value: string | null | undefined): number {
  if (!value) return 0;
  const normalized = value.includes("T") ? value : value.replace(" ", "T");
  const parsed = new Date(normalized).getTime();
  return Number.isFinite(parsed) ? parsed : 0;
}

function formatDateTime(value: string | null | undefined): string {
  if (!value) return "—";
  const normalized = value.includes("T") ? value : value.replace(" ", "T");
  const date = new Date(normalized);
  if (Number.isNaN(date.getTime())) return value;

  return new Intl.DateTimeFormat("uk-UA", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  }).format(date);
}

type LotDetails = {
  id: string;
  title: string;
  description: string;
  start_price: number;
  current_price: number;
  seller_id: string;
  image_url: string | null;
  auction_end: string;
  created_at: string;
  bids_count: number;
  max_bid: number;
};

type LotShort = {
  id: string;
  title: string;
  current_price: number;
  image_url: string | null;
};

const API_BASE = "http://localhost:8080";

export default function LotPage() {
  const navigate = useNavigate();
  const { id } = useParams();

  const [lot, setLot] = useState<LotDetails | null>(null);
  const [recommendedLots, setRecommendedLots] = useState<LotShort[]>([]);
  const [loading, setLoading] = useState(true);
  const [bidValue, setBidValue] = useState(0);
  const [placingBid, setPlacingBid] = useState(false);
  const [endsInMs, setEndsInMs] = useState(0);

  const goToLots = () => navigate("/lots");

  const loadLot = async () => {
    try {
      setLoading(true);

      const response = await fetch(`${API_BASE}/api/lots/${id}`);
      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(errorText || "Не вдалося завантажити лот");
      }

      const data: LotDetails = await response.json();
      setLot(data);
      setBidValue(data.current_price + 5);

      const endMs = toDateMs(data.auction_end) - Date.now();
      setEndsInMs(Math.max(0, endMs));
    } catch (error) {
      console.error("Помилка завантаження лота:", error);
    } finally {
      setLoading(false);
    }
  };

  const loadRecommendedLots = async () => {
    try {
      const response = await fetch(`${API_BASE}/api/lots`);
      if (!response.ok) return;

      const data: LotShort[] = await response.json();
      setRecommendedLots(data.filter((x) => String(x.id) !== String(id)).slice(0, 4));
    } catch (error) {
      console.error("Помилка завантаження рекомендацій:", error);
    }
  };

  useEffect(() => {
    loadLot();
    loadRecommendedLots();
  }, [id]);

  useEffect(() => {
    const t = window.setInterval(() => {
      setEndsInMs((prev) => Math.max(0, prev - 1000));
    }, 1000);

    return () => window.clearInterval(t);
  }, []);

  const step = 5;

  const decBid = () => setBidValue((v) => Math.max(step, v - step));
  const incBid = () => setBidValue((v) => v + step);

  const placeBid = async () => {
    const token = localStorage.getItem("access_token");

    if (!token) {
      window.location.href = "/authorization";
      return;
    }

    if (!lot) return;

    if (bidValue <= lot.current_price) {
      alert("Ставка має бути більшою за поточну ціну.");
      return;
    }

    try {
      setPlacingBid(true);

      const response = await fetch(`${API_BASE}/api/lots/${lot.id}/bids`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify({
          amount: bidValue,
        }),
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(errorText || "Не вдалося зробити ставку");
      }

      await loadLot();
      await loadRecommendedLots();
    } catch (error) {
      console.error("Помилка ставки:", error);
      alert("Не вдалося зробити ставку.");
    } finally {
      setPlacingBid(false);
    }
  };

  if (loading) return <div className="lp-page">Завантаження...</div>;
  if (!lot) return <div className="lp-page">Лот не знайдено</div>;

  const mainImage = lot.image_url || "/img/no-image.png";

  return (
      <div className="lp-page">
        <div className="lp-container">
          <div className="lp-layout">
            <div className="lp-titleRow">
              <button className="lp-backBtn" aria-label="back" onClick={goToLots}>
                ←
              </button>
              <h1 className="lp-title">{lot.title}</h1>
            </div>

            <div className="lp-chips">
              <div className="lp-chip">
                <span className="lp-chip-text">Продавець</span>
                <span className="lp-chip-num">{lot.seller_id}</span>
              </div>

              <div className="lp-chip">
                <span className="lp-chip-text">Ставок</span>
                <span className="lp-chip-num">{lot.bids_count}</span>
              </div>

              <div className="lp-chip">
                <span className="lp-chip-text">Макс. ставка</span>
                <span className="lp-chip-num">{lot.max_bid}</span>
              </div>

              <div className="lp-chip">
                <span className="lp-chip-text">Стежати за лотом</span>
                <span className="lp-chip-num">—</span>
              </div>

              <div className="lp-chip">
                <span className="lp-chip-text">Додано</span>
                <span className="lp-chip-num">{formatDateTime(lot.created_at)}</span>
              </div>
            </div>

            <div className="lp-imageCard">
              <img className="lp-mainImage" src={mainImage} alt={lot.title} />
            </div>

            <div className="lp-right">
              <div className="lp-card lp-ends">
                <div className="lp-ends-row">
                  <div>
                    <div className="lp-muted">Закінчення</div>
                    <div className="lp-strong">{formatDateTime(lot.auction_end)}</div>
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
                    <span>{lot.current_price}</span>
                    <span className="lp-currency">грн</span>
                  </div>
                </div>
              </div>

              <div className="lp-card lp-bidCard">
                <div className="lp-bidRow">
                  <div className="lp-stepper">
                    <button className="lp-stepBtn" onClick={decBid} type="button">
                      –
                    </button>

                    <div className="lp-stepValue">
                      {bidValue}
                      <span className="lp-currency"> грн</span>
                    </div>

                    <button className="lp-stepBtn" onClick={incBid} type="button">
                      +
                    </button>
                  </div>

                  <button
                      className="lp-primaryBtn"
                      onClick={placeBid}
                      type="button"
                      disabled={placingBid}
                  >
                    {placingBid ? "Обробка..." : "Зробити ставку"}
                  </button>
                </div>
              </div>

              <div className="lp-card lp-desc">{lot.description}</div>
            </div>

            <aside className="lp-side">
              <div className="lp-recommend">Можливо вам сподобається</div>

              <div className="lp-sideInner">
                {recommendedLots.map((x) => (
                    <a key={x.id} href={`/lot/${x.id}`} className="lp-sideCard">
                      <div className="lp-sideImageWrap">
                        <img src={x.image_url || "/img/no-image.png"} alt={x.title} />
                      </div>

                      <div className="lp-sideTitleText">{x.title}</div>
                      <div className="lp-priceTag2">{x.current_price} грн</div>
                    </a>
                ))}
              </div>
            </aside>
          </div>
        </div>
      </div>
  );
}