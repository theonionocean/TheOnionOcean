import { createContext, useContext } from "react";
import { cn } from "../../utils/style";

type CardSize = "sm" | "md" | "lg" | "xl";

const CardContext = createContext<CardSize>("md");

const sizeClasses: Record<CardSize, string> = {
	sm: "p-4",
	md: "p-5",
	lg: "p-6",
	xl: "p-8",
};

const paddingBottomClasses: Record<CardSize, string> = {
	sm: "pb-4",
	md: "pb-5",
	lg: "pb-6",
	xl: "pb-8",
};

const paddingTopClasses: Record<CardSize, string> = {
	sm: "pt-4",
	md: "pt-5",
	lg: "pt-6",
	xl: "pt-8",
};

export function Card({
	children,
	size,
	...props
}: {
	children: React.ReactNode;
	size: CardSize;
} & React.HTMLAttributes<HTMLDivElement>) {
	return (
		<CardContext.Provider value={size}>
			<div
				{...props}
				className={cn(
					"rounded-md border border-brand-500 border-b-2 bg-background",
					sizeClasses[size],
					props.className,
				)}
			>
				{children}
			</div>
		</CardContext.Provider>
	);
}

export function CardHeader({
	children,
	...props
}: { children: React.ReactNode } & React.HTMLAttributes<HTMLDivElement>) {
	const size = useContext(CardContext);

	return (
		<div
			{...props}
			className={cn(
				"grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 border-neutral-700 border-b-2 border-dashed has-[>[data-slot=card-action]]:grid-cols-[1fr_auto]",
				paddingBottomClasses[size],
				props.className,
			)}
		>
			{children}
		</div>
	);
}

export function CardTitle({
	children,
	...props
}: { children: React.ReactNode } & React.HTMLAttributes<HTMLDivElement>) {
	return (
		<h3
			{...props}
			className={cn(
				"font-semibold text-lg leading-none tracking-tight",
				props.className,
			)}
		>
			{children}
		</h3>
	);
}

export function CardDescription({
	children,
	...props
}: { children: React.ReactNode } & React.HTMLAttributes<HTMLDivElement>) {
	return (
		<p
			{...props}
			className={cn("text-muted-foreground text-sm", props.className)}
		>
			{children}
		</p>
	);
}

export function CardAction({
	children,
	...props
}: { children: React.ReactNode } & React.HTMLAttributes<HTMLDivElement>) {
	return (
		<div
			{...props}
			data-slot="card-action"
			className={cn(
				"col-start-2 row-span-2 row-start-1 self-start justify-self-end",
				props.className,
			)}
		>
			{children}
		</div>
	);
}

export function CardContent({
	children,
	...props
}: { children: React.ReactNode } & React.HTMLAttributes<HTMLDivElement>) {
	const size = useContext(CardContext);

	return (
		<div {...props} className={cn(paddingTopClasses[size], props.className)}>
			{children}
		</div>
	);
}

export function CardFooter({
	children,
	...props
}: { children: React.ReactNode } & React.HTMLAttributes<HTMLDivElement>) {
	const size = useContext(CardContext);

	return (
		<div
			{...props}
			className={cn(
				"mt-6 flex gap-2 border-neutral-700 border-t-2 border-dashed",
				paddingTopClasses[size],
				props.className,
			)}
		>
			{children}
		</div>
	);
}
