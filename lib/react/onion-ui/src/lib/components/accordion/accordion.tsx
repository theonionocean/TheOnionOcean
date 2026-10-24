import { createContext, useContext, useState } from "react";
import { cn } from "../../utils/style";
import { Minus, Plus } from "lucide-react";

type AccordionContextType = {
	openValues: string[];
	toggle: (value: string) => void;
};

type AccordionItemContextType = {
	value: string;
	open: boolean;
};

const AccordionContext = createContext<AccordionContextType>({
	openValues: [],
	toggle: () => {},
});

const AccordionItemContext = createContext<AccordionItemContextType>({
	value: "",
	open: false,
});

function normalizeDefaultValue(defaultValue?: string | string[]) {
	if (defaultValue == null) return [];
	return Array.isArray(defaultValue) ? defaultValue : [defaultValue];
}

export function Accordion({
	children,
	multiple = false,
	defaultValue,
	...props
}: {
	children: React.ReactNode;
	multiple?: boolean;
	defaultValue?: string | string[];
} & React.HTMLAttributes<HTMLDivElement>) {
	const [openValues, setOpenValues] = useState(() => {
		const initial = normalizeDefaultValue(defaultValue);
		return multiple ? initial : initial.slice(0, 1);
	});

	function toggle(value: string) {
		setOpenValues((current) => {
			const isOpen = current.includes(value);

			if (multiple) {
				return isOpen
					? current.filter((item) => item !== value)
					: [...current, value];
			}

			return isOpen ? [] : [value];
		});
	}

	return (
		<AccordionContext.Provider value={{ openValues, toggle }}>
			<div {...props}>{children}</div>
		</AccordionContext.Provider>
	);
}

export function AccordionItem({
	children,
	value,
	...props
}: {
	children: React.ReactNode;
	value: string;
} & React.HTMLAttributes<HTMLDivElement>) {
	const { openValues } = useContext(AccordionContext);
	const open = openValues.includes(value);

	return (
		<AccordionItemContext.Provider value={{ value, open }}>
			<div
				{...props}
				className={cn("rounded border border-neutral-800", props.className)}
			>
				{children}
			</div>
		</AccordionItemContext.Provider>
	);
}

export function AccordionTrigger({
	children,
	...props
}: { children: React.ReactNode } & React.HTMLAttributes<HTMLButtonElement>) {
	const { toggle } = useContext(AccordionContext);
	const { value, open } = useContext(AccordionItemContext);

	return (
		<button
			type="button"
			onClick={() => toggle(value)}
			{...props}
			className={cn(
				"flex w-full items-center justify-between border-b-2 px-3.5 py-3 text-left font-semibold transition-colors duration-300 ease-in-out",
				open
					? "rounded-b border-brand-600 bg-brand-900 text-brand-100"
					: "border-transparent bg-transparent",
				props.className,
			)}
		>
			<span>{children}</span>
			{open ? (
				<Minus className="ml-8 shrink-0 fill-brand-100" />
			) : (
				<Plus className="ml-8 shrink-0 fill-brand-100" />
			)}
		</button>
	);
}

export function AccordionContent({
	children,
	...props
}: {
	children: React.ReactNode;
} & React.HTMLAttributes<HTMLDivElement>) {
	const { open } = useContext(AccordionItemContext);

	return (
		<div
			{...props}
			className={cn(
				"grid overflow-hidden text-slate-600 text-sm transition-[grid-template-rows] duration-300 ease-in-out",
				open ? "grid-rows-[1fr]" : "grid-rows-[0fr]",
				props.className,
			)}
		>
			<div className="min-h-0 overflow-hidden">
				<p className="px-3.5 py-3 pb-3">{children}</p>
			</div>
		</div>
	);
}
