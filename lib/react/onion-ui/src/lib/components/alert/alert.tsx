import { cn } from "../../utils/style";

export function Alert({
	children,
	variant,
	...props
}: {
	children: React.ReactNode;
	variant: "default" | "success" | "danger" | "warning" | "info";
} & React.HTMLAttributes<HTMLDivElement>) {
	const variantClasses = {
		default: "border-brand-300 bg-brand-100 text-brand-700",
		success: "border-success-300 bg-success-100 text-success-700",
		danger: "border-danger-300 bg-danger-100 text-danger-700",
		warning: "border-warning-300 bg-warning-100 text-warning-700",
		info: "border-info-300 bg-info-100 text-info-700",
	};

	return (
		<div
			{...props}
			className={cn(
				"inline-block gap-0.5 rounded-md border border-b-2 p-2.5",
				variantClasses[variant],
				props.className,
			)}
		>
			{children}
		</div>
	);
}

export function AlertTitle({ children }: { children: React.ReactNode }) {
	return <h3 className="font-medium text-sm">{children}</h3>;
}

export function AlertDescription({ children }: { children: React.ReactNode }) {
	return <p className="text-gray-500 text-sm">{children}</p>;
}

export function AlertAction({
	children,
	...props
}: { children: React.ReactNode } & React.HTMLAttributes<HTMLDivElement>) {
	return (
		<div
			className={cn("inline-flex items-center gap-1", props.className)}
			{...props}
		>
			{children}
		</div>
	);
}
