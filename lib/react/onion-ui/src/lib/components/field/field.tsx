import { cn } from "../../utils/style";

export function Field({
	orientation,
	children,
	...props
}: {
	orientation: "horizontal" | "vertical";
} & React.InputHTMLAttributes<HTMLInputElement>) {
	const orientationClasses = {
		horizontal: "flex-row",
		vertical: "flex-col",
	};

	return (
		<div
			{...props}
			className={cn(
				"flex gap-2",
				orientationClasses[orientation],
				props.className,
			)}
		>
			{children}
		</div>
	);
}

export function FieldDescription({
	children,
	...props
}: React.InputHTMLAttributes<HTMLInputElement>) {
	return (
		<p className={cn("text-neutral-600 text-sm", props.className)}>
			{children}
		</p>
	);
}

export function FieldGroup({
	orientation,
	children,
	...props
}: {
	orientation: "horizontal" | "vertical";
} & React.InputHTMLAttributes<HTMLInputElement>) {
	const orientationClasses = {
		horizontal: "flex-row",
		vertical: "flex-col",
	};

	return (
		<div
			className={cn(
				"flex gap-2",
				orientationClasses[orientation],
				props.className,
			)}
		>
			{children}
		</div>
	);
}

export function FieldLabel({
	children,
	errorMessage,
	required,
	...props
}: {
	errorMessage?: string;
	required?: boolean;
} & React.InputHTMLAttributes<HTMLInputElement>) {
	const errorMessageClasses = "text-danger-500";
	return (
		<label
			htmlFor={props.id}
			className={cn(
				"relative block text-sm",
				errorMessage && errorMessageClasses,
				props.className,
			)}
		>
			{children}
		</label>
	);
}
