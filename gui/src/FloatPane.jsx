import "./assets/FloatPaneStyle.css";

function FloatPane(props) {
    const style = {
        color: props.color,
        [props.bottom ? "bottom" : "top"]: props.vertical_interval,
        [props.right ? "right" : "left"]: props.horizontal_interval,
    };

    return (
        <div className="float_pane">
            <p className="float_pane_title" style={style}>
                {props.title}
            </p>
            <p className="float_pane_content">{props.children}</p>
        </div>
    );
}

export default FloatPane;
