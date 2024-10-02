import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { App } from "./App.tsx";
import { OpenAPI } from "./api/index.ts";
import "./style.css";
import "@fontsource/roboto/300.css";
import "@fontsource/roboto/400.css";
import "@fontsource/roboto/500.css";
import "@fontsource/roboto/700.css";

OpenAPI.BASE = "http://192.168.2.56:8080";
//if the port is 9080, then base is 9090
if (window.location.port === "9080") {
  OpenAPI.BASE = "http://192.168.2.56:9090";
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>
);
