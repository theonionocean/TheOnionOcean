import { Check } from "lucide-react";
import { cn } from "../../utils/style";
import { Field, FieldDescription, FieldLabel } from "../field/field";

export function Checkbox({
	label,
	description,
	invalid,
	...props
}: {
	label: string;
	description?: string;
	invalid?: boolean;
} & React.InputHTMLAttributes<HTMLInputElement>) {
	const invalidClasses = invalid
		? "border-red-500 text-red-500 hover:not-checked:border-red-700"
		: "";

	return (
		<Field orientation="horizontal" className="items-start">
			<FieldLabel className="flex cursor-pointer items-start gap-2">
				<span className="relative mt-0.5 inline-flex size-4 shrink-0">
					<input
						type="checkbox"
						{...props}
						id={props.id}
						className={cn(
							"peer size-4 cursor-pointer appearance-none rounded-xs border border-neutral-700 border-b-2 checked:border-brand-800 checked:bg-brand-600 not-checked:hover:border-neutral-400 checked:hover:border-brand-800 checked:hover:bg-brand-700 disabled:not-checked:hover:border-neutral-700 disabled:checked:hover:bg-brand-600",
							props.className,
							invalidClasses,
						)}
					/>
					<Check className="pointer-events-none absolute inset-0 m-auto size-3 text-white opacity-0 peer-checked:opacity-100" />
				</span>
				<span className="flex min-w-0 flex-col gap-0.5">
					<span
						className={cn(
							"select-none font-medium text-heading text-sm leading-4",
							invalidClasses,
						)}
					>
						{label}
					</span>
					{description && (
						<FieldDescription className="text-neutral-600 text-sm">
							{description}
						</FieldDescription>
					)}
				</span>
			</FieldLabel>
		</Field>
	);
}
