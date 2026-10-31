import { createContext, useContext, useState } from "react";
import { Toggle } from "../toggle/toggle";

const ToggleGroupContext = createContext<{
	value: string[];
	setValue: (_value: string) => void;
	variant: ToggleGroupProps["variant"];
	size: ToggleGroupProps["size"];
}>({
	value: [],
	setValue: (_value: string) => {},
	variant: "primary",
	size: "default",
});

type ToggleGroupProps = {
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
};

export function ToggleGroup({
	children,
	variant,
	size,
	defaultValue,
	multiple = false,
	...props
}: {
	variant: ToggleGroupProps["variant"];
	size: ToggleGroupProps["size"];
	defaultValue?: string[];
	multiple?: boolean;
} & React.ComponentProps<"div">) {
	const [valueState, setValueState] = useState<string[]>(defaultValue ?? []);

	const handleClick = (value: string) => {
		setValueState((current) => {
			const isSelected = current.includes(value);

			if (multiple) {
				return isSelected
					? current.filter((item) => item !== value)
					: [...current, value];
			}

			return isSelected ? [] : [value];
		});
	};

	return (
		<ToggleGroupContext.Provider
			value={{
				value: valueState,
				setValue: handleClick,
				variant,
				size,
			}}
		>
			<div {...props}>{children}</div>
		</ToggleGroupContext.Provider>
	);
}

export function ToggleGroupItem({
	children,
	value: valueProp,
	onClick,
	...props
}: {
	value: string;
} & React.ComponentProps<"button">) {
	const { value, setValue, variant, size } = useContext(ToggleGroupContext);

	return (
		<Toggle
			variant={variant}
			size={size}
			value={valueProp}
			pressed={value.includes(valueProp)}
			{...props}
			onClick={(event) => {
				setValue(valueProp);
				onClick?.(event);
			}}
		>
			{children}
		</Toggle>
	);
}
