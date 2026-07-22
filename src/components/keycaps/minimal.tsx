import { keymaps } from "@/lib/keymaps";
import { useKeyStyle } from "@/stores/key_style";
import type { KeycapProps } from ".";

export const MinimalKeycap = ({ event }: KeycapProps) => {
    const text = useKeyStyle((state) => state.text);
    const modifier = useKeyStyle((state) => state.modifier);
    const layout = useKeyStyle((state) => state.layout);

    const display = keymaps[event.name];
    const color = event.isModifier() && modifier.highlight ? modifier.textColor : text.color;
    const textStyle: React.CSSProperties = {
        color,
        lineHeight: 1.2,
        fontSize: text.size,
        textTransform: text.caps,
        gap: ".1em",
    };

    const label = display?.shortLabel ?? display.label;
    let child = <>{label}</>;

    if (event.isModifier() && layout.showIcon && display.icon) {
        const Icon = display.icon;
        if (text.variant === "icon" || event.isArrow()) {
            child = <Icon color={color} size={text.size} />;
        } else {
            child = <>
                <Icon color={color} size={text.size} />
                <div style={{ ...textStyle }}>
                    {text.variant === "text" ? display.label : label}
                </div>
            </>;
        }
    }

    return (
        <div
            className="flex items-center h-full"
            style={textStyle}
        >
            {child}
        </div>
    );
};
