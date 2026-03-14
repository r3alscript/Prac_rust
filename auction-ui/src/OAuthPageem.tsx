import { useEffect } from "react";
import { useNavigate } from "react-router-dom";

export default function AuthorizationPage() {
    const navigate = useNavigate();

    useEffect(() => {
        const params = new URLSearchParams(window.location.search);
        const token = params.get("token");

        if (token) {
            localStorage.setItem("access_token", token);
            navigate("/lots", { replace: true });
        }
    }, [navigate]);

    return <div>Авторизація...</div>;
}