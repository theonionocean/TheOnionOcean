import { cn } from "../../utils/style";
import { Field, FieldDescription, FieldLabel } from "../field/field";

export function Input({
	...props
}: React.InputHTMLAttributes<HTMLInputElement>) {
	return (
		<input
			type={props.type || "text"}
			{...props}
			id={props.id}
			className={cn(
				"h-9 rounded-md border border-brand-800 border-b-2 px-3 py-2 text-base placeholder:text-neutral-500 hover:border-brand-700 focus:border-brand-700 disabled:cursor-not-allowed disabled:border-neutral-400 disabled:text-neutral-500",
				props.className,
			)}
		/>
	);
}

export function FieldInput({
	label,
	description,
	errorMessage,
	required,
	...props
}: {
	label: string;
	description?: string;
	errorMessage?: string;
	required?: boolean;
} & React.InputHTMLAttributes<HTMLInputElement>) {
	const errorMessageClasses =
		"text-danger-500 border-danger-500 focus:border-danger-500";
	return (
		<Field orientation="vertical">
			<FieldLabel id={props.id} errorMessage={errorMessage} required={required}>
				<span
					className={cn(
						"absolute top-0 left-3 -translate-y-1/2 bg-background px-1.5 font-bold text-sm after:content-['_*']",
						required && "after:text-danger-500",
						errorMessage && errorMessageClasses,
						props.className,
					)}
				>
					{label}
				</span>
				<Input
					id={props.id}
					{...props}
					className={errorMessage && errorMessageClasses}
				/>
			</FieldLabel>
			<FieldDescription className={errorMessage && errorMessageClasses}>
				{errorMessage || description}
			</FieldDescription>
		</Field>
	);
}
