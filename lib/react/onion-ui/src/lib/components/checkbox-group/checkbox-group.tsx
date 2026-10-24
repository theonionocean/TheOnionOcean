import { FieldGroup } from "../field/field";

export function CheckboxGroup({
	children,
	orientation,
}: {
	children: React.ReactNode;
	orientation: "horizontal" | "vertical";
}) {
	return <FieldGroup orientation={orientation}>{children}</FieldGroup>;
}
