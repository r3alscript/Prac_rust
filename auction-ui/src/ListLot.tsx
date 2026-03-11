import { useState } from "react";
import type { ChangeEvent, FormEvent } from "react";
import "./ListLot.css";

type LotCard = {
  id: number;
  title: string;
  price: number;
  image: string;
};

const lots: LotCard[] = [
  { id: 1, title: "Стіл дерев’яний", price: 125, image: "/img/chair.jpg" },
  { id: 2, title: "Будинок", price: 125, image: "/img/house.png" },
  { id: 3, title: "Будинок", price: 125, image: "/img/house.png" },
  { id: 4, title: "Будинок", price: 125, image: "/img/house.png" },
{ id: 5, title: "Стіл дерев’яний", price: 125, image: "/img/chair.jpg" },
{ id: 6, title: "Стіл дерев’яний", price: 125, image: "/img/chair.jpg" },
];

export default function LotsPage() {
  const username = "RjhbcnefX";

  const [isModalOpen, setIsModalOpen] = useState(false);
  const [startPrice, setStartPrice] = useState("");
  const [auctionEnd, setAuctionEnd] = useState("");
  const [description, setDescription] = useState("");
  const [photoFile, setPhotoFile] = useState<File | null>(null);
  const [photoPreview, setPhotoPreview] = useState("");

  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);

  const handlePhotoChange = (e: ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0] || null;
    setPhotoFile(file);

    if (file) {
      const imageUrl = URL.createObjectURL(file);
      setPhotoPreview(imageUrl);
    } else {
      setPhotoPreview("");
    }
  };

  const handleCreateLot = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (!startPrice || !auctionEnd || !description || !photoFile) {
      alert("Будь ласка, заповніть усі поля.");
      return;
    }

    alert(
      `Лот створено!
Початкова ціна: ${startPrice} грн
Закінчення: ${auctionEnd}
Опис: ${description}`
    );

    setStartPrice("");
    setAuctionEnd("");
    setDescription("");
    setPhotoFile(null);
    setPhotoPreview("");
    setIsModalOpen(false);
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
            <button className="lots-logoutBtn">
      <img src="/img/Logout.jpg" alt="logout"/>
    </button>
          </div>

          <div className="lots-grid">
            {lots.map((x) => (
              <a key={x.id} href={`/lot/${x.id}`} className="lots-card">
                <div className="lots-imgWrap">
                  <img src={x.image} alt={x.title} />
                </div>

                <div className="lots-cardTitle">{x.title}</div>

                <div className="lots-priceTag">
                  {x.price} <span className="lots-currency">грн</span>
                </div>
              </a>
            ))}
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
                  Створити
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}