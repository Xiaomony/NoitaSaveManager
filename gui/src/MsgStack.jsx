import FloatPane from "./FloatPane.jsx";
import { motion, AnimatePresence } from "framer-motion";
import { getGlobals } from "./Globals.jsx";

/*
message object:
{
    content: "",
    is_displaying: bool,
    log_grade: 1 2 3 4 5,        // fatal  warning  log  log_green  debug
    msg_id: 0 1 3 4 ...
}
 */

export default function MsgStack() {
    const {
        msg_stack_utils: { stack, setMsgStack, msgBoxDisappear },
        stack_state_utils: { stackState, setStackState },
        bkg_disability_utils: { setBkgDisability },
    } = getGlobals();
    function msg_mapper(item) {
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
        const crr_state = stackState;
        return (
            <motion.div
                key={item.msg_id}
                layout
                initial={{ x: 200, opacity: 0 }}
                animate={{ x: 0, opacity: 1 }}
                exit={{ x: 200, opacity: 0 }}
                transition={{
                    type: "spring",
                    stiffness: 300,
                    damping: 20,
                    layout: { duration: 0.3 },
                }}
                drag="x"
                dragConstraints={{ left: 0, right: 200 }}
                dragElastic={0.3}
                onDragEnd={(_, info) => {
                    if (info.offset.x > 10) {
                        msgBoxDisappear(item.msg_id, crr_state);
                    }
                }}
            >
                <FloatPane className="msgbox" title={title} color={color}>
                    {item.content}
                </FloatPane>
            </motion.div>
        );
    }
    const messages = stackState
        ? stack.map(msg_mapper)
        : stack.filter((item) => item.is_showing).map(msg_mapper);
    if (stackState) {
        messages.reverse();
    }

    return (
        <AnimatePresence>
            {stackState ? (
                <motion.div
                    className="centered_floating_pane pane"
                    key={-1}
                    initial={{ y: +500, opacity: 0 }}
                    animate={{ x: "-50%", y: "-50%", opacity: 1 }}
                    exit={{ opacity: 0 }}
                    transition={{
                        type: "spring",
                        stiffness: 300,
                        damping: 20,
                    }}
                >
                    <div
                        style={{
                            display: "flex",
                            justifyContent: "space-evenly",
                        }}
                    >
                        <button
                            type="button"
                            style={{ width: "45%", height: "50px" }}
                            onClick={() => setMsgStack([])}
                        >
                            Clear History
                        </button>
                        <button
                            type="button"
                            style={{ width: "45%", height: "50px" }}
                            onClick={() => {
                                setStackState(false);
                                setBkgDisability(false);
                            }}
                        >
                            Close
                        </button>
                    </div>
                    {messages.length == 0 ? (
                        <p>No history</p>
                    ) : (
                        <>
                            <p>向右拖动以删除某条消息</p>
                            {messages}
                        </>
                    )}
                </motion.div>
            ) : (
                <div className="msg_stack">
                    <AnimatePresence>{messages}</AnimatePresence>
                </div>
            )}
        </AnimatePresence>
    );
}
