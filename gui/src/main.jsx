import React from "react";
import ReactDOM from "react-dom/client";
import "./msgListener.jsx";

// import { useState } from "react";
// import { invoke } from "@tauri-apps/api/core";

import "./assets/Style.css";
import "./assets/Button.css";
import CommandPane from "./CommandPane.jsx";
import SavesPane from "./SavesPane.jsx";
import MsgStack from "./MsgStack.jsx";
import { GlobalProvider } from "./Globals.jsx";

function App() {
    return (
        <GlobalProvider>
            <CommandPane className="pane" />
            <SavesPane className="pane" />
            <MsgStack />
        </GlobalProvider>
    );
}

ReactDOM.createRoot(document.getElementById("root")).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
);
