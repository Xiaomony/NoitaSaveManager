import { createContext, useContext, useState } from "react";
import FloatPane from "./FloatPane.jsx";

const stackHandle = createContext(null);

export function StackProvider({ children }) {
    const stackState = useState([]);
    return (
        <stackHandle.Provider value={stackState}>
            {children}
        </stackHandle.Provider>
    );
}

export function getStack() {
    return useContext(stackHandle);
}

/*
message object:
{
    content: "",
    is_showing: bool,
    log_grade: 1 2 3 4 5        // fatal  warning  log  log_green  debug
}
 */

export default function MsgStack() {
    const [stack, _] = getStack();
    const messages = stack
        .filter((item) => item.is_showing)
        .map((item, index) => {
            let color = null;
            let title = null;
            switch (item.log_grade) {
                case 1:
                    title = "[FATAL]";
                    color = "#e06c75";
                    break;
                case 2:
                    title = "[WARNING]";
                    color = "#e5c07b";
                    break;
                case 3:
                    title = "[LOG]";
                    color = "#56b6c2";
                    break;
                case 4:
                    title = "[LOG]";
                    color = "#98c379";
                    break;
                case 5:
                    title = "[DEBUG]";
                    color = "#c678dd";
                    break;
            }
            return (
                <FloatPane title={title} color={color} key={index}>
                    {item.content}
                </FloatPane>
            );
        });
    return <div className="msg_stack">{messages}</div>;
}
