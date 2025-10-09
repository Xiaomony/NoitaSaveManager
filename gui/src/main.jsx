import React from "react";
import ReactDOM from "react-dom/client";

// import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import "./assets/Style.css";
import "./assets/Button.css";
import CommandPane from "./CommandPane.jsx";
import SavesPane from "./SavesPane.jsx";

ReactDOM.createRoot(document.getElementById("root")).render(
    <React.StrictMode>
        <CommandPane className="pane" />
        <SavesPane className="pane" />
    </React.StrictMode>,
);
