import "./assets/TableStyle.css";
import { getStack } from "./MsgStack.jsx";

function SavesPane(props) {
    const [stack, setStack] = getStack();
    const btn_callback = () => {
        setStack([
            ...stack,
            {
                content: "soasdfasdf",
                is_showing: true,
                log_grade: 1,
            },
        ]);
    };
    return (
        <div className={props.className} id="saves_pane">
            <table className="saves_table">
                <caption>Saves Information</caption>
                <thead>
                    <tr>
                        <th scope="col"></th>
                        <th scope="col">Name</th>
                        <th scope="col">Date</th>
                        <th scope="col">Time</th>
                        <th scope="col">Note</th>
                    </tr>
                </thead>
                <tbody></tbody>
            </table>
            <button type="button" onClick={btn_callback}>
                aaa
            </button>
        </div>
    );
}

export default SavesPane;
