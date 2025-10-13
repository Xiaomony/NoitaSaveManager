import React from "react";
import ReactDOM from "react-dom/client";

import "./assets/Style.css";
import "./assets/Button.css";
import CommandPane from "./CommandPane.jsx";
import SavesPane from "./SavesPane.jsx";
import MsgStack from "./MsgStack.jsx";
import { GlobalProvider, getGlobals } from "./Globals.jsx";
import { CenteredFloatingPane } from "./MessagePane.jsx";

function App() {
    const {
        query_window_utils: { queryWindowState },
    } = getGlobals();
    return (
        <>
            <CommandPane className="pane" />
            <SavesPane className="pane" />
            {queryWindowState.enabled ? (
                <CenteredFloatingPane key={-2}>
                    {queryWindowState.child}
                </CenteredFloatingPane>
            ) : null}
            <MsgStack />
        </>
    );
}

ReactDOM.createRoot(document.getElementById("root")).render(
    <React.StrictMode>
        <GlobalProvider>
            <App />
        </GlobalProvider>
    </React.StrictMode>,
);
