import { cn } from "../../utils/style";

export function Badge({
	children,
	variant = "default",
}: {
	children: React.ReactNode;
	variant: "default" | "success" | "danger" | "warning" | "info";
}) {
	const variantClasses = {
		default: {
			body: "bg-brand-700 text-brand-100",
			left: "border-r-brand-700",
			right: "border-l-brand-700",
		},
		success: {
			body: "bg-success-300 text-success-700",
			left: "border-r-success-300",
			right: "border-l-success-300",
		},
		danger: {
			body: "bg-danger-300 text-danger-700",
			left: "border-r-danger-300",
			right: "border-l-danger-300",
		},
		warning: {
			body: "bg-warning-300 text-warning-800",
			left: "border-r-warning-300",
			right: "border-l-warning-300",
		},
		info: {
			body: "bg-info-300 text-info-700",
			left: "border-r-info-300",
			right: "border-l-info-300",
		},
	};

	const { body, left, right } = variantClasses[variant];

	return (
		<span className="inline-flex h-6 items-center font-medium text-xs">
			<span
				aria-hidden
				className={cn(
					"block size-0 border-y-12 border-y-transparent border-r-8 border-solid",
					left,
				)}
			/>
			<span className={cn("inline-flex h-full items-center px-2", body)}>
				{children}
			</span>
			<span
				aria-hidden
				className={cn(
					"block size-0 border-y-12 border-y-transparent border-l-8 border-solid",
					right,
				)}
			/>
		</span>
	);
}
