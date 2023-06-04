import ReactDOM from "react-dom/client";
import React from "react";
import App from "./app";

import "allotment/dist/style.css";
import "./style.css"

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
