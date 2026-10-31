import { Check, ChevronRightIcon } from "lucide-react";
import {
	cloneElement,
	createContext,
	useContext,
	useEffect,
	useRef,
	useState,
} from "react";
import { cn } from "../../utils/style";

const DropdownContext = createContext({
	open: false,
	setOpen: (_open: boolean) => {},
});

const DropdownRadioGroupContext = createContext({
	value: "",
	setValue: (_value: string) => {},
});

export function Dropdown({ children }: { children: React.ReactNode }) {
	const [open, setOpen] = useState(false);

	return (
		<DropdownContext.Provider value={{ open, setOpen }}>
			<div className="relative">{children}</div>
		</DropdownContext.Provider>
	);
}

export function DropdownTrigger({
	render,
}: {
	render: React.ReactElement<{ onClick?: () => void }>;
}) {
	const { open, setOpen } = useContext(DropdownContext);
	return cloneElement(render, { onClick: () => setOpen(!open) });
}

export function DropdownContent({
	children,
	align,
	...props
}: {
	children: React.ReactNode;
	align?: "start" | "center" | "end";
} & React.HTMLAttributes<HTMLDivElement>) {
	const { open } = useContext(DropdownContext);
	if (!open) return null;

	return (
		<div
			{...props}
			className={cn(
				"absolute left-0 mt-0.5 h-auto w-full rounded-md border border-neutral-700 bg-neutral-900 p-1.5 text-neutral-50",
				props.className,
			)}
		>
			{children}
		</div>
	);
}

export function DropdownItem({
	children,
	disabled,
	variant,
	icon,
}: {
	children: React.ReactNode;
	disabled?: boolean;
	variant?: "default" | "destructive";
	icon?: React.ReactElement<{ className?: string }>;
}) {
	const disabledClass = disabled
		? "opacity-50 cursor-not-allowed hover:bg-transparent hover:border-transparent"
		: "";

	const variantClass = variant === "destructive" ? "text-danger-500" : "";

	return (
		<div
			className={cn(
				"flex items-center gap-2 px-2.5 py-1.5 text-neutral-50 hover:rounded-md hover:border-brand-600 hover:border-b hover:bg-brand-900",
				disabledClass,
				variantClass,
			)}
		>
			{icon && cloneElement(icon, { className: "size-4" })}
			{children}
		</div>
	);
}

export function DropdownSeparator() {
	return <div className="h-px w-full bg-neutral-700" />;
}

export function DropdownLabel({ children }: { children: React.ReactNode }) {
	return (
		<div className="pointer-events-none px-2.5 py-1.5 font-bold text-muted-foreground">
			{children}
		</div>
	);
}

export function DropdownShortcut({ children }: { children: React.ReactNode }) {
	return (
		<div data-slot="dropdown-shortcut" className="ml-auto text-muted-foreground">
			{children}
		</div>
	);
}

export function DropdownGroup({ children }: { children: React.ReactNode }) {
	return <div className="">{children}</div>;
}

export function DropdownSub({ children }: { children: React.ReactNode }) {
	return <div className="group/sub relative">{children}</div>;
}

export function DropdownSubTrigger({
	children,
}: {
	children: React.ReactNode;
}) {
	return (
		<div className="flex items-center justify-between px-2.5 py-1.5 hover:rounded-md hover:border-brand-600 hover:border-b hover:bg-brand-900">
			{children} <ChevronRightIcon className="size-4" />
		</div>
	);
}

export function DropdownSubContent({
	children,
	...props
}: {
	children: React.ReactNode;
} & React.HTMLAttributes<HTMLDivElement>) {
	return (
		<div
			{...props}
			className={cn(
				"min-w-32 rounded-md border border-neutral-700 bg-neutral-900 p-1.5 text-neutral-50",
				props.className,
			)}
		>
			{children}
		</div>
	);
}

export function DropdownPortal({ children }: { children: React.ReactNode }) {
	const ref = useRef<HTMLDivElement>(null);

	useEffect(() => {
		const portal = ref.current;
		const parent = portal?.parentElement;
		if (!portal || !parent) return;

		const flip = () => {
			const { right } = parent.getBoundingClientRect();
			portal.dataset.side =
				right + portal.offsetWidth > window.innerWidth ? "left" : "right";
		};

		parent.addEventListener("mouseenter", flip);
		return () => parent.removeEventListener("mouseenter", flip);
	}, []);

	return (
		<div
			ref={ref}
			data-side="right"
			className="pointer-events-none invisible absolute top-0 left-full z-50 group-hover/sub:pointer-events-auto group-hover/sub:visible data-[side=left]:right-full data-[side=left]:left-auto"
		>
			{children}
		</div>
	);
}

export function DropdownCheckboxGroup({
	children,
}: {
	children: React.ReactNode;
}) {
	return <div className="">{children}</div>;
}

export function DropdownCheckboxItem({
	children,
	disabled,
	checked,
	onCheckedChange,
	icon,
}: {
	children: React.ReactNode;
	disabled?: boolean;
	checked: boolean;
	onCheckedChange: (checked: boolean) => void;
	icon?: React.ReactElement<{ className?: string }>;
}) {
	const disabledClass = disabled
		? "opacity-50 cursor-not-allowed hover:bg-transparent hover:border-transparent"
		: "";

	return (
		<button
			type="button"
			onClick={() => onCheckedChange(checked)}
			className={cn(
				"flex w-full items-center justify-between px-2.5 py-1.5 text-neutral-50 hover:rounded-md hover:border-brand-600 hover:border-b hover:bg-brand-900",
				disabledClass,
			)}
		>
			<div className="flex items-center gap-2">
				{icon && cloneElement(icon, { className: "size-4" })}
				{children}
			</div>
			{checked && <Check className="size-4" />}
		</button>
	);
}

export function DropdownRadioGroup({
	children,
	value,
	onValueChange,
}: {
	children: React.ReactNode;
	value: string;
	onValueChange: (value: string) => void;
}) {
	return (
		<DropdownRadioGroupContext.Provider
			value={{ value: value, setValue: onValueChange }}
		>
			<div className="">{children}</div>
		</DropdownRadioGroupContext.Provider>
	);
}

export function DropdownRadioItem({
	children,
	disabled,
	value,
	icon,
}: {
	children: React.ReactNode;
	disabled?: boolean;
	value: string;
	icon?: React.ReactElement<{ className?: string }>;
}) {
	const { value: currentValue, setValue } = useContext(
		DropdownRadioGroupContext,
	);

	const disabledClass = disabled
		? "opacity-50 cursor-not-allowed hover:bg-transparent hover:border-transparent"
		: "";

	return (
		<button
			type="button"
			onClick={() => setValue(value)}
			className={cn(
				"flex w-full items-center justify-between px-2.5 py-1.5 text-neutral-50 hover:rounded-md hover:border-brand-600 hover:border-b hover:bg-brand-900",
				disabledClass,
			)}
		>
			<div className="flex items-center gap-2">
				{icon && cloneElement(icon, { className: "size-4" })}
				{children}
			</div>
			{value === currentValue && <Check className="size-4" />}
		</button>
	);
}
