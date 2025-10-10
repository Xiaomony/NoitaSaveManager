import React from "react";
import ReactDOM from "react-dom/client";

// import { useState } from "react";
// import { invoke } from "@tauri-apps/api/core";

import "./assets/Style.css";
import "./assets/Button.css";
import CommandPane from "./CommandPane.jsx";
import SavesPane from "./SavesPane.jsx";
import MsgStack, { StackProvider } from "./MsgStack.jsx";

function App() {
    return (
        <>
            <StackProvider>
                <CommandPane className="pane" />
                <SavesPane className="pane" />
                <MsgStack />
            </StackProvider>
        </>
    );
}

ReactDOM.createRoot(document.getElementById("root")).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
);
