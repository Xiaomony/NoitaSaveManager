import "./assets/MessagePaneStyle.css";
import { motion, AnimatePresence } from "framer-motion";

function MessagePane(props) {
    const style = {
        color: props.color,
        [props.bottom ? "bottom" : "top"]: props.vertical_interval,
        [props.right ? "right" : "left"]: props.horizontal_interval,
    };

    return (
        <div className={`${props.className || ""} message_pane`}>
            <p className="message_pane_title" style={style}>
                {props.title}
            </p>
            <p
                className="message_pane_content"
                style={{ whiteSpace: "pre-line" }}
            >
                {props.children}
            </p>
        </div>
    );
}

export function CenteredFloatingPane(props) {
    return (
        <AnimatePresence>
            {props.display ? (
                <motion.div
                    className="centered_floating_pane pane"
                    initial={{ y: +500, opacity: 0 }}
                    animate={{ x: "-50%", y: "-50%", opacity: 1 }}
                    exit={{ opacity: 0 }}
                    transition={{
                        type: "spring",
                        stiffness: 300,
                        damping: 20,
                    }}
                    key={props.key}
                >
                    {props.children}
                </motion.div>
            ) : null}
        </AnimatePresence>
    );
}

export default MessagePane;
