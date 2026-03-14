import "./OAuthPage.css";
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";

const API_BASE = "http://localhost:8080";

export default function OAuthPage() {

    const navigate = useNavigate();

    useEffect(() => {
        const params = new URLSearchParams(window.location.search);
        const token = params.get("token");

        if (token && token.trim() !== "") {
            console.log("JWT token received:", token);

            localStorage.setItem("access_token", token);

            navigate("/lots", { replace: true });
        }
    }, [navigate]);

    const handleGoogleLogin = () => {
        window.location.href = `${API_BASE}/auth/google/login`;
    };

    return (
        <div className="oauth-page">

            <div className="auth-card">

                <div className="auth-left">
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
                </div>

                <div className="auth-right">
                    <img src="/img/2456073.jpg" alt="login" />
                </div>

            </div>

        </div>
    );
}