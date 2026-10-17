import { createContext, useContext } from "react";
import { cn } from "../../utils/style";

type AvatarSize = "sm" | "md" | "lg";

const AvatarContext = createContext<AvatarSize>("md");

const sizeClasses: Record<AvatarSize, string> = {
	sm: "size-6",
	md: "size-8",
	lg: "size-10",
};

const radiusClasses: Record<AvatarSize, string> = {
	sm: "rounded-sm rounded-bl-none",
	md: "rounded-md rounded-bl-none",
	lg: "rounded-lg rounded-bl-none",
};

const badgeSizeClasses: Record<AvatarSize, string> = {
	sm: "size-2.5",
	md: "size-3",
	lg: "size-3.5",
};

const badgeRadiusClasses: Record<AvatarSize, string> = {
	sm: "rounded-sm rounded-bl-none",
	md: "rounded rounded-bl-none",
	lg: "rounded-md rounded-bl-none",
};

export function Avatar({
	size,
	children,
	...props
}: {
	size: AvatarSize;
	children: React.ReactNode;
} & React.HTMLAttributes<HTMLDivElement>) {
	return (
		<AvatarContext.Provider value={size}>
			<div
				className={cn(
					"relative",
					sizeClasses[size],
					radiusClasses[size],
					props.className,
				)}
				{...props}
			>
				{children}
			</div>
		</AvatarContext.Provider>
	);
}

export function AvatarImage({
	src,
	alt,
	...props
}: { src: string; alt: string } & React.HTMLAttributes<HTMLImageElement>) {
	const size = useContext(AvatarContext);

	return (
		<img
			src={src}
			alt={alt}
			{...props}
			className={cn("size-fit", radiusClasses[size], props.className)}
		/>
	);
}

export function AvatarFallback({
	children,
	...props
}: { children: React.ReactNode } & React.HTMLAttributes<HTMLDivElement>) {
	const size = useContext(AvatarContext);

	return (
		<div
			{...props}
			className={cn(
				"flex size-full items-center justify-center bg-brand-600 text-center text-white",
				radiusClasses[size],
				props.className,
			)}
		>
			{children}
		</div>
	);
}

export function AvatarBadge({
	...props
}: React.HTMLAttributes<HTMLDivElement>) {
	const size = useContext(AvatarContext);

	return (
		<div
			{...props}
			className={cn(
				"absolute bottom-0 left-0 outline-2 outline-background",
				badgeSizeClasses[size],
				badgeRadiusClasses[size],
				props.className,
			)}
		></div>
	);
}

export function AvatarGroup({
	children,
	...props
}: {
	children: React.ReactNode;
} & React.HTMLAttributes<HTMLDivElement>) {
	return (
		<div
			{...props}
			className={cn(
				"flex items-center -space-x-2 *:outline-2 *:outline-background",
				props.className,
			)}
		>
			{children}
		</div>
	);
}
