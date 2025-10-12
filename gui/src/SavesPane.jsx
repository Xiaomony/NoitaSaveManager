import "./assets/TableStyle.css";
import { getGlobals } from "./Globals.jsx";

function SavesPane(props) {
    const {
        stack_state_utils: { stackState },
        save_info_utils: { saveInfos },
    } = getGlobals();

    console.log(saveInfos);

    return (
        <div
            className={`${props.className || ""} ${stackState ? "disabled" : ""}`}
            id="saves_pane"
            style={{ filter: stackState ? "blur(5px)" : null }}
        >
            <table className="saves_table">
                <caption>Saves Information</caption>
                <colgroup>
                    <col style={{ width: "7%" }} />
                    <col style={{ width: "15%" }} />
                    <col style={{ width: "15%" }} />
                    <col style={{ width: "28%" }} />
                    <col style={{ width: "35%" }} />
                </colgroup>
                <thead>
                    <tr>
                        <th scope="col">Select</th>
                        <th scope="col">Date</th>
                        <th scope="col">Time</th>
                        <th scope="col">Name</th>
                        <th scope="col">Note</th>
                    </tr>
                </thead>
                <tbody>
                    {saveInfos.map((save) => (
                        <tr>
                            <th scope="col">
                                <input type="checkbox" />
                            </th>
                            <th scope="col">{save.m_date}</th>
                            <th scope="col">{save.m_time}</th>
                            <th scope="col">{save.m_name}</th>
                            <th scope="col">{save.m_note}</th>
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    );
}

export default SavesPane;
