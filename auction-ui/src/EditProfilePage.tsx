import { useState } from "react";
import type { ChangeEvent, FormEvent } from "react";
import "./EditProfilePage.css";

export default function EditProfilePage() {
  const username = "RjhbcnefX";

  const [firstName, setFirstName] = useState("Іван");
  const [lastName, setLastName] = useState("Петренко");
  const [nickname, setNickname] = useState("RjhbcnefX");
  const [photoFile, setPhotoFile] = useState<File | null>(null);
  const [photoPreview, setPhotoPreview] = useState("/img/default-avatar.png");

  const handlePhotoChange = (e: ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0] || null;
    setPhotoFile(file);

    if (file) {
      const imageUrl = URL.createObjectURL(file);
      setPhotoPreview(imageUrl);
    }
  };

  const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (!firstName.trim() || !lastName.trim() || !nickname.trim()) {
      alert("Будь ласка, заповніть усі поля.");
      return;
    }

    alert(
      `Профіль оновлено!\nІм’я: ${firstName}\nПрізвище: ${lastName}\nНікнейм: ${nickname}`
    );
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
            className="lots-sidebarItem"
            type="button"
            onClick={() => (window.location.href = "/mylot")}
          >
            <img src="/img/Purchaseorder.svg" className="lots-menuIcon" alt="" />
            Мої лоти
          </button>

          <button
            className="lots-sidebarItem active"
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
            <h1 className="lots-title">Редагувати профіль</h1>

            <button className="lots-logoutBtn" type="button">
              <img src="/img/Logout.jpg" alt="logout" />
            </button>
          </div>

          <div className="profile-card">
            <form className="profile-form" onSubmit={handleSubmit}>
              <div className="profile-photoSection">
                <label className="profile-photoBox">
                  {photoPreview ? (
                    <img
                      src={photoPreview}
                      alt="profile preview"
                      className="profile-previewImg"
                    />
                  ) : (
                    <div className="profile-photoPlaceholder" />
                  )}

                  <input
                    className="profile-hiddenFile"
                    type="file"
                    accept="image/*"
                    onChange={handlePhotoChange}
                  />
                </label>

                <div className="profile-photoHint">
                  Натисніть на фото, щоб змінити його
                </div>
              </div>

              <div className="profile-fields">
                <div className="profile-field">
                  <input
                    className="profile-input"
                    type="text"
                    placeholder=" "
                    id="firstName"
                    value={firstName}
                    onChange={(e) => setFirstName(e.target.value)}
                  />
                  <label className="profile-label" htmlFor="firstName">
                    Ім’я
                  </label>
                </div>

                <div className="profile-field">
                  <input
                    className="profile-input"
                    type="text"
                    placeholder=" "
                    id="lastName"
                    value={lastName}
                    onChange={(e) => setLastName(e.target.value)}
                  />
                  <label className="profile-label" htmlFor="lastName">
                    Прізвище
                  </label>
                </div>

                <div className="profile-field">
                  <input
                    className="profile-input"
                    type="text"
                    placeholder=" "
                    id="nickname"
                    value={nickname}
                    onChange={(e) => setNickname(e.target.value)}
                  />
                  <label className="profile-label" htmlFor="nickname">
                    Нікнейм
                  </label>
                </div>

                <div className="profile-actions">
                  <button
                    className="profile-cancelBtn"
                    type="button"
                    onClick={() => window.location.reload()}
                  >
                    Скасувати
                  </button>

                  <button className="profile-saveBtn" type="submit">
                    Зберегти
                  </button>
                </div>
              </div>
            </form>
          </div>
        </div>
      </div>
    </div>
  );
}