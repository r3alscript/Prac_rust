import { useState } from "react";
import type { ChangeEvent, FormEvent } from "react";
import "./ListLot.css";
import "./MyLotsPage.css";

type MyLotItem = {
  id: number;
  title: string;
  price: number;
  image: string;
  description: string;
  auctionEnd: string;
};

const initialLots: MyLotItem[] = [
  {
    id: 1,
    title: "Стіл дерев’яний",
    price: 125,
    image: "/img/chair.jpg",
    description: "Дерев’яний стіл у хорошому стані",
    auctionEnd: "2026-03-22T19:00",
  },
  {
    id: 2,
    title: "Будинок",
    price: 2300,
    image: "/img/house.png",
    description: "Декоративний будинок",
    auctionEnd: "2026-03-25T14:30",
  },
  {
    id: 3,
    title: "Шафа",
    price: 4100,
    image: "/img/chair.jpg",
    description: "Велика шафа для одягу",
    auctionEnd: "2026-03-26T12:00",
  },
];

export default function MyLotsPage() {
  const username = "RjhbcnefX";

  const [lots, setLots] = useState<MyLotItem[]>(initialLots);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [editingLotId, setEditingLotId] = useState<number | null>(null);

  const [title, setTitle] = useState("");
  const [startPrice, setStartPrice] = useState("");
  const [auctionEnd, setAuctionEnd] = useState("");
  const [description, setDescription] = useState("");
  const [photoFile, setPhotoFile] = useState<File | null>(null);
  const [photoPreview, setPhotoPreview] = useState("");

  const openEditModal = (lot: MyLotItem) => {
    setEditingLotId(lot.id);
    setTitle(lot.title);
    setStartPrice(String(lot.price));
    setAuctionEnd(lot.auctionEnd);
    setDescription(lot.description);
    setPhotoPreview(lot.image);
    setPhotoFile(null);
    setIsModalOpen(true);
  };

  const closeModal = () => {
    setIsModalOpen(false);
    setEditingLotId(null);
    setTitle("");
    setStartPrice("");
    setAuctionEnd("");
    setDescription("");
    setPhotoFile(null);
    setPhotoPreview("");
  };

  const handlePhotoChange = (e: ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0] || null;
    setPhotoFile(file);

    if (file) {
      const imageUrl = URL.createObjectURL(file);
      setPhotoPreview(imageUrl);
    }
  };

  const handleSaveLot = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (!editingLotId || !title || !startPrice || !auctionEnd || !description) {
      alert("Будь ласка, заповніть усі поля.");
      return;
    }

    setLots((prev) =>
      prev.map((lot) =>
        lot.id === editingLotId
          ? {
              ...lot,
              title,
              price: Number(startPrice),
              auctionEnd,
              description,
              image: photoPreview || lot.image,
            }
          : lot
      )
    );

    closeModal();
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
            className="lots-sidebarItem"
            type="button"
            onClick={() => (window.location.href = "/bidhistory")}
          >
            <img src="/img/Auction.svg" className="lots-menuIcon" alt="" />
            Історія ставок
          </button>

          <button
            className="lots-sidebarItem active"
            type="button"
            onClick={() => (window.location.href = "/my-lots")}
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
            <h1 className="lots-title">Мої лоти</h1>

            <button className="lots-logoutBtn" type="button">
              <img src="/img/Logout.jpg" alt="logout" />
            </button>
          </div>

          <div className="lots-grid">
            {lots.map((lot) => (
              <div key={lot.id} className="lots-card mylot-card">
                <a href={`/lot/${lot.id}`} className="mylot-link">
                  <div className="lots-imgWrap">
                    <img src={lot.image} alt={lot.title} />
                  </div>

                  <div className="lots-cardTitle">{lot.title}</div>

                  <div className="lots-priceTag">
                    {lot.price} <span className="lots-currency">грн</span>
                  </div>
                </a>

                <button
                  className="mylot-editBtn"
                  type="button"
                  onClick={() => openEditModal(lot)}
                  aria-label="Редагувати лот"
                >
                  <img src="/img/Edit.svg" alt="" />
                </button>
              </div>
            ))}
          </div>
        </div>
      </div>

      {isModalOpen && (
        <div className="lots-modalOverlay" onClick={closeModal}>
          <div className="lots-modal" onClick={(e) => e.stopPropagation()}>
            <div className="lots-modalHeader">
              <h2 className="lots-modalTitle">Редагувати лот</h2>

              <button
                className="lots-modalClose"
                type="button"
                onClick={closeModal}
                aria-label="close"
              >
                ×
              </button>
            </div>

            <form className="lots-form" onSubmit={handleSaveLot}>
              <div className="lots-modalBody">
                <div className="lots-photoCol">
                  <label className="lots-photoBox">
                    {photoPreview ? (
                      <img
                        className="lots-previewImg"
                        src={photoPreview}
                        alt="preview"
                      />
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
                      id="lotTitle"
                    />
                    <label className="ll-label" htmlFor="lotTitle">
                      Назва лоту
                    </label>
                    <div className="ll-helper">Вкажіть назву лоту</div>
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
                    <div className="ll-helper">
                      Вкажіть стартову вартість у грн
                    </div>
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
                    <div className="ll-helper">
                      Оберіть дату і час завершення
                    </div>
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
                    <div className="ll-helper">Коротко опишіть лот</div>
                  </div>
                </div>
              </div>

              <div className="lots-modalActions">
                <button
                  className="lots-cancelBtn"
                  type="button"
                  onClick={closeModal}
                >
                  Скасувати
                </button>

                <button className="lots-saveBtn" type="submit">
                  Зберегти
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}