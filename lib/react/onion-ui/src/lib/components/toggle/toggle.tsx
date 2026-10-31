import { useState } from "react";
import { cn } from "../../utils/style";
import { Button } from "../button/button";

export function Toggle({
	children,
	variant,
	size,
	className,
	onClick,
	pressed: pressedProp,
	...props
}: React.ComponentProps<"button"> & {
	variant: "primary" | "secondary" | "ghost" | "danger" | "disabled";
	size:
		| "default"
		| "xs"
		| "icon-xs"
		| "sm"
		| "icon-sm"
		| "icon"
		| "lg"
		| "icon-lg";
	pressed?: boolean;
}) {
	const [uncontrolledPressed, setUncontrolledPressed] = useState(false);
	const isControlled = pressedProp !== undefined;
	const pressed = isControlled ? pressedProp : uncontrolledPressed;

	return (
		<Button
			variant={variant}
			size={size}
			aria-pressed={pressed}
			{...props}
			className={cn(
				"group/toggle",
				pressed && "mb-0.75 translate-y-0.5 border-b",
				className,
			)}
			onClick={(event) => {
				if (!isControlled) setUncontrolledPressed((current) => !current);
				onClick?.(event);
			}}
		>
			{children}
		</Button>
	);
}
