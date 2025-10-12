import "./assets/TableStyle.css";
import { getGlobals } from "./Globals.jsx";

function SavesPane(props) {
    const [[, ,], [stackState, ,]] = getGlobals();
    return (
        <div
            className={`${props.className || ""} ${stackState ? "disabled" : ""}`}
            id="saves_pane"
            style={{ filter: stackState ? "blur(5px)" : null }}
        >
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
        </div>
    );
}

export default SavesPane;
