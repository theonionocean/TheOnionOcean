import { cn } from "../../utils/style";

export function InputGroup({
	children,
	...props
}: { children: React.ReactNode } & React.HTMLAttributes<HTMLDivElement>) {
	return (
		<div
			{...props}
			className={cn(
				"flex h-9 text-nowrap rounded-md border border-brand-800 border-b-2 px-3 py-2 text-base placeholder:text-neutral-500 focus-within:border-brand-700 hover:border-brand-700 disabled:cursor-not-allowed disabled:border-neutral-400 disabled:text-neutral-500",
				"[&_svg]:size-4",
				"[&_input]:h-auto [&_input]:w-full [&_input]:border-0 [&_input]:bg-transparent [&_input]:p-0 [&_input]:px-2 [&_input]:outline-none [&_input]:hover:border-0 [&_input]:focus:border-0 [&_input]:focus:outline-none [&_input]:focus-visible:outline-none",
				props.className,
			)}
		>
			{children}
		</div>
	);
}

export function InputGroupAddon({
	children,
	align,
	...props
}: {
	children: React.ReactNode;
	align: "line" | "column";
} & React.HTMLAttributes<HTMLDivElement>) {
	const alignClasses = {
		line: "flex-row",
		column: "flex-col",
	};

	return (
		<div
			{...props}
			className={cn(
				"flex items-center justify-center",
				alignClasses[align],
				props.className,
			)}
		>
			{children}
		</div>
	);
}
