import { cn } from "../../utils/style";

// TODO: Add link variant
export function Button({
	children,
	variant,
	size,
	...props
}: {
	children: React.ReactNode;
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
} & React.ButtonHTMLAttributes<HTMLButtonElement>) {
	const variantClasses = {
		primary: "bg-brand-600 text-white border-brand-800 hover:bg-brand-700 ",
		secondary:
			"bg-secondary-400 text-white border-secondary-600 hover:bg-secondary-500 ",
		ghost: "bg-neutral-950 text-white border-neutral-700 hover:bg-neutral-800 ",
		danger: "bg-danger-400 text-white border-danger-600 hover:bg-danger-500 ",
		disabled:
			"disabled:cursor-not-allowed disabled:border-neutral-400 disabled:bg-neutral-100 disabled:text-neutral-500 disabled:active:translate-y-0 disabled:active:mb-0",
	};

	const sizeClasses = {
		default: "text-base px-2 py-1",
		xs: "text-xs px-1.5 py-0.5",
		"icon-xs": "text-xs size-6 [&_svg]:size-3",
		sm: "text-sm px-1.5 py-0.5",
		"icon-sm": "text-sm size-7 [&_svg]:size-4",
		icon: "text-base size-8",
		lg: "text-lg px-2 py-0.5",
		"icon-lg": "text-lg size-9 [&_svg]:size-4",
	};

	return (
		<button
			type="button"
			{...props}
			disabled={variant === "disabled" ? true : props.disabled}
			className={cn(
				"flex cursor-pointer items-center justify-center gap-4 rounded-md border border-b-4 active:mb-0.75 active:translate-y-0.5 active:border-b [&_svg]:pointer-events-none [&_svg]:shrink-0",
				variantClasses[variant],
				sizeClasses[size],
				props.className,
			)}
		>
			{children}
		</button>
	);
}
