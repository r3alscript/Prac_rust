import "./OAuthPage.css";

const API_BASE = "http://localhost:8080";

export default function OAuthPage() {
    const handleGoogleLogin = () => {
        window.location.href = `${API_BASE}/auth/google/login`;
    };


    return (
        <div className="oauth-page">
            <div className="oauth-card">
                <div className="oauth-header">
                    <h1>Авторизація</h1>
                    <p>
                        Увійдіть у систему за допомогою вашого облікового запису Google
                    </p>
                </div>

                <div className="oauth-buttons">
                    <button className="oauth-btn google-btn" onClick={handleGoogleLogin}>
                        <span className="oauth-icon">G</span>
                        <span>Увійти через Google</span>
                    </button>


                </div>

                <div className="oauth-footer">

                </div>
            </div>
        </div>
    );
}