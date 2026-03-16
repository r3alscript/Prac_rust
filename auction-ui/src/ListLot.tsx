import { useEffect, useState } from "react";
import type { ChangeEvent, FormEvent } from "react";
import "./ListLot.css";

type LotCard = {
  id: string;
  title: string;
  start_price: number;
  current_price: number;
  image_url: string | null;
};

type CurrentUser = {
  id: string;
  email: string;
  name: string;
  surname: string;
  photo_url: string | null;
  balance: number;
  created_at_utc: string;
};

const API_BASE = "http://localhost:8080";

export default function LotsPage() {
  const [currentUser, setCurrentUser] = useState<CurrentUser | null>(null);
  const [userLoading, setUserLoading] = useState(true);

  const [lots, setLots] = useState<LotCard[]>([]);
  const [lotsLoading, setLotsLoading] = useState(true);

  const [isModalOpen, setIsModalOpen] = useState(false);
  const [title, setTitle] = useState("");
  const [startPrice, setStartPrice] = useState("");
  const [auctionEnd, setAuctionEnd] = useState("");
  const [description, setDescription] = useState("");
  const [photoFile, setPhotoFile] = useState<File | null>(null);
  const [photoPreview, setPhotoPreview] = useState("");
  const [creatingLot, setCreatingLot] = useState(false);

  const token = localStorage.getItem("access_token");

  const resolveImageUrl = (imageUrl: string | null) => {
    if (!imageUrl) return "/img/no-image.png";
    if (imageUrl.startsWith("http://") || imageUrl.startsWith("https://")) {
      return imageUrl;
    }
    return `${API_BASE}${imageUrl}`;
  };

  const loadLots = async () => {
    try {
      setLotsLoading(true);

      const response = await fetch(`${API_BASE}/api/lots`, {
        method: "GET",
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(errorText || "Не вдалося завантажити список лотів");
      }

      const data: LotCard[] = await response.json();
      setLots(data);
    } catch (error) {
      console.error("Помилка завантаження лотів:", error);
    } finally {
      setLotsLoading(false);
    }
  };

  useEffect(() => {
    const loadCurrentUser = async () => {
      if (!token) {
        window.location.href = "/authorization";
        return;
      }

      try {
        const response = await fetch(`${API_BASE}/api/users/me`, {
          method: "GET",
          headers: {
            Authorization: `Bearer ${token}`,
          },
        });

        if (response.status === 401) {
          localStorage.removeItem("access_token");
          window.location.href = "/authorization";
          return;
        }

        if (response.status === 404) {
          setUserLoading(false);
          return;
        }

        if (!response.ok) {
          const errorText = await response.text();
          throw new Error(errorText || "Не вдалося отримати дані користувача");
        }

        const user: CurrentUser = await response.json();
        setCurrentUser(user);
      } catch (error) {
        console.error("Помилка завантаження користувача:", error);
      } finally {
        setUserLoading(false);
      }
    };

    loadCurrentUser();
    loadLots();
  }, []);

  useEffect(() => {
    return () => {
      if (photoPreview) {
        URL.revokeObjectURL(photoPreview);
      }
    };
  }, [photoPreview]);

  const openModal = () => setIsModalOpen(true);

  const closeModal = () => {
    if (photoPreview) {
      URL.revokeObjectURL(photoPreview);
    }

    setIsModalOpen(false);
    setTitle("");
    setStartPrice("");
    setAuctionEnd("");
    setDescription("");
    setPhotoFile(null);
    setPhotoPreview("");
  };

  const handleLogout = () => {
    localStorage.removeItem("access_token");
    window.location.href = "/authorization";
  };

  const handlePhotoChange = (e: ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0] || null;
    setPhotoFile(file);

    if (photoPreview) {
      URL.revokeObjectURL(photoPreview);
    }

    if (file) {
      const imageUrl = URL.createObjectURL(file);
      setPhotoPreview(imageUrl);
    } else {
      setPhotoPreview("");
    }
  };

  const handleCreateLot = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (!title.trim() || !startPrice || !auctionEnd || !description.trim()) {
      alert("Будь ласка, заповніть усі поля.");
      return;
    }

    if (!token) {
      window.location.href = "/authorization";
      return;
    }

    const parsedPrice = Number(startPrice);
    if (!Number.isFinite(parsedPrice) || parsedPrice <= 0) {
      alert("Вкажіть коректну початкову ціну.");
      return;
    }

    const parsedDate = new Date(auctionEnd);
    if (Number.isNaN(parsedDate.getTime())) {
      alert("Некоректна дата завершення аукціону.");
      return;
    }

    try {
      setCreatingLot(true);

      const formData = new FormData();
      formData.append("title", title.trim());
      formData.append("description", description.trim());
      formData.append("start_price", String(parsedPrice));
      formData.append("auction_end", parsedDate.toISOString());

      if (photoFile) {
        formData.append("image", photoFile);
      }

      const response = await fetch(`${API_BASE}/api/lots`, {
        method: "POST",
        headers: {
          Authorization: `Bearer ${token}`,
        },
        body: formData,
      });

      if (response.status === 401) {
        localStorage.removeItem("access_token");
        window.location.href = "/authorization";
        return;
      }

      const responseText = await response.text();

      if (!response.ok) {
        throw new Error(responseText || "Не вдалося створити лот");
      }

      await loadLots();
      closeModal();
    } catch (error) {
      console.error("Помилка створення лота:", error);
      alert(error instanceof Error ? error.message : "Не вдалося створити лот.");
    } finally {
      setCreatingLot(false);
    }
  };

  const fullName = currentUser
      ? `${currentUser.name} ${currentUser.surname}`.trim()
      : "Користувач";

  const avatarLetter = currentUser?.name?.[0]?.toUpperCase() || "U";

  return (
      <div className="lots-page">
        <aside className="lots-sidebarStatic">
          <div className="lots-sidebarProfile">
            {currentUser?.photo_url ? (
                <img
                    src={resolveImageUrl(currentUser.photo_url)}
                    alt={fullName}
                    className="lots-avatar lots-avatarImage"
                />
            ) : (
                <div className="lots-avatar lots-avatarFallback">{avatarLetter}</div>
            )}

            <div className="lots-userMeta">
              <div className="lots-username">
                {userLoading ? "Завантаження..." : fullName}
              </div>
              {currentUser && <div className="lots-userEmail">{currentUser.email}</div>}
            </div>
          </div>

          <div className="lots-sidebarMenu">
            <button
                className="lots-sidebarItem active"
                type="button"
                onClick={() => (window.location.href = "/lots")}
            >
              <img src="/img/Purchaseorder.svg" className="lots-menuIcon" alt="" />
              Список лотів
            </button>

            <button
                className="lots-sidebarItem"
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
              <h1 className="lots-title">Лоти</h1>

              <button className="lots-createBtn" type="button" onClick={openModal}>
                Створити лот +
              </button>

              <button className="lots-logoutBtn" type="button" onClick={handleLogout}>
                <img src="/img/Logout.jpg" alt="logout" />
              </button>
            </div>

            <div className="lots-balanceRow">
              <div className="lots-balanceCard">
                Баланс: <span className="lots-balanceValue">{currentUser ? currentUser.balance : 0} грн</span>
              </div>
            </div>

            <div className="lots-grid">
              {lotsLoading ? (
                  <div>Завантаження лотів...</div>
              ) : lots.length === 0 ? (
                  <div>Лотів поки немає</div>
              ) : (
                  lots.map((x) => (
                      <a key={x.id} href={`/lot/${x.id}`} className="lots-card">
                        <div className="lots-imgWrap">
                          <img src={resolveImageUrl(x.image_url)} alt={x.title} />
                        </div>

                        <div className="lots-cardTitle">{x.title}</div>

                        <div className="lots-priceTag">
                          {x.current_price ?? x.start_price}
                          <span className="lots-currency"> грн</span>
                        </div>
                      </a>
                  ))
              )}
            </div>
          </div>
        </div>

        {isModalOpen && (
            <div className="lots-modalOverlay" onClick={closeModal}>
              <div className="lots-modal" onClick={(e) => e.stopPropagation()}>
                <div className="lots-modalHeader">
                  <h2 className="lots-modalTitle">Створити лот</h2>

                  <button
                      className="lots-modalClose"
                      type="button"
                      onClick={closeModal}
                      aria-label="close"
                  >
                    ×
                  </button>
                </div>

                <form className="lots-form" onSubmit={handleCreateLot}>
                  <div className="lots-modalBody">
                    <div className="lots-photoCol">
                      <label className="lots-photoBox">
                        {photoPreview ? (
                            <img className="lots-previewImg" src={photoPreview} alt="preview" />
                        ) : (
                            <div className="lots-photoPlaceholder" />
                        )}

                        <input
                            className="lots-hiddenFile"
                            type="file"
                            accept="image/*"
                            onChange={handlePhotoChange}
                        />
                      </label>
                    </div>

                    <div className="lots-infoCol">
                      <div className="ll-field">
                        <input
                            className="ll-input"
                            type="text"
                            placeholder=" "
                            value={title}
                            onChange={(e) => setTitle(e.target.value)}
                            id="title"
                        />
                        <label className="ll-label" htmlFor="title">
                          Назва лота
                        </label>
                      </div>

                      <div className="ll-field">
                        <input
                            className="ll-input"
                            type="number"
                            min="1"
                            placeholder=" "
                            value={startPrice}
                            onChange={(e) => setStartPrice(e.target.value)}
                            id="startPrice"
                        />
                        <label className="ll-label" htmlFor="startPrice">
                          Початкова ціна
                        </label>
                      </div>

                      <div className="ll-field">
                        <input
                            className="ll-input"
                            type="datetime-local"
                            placeholder=" "
                            value={auctionEnd}
                            onChange={(e) => setAuctionEnd(e.target.value)}
                            id="auctionEnd"
                        />
                        <label className="ll-label" htmlFor="auctionEnd">
                          Час закінчення
                        </label>
                      </div>

                      <div className="ll-field ll-field-textarea">
                    <textarea
                        className="ll-input ll-textarea"
                        placeholder=" "
                        value={description}
                        onChange={(e) => setDescription(e.target.value)}
                        id="description"
                    />
                        <label className="ll-label" htmlFor="description">
                          Опис
                        </label>
                      </div>
                    </div>
                  </div>

                  <div className="lots-modalActions">
                    <button className="lots-cancelBtn" type="button" onClick={closeModal}>
                      Скасувати
                    </button>

                    <button className="lots-saveBtn" type="submit" disabled={creatingLot}>
                      {creatingLot ? "Створення..." : "Створити"}
                    </button>
                  </div>
                </form>
              </div>
            </div>
        )}
      </div>
  );
}