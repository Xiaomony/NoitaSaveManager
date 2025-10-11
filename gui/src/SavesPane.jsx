import "./assets/TableStyle.css";

function SavesPane(props) {
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
        </div>
    );
}

export default SavesPane;
